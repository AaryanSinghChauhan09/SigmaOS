// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/linux_compat/elf_loader.rs — Linux ELF64 loader for linuxulator
//
// Parses and loads Linux ELF64 (x86-64) executables into the linux-compat
// address space, sets up auxv/stack per Linux ABI, then hands off to the
// entry point under the syscall translation layer.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

// ── ELF64 constants ───────────────────────────────────────────────────────
const ELFMAG:       [u8; 4] = [0x7f, b'E', b'L', b'F'];
const ELFCLASS64:   u8 = 2;
const ELFDATA2LSB:  u8 = 1;  // Little-endian
const ET_EXEC:      u16 = 2; // Executable
const ET_DYN:       u16 = 3; // Shared object (PIE)
const EM_X86_64:    u16 = 62;
const PT_LOAD:      u32 = 1;
const PT_INTERP:    u32 = 3;
const PT_DYNAMIC:   u32 = 2;
const PT_GNU_STACK: u32 = 0x6474e551;
const PT_PHDR:      u32 = 6;
const PT_TLS:       u32 = 7;

const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;

// ── ELF64 header ─────────────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct Elf64Hdr {
    e_ident:     [u8; 16],
    e_type:      u16,
    e_machine:   u16,
    e_version:   u32,
    e_entry:     u64,
    e_phoff:     u64,
    e_shoff:     u64,
    e_flags:     u32,
    e_ehsize:    u16,
    e_phentsize: u16,
    e_phnum:     u16,
    e_shentsize: u16,
    e_shnum:     u16,
    e_shstrndx:  u16,
}

// ── ELF64 program header ──────────────────────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct Elf64Phdr {
    p_type:   u32,
    p_flags:  u32,
    p_offset: u64,
    p_vaddr:  u64,
    p_paddr:  u64,
    p_filesz: u64,
    p_memsz:  u64,
    p_align:  u64,
}

// ── Loaded segment ────────────────────────────────────────────────────────
const MAX_SEGMENTS: usize = 16;

#[derive(Copy, Clone)]
pub struct LoadedSegment {
    pub vaddr:   u64,
    pub size:    u64,
    pub perms:   u32, // PF_R | PF_W | PF_X
}

// ── Loaded image result ───────────────────────────────────────────────────
pub struct LoadedElf {
    pub entry_point:  u64,
    pub base_addr:    u64,  // actual load base (PIE: randomized)
    pub load_bias:    i64,  // base - preferred base
    pub interp_path:  [u8; 256],
    pub interp_len:   usize,
    pub segments:     [Option<LoadedSegment>; MAX_SEGMENTS],
    pub seg_count:    usize,
    pub stack_exec:   bool,  // PT_GNU_STACK flags
    pub tls_base:     u64,
    pub phdr_vaddr:   u64,
    pub phdr_count:   u16,
}

impl LoadedElf {
    pub const fn zeroed() -> Self {
        Self {
            entry_point: 0, base_addr: 0, load_bias: 0,
            interp_path: [0u8; 256], interp_len: 0,
            segments: [const { None }; MAX_SEGMENTS],
            seg_count: 0, stack_exec: false,
            tls_base: 0, phdr_vaddr: 0, phdr_count: 0,
        }
    }
}

// ── ELF load errors ───────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ElfError {
    NotElf,
    Not64Bit,
    NotLittleEndian,
    NotExecutable,
    NotX86_64,
    TruncatedHeader,
    TooManySegments,
    AllocFailed,
}

pub type ElfResult<T> = Result<T, ElfError>;

// ── ASLR seed (simple xorshift) ───────────────────────────────────────────
static mut ASLR_STATE: u64 = 0x9e37_79b9_7f4a_7c15;

unsafe fn aslr_next() -> u64 {
    ASLR_STATE ^= ASLR_STATE << 13;
    ASLR_STATE ^= ASLR_STATE >> 7;
    ASLR_STATE ^= ASLR_STATE << 17;
    ASLR_STATE
}

fn page_align_up(v: u64) -> u64 { (v + 0xFFF) & !0xFFF }
fn page_align_dn(v: u64) -> u64 { v & !0xFFF }

// ── ELF loader ────────────────────────────────────────────────────────────
pub struct ElfLoader;

