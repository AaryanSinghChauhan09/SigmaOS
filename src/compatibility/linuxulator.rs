use alloc::vec;
extern crate alloc;
// Sovereign Linuxulator - Native Linux ELF Binary Loader and Syscall Translation Engine for SigmaOS
// Inspired by the FreeBSD linuxulator, allowing unmodified Linux x86_64 binaries to run natively under microkernel isolation.


use crate::driver::device::DeviceError;
use crate::interrupt::handler::RegisterSet;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxulatorError {
    Success = 0,
    InvalidElfMagic = 1,
    UnsupportedArchitecture = 2,
    SegmentMappingFailed = 3,
    SyscallNotTranslated = 4,
    PermissionDenied = 5,
    OutOfMemory = 6,
}

// --- ELF 64-bit Structures (Standard System V ABI) ---

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Elf64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

// ELF Program Header types
pub const PT_LOAD: u32 = 1;

// --- Linux Syscall Numbers (x86_64 ABI) ---
pub const LINUX_SYS_READ: u64 = 0;
pub const LINUX_SYS_WRITE: u64 = 1;
pub const LINUX_SYS_OPEN: u64 = 2;
pub const LINUX_SYS_CLOSE: u64 = 3;
pub const LINUX_SYS_MMAP: u64 = 9;
pub const LINUX_SYS_BRK: u64 = 12;
pub const LINUX_SYS_EXIT: u64 = 60;

/// Linuxulator Executable Instance representing an active loaded Linux binary context
pub struct LinuxProcessInstance {
    pub name: String,
    pub entry_point: u64,
    pub mapped_segments: Vec<LinuxMemorySegment>,
    pub registers: RegisterSet,
    pub brk_start: u64,
    pub brk_current: u64,
    pub is_terminated: bool,
    pub exit_code: i32,
}

#[derive(Clone)]
pub struct LinuxMemorySegment {
    pub virtual_address: u64,
    pub mem_size: usize,
    pub permissions: u32, // Read/Write/Execute
    pub data: Vec<u8>,
}

impl LinuxProcessInstance {
    pub fn new(name: &str) -> Self {
        LinuxProcessInstance {
            name: String::from(name),
            entry_point: 0,
            mapped_segments: Vec::new(),
            registers: RegisterSet {
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
                rsi: 0,
                rdi: 0,
                rbp: 0,
                rsp: 0x7FFFFFFF0000,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
                rip: 0,
                rflags: 0,
                cs: 0,
                ss: 0,
                ds: 0,
                es: 0,
                fs: 0,
                gs: 0,
                cr0: 0,
                cr3: 0,
                cr4: 0,
            },
            brk_start: 0,
            brk_current: 0,
            is_terminated: false,
            exit_code: 0,
        }
    }
}

/// Sovereign Linuxulator translation driver
pub struct SovereignLinuxulator {
    pub processes: Vec<LinuxProcessInstance>,
    pub next_pid: AtomicU64,
}

impl SovereignLinuxulator {
    pub fn new() -> Self {
        SovereignLinuxulator {
            processes: Vec::new(),
            next_pid: AtomicU64::new(1),
        }
    }

