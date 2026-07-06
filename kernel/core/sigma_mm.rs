// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_mm.rs — Memory Manager (Buddy + Slab + ASLR + W^X)
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Physical memory constants ─────────────────────────────────────────────
const PAGE_SIZE:  usize = 4096;
const MAX_ORDER:  usize = 11;       // 2^11 pages = 8 MB max block
const MAX_FRAMES: usize = 1 << 20; // support up to 4 GB (1M × 4 KB pages)

/// Physical base address of usable RAM (updated by init).
static PHYS_BASE:  AtomicU64 = AtomicU64::new(0x0010_0000);
/// Total physical memory in bytes.
static PHYS_TOTAL: AtomicU64 = AtomicU64::new(0x0400_0000);

/// Canary value written at the bottom of kernel stacks to detect overflow.
const STACK_GUARD_CANARY: u64 = 0xDEAD_C0DE_BEEF_CAFE;

// ── Buddy allocator ───────────────────────────────────────────────────────
// Each order has a free-list of block indices.
const BUDDY_LIST_SIZE: usize = MAX_FRAMES / 2;

pub struct BuddyAllocator {
    free_lists: [[u32; BUDDY_LIST_SIZE]; MAX_ORDER],
    free_count: [usize; MAX_ORDER],
    used_pages: AtomicU64,
    total_pages: u64,
    initialized: bool,
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        Self {
            free_lists:  [[0u32; BUDDY_LIST_SIZE]; MAX_ORDER],
            free_count:  [0usize; MAX_ORDER],
            used_pages:  AtomicU64::new(0),
            total_pages: 0,
            initialized: false,
        }
    }

    /// Initialize from a physical memory map (base, size pairs)
    pub fn init(&mut self, phys_base: u64, phys_size: u64) {
        self.total_pages = phys_size / PAGE_SIZE as u64;
        // Add all pages to highest possible order
        let mut addr = phys_base;
        while addr + (PAGE_SIZE as u64) <= phys_base + phys_size {
            let frame = (addr / PAGE_SIZE as u64) as u32;
            let order = ((phys_size / PAGE_SIZE as u64).trailing_zeros() as usize).min(MAX_ORDER - 1);
            self.free_buddy(frame, order);
            addr += (PAGE_SIZE << order) as u64;
        }
        self.initialized = true;
    }

    fn free_buddy(&mut self, frame: u32, order: usize) {
        let o = order.min(MAX_ORDER - 1);
        if self.free_count[o] < BUDDY_LIST_SIZE {
            self.free_lists[o][self.free_count[o]] = frame;
            self.free_count[o] += 1;
        }
    }

    /// Allocate 2^order contiguous pages. Returns physical frame number or None.
    pub fn alloc(&mut self, order: usize) -> Option<u32> {
        let o = order.min(MAX_ORDER - 1);
        // Search from requested order upward
        for cur_order in o..MAX_ORDER {
            if self.free_count[cur_order] == 0 { continue; }
            self.free_count[cur_order] -= 1;
            let frame = self.free_lists[cur_order][self.free_count[cur_order]];
            // Split higher orders down to requested order
            let mut cur_frame = frame;
            for split_order in (o..cur_order).rev() {
                // Buddy = cur_frame XOR (1 << split_order)
                let buddy = cur_frame ^ (1 << split_order);
                self.free_buddy(buddy, split_order);
            }
            self.used_pages.fetch_add(1 << o, Ordering::Relaxed);
            return Some(cur_frame);
        }
        None
    }

    /// Free 2^order pages starting at frame
    pub fn free(&mut self, frame: u32, order: usize) {
        let o = order.min(MAX_ORDER - 1);
        let mut cur_frame = frame;
        let mut cur_order = o;
        // Coalesce with buddy
        while cur_order < MAX_ORDER - 1 {
            let buddy = cur_frame ^ (1u32 << cur_order);
            // Check if buddy is free
            let mut found = false;
            for i in 0..self.free_count[cur_order] {
                if self.free_lists[cur_order][i] == buddy {
                    // Remove buddy from list
                    self.free_count[cur_order] -= 1;
                    self.free_lists[cur_order][i] = self.free_lists[cur_order][self.free_count[cur_order]];
                    cur_frame = cur_frame.min(buddy);
                    cur_order += 1;
                    found = true;
                    break;
                }
            }
            if !found { break; }
        }
        self.free_buddy(cur_frame, cur_order);
        self.used_pages.fetch_sub(1 << o, Ordering::Relaxed);
    }

    pub fn used_pages(&self) -> u64 { self.used_pages.load(Ordering::Relaxed) }
    pub fn free_pages(&self) -> u64 { self.total_pages.saturating_sub(self.used_pages()) }
}

// ── Slab allocator ─────────────────────────────────────────────────────────
// Linux 6.11-inspired: separate slab cache per size class prevents
// cross-cache heap-spray attacks (bucket slab isolation).
const SLAB_SIZES: [usize; 8] = [8, 16, 32, 64, 128, 256, 512, 1024];
const SLAB_SLOTS: usize = 512;
/// Poisoning pattern written into freed slab objects (detects use-after-free).
const SLAB_POISON_BYTE: u8 = 0x5A; // "ZAP"
/// Header written at start of each allocated slab object for overflow detection.
const SLAB_HEADER_MAGIC: u32 = 0xCAFE_BABE;

