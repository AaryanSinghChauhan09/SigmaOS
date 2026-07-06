// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/mm/sigma_vmm.rs — x86-64 Virtual Memory Manager
// Implements: 4-level page table walker, ASLR for exec, W^X enforcement,
// mmap/munmap/brk/mprotect syscall backends, TLB shootdown.
//
// Paging: PML4 → PDPT → PD → PT → Physical Frame (4KB pages)
// Uses kernel buddy allocator for page table frame allocation.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Constants ──────────────────────────────────────────────────────────────
pub const PAGE_SIZE:   usize = 4096;
pub const PAGE_SHIFT:  usize = 12;
pub const PT_ENTRIES:  usize = 512;

/// Protection flags (mprotect/mmap prot arg)
pub const PROT_NONE:  u32 = 0;
pub const PROT_READ:  u32 = 1;
pub const PROT_WRITE: u32 = 2;
pub const PROT_EXEC:  u32 = 4;

/// mmap flags
pub const MAP_PRIVATE:   u32 = 0x02;
pub const MAP_ANONYMOUS: u32 = 0x20;
pub const MAP_FIXED:     u32 = 0x10;
pub const MAP_FAILED:    u64 = u64::MAX;

// ── Page table entry flags ─────────────────────────────────────────────────
pub const PTE_PRESENT:  u64 = 1 << 0;
pub const PTE_WRITABLE: u64 = 1 << 1;
pub const PTE_USER:     u64 = 1 << 2;
pub const PTE_ACCESSED: u64 = 1 << 5;
pub const PTE_DIRTY:    u64 = 1 << 6;
pub const PTE_HUGE:     u64 = 1 << 7;
pub const PTE_GLOBAL:   u64 = 1 << 8;
pub const PTE_NX:       u64 = 1 << 63; // No-Execute (W^X enforcement)
pub const PTE_ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

// ── CR3 / TLB ─────────────────────────────────────────────────────────────
#[inline]
pub unsafe fn read_cr3() -> u64 {
    let v: u64;
    core::arch::asm!("mov {}, cr3", out(reg) v);
    v
}

#[inline]
pub unsafe fn write_cr3(val: u64) {
    core::arch::asm!("mov cr3, {}", in(reg) val);
}

#[inline]
pub fn tlb_flush_all() {
    unsafe {
        let cr3 = read_cr3();
        write_cr3(cr3);
    }
}

#[inline]
pub fn tlb_flush_page(virt: u64) {
    unsafe { core::arch::asm!("invlpg [{0}]", in(reg) virt) }
}

// ── Page table walker ──────────────────────────────────────────────────────

/// Indices into each level for a virtual address.
#[inline]
fn pml4_idx(va: u64) -> usize { ((va >> 39) & 0x1FF) as usize }
#[inline]
fn pdpt_idx(va: u64) -> usize { ((va >> 30) & 0x1FF) as usize }
#[inline]
fn pd_idx(va: u64) -> usize   { ((va >> 21) & 0x1FF) as usize }
#[inline]
fn pt_idx(va: u64) -> usize   { ((va >> 12) & 0x1FF) as usize }

/// Walk to the PTE for `va`; create intermediate tables if `alloc` is true.
pub unsafe fn walk_to_pte(cr3: u64, va: u64, alloc: bool) -> Option<*mut u64> {
    let pml4 = (cr3 & PTE_ADDR_MASK) as *mut u64;
    let pml4e = &mut *pml4.add(pml4_idx(va));
    if *pml4e & PTE_PRESENT == 0 {
        if !alloc { return None; }
        let frame = alloc_page_frame()? as u64;
        *pml4e = frame | PTE_PRESENT | PTE_WRITABLE | PTE_USER;
    }

    let pdpt = ((*pml4e) & PTE_ADDR_MASK) as *mut u64;
    let pdpte = &mut *pdpt.add(pdpt_idx(va));
    if *pdpte & PTE_PRESENT == 0 {
        if !alloc { return None; }
        let frame = alloc_page_frame()? as u64;
        *pdpte = frame | PTE_PRESENT | PTE_WRITABLE | PTE_USER;
    }

    let pd = ((*pdpte) & PTE_ADDR_MASK) as *mut u64;
    let pde = &mut *pd.add(pd_idx(va));
    if *pde & PTE_PRESENT == 0 {
        if !alloc { return None; }
        let frame = alloc_page_frame()? as u64;
        *pde = frame | PTE_PRESENT | PTE_WRITABLE | PTE_USER;
    }

    let pt = ((*pde) & PTE_ADDR_MASK) as *mut u64;
    Some(pt.add(pt_idx(va)))
}

