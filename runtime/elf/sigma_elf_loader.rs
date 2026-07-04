// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// runtime/elf/sigma_elf_loader.rs — ELF64 Loader (no_std, cleanroom)
// Language: Rust #![no_std] — OOP via ElfLoader struct

#![no_std]

// ── ELF Constants ─────────────────────────────────────────────────────────────
const ELFMAG:    [u8;4] = [0x7F, b'E', b'L', b'F'];
const ELFCLASS64: u8  = 2;
const ELFDATA2LSB: u8 = 1;
const ET_EXEC:   u16  = 2;
const ET_DYN:    u16  = 3;
const EM_X86_64: u16  = 62;
const EM_AARCH64: u16 = 183;
const PT_LOAD:   u32  = 1;
const PT_DYNAMIC: u32 = 2;
const PT_INTERP:  u32 = 3;
const PT_NOTE:    u32 = 4;
const PF_X:       u32 = 1;
const PF_W:       u32 = 2;
const PF_R:       u32 = 4;

// ── ELF64 Header ──────────────────────────────────────────────────────────────
#[repr(C)]
pub struct Elf64Hdr {
    pub e_ident:     [u8; 16],
    pub e_type:      u16,
    pub e_machine:   u16,
    pub e_version:   u32,
    pub e_entry:     u64,
    pub e_phoff:     u64,
    pub e_shoff:     u64,
    pub e_flags:     u32,
    pub e_ehsize:    u16,
    pub e_phentsize: u16,
    pub e_phnum:     u16,
    pub e_shentsize: u16,
    pub e_shnum:     u16,
    pub e_shstrndx:  u16,
}

// ── ELF64 Program Header ──────────────────────────────────────────────────────
#[repr(C)]
pub struct Elf64Phdr {
    pub p_type:   u32,
    pub p_flags:  u32,
    pub p_offset: u64,
    pub p_vaddr:  u64,
    pub p_paddr:  u64,
    pub p_filesz: u64,
    pub p_memsz:  u64,
    pub p_align:  u64,
}

// ── Load Result ───────────────────────────────────────────────────────────────
#[derive(Debug)]
pub struct LoadedElf {
    pub entry:    u64,
    pub base:     u64,
    pub load_min: u64,
    pub load_max: u64,
    pub interp:   Option<[u8; 128]>, // path to interpreter (dynamic ELF)
}

// ── ELF Loader ────────────────────────────────────────────────────────────────
pub struct ElfLoader;

impl ElfLoader {
    /// Validate ELF magic, class, data, machine
    pub fn validate(data: &[u8], expected_machine: u16) -> Result<(), &'static str> {
        if data.len() < core::mem::size_of::<Elf64Hdr>() {
            return Err("ELF too small");
        }
        let hdr = unsafe { &*(data.as_ptr() as *const Elf64Hdr) };
        if hdr.e_ident[..4] != ELFMAG           { return Err("Bad ELF magic"); }
        if hdr.e_ident[4] != ELFCLASS64          { return Err("Not ELF64"); }
        if hdr.e_ident[5] != ELFDATA2LSB         { return Err("Not little-endian"); }
        if hdr.e_machine != expected_machine      { return Err("Wrong machine"); }
        if hdr.e_type != ET_EXEC && hdr.e_type != ET_DYN { return Err("Not executable or shared"); }
        Ok(())
    }

    /// Parse program headers and return load info
    pub fn parse_load_info(data: &[u8]) -> Option<LoadedElf> {
        if data.len() < core::mem::size_of::<Elf64Hdr>() { return None; }
        let hdr = unsafe { &*(data.as_ptr() as *const Elf64Hdr) };

        let mut load_min = u64::MAX;
        let mut load_max = 0u64;
        let mut interp: Option<[u8; 128]> = None;

        for i in 0..hdr.e_phnum as usize {
            let off = hdr.e_phoff as usize + i * hdr.e_phentsize as usize;
            if off + core::mem::size_of::<Elf64Phdr>() > data.len() { break; }
            let phdr = unsafe { &*(data.as_ptr().add(off) as *const Elf64Phdr) };

            match phdr.p_type {
                PT_LOAD => {
                    if phdr.p_vaddr < load_min { load_min = phdr.p_vaddr; }
                    let end = phdr.p_vaddr + phdr.p_memsz;
                    if end > load_max { load_max = end; }
                }
                PT_INTERP => {
                    let start = phdr.p_offset as usize;
                    let len   = (phdr.p_filesz as usize).min(127);
                    if start + len <= data.len() {
                        let mut path = [0u8; 128];
                        path[..len].copy_from_slice(&data[start..start+len]);
                        interp = Some(path);
                    }
                }
                _ => {}
            }
        }
        if load_min == u64::MAX { return None; }
        Some(LoadedElf {
            entry: hdr.e_entry, base: load_min,
            load_min, load_max, interp,
        })
    }

    /// Map all PT_LOAD segments into memory via a physical allocator
    /// `alloc_pages`: fn(virt_addr, page_count, writable, executable) -> bool
    pub fn map_segments<F>(data: &[u8], base_offset: u64, alloc: F) -> bool
    where F: Fn(u64, u64, bool, bool) -> bool
    {
        if data.len() < core::mem::size_of::<Elf64Hdr>() { return false; }
        let hdr = unsafe { &*(data.as_ptr() as *const Elf64Hdr) };

        for i in 0..hdr.e_phnum as usize {
            let off = hdr.e_phoff as usize + i * hdr.e_phentsize as usize;
            if off + core::mem::size_of::<Elf64Phdr>() > data.len() { return false; }
            let phdr = unsafe { &*(data.as_ptr().add(off) as *const Elf64Phdr) };
            if phdr.p_type != PT_LOAD { continue; }

            let vaddr   = phdr.p_vaddr + base_offset;
            let pages   = (phdr.p_memsz + 4095) / 4096;
            let write   = phdr.p_flags & PF_W != 0;
            let execute = phdr.p_flags & PF_X != 0;

            if !alloc(vaddr, pages, write, execute) { return false; }

            // Copy file data into mapped region
            let src_off  = phdr.p_offset as usize;
            let src_len  = (phdr.p_filesz as usize).min(data.len() - src_off);
            let dst_ptr  = vaddr as *mut u8;
            unsafe {
                core::ptr::copy_nonoverlapping(data.as_ptr().add(src_off), dst_ptr, src_len);
                // Zero BSS (memsz > filesz)
                if phdr.p_memsz > phdr.p_filesz {
                    core::ptr::write_bytes(
                        dst_ptr.add(src_len),
                        0,
                        (phdr.p_memsz - phdr.p_filesz) as usize,
                    );
                }
            }
        }
        true
    }

    /// Full load: validate + parse + map. Returns entry point or 0 on error.
    pub fn load<F>(data: &[u8], aslr_offset: u64, alloc: F) -> u64
    where F: Fn(u64, u64, bool, bool) -> bool
    {
        let machine = if cfg!(target_arch = "aarch64") { EM_AARCH64 } else { EM_X86_64 };
        if Self::validate(data, machine).is_err() { return 0; }
        let info = match Self::parse_load_info(data) { Some(i) => i, None => return 0 };
        let base = if info.entry < 0x1000 { aslr_offset } else { 0 };
        if !Self::map_segments(data, base, alloc) { return 0; }
        info.entry + base
    }
}