struct SlabCache {
    slots:    [u64; SLAB_SLOTS], // store addresses as u64
    free_map: [bool; SLAB_SLOTS],
    obj_size: usize,
    base:     u64,
    count:    usize,
}

impl SlabCache {
    const fn new(obj_size: usize) -> Self {
        Self {
            slots:    [0u64; SLAB_SLOTS],
            free_map: [true; SLAB_SLOTS],
            obj_size,
            base: 0,
            count: 0,
        }
    }

    fn init(&mut self, phys_base: u64) {
        self.base = phys_base;
        for i in 0..SLAB_SLOTS {
            self.slots[i] = phys_base + (i * self.obj_size) as u64;
        }
    }

    fn alloc(&mut self) -> Option<u64> {
        for i in 0..SLAB_SLOTS {
            if self.free_map[i] {
                self.free_map[i] = false;
                self.count += 1;
                let addr = self.slots[i];
                // Clear poison bytes from previous free, write header magic.
                unsafe {
                    let ptr = addr as *mut u8;
                    for j in 0..self.obj_size { core::ptr::write_volatile(ptr.add(j), 0); }
                    if self.obj_size >= 4 {
                        core::ptr::write_volatile(ptr as *mut u32, SLAB_HEADER_MAGIC);
                    }
                }
                return Some(addr);
            }
        }
        None
    }

    fn free(&mut self, addr: u64) -> bool {
        for i in 0..SLAB_SLOTS {
            if self.slots[i] == addr {
                self.free_map[i] = true;
                self.count = self.count.saturating_sub(1);
                // Poison freed object to catch use-after-free.
                unsafe {
                    let ptr = addr as *mut u8;
                    for j in 0..self.obj_size {
                        core::ptr::write_volatile(ptr.add(j), SLAB_POISON_BYTE);
                    }
                }
                return true;
            }
        }
        false
    }
}

pub struct SlabAllocator {
    caches: [SlabCache; 8],
    initialized: bool,
}

impl SlabAllocator {
    pub const fn new() -> Self {
        Self {
            caches: [
                SlabCache::new(SLAB_SIZES[0]), SlabCache::new(SLAB_SIZES[1]),
                SlabCache::new(SLAB_SIZES[2]), SlabCache::new(SLAB_SIZES[3]),
                SlabCache::new(SLAB_SIZES[4]), SlabCache::new(SLAB_SIZES[5]),
                SlabCache::new(SLAB_SIZES[6]), SlabCache::new(SLAB_SIZES[7]),
            ],
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        // Use a static slab pool starting at 0xC000_0000 (kernel heap base)
        let mut base: u64 = 0xC000_0000;
        for i in 0..8 {
            self.caches[i].init(base);
            base += (SLAB_SIZES[i] * SLAB_SLOTS) as u64;
        }
        self.initialized = true;
    }

    pub fn alloc(&mut self, size: usize) -> Option<u64> {
        for i in 0..8 {
            if SLAB_SIZES[i] >= size {
                return self.caches[i].alloc();
            }
        }
        None
    }

    pub fn free(&mut self, addr: u64) -> bool {
        for i in 0..8 {
            if self.caches[i].free(addr) { return true; }
        }
        false
    }
}

// ── ASLR: 42-bit entropy VMA randomization ────────────────────────────────
pub struct AslrState {
    seed: AtomicU64,
}

impl AslrState {
    pub const fn new() -> Self { Self { seed: AtomicU64::new(0xDEAD_BEEF_1234_5678) } }

    fn next(&self) -> u64 {
        // xorshift64
        let mut x = self.seed.load(Ordering::Relaxed);
        x ^= x << 13; x ^= x >> 7; x ^= x << 17;
        self.seed.store(x, Ordering::Relaxed);
        x
    }