impl ElfLoader {
    /// Load an ELF64 binary from a byte slice.
    /// `load_base` = 0 → use ASLR randomization.
    pub unsafe fn load(data: &[u8], load_base: u64) -> ElfResult<LoadedElf> {
        if data.len() < core::mem::size_of::<Elf64Hdr>() {
            return Err(ElfError::TruncatedHeader);
        }

        let hdr = &*(data.as_ptr() as *const Elf64Hdr);

        // Validate ELF magic
        if { hdr.e_ident }[..4] != ELFMAG { return Err(ElfError::NotElf); }
        if { hdr.e_ident }[4] != ELFCLASS64 { return Err(ElfError::Not64Bit); }
        if { hdr.e_ident }[5] != ELFDATA2LSB { return Err(ElfError::NotLittleEndian); }
        let e_type = { hdr.e_type };
        if e_type != ET_EXEC && e_type != ET_DYN { return Err(ElfError::NotExecutable); }
        if { hdr.e_machine } != EM_X86_64 { return Err(ElfError::NotX86_64); }

        let phoff    = { hdr.e_phoff } as usize;
        let phentsize= { hdr.e_phentsize } as usize;
        let phnum    = { hdr.e_phnum } as usize;

        // Compute load bias for PIE (ET_DYN)
        let preferred_base = if e_type == ET_DYN { 0u64 } else {
            // Find lowest PT_LOAD vaddr
            let mut low = u64::MAX;
            for i in 0..phnum {
                let off = phoff + i * phentsize;
                if off + phentsize > data.len() { break; }
                let ph = &*(data.as_ptr().add(off) as *const Elf64Phdr);
                if { ph.p_type } == PT_LOAD && { ph.p_vaddr } < low {
                    low = { ph.p_vaddr };
                }
            }
            if low == u64::MAX { 0 } else { low }
        };

        let actual_base = if load_base != 0 {
            load_base
        } else if e_type == ET_DYN {
            // ASLR: random base in user space (1 GB - 127 GB range)
            0x4000_0000 + (aslr_next() & 0x3FFF_F000)
        } else {
            preferred_base
        };

        let load_bias = actual_base.wrapping_sub(preferred_base) as i64;

        let mut image = LoadedElf::zeroed();
        image.base_addr    = actual_base;
        image.load_bias    = load_bias;
        image.entry_point  = ({ hdr.e_entry } as i64 + load_bias) as u64;
        image.phdr_count   = { hdr.e_phnum };

        // Allocate memory helper
        extern "C" { fn sigma_slab_alloc(size: usize) -> *mut u8; }

        // Process program headers
        for i in 0..phnum {
            let off = phoff + i * phentsize;
            if off + phentsize > data.len() { break; }
            let ph = &*(data.as_ptr().add(off) as *const Elf64Phdr);

            match { ph.p_type } {
                PT_LOAD => {
                    if image.seg_count >= MAX_SEGMENTS {
                        return Err(ElfError::TooManySegments);
                    }
                    let seg_vaddr = ({ ph.p_vaddr } as i64 + load_bias) as u64;
                    let seg_size  = page_align_up({ ph.p_memsz });
                    let flags     = { ph.p_flags };

                    // W^X: never map exec+write
                    let perms = if flags & PF_X != 0 {
                        flags & !PF_W  // strip write from exec segments
                    } else { flags };

                    // Map segment: allocate and copy
                    let dst = sigma_slab_alloc(seg_size as usize);
                    if dst.is_null() { return Err(ElfError::AllocFailed); }
                    core::ptr::write_bytes(dst, 0, seg_size as usize);

                    let src_off  = { ph.p_offset } as usize;
                    let src_size = { ph.p_filesz } as usize;
                    if src_off + src_size <= data.len() {
                        core::ptr::copy_nonoverlapping(
                            data.as_ptr().add(src_off), dst, src_size
                        );
                    }

                    image.segments[image.seg_count] = Some(LoadedSegment {
                        vaddr: seg_vaddr,
                        size:  seg_size,
                        perms,
                    });
                    image.seg_count += 1;

                    // PHDR segment: record for auxv
                    if seg_vaddr <= (({ hdr.e_phoff } as i64 + load_bias) as u64)
                        && (({ hdr.e_phoff } as i64 + load_bias) as u64) < seg_vaddr + seg_size
                    {
                        image.phdr_vaddr = (hdr.e_phoff as i64 + load_bias) as u64;
                    }
                }
                PT_INTERP => {
                    // Read interpreter path (e.g. /lib64/ld-linux-x86-64.so.2)
                    let off2  = { ph.p_offset } as usize;
                    let fsize = { ph.p_filesz } as usize;
                    let copy  = fsize.min(255);
                    if off2 + copy <= data.len() {
                        core::ptr::copy_nonoverlapping(
                            data.as_ptr().add(off2),
                            image.interp_path.as_mut_ptr(), copy
                        );
                        image.interp_len = copy;
                    }
                }
                PT_GNU_STACK => {
                    image.stack_exec = { ph.p_flags } & PF_X != 0;
                }
                PT_TLS => {
                    image.tls_base = ({ ph.p_vaddr } as i64 + load_bias) as u64;
                }
                _ => {}
            }
        }

        Ok(image)
    }