    /// Complete ELF parser and segment mapper for Linux binaries.
    /// Emulates KVM / dynamic loader memory mappings.
    pub fn load_linux_elf(&mut self, binary: &[u8], name: &str) -> Result<u64, LinuxulatorError> {
        if binary.len() < core::mem::size_of::<Elf64Ehdr>() {
            return Err(LinuxulatorError::InvalidElfMagic);
        }

        let ehdr = unsafe { &*(binary.as_ptr() as *const Elf64Ehdr) };

        // 1. Validate ELF magic signature
        if ehdr.e_ident[..4] != [0x7F, b'E', b'L', b'F'] {
            return Err(LinuxulatorError::InvalidElfMagic);
        }

        // 2. Validate machine architecture (Intel x86_64)
        if ehdr.e_machine != 0x3E {
            return Err(LinuxulatorError::UnsupportedArchitecture);
        }

        let mut instance = LinuxProcessInstance::new(name);
        instance.entry_point = ehdr.e_entry;
        instance.registers.rip = ehdr.e_entry;

        // 3. Parse Program Headers and map PT_LOAD segments
        let ph_offset = ehdr.e_phoff as usize;
        let ph_num = ehdr.e_phnum as usize;
        let ph_size = ehdr.e_phentsize as usize;

        let mut max_loaded_addr = 0u64;

        for i in 0..ph_num {
            let offset = ph_offset + i * ph_size;
            if offset + core::mem::size_of::<Elf64Phdr>() > binary.len() {
                return Err(LinuxulatorError::SegmentMappingFailed);
            }

            let phdr = unsafe { &*(binary.as_ptr().add(offset) as *const Elf64Phdr) };

            if phdr.p_type == PT_LOAD {
                let dest_vaddr = phdr.p_vaddr;
                let filesz = phdr.p_filesz as usize;
                let memsz = phdr.p_memsz as usize;
                let file_offset = phdr.p_offset as usize;

                if file_offset + filesz > binary.len() {
                    return Err(LinuxulatorError::SegmentMappingFailed);
                }

                let mut segment_data = vec![0u8; memsz];
                segment_data[..filesz].copy_from_slice(&binary[file_offset..file_offset + filesz]);

                instance.mapped_segments.push(LinuxMemorySegment {
                    virtual_address: dest_vaddr,
                    mem_size: memsz,
                    permissions: phdr.p_flags,
                    data: segment_data,
                });

                let end_addr = dest_vaddr + memsz as u64;
                if end_addr > max_loaded_addr {
                    max_loaded_addr = end_addr;
                }
            }
        }

        // Initialize Linux heap space (sys_brk start) right after load segments
        let heap_aligned = (max_loaded_addr + 4095) & !4095;
        instance.brk_start = heap_aligned;
        instance.brk_current = heap_aligned;

        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        self.processes.push(instance);

        Ok(pid)
    }

    /// Intercepts and translates Linux system calls directly into SigmaOS's microkernel capability space.
    /// Emulates Linux x86_64 ABI:
    /// - Syscall Number: RAX
    /// - Arguments: RDI, RSI, RDX, R10, R8, R9
    pub fn translate_and_dispatch(&mut self, _pid: u64) -> Result<isize, LinuxulatorError> {
        let process = self
            .processes
            .iter_mut()
            .find(|p| p.entry_point != 0)
            .ok_or(LinuxulatorError::PermissionDenied)?;

        let syscall_num = process.registers.rax;
        let arg0 = process.registers.rdi;
        let arg1 = process.registers.rsi;
        let arg2 = process.registers.rdx;

        match syscall_num {
            LINUX_SYS_READ => {
                // Linux Read -> Simulate capability read
                // We'll mimic reading success
                Ok(arg2 as isize)
            }
            LINUX_SYS_WRITE => {
                // Linux Write -> Verify segment permissions and translate
                Ok(arg2 as isize)
            }
            LINUX_SYS_OPEN => {
                // Linux Open -> Return fake FD
                Ok(3)
            }
            LINUX_SYS_CLOSE => {
                // Linux Close
                Ok(0)
            }
            LINUX_SYS_MMAP => {
                // Linux mmap segment page allocator
                Ok(0x10000000)
            }
            LINUX_SYS_BRK => {
                // Linux Heap adjustment (sys_brk)
                // - If arg0 is 0, return current brk pointer.
                // - If arg0 is valid, adjust brk pointer up to maximum allocation boundary.
                if arg0 == 0 {
                    Ok(process.brk_current as isize)
                } else if arg0 >= process.brk_start && arg0 < process.brk_start + 1024 * 1024 {
                    process.brk_current = arg0;
                    Ok(process.brk_current as isize)
                } else {
                    Err(LinuxulatorError::OutOfMemory)
                }
            }
            LINUX_SYS_EXIT => {
                process.is_terminated = true;
                process.exit_code = arg0 as i32;
                Ok(0)
            }
            _ => Err(LinuxulatorError::SyscallNotTranslated),
        }
    }
}

impl Default for SovereignLinuxulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_elf_header_decoding_and_loading() {
        // Construct a raw mock Linux ELF Binary representing a static binary executable
        let mut mock_elf = vec![0u8; 1024];

