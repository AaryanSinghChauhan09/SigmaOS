// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Custom memory allocators for specific use cases
// Zero-allocation, performance-optimized allocators

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Arena allocator for temporary allocations
/// Fast allocation, no individual deallocation
pub struct ArenaAllocator {
    start: *mut u8,
    current: AtomicUsize,
    size: usize,
}

unsafe impl GlobalAlloc for ArenaAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        let current = self.current.load(Ordering::Relaxed);
        let aligned = (current + align - 1) & !(align - 1);
        
        if aligned + size > self.size {
            return core::ptr::null_mut();
        }
        
        let ptr = self.start.add(aligned);
        self.current.store(aligned + size, Ordering::Relaxed);
        
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Arena allocator doesn't support individual deallocation
        // Reset the arena to free all memory at once
    }
}

impl ArenaAllocator {
    pub const unsafe fn new(start: *mut u8, size: usize) -> Self {
        Self {
            start,
            current: AtomicUsize::new(0),
            size,
        }
    }

    pub fn reset(&self) {
        self.current.store(0, Ordering::Relaxed);
    }

    pub fn used(&self) -> usize {
        self.current.load(Ordering::Relaxed)
    }

    pub fn available(&self) -> usize {
        self.size - self.used()
    }
}

/// Pool allocator for fixed-size objects
/// Eliminates fragmentation for uniform allocations
pub struct PoolAllocator<T, const N: usize> {
    pool: [Option<T>; N],
    free_list: [bool; N],
}

impl<T, const N: usize> PoolAllocator<T, N> {
    pub const fn new() -> Self {
        Self {
            pool: [None; N],
            free_list: [true; N],
        }
    }

    pub fn allocate(&mut self) -> Option<&mut T> {
        for i in 0..N {
            if self.free_list[i] {
                self.free_list[i] = false;
                self.pool[i] = None; // Placeholder
                return self.pool[i].as_mut();
            }
        }
        None
    }

    pub fn deallocate(&mut self, item: &mut T) {
        for i in 0..N {
            if let Some(ref mut pool_item) = self.pool[i] {
                if core::ptr::eq(pool_item, item) {
                    self.free_list[i] = true;
                    return;
                }
            }
        }
    }

    pub fn available(&self) -> usize {
        self.free_list.iter().filter(|&&x| x).count()
    }

    pub fn used(&self) -> usize {
        N - self.available()
    }
}

/// Bump pointer allocator for linear allocation
/// Extremely fast, no deallocation
pub struct BumpAllocator {
    start: *mut u8,
    current: AtomicUsize,
    end: *mut u8,
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        let current = self.current.load(Ordering::Relaxed);
        let aligned = (current + align - 1) & !(align - 1);
        
        let ptr = self.start.add(aligned);
        
        if ptr.add(size) > self.end {
            return core::ptr::null_mut();
        }
        
        self.current.store(aligned + size, Ordering::Relaxed);
        ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't support deallocation
    }
}

impl BumpAllocator {
    pub const unsafe fn new(start: *mut u8, size: usize) -> Self {
        Self {
            start,
            current: AtomicUsize::new(0),
            end: start.add(size),
        }
    }

    pub fn reset(&self) {
        self.current.store(0, Ordering::Relaxed);
    }

    pub fn used(&self) -> usize {
        self.current.load(Ordering::Relaxed)
    }

    pub fn capacity(&self) -> usize {
        self.end as usize - self.start as usize
    }
}

/// Stack allocator for LIFO allocations
/// Fast allocation/deallocation for stack-like patterns
pub struct StackAllocator {
    start: *mut u8,
    current: AtomicUsize,
    size: usize,
}

unsafe impl GlobalAlloc for StackAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        let current = self.current.load(Ordering::Relaxed);
        let aligned = (current + align - 1) & !(align - 1);
        
        if aligned + size > self.size {
            return core::ptr::null_mut();
        }
        
        let ptr = self.start.add(aligned);
        self.current.store(aligned + size, Ordering::Relaxed);
        
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let offset = ptr as usize - self.start as usize;
        
        // Only allow deallocation if it's the top of the stack
        if offset + size == self.current.load(Ordering::Relaxed) {
            self.current.store(offset, Ordering::Relaxed);
        }
    }
}

