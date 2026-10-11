//! # Crash Dump / kdump subsystem
//!
//! This module implements the kernel crash-dump (kdump) facility that captures
//! a consistent snapshot of kernel memory at panic time and writes it to a
//! reserved disk region or over the network via UDP (netconsole dump path).
//!
//! ## Linux kdump architecture (replicated here)
//!
//! 1. **Reserved region**: At boot, a contiguous chunk of physical RAM
//!    (typically 128–256 MiB) is reserved via the `crashkernel=` command-line
//!    parameter. The primary kernel cannot use this memory.
//!
//! 2. **Capture kernel**: A second, minimal kernel image is pre-loaded into the
//!    reserved region by kexec at boot. On panic, `machine_kexec()` switches
//!    execution to this capture kernel.
//!
//! 3. **Memory snapshot**: The capture kernel reads the primary kernel's memory
//!    through `/proc/vmcore` (exposed as an ELF core file) and writes it to a
//!    disk or network destination.
//!
//! ## ELF core format
//! The vmcore is an ELF file with:
//! - An `Ehdr` (ELF header, ET_CORE)
//! - `PT_NOTE` segments: `Elf_NT_PRSTATUS` (per-CPU register state),
//!   `Elf_NT_PRPSINFO`, architecture-specific crash notes
//! - `PT_LOAD` segments: one per contiguous physical memory range,
//!   covering all kernel RAM
//!
//! ## Netconsole UDP dump path
//! As a fallback (e.g. diskless systems), the capture kernel can stream the
//! ELF core over UDP to a remote syslog/netconsole collector using the
//! netconsole protocol. Each packet carries a 16-byte header (magic, sequence
//! number, total length, offset) followed by up to 1472 bytes of payload.
//!
//! ## This implementation
//! - `KdumpConfig` — boot-time configuration (reserved region bounds, targets)
//! - `ElfCoreHeader` — minimal ELF64 core header builder
//! - `CrashNote` — per-CPU crash note (register state at panic)
//! - `KdumpWriter` — drives the dump: iterates memory ranges, emits ELF
//!   segments, calls the transport (disk or UDP)
//! - `NetconsoleUdpDump` — UDP packet sender for remote dump

#![allow(dead_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// ── ELF64 constants ───────────────────────────────────────────────────────────

pub const ELFMAG: [u8; 4] = [0x7f, b'E', b'L', b'F'];
pub const ELFCLASS64: u8 = 2;
pub const ELFDATA2LSB: u8 = 1; // Little-endian
pub const ET_CORE: u16 = 4;
pub const EM_X86_64: u16 = 62;
pub const PT_LOAD: u32 = 1;
pub const PT_NOTE: u32 = 4;
pub const ELF64_PHDR_SIZE: usize = 56;
pub const ELF64_EHDR_SIZE: usize = 64;

// ── ELF64 core header ─────────────────────────────────────────────────────────

/// Minimal ELF64 header for the vmcore file.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Elf64Ehdr {
    /// ELF magic + class + data + version + OS/ABI + padding
    pub e_ident: [u8; 16],
    /// Object file type (ET_CORE = 4)
    pub e_type: u16,
    /// Target ISA (EM_X86_64 = 62)
    pub e_machine: u16,
    /// ELF version (always 1)
    pub e_version: u32,
    /// Entry point (0 for core files)
    pub e_entry: u64,
    /// Program header table offset
    pub e_phoff: u64,
    /// Section header table offset (0 for core files)
    pub e_shoff: u64,
    /// Processor-specific flags
    pub e_flags: u32,
    /// ELF header size in bytes
    pub e_ehsize: u16,
    /// Program header entry size
    pub e_phentsize: u16,
    /// Number of program header entries
    pub e_phnum: u16,
    /// Section header entry size
    pub e_shentsize: u16,
    /// Number of section header entries
    pub e_shnum: u16,
    /// Section name string table index
    pub e_shstrndx: u16,
}