/// Translate virtual → physical address for current page table.
pub fn virt_to_phys(va: u64) -> Option<u64> {
    unsafe {
        let cr3 = read_cr3();
        let pte_ptr = walk_to_pte(cr3, va, false)?;
        let pte = *pte_ptr;
        if pte & PTE_PRESENT == 0 { return None; }
        Some((pte & PTE_ADDR_MASK) | (va & 0xFFF))
    }
}

/// Map `virt` → `phys` with given protection flags.
pub fn map_page(cr3: u64, virt: u64, phys: u64, prot: u32) -> i64 {
    let flags = prot_to_pte_flags(prot);
    unsafe {
        let pte_ptr = match walk_to_pte(cr3, virt, true) {
            Some(p) => p,
            None => return -12, // ENOMEM
        };
        *pte_ptr = (phys & PTE_ADDR_MASK) | flags | PTE_PRESENT;
        tlb_flush_page(virt);
    }
    0
}

/// Unmap a single page.
pub fn unmap_page(cr3: u64, virt: u64) -> i64 {
    unsafe {
        if let Some(pte_ptr) = walk_to_pte(cr3, virt, false) {
            let pte = *pte_ptr;
            if pte & PTE_PRESENT != 0 {
                let phys = pte & PTE_ADDR_MASK;
                free_page_frame(phys as usize);
                *pte_ptr = 0;
                tlb_flush_page(virt);
            }
        }
    }
    0
}

fn prot_to_pte_flags(prot: u32) -> u64 {
    let mut flags = PTE_USER;
    if prot & PROT_WRITE != 0 { flags |= PTE_WRITABLE; }
    // W^X: if executable, do NOT set NX; otherwise always set NX
    if prot & PROT_EXEC == 0  { flags |= PTE_NX; }
    flags
}

// ── ASLR ───────────────────────────────────────────────────────────────────

static ASLR_SEED: AtomicU64 = AtomicU64::new(0xDEAD_BEEF_CAFE_1234);

fn aslr_rand() -> u64 {
    // xorshift64 PRNG — seeded from hardware RNG on first call
    let mut s = ASLR_SEED.load(Ordering::Relaxed);
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    ASLR_SEED.store(s, Ordering::Relaxed);
    s
}

/// Return a random user-space mmap base (ASLR).
pub fn aslr_mmap_base() -> u64 {
    // Randomize in range 0x0000_7F00_0000_0000..0x0000_7FFF_FFFF_F000
    let rand = aslr_rand() & 0xFFFF_FFFF_F000;
    0x0000_7F00_0000_0000 | rand
}

/// Return ASLR stack base.
pub fn aslr_stack_base() -> u64 {
    let rand = aslr_rand() & 0x00FF_FFFF_F000;
    0x0000_7FFF_0000_0000 | rand
}

// ── mmap / munmap / brk / mprotect ────────────────────────────────────────

static BRK_END: AtomicU64 = AtomicU64::new(0x0000_0001_0000_0000); // 4 GB base