impl StackAllocator {
    pub const unsafe fn new(start: *mut u8, size: usize) -> Self {
        Self {
            start,
            current: AtomicUsize::new(0),
            size,
        }
    }

    pub fn reset(&self) {
        self.current.store(0, Ordering::Relaxed);
    }

    pub fn used(&self) -> usize {
        self.current.load(Ordering::Relaxed)
    }

    pub fn available(&self) -> usize {
        self.size - self.used()
    }
}

/// Slab allocator for fixed-size blocks
/// Reduces fragmentation for common allocation sizes
pub struct SlabAllocator<const BLOCK_SIZE: usize, const BLOCK_COUNT: usize> {
    blocks: [Option<[u8; BLOCK_SIZE]>; BLOCK_COUNT],
    free_list: [bool; BLOCK_COUNT],
}

impl<const BLOCK_SIZE: usize, const BLOCK_COUNT: usize> SlabAllocator<BLOCK_SIZE, BLOCK_COUNT> {
    pub const fn new() -> Self {
        Self {
            blocks: [None; BLOCK_COUNT],
            free_list: [true; BLOCK_COUNT],
        }
    }

    pub fn allocate(&mut self) -> Option<&mut [u8; BLOCK_SIZE]> {
        for i in 0..BLOCK_COUNT {
            if self.free_list[i] {
                self.free_list[i] = false;
                self.blocks[i] = Some([0u8; BLOCK_SIZE]);
                return self.blocks[i].as_mut();
            }
        }
        None
    }

    pub fn deallocate(&mut self, block: &mut [u8; BLOCK_SIZE]) {
        for i in 0..BLOCK_COUNT {
            if let Some(ref slab_block) = self.blocks[i] {
                if core::ptr::eq(slab_block.as_ptr(), block.as_ptr()) {
                    self.free_list[i] = true;
                    return;
                }
            }
        }
    }

    pub fn available(&self) -> usize {
        self.free_list.iter().filter(|&&x| x).count()
    }

    pub fn used(&self) -> usize {
        BLOCK_COUNT - self.available()
    }
}

/// Tiered allocator for different allocation sizes
/// Routes allocations to appropriate specialized allocators
pub struct TieredAllocator {
    small: SlabAllocator<64, 128>,
    medium: SlabAllocator<256, 64>,
    large: SlabAllocator<1024, 32>,
    huge: SlabAllocator<4096, 16>,
}

impl TieredAllocator {
    pub const fn new() -> Self {
        Self {
            small: SlabAllocator::new(),
            medium: SlabAllocator::new(),
            large: SlabAllocator::new(),
            huge: SlabAllocator::new(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        match size {
            0..=64 => self.small.allocate().map(|b| b.as_mut_ptr()),
            65..=256 => self.medium.allocate().map(|b| b.as_mut_ptr()),
            257..=1024 => self.large.allocate().map(|b| b.as_mut_ptr()),
            1025..=4096 => self.huge.allocate().map(|b| b.as_mut_ptr()),
            _ => None,
        }
    }

    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        match size {
            0..=64 => {
                if let Some(block) = unsafe { (ptr as *mut [u8; 64]).as_mut() } {
                    self.small.deallocate(block);
                }
            }
            65..=256 => {
                if let Some(block) = unsafe { (ptr as *mut [u8; 256]).as_mut() } {
                    self.medium.deallocate(block);
                }
            }
            257..=1024 => {
                if let Some(block) = unsafe { (ptr as *mut [u8; 1024]).as_mut() } {
                    self.large.deallocate(block);
                }
            }
            1025..=4096 => {
                if let Some(block) = unsafe { (ptr as *mut [u8; 4096]).as_mut() } {
                    self.huge.deallocate(block);
                }
            }
            _ => {}
        }
    }

    pub fn stats(&self) -> TieredStats {
        TieredStats {
            small_used: self.small.used(),
            small_available: self.small.available(),
            medium_used: self.medium.used(),
            medium_available: self.medium.available(),
            large_used: self.large.used(),
            large_available: self.large.available(),
            huge_used: self.huge.used(),
            huge_available: self.huge.available(),
        }
    }
}

pub struct TieredStats {
    pub small_used: usize,
    pub small_available: usize,
    pub medium_used: usize,
    pub medium_available: usize,
    pub large_used: usize,
    pub large_available: usize,
    pub huge_used: usize,
    pub huge_available: usize,
}