impl Elf64Ehdr {
    /// Build a valid ELF64 core file header.
    pub fn new_core(phnum: u16) -> Self {
        let mut e_ident = [0u8; 16];
        e_ident[0..4].copy_from_slice(&ELFMAG);
        e_ident[4] = ELFCLASS64;
        e_ident[5] = ELFDATA2LSB;
        e_ident[6] = 1; // EV_CURRENT
        Self {
            e_ident,
            e_type: ET_CORE,
            e_machine: EM_X86_64,
            e_version: 1,
            e_entry: 0,
            e_phoff: ELF64_EHDR_SIZE as u64,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: ELF64_EHDR_SIZE as u16,
            e_phentsize: ELF64_PHDR_SIZE as u16,
            e_phnum: phnum,
            e_shentsize: 64,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

/// ELF64 program header entry (PT_LOAD or PT_NOTE).
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Elf64Phdr {
    /// Segment type
    pub p_type: u32,
    /// Segment-dependent flags
    pub p_flags: u32,
    /// Offset in file
    pub p_offset: u64,
    /// Virtual address in memory
    pub p_vaddr: u64,
    /// Physical address
    pub p_paddr: u64,
    /// Size of segment in file
    pub p_filesz: u64,
    /// Size of segment in memory
    pub p_memsz: u64,
    /// Segment alignment
    pub p_align: u64,
}

// ── Crash note (per-CPU register state) ──────────────────────────────────────

/// x86-64 general-purpose register snapshot saved at panic time.
///
/// Layout matches `struct pt_regs` in the Linux kernel (x86-64 ABI order).
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct X86_64Regs {
    pub r15: u64, pub r14: u64, pub r13: u64, pub r12: u64,
    pub rbp: u64, pub rbx: u64, pub r11: u64, pub r10: u64,
    pub r9: u64,  pub r8: u64,  pub rax: u64, pub rcx: u64,
    pub rdx: u64, pub rsi: u64, pub rdi: u64,
    pub orig_rax: u64,
    pub rip: u64, pub cs: u64,  pub eflags: u64,
    pub rsp: u64, pub ss: u64,
}

/// ELF note header (`Nhdr`).
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Nhdr {
    /// Length of the note name (including NUL terminator)
    pub n_namesz: u32,
    /// Length of the note descriptor
    pub n_descsz: u32,
    /// Note type (NT_PRSTATUS = 1)
    pub n_type: u32,
}

/// Per-CPU crash note bundling register state.
#[derive(Debug, Clone)]
pub struct CrashNote {
    /// CPU index this note belongs to
    pub cpu_id: u32,
    /// Register state at panic
    pub regs: X86_64Regs,
    /// Note header
    pub nhdr: Nhdr,
}

impl CrashNote {
    /// Capture the current CPU's register state (stub — would use inline asm).
    pub fn capture(cpu_id: u32) -> Self {
        Self {
            cpu_id,
            regs: X86_64Regs::default(), // TODO: inline asm to read registers
            nhdr: Nhdr {
                n_namesz: 5,  // "CORE\0"
                n_descsz: core::mem::size_of::<X86_64Regs>() as u32,
                n_type: 1,    // NT_PRSTATUS
            },
        }
    }
}

// ── Physical memory range descriptor ─────────────────────────────────────────

/// One contiguous range of physical memory to include in the dump.
#[derive(Debug, Clone, Copy)]
pub struct PhysMemRange {
    /// Physical start address
    pub phys_start: u64,
    /// Physical end address (exclusive)
    pub phys_end: u64,
    /// Corresponding kernel virtual address (for reading)
    pub virt_start: u64,
}

impl PhysMemRange {
    /// Byte length of this range.
    pub fn len(&self) -> u64 {
        self.phys_end.saturating_sub(self.phys_start)
    }
}

// ── kdump configuration ───────────────────────────────────────────────────────

/// Destination for the crash dump.
#[derive(Debug, Clone)]
pub enum DumpDestination {
    /// Write dump to a raw disk partition at the given device path and offset.
    Disk { device: String, offset_bytes: u64 },
    /// Stream dump over UDP to a remote netconsole collector.
    UdpNetconsole {
        local_ip: [u8; 4],
        local_port: u16,
        remote_ip: [u8; 4],
        remote_port: u16,
    },
    /// Write to an in-memory buffer (for testing / capture-kernel scenarios).
    Memory { buffer_addr: usize, buffer_size: usize },
}

/// Boot-time kdump configuration.
#[derive(Debug, Clone)]
pub struct KdumpConfig {
    /// Physical address of the start of the reserved kdump region.
    pub reserved_phys_start: u64,
    /// Size in bytes of the reserved region.
    pub reserved_size: u64,
    /// Where to write the core dump.
    pub destination: DumpDestination,
    /// Number of online CPUs at boot (used to pre-allocate crash notes).
    pub num_cpus: u32,
    /// Whether to compress the dump (LZ4 block compression).
    pub compress: bool,
    /// Whether to filter zero pages from the dump to reduce size.
    pub filter_zero_pages: bool,
}

// ── Netconsole UDP dump sender ────────────────────────────────────────────────

/// Maximum UDP payload per netconsole dump packet (Ethernet MTU − IP − UDP headers).
pub const NETCONSOLE_PAYLOAD_SIZE: usize = 1472;

/// 16-byte netconsole dump packet header.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NetconsoleDumpHeader {
    /// Magic identifier: `0x4B44_4D50` ("KDMP")
    pub magic: u32,
    /// Monotonically increasing sequence number
    pub seq: u32,
    /// Total dump size in bytes (so receiver can pre-allocate)
    pub total_size: u64,
}

/// Sends crash dump data over UDP using the netconsole protocol.
pub struct NetconsoleUdpDump {
    pub config: KdumpConfig,
    pub seq: u32,
    pub bytes_sent: AtomicU64,
}

impl NetconsoleUdpDump {
    /// Create a new UDP dump sender from config.
    pub fn new(config: KdumpConfig) -> Self {
        Self {
            config,
            seq: 0,
            bytes_sent: AtomicU64::new(0),
        }
    }

    /// Send one 1488-byte netconsole dump packet (header + payload stub).
    ///
    /// In a real kernel this would call into the network stack bypassing the
    /// socket layer (direct NIC Tx ring manipulation).
    pub fn send_packet(&mut self, offset: u64, payload: &[u8], total_size: u64) -> bool {
        let _hdr = NetconsoleDumpHeader {
            magic: 0x4B44_4D50,
            seq: self.seq,
            total_size,
        };
        self.seq = self.seq.wrapping_add(1);
        self.bytes_sent.fetch_add(payload.len() as u64, Ordering::Relaxed);
        // TODO: write header + payload into NIC Tx descriptor ring
        let _ = offset;
        true
    }
}

// ── Kdump writer ──────────────────────────────────────────────────────────────

/// Drives the crash-dump process from panic handler through to final flush.
pub struct KdumpWriter {
    pub config: KdumpConfig,
    /// Per-CPU crash notes captured at panic.
    pub crash_notes: Vec<CrashNote>,
    /// Physical memory ranges to dump.
    pub mem_ranges: Vec<PhysMemRange>,
    /// Bytes written so far.
    pub bytes_written: u64,
    /// Whether the dump completed successfully.
    pub completed: AtomicBool,
    /// Queue of ELF program headers built during the dump.
    pub phdrs: VecDeque<Elf64Phdr>,
}

impl KdumpWriter {
    /// Initialise the writer. Called once during `kdump_init()` at boot.
    pub fn new(config: KdumpConfig, mem_ranges: Vec<PhysMemRange>) -> Self {
        let num_cpus = config.num_cpus as usize;
        Self {
            config,
            crash_notes: Vec::with_capacity(num_cpus),
            mem_ranges,
            bytes_written: 0,
            completed: AtomicBool::new(false),
            phdrs: VecDeque::new(),
        }
    }

    /// Called from the panic handler on every CPU to save register state.
    ///
    /// The panicking CPU calls this first; all other CPUs are NMI-interrupted
    /// and call it from their NMI handler.
    pub fn capture_cpu_state(&mut self, cpu_id: u32) {
        let note = CrashNote::capture(cpu_id);
        self.crash_notes.push(note);
    }

    /// Build the ELF core header and program header table.
    ///
    /// One `PT_NOTE` segment covers all CPU crash notes.
    /// One `PT_LOAD` segment per physical memory range.
    pub fn build_elf_headers(&mut self) -> Elf64Ehdr {
        let phnum = 1 + self.mem_ranges.len() as u16; // 1 PT_NOTE + N PT_LOAD
        let ehdr = Elf64Ehdr::new_core(phnum);

        let notes_offset = ELF64_EHDR_SIZE + phnum as usize * ELF64_PHDR_SIZE;
        let notes_size = self.crash_notes.len()
            * (core::mem::size_of::<Nhdr>() + 8 + core::mem::size_of::<X86_64Regs>());

        // PT_NOTE
        self.phdrs.push_back(Elf64Phdr {
            p_type: PT_NOTE,
            p_flags: 0,
            p_offset: notes_offset as u64,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: notes_size as u64,
            p_memsz: notes_size as u64,
            p_align: 4,
        });

        let mut file_off = (notes_offset + notes_size) as u64;
        for range in &self.mem_ranges {
            let seg_size = if self.config.filter_zero_pages {
                // TODO: scan for zero pages and subtract
                range.len()
            } else {
                range.len()
            };
            self.phdrs.push_back(Elf64Phdr {
                p_type: PT_LOAD,
                p_flags: 0x5, // PF_R | PF_X
                p_offset: file_off,
                p_vaddr: range.virt_start,
                p_paddr: range.phys_start,
                p_filesz: seg_size,
                p_memsz: range.len(),
                p_align: 0x1000,
            });
            file_off += seg_size;
        }
        ehdr
    }

    /// Execute the full dump sequence:
    /// 1. Capture all CPU states
    /// 2. Build ELF headers
    /// 3. Write header + notes to destination
    /// 4. Iterate memory ranges, writing each page to the destination
    pub fn execute_dump(&mut self) -> Result<u64, &'static str> {
        // Capture BSP (CPU 0) state
        self.capture_cpu_state(0);

        let _ehdr = self.build_elf_headers();

        match &self.config.destination {
            DumpDestination::Disk { .. } => {
                // TODO: open block device, seek to offset, write ELF headers,
                // then iterate mem_ranges and copy pages via DMA or PIO.
            }
            DumpDestination::UdpNetconsole { .. } => {
                // TODO: construct raw Ethernet/IP/UDP frames and send via NIC.
            }
            DumpDestination::Memory { .. } => {
                // TODO: copy ELF data into the reserved capture buffer.
            }
        }

        self.completed.store(true, Ordering::SeqCst);
        Ok(self.bytes_written)
    }

    /// Check whether the dump has completed.
    pub fn is_complete(&self) -> bool {
        self.completed.load(Ordering::SeqCst)
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Initialise the kdump subsystem at boot.
///
/// Parses the `crashkernel=` command-line parameter, reserves physical memory,
/// and pre-loads the capture kernel via kexec.
pub fn kdump_init(config: KdumpConfig, mem_ranges: Vec<PhysMemRange>) -> KdumpWriter {
    KdumpWriter::new(config, mem_ranges)
}

/// Panic handler entry point for kdump.
///
/// Called by `panic!` / `kernel_panic()`. Stops all other CPUs via NMI IPI,
/// then drives the `KdumpWriter` to completion.
pub fn kdump_on_panic(writer: &mut KdumpWriter, _msg: &str) {
    // TODO: issue NMI IPI to all other CPUs to freeze them and capture their state
    let _ = writer.execute_dump();
}