    /// Set up the Linux-ABI initial stack:
    /// [argc][argv ptrs...][NULL][envp ptrs...][NULL][auxv...][NULL]
    pub unsafe fn setup_stack(
        stack_top: *mut u64,
        stack_size: usize,
        image: &LoadedElf,
        argv: &[*const u8],
        envp: &[*const u8],
    ) -> *mut u64 {
        // AT_* auxv types
        const AT_PHDR:    u64 = 3;
        const AT_PHENT:   u64 = 4;
        const AT_PHNUM:   u64 = 5;
        const AT_PAGESZ:  u64 = 6;
        const AT_ENTRY:   u64 = 9;
        const AT_UID:     u64 = 11;
        const AT_EUID:    u64 = 12;
        const AT_GID:     u64 = 13;
        const AT_EGID:    u64 = 14;
        const AT_RANDOM:  u64 = 25;
        const AT_NULL:    u64 = 0;

        let mut sp = stack_top.sub(1);

        // Auxv (in reverse order — we build top-down)
        let auxv: [(u64, u64); 11] = [
            (AT_NULL,  0),
            (AT_RANDOM, sp as u64),         // 16 random bytes
            (AT_EGID,  0), (AT_GID,  0),
            (AT_EUID,  0), (AT_UID,  0),
            (AT_ENTRY, image.entry_point),
            (AT_PAGESZ, 4096),
            (AT_PHNUM, image.phdr_count as u64),
            (AT_PHENT, core::mem::size_of::<Elf64Phdr>() as u64),
            (AT_PHDR,  image.phdr_vaddr),
        ];

        for (tag, val) in auxv.iter().rev() {
            sp = sp.sub(1); sp.write(*val);
            sp = sp.sub(1); sp.write(*tag);
        }

        // NULL terminator for envp
        sp = sp.sub(1); sp.write(0);
        // envp pointers
        for ptr in envp.iter().rev() {
            sp = sp.sub(1); sp.write(*ptr as u64);
        }
        // NULL terminator for argv
        sp = sp.sub(1); sp.write(0);
        // argv pointers
        for ptr in argv.iter().rev() {
            sp = sp.sub(1); sp.write(*ptr as u64);
        }
        // argc
        sp = sp.sub(1); sp.write(argv.len() as u64);

        sp
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_elf_load(
    data: *const u8, len: usize, load_base: u64,
    out: *mut LoadedElf,
) -> i32 {
    if data.is_null() || out.is_null() { return -22; }
    let slice = core::slice::from_raw_parts(data, len);
    match ElfLoader::load(slice, load_base) {
        Ok(img)  => { *out = img; 0 }
        Err(ElfError::NotElf)           => -1,
        Err(ElfError::Not64Bit)         => -2,
        Err(ElfError::NotLittleEndian)  => -3,
        Err(ElfError::NotExecutable)    => -4,
        Err(ElfError::NotX86_64)        => -5,
        Err(ElfError::TruncatedHeader)  => -6,
        Err(ElfError::TooManySegments)  => -7,
        Err(ElfError::AllocFailed)      => -12,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_elf_validate(data: *const u8, len: usize) -> i32 {
    if data.is_null() { return -22; }
    let slice = core::slice::from_raw_parts(data, len);
    match ElfLoader::load(slice, 0) {
        Ok(img) => img.seg_count as i32,
        Err(_)  => -1,
    }
}