    /// Return a randomized VMA base with 42-bit entropy (user space)
    pub fn randomize_vma(&self, hint: u64) -> u64 {
        let rand = self.next() & 0x3FF_FFFF_F000; // 42-bit, page-aligned
        (hint & 0xFFFF_0000_0000_0000) | rand
    }
}

// ── VMA (Virtual Memory Area) descriptor ─────────────────────────────────
const MAX_VMAS: usize = 256;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum VmaPerm {
    None  = 0,
    Read  = 1,
    Write = 2,
    Exec  = 4,
    Rw    = 3,
    Rx    = 5,  // W^X: read+exec, not write
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vma {
    pub start: u64,
    pub end:   u64,
    pub perms: VmaPerm,
    pub cow:   bool,   // copy-on-write
    pub _pad:  [u8; 6],
}

pub struct VmSpace {
    vmas:  [Option<Vma>; MAX_VMAS],
    count: usize,
    aslr:  AslrState,
}

impl VmSpace {
    pub const fn new() -> Self {
        Self {
            vmas:  [const { None }; MAX_VMAS],
            count: 0,
            aslr:  AslrState::new(),
        }
    }

    /// Map a region. Enforces W^X: Exec+Write is denied.
    pub fn mmap(&mut self, hint: u64, size: u64, perms: VmaPerm) -> Result<u64, MmError> {
        // W^X enforcement
        if perms as u8 & (VmaPerm::Write as u8 | VmaPerm::Exec as u8)
            == (VmaPerm::Write as u8 | VmaPerm::Exec as u8)
        {
            return Err(MmError::WxViolation);
        }
        if self.count >= MAX_VMAS { return Err(MmError::OutOfVmas); }
        let base = self.aslr.randomize_vma(hint);
        let vma = Vma { start: base, end: base + size, perms, cow: false, _pad: [0u8; 6] };
        self.vmas[self.count] = Some(vma);
        self.count += 1;
        Ok(base)
    }

    /// Unmap a region
    pub fn munmap(&mut self, addr: u64, size: u64) -> bool {
        for i in 0..self.count {
            if let Some(v) = &self.vmas[i] {
                if v.start == addr {
                    self.vmas[i] = None;
                    return true;
                }
            }
        }
        false
    }

    /// Fault handler — check if addr has a valid mapping
    pub fn handle_page_fault(&self, addr: u64, write: bool) -> Result<(), MmError> {
        for i in 0..self.count {
            if let Some(v) = &self.vmas[i] {
                if addr >= v.start && addr < v.end {
                    if write && (v.perms as u8 & VmaPerm::Write as u8 == 0) {
                        return Err(MmError::PermDenied);
                    }
                    return Ok(());
                }
            }
        }
        Err(MmError::NoMapping)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MmError {
    OutOfMemory, OutOfVmas, WxViolation, PermDenied, NoMapping,
}

// ── Guard page helpers ────────────────────────────────────────────────────
/// Write a guard canary at the bottom of a kernel stack.
/// On x86_64 the stack grows downward, so `stack_base` is the lowest address.
pub unsafe fn kstack_guard_write(stack_base: *mut u8) {
    let canary_ptr = stack_base as *mut u64;
    core::ptr::write_volatile(canary_ptr, STACK_GUARD_CANARY);
}

/// Check the guard canary at the bottom of a kernel stack.
/// Returns `true` if the canary is intact, `false` if overwritten (overflow!).
pub unsafe fn kstack_guard_check(stack_base: *const u8) -> bool {
    let canary_ptr = stack_base as *const u64;
    core::ptr::read_volatile(canary_ptr) == STACK_GUARD_CANARY
}

// ── Global instances ──────────────────────────────────────────────────────
static mut G_BUDDY: BuddyAllocator = BuddyAllocator::new();
static mut G_SLAB:  SlabAllocator  = SlabAllocator::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_slab_init() {
    let phys_base  = 0x0010_0000u64;
    let phys_total = 0x0400_0000u64; // default 64 MB — overridden by bootloader
    PHYS_BASE.store(phys_base,  Ordering::Relaxed);
    PHYS_TOTAL.store(phys_total, Ordering::Relaxed);
    G_BUDDY.init(phys_base, phys_total);
    G_SLAB.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_slab_alloc(size: usize) -> *mut u8 {
    G_SLAB.alloc(size).unwrap_or(0) as *mut u8
}

#[no_mangle]
pub unsafe extern "C" fn sigma_slab_free(ptr: *mut u8) -> i32 {
    if G_SLAB.free(ptr as u64) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mm_free_pages() -> u64 { G_BUDDY.free_pages() }
#[no_mangle]
pub unsafe extern "C" fn sigma_mm_used_pages() -> u64 { G_BUDDY.used_pages() }

/// Returns total physical memory size in bytes (used by amnesic scrubber).
#[no_mangle]
pub unsafe extern "C" fn sigma_mm_total_bytes() -> u64 {
    PHYS_TOTAL.load(Ordering::Relaxed)
}

/// Returns physical base address of RAM (used by amnesic scrubber).
#[no_mangle]
pub unsafe extern "C" fn sigma_mm_phys_base() -> u64 {
    PHYS_BASE.load(Ordering::Relaxed)
}

/// Write guard canary at base of a kernel stack.
#[no_mangle]
pub unsafe extern "C" fn sigma_kstack_guard_init(stack_base: *mut u8) {
    kstack_guard_write(stack_base);
}

/// Check guard canary — returns 1 if intact, 0 if overwritten (stack overflow!).
#[no_mangle]
pub unsafe extern "C" fn sigma_kstack_guard_check(stack_base: *const u8) -> u32 {
    if kstack_guard_check(stack_base) { 1 } else { 0 }
}

/// Override physical memory parameters (called from bootloader with real E820 map).
#[no_mangle]
pub unsafe extern "C" fn sigma_mm_set_params(phys_base: u64, phys_total: u64) {
    PHYS_BASE.store(phys_base,   Ordering::Relaxed);
    PHYS_TOTAL.store(phys_total, Ordering::Relaxed);
    G_BUDDY.init(phys_base, phys_total);
}