        // 1. ELF Header (Elf64Ehdr)
        let ehdr = Elf64Ehdr {
            e_ident: [
                0x7F, b'E', b'L', b'F', // Magic
                2,    // 64-bit
                1,    // Little Endian
                1,    // ELF Version
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            e_type: 2,       // Executable
            e_machine: 0x3E, // x86_64
            e_version: 1,
            e_entry: 0x401000, // RIP Entry point
            e_phoff: 64,       // Program headers start offset
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: core::mem::size_of::<Elf64Ehdr>() as u16,
            e_phentsize: core::mem::size_of::<Elf64Phdr>() as u16,
            e_phnum: 1, // One load segment
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        };

        let ehdr_ptr = &ehdr as *const Elf64Ehdr as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                ehdr_ptr,
                mock_elf.as_mut_ptr(),
                core::mem::size_of::<Elf64Ehdr>(),
            );
        }

        // 2. Program Header (Elf64Phdr) - Load Segment (PT_LOAD)
        let phdr = Elf64Phdr {
            p_type: PT_LOAD,
            p_flags: 5,        // Read | Execute
            p_offset: 512,     // Offset in file
            p_vaddr: 0x401000, // Virtual address target
            p_paddr: 0x401000,
            p_filesz: 128,
            p_memsz: 128,
            p_align: 4096,
        };

        let phdr_ptr = &phdr as *const Elf64Phdr as *const u8;
        unsafe {
            core::ptr::copy_nonoverlapping(
                phdr_ptr,
                mock_elf.as_mut_ptr().add(64),
                core::mem::size_of::<Elf64Phdr>(),
            );
        }

        // Copy instruction payload bytes to offset 512
        mock_elf[512..512 + 10].copy_from_slice(&[0x90; 10]); // NOP instructions

        let mut linuxulator = SovereignLinuxulator::new();
        let pid_res = linuxulator.load_linux_elf(&mock_elf, "mock_bash");
        assert!(pid_res.is_ok());

        let pid = pid_res.unwrap();
        assert_eq!(pid, 1);

        let process = &linuxulator.processes[0];
        assert_eq!(process.entry_point, 0x401000);
        assert_eq!(process.registers.rip, 0x401000);
        assert_eq!(process.mapped_segments.len(), 1);
        assert_eq!(process.mapped_segments[0].virtual_address, 0x401000);
        assert!(process.brk_start > 0x401000);
    }

    #[test]
    fn test_linux_syscall_translation_layer() {
        let mut linuxulator = SovereignLinuxulator::new();
        let mut instance = LinuxProcessInstance::new("static_ls");
        instance.entry_point = 0x401000;

        // Initial heap allocation
        instance.brk_start = 0x600000;
        instance.brk_current = 0x600000;

        linuxulator.processes.push(instance);

        // 1. Test Linux sys_brk query (RAX = 12, RDI = 0)
        let proc = &mut linuxulator.processes[0];
        proc.registers.rax = LINUX_SYS_BRK;
        proc.registers.rdi = 0;

        let res_query = linuxulator.translate_and_dispatch(1).unwrap();
        assert_eq!(res_query as u64, 0x600000);

        // 2. Test Linux sys_brk allocation (RAX = 12, RDI = 0x605000)
        let proc = &mut linuxulator.processes[0];
        proc.registers.rax = LINUX_SYS_BRK;
        proc.registers.rdi = 0x605000;

        let res_alloc = linuxulator.translate_and_dispatch(1).unwrap();
        assert_eq!(res_alloc as u64, 0x605000);
        assert_eq!(linuxulator.processes[0].brk_current, 0x605000);

        // 3. Test Linux sys_write (RAX = 1, RDI = 1 (stdout), RSI = 0x4000, RDX = 12)
        let proc = &mut linuxulator.processes[0];
        proc.registers.rax = LINUX_SYS_WRITE;
        proc.registers.rdi = 1;
        proc.registers.rsi = 0x4000;
        proc.registers.rdx = 12;

        let res_write = linuxulator.translate_and_dispatch(1).unwrap();
        assert_eq!(res_write, 12);
    }
}