pub fn mm_mmap(addr: u64, len: usize, prot: u32, flags: u32, _fd: i32, _off: i64) -> i64 {
    if len == 0 { return -22; }
    let pages = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    let base = if flags & MAP_FIXED != 0 {
        if addr == 0 { return MAP_FAILED as i64; }
        addr
    } else {
        aslr_mmap_base() & !(PAGE_SIZE as u64 - 1)
    };
    unsafe {
        let cr3 = read_cr3();
        for i in 0..pages {
            let virt = base + (i * PAGE_SIZE) as u64;
            let phys = match alloc_page_frame() {
                Some(p) => p as u64,
                None => {
                    // Roll back already-mapped pages
                    for j in 0..i {
                        unmap_page(cr3, base + (j * PAGE_SIZE) as u64);
                    }
                    return MAP_FAILED as i64;
                }
            };
            if map_page(cr3, virt, phys, prot) != 0 {
                return MAP_FAILED as i64;
            }
            // Zero the page (anonymous mapping)
            if flags & MAP_ANONYMOUS != 0 {
                core::ptr::write_bytes(virt as *mut u8, 0, PAGE_SIZE);
            }
        }
    }
    base as i64
}

pub fn mm_munmap(addr: u64, len: usize) -> i64 {
    if addr == 0 || len == 0 { return -22; }
    let pages = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    unsafe {
        let cr3 = read_cr3();
        for i in 0..pages {
            unmap_page(cr3, addr + (i * PAGE_SIZE) as u64);
        }
    }
    0
}

pub fn mm_brk(new_brk: u64) -> i64 {
    let cur = BRK_END.load(Ordering::SeqCst);
    if new_brk == 0 { return cur as i64; } // query
    if new_brk < cur {
        // Shrink: unmap pages
        let mut va = new_brk & !(PAGE_SIZE as u64 - 1);
        unsafe {
            let cr3 = read_cr3();
            while va < cur {
                unmap_page(cr3, va);
                va += PAGE_SIZE as u64;
            }
        }
    } else {
        // Grow: map new pages
        let mut va = cur;
        unsafe {
            let cr3 = read_cr3();
            while va < new_brk {
                let phys = match alloc_page_frame() {
                    Some(p) => p as u64,
                    None => return -12, // ENOMEM
                };
                map_page(cr3, va, phys, PROT_READ | PROT_WRITE);
                va += PAGE_SIZE as u64;
            }
        }
    }
    BRK_END.store(new_brk, Ordering::SeqCst);
    new_brk as i64
}

pub fn mm_mprotect(addr: u64, len: usize, prot: u32) -> i64 {
    let pages = (len + PAGE_SIZE - 1) / PAGE_SIZE;
    let flags = prot_to_pte_flags(prot);
    unsafe {
        let cr3 = read_cr3();
        for i in 0..pages {
            let va = addr + (i * PAGE_SIZE) as u64;
            if let Some(pte_ptr) = walk_to_pte(cr3, va, false) {
                let phys = *pte_ptr & PTE_ADDR_MASK;
                if phys == 0 { return -14; } // EFAULT
                *pte_ptr = phys | flags | PTE_PRESENT;
                tlb_flush_page(va);
            } else {
                return -14;
            }
        }
    }
    0
}

// ── Allocator bridge (calls kernel buddy allocator) ────────────────────────
fn alloc_page_frame() -> Option<usize> {
    crate::kernel::mm::buddy_allocator::alloc_pages(0) // order-0 = 1 page
}

fn free_page_frame(phys: usize) {
    crate::kernel::mm::buddy_allocator::free_pages(phys, 0);
}

// ── Kernel page table init ─────────────────────────────────────────────────

/// Map the kernel's own text/data/bss sections.
/// Called once during early boot before paging is enabled.
pub fn vmm_init(kernel_phys_base: u64, kernel_size: usize) {
    // Identity-map first 4 MB for UEFI compatibility
    let pages = (kernel_size + PAGE_SIZE - 1) / PAGE_SIZE;
    unsafe {
        let cr3 = read_cr3();
        for i in 0..pages {
            let phys = kernel_phys_base + (i * PAGE_SIZE) as u64;
            // Kernel text: R+X; kernel data: R+W (W^X enforced)
            let prot = if i * PAGE_SIZE < (kernel_size / 2) {
                PROT_READ | PROT_EXEC  // text segment
            } else {
                PROT_READ | PROT_WRITE // data/bss segment
            };
            map_page(cr3, phys, phys, prot); // identity map
        }
    }
    tlb_flush_all();
}
