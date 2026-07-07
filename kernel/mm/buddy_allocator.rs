/// SigmaOS: Buddy Physical Memory Allocator
/// Phase G Blocker #2: Buddy physical allocator
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Constants ─────────────────────────────────────────────────────────────

pub const MAX_ORDER: usize = 10; // Maximum order (2^10 = 1024 pages)
pub const PAGE_SIZE: usize = 4096; // 4KB pages
pub const MIN_ORDER: usize = 0; // Minimum order (1 page)

// ─── Physical Frame ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalFrame {
    pub pfn: SigmaU64,        // Physical frame number
    pub order: SigmaU8,        // Order of allocation
    pub allocated: SigmaBool, // Allocation status
    pub reserved: SigmaBool,  // Reserved (kernel, etc.)
}

// ─── Buddy Block ───────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BuddyBlock {
    pub order: SigmaU8,
    pub free: SigmaBool,
    pub split: SigmaBool,
}

// ─── Buddy Allocator ─────────────────────────────────────────────────────

pub struct BuddyAllocator {
    initialized: SigmaBool,
    total_frames: SigmaU64,
    free_frames: SigmaU64,
    allocated_frames: SigmaU64,
    max_order: SigmaU8,
    free_lists: [Option<SigmaU64>; MAX_ORDER + 1],
    frame_table: Option<&'static mut [PhysicalFrame]>,
    block_table: Option<&'static mut [BuddyBlock]>,
    base_address: SigmaU64,
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            total_frames: 0,
            free_frames: 0,
            allocated_frames: 0,
            max_order: MAX_ORDER as SigmaU8,
            free_lists: [None; MAX_ORDER + 1],
            frame_table: None,
            block_table: None,
            base_address: 0,
        }
    }

    /// Initialize buddy allocator with physical memory
    pub unsafe fn init(&mut self, base_addr: SigmaU64, total_mem: SigmaU64) -> Result<(), &'static str> {
        if self.initialized {
            return Err("Allocator already initialized");
        }

        self.base_address = base_addr;
        self.total_frames = total_mem / PAGE_SIZE as SigmaU64;
        self.free_frames = self.total_frames;
        self.max_order = self.calculate_max_order(self.total_frames);

        // Initialize frame table (placeholder - needs actual memory)
        // TODO: Allocate frame table from boot memory
        self.frame_table = None;

        // Initialize block table (placeholder - needs actual memory)
        // TODO: Allocate block table from boot memory
        self.block_table = None;

        // Clear free lists
        for i in 0..=MAX_ORDER {
            self.free_lists[i] = None;
        }

        // Add all memory to free list at max order
        self.add_to_free_list(0, self.max_order);

        self.initialized = true;

        Ok(())
    }

    /// Allocate physical frames
    pub unsafe fn alloc(&mut self, order: SigmaU8) -> Result<SigmaU64, &'static str> {
        if !self.initialized {
            return Err("Allocator not initialized");
        }

        if order > self.max_order {
            return Err("Order too large");
        }

        let order_usize = order as usize;

        // Find free block of requested order or larger
        let mut current_order = order;
        let mut block_addr = None;

        while current_order <= self.max_order as SigmaU8 {
            if let Some(addr) = self.free_lists[current_order as usize] {
                block_addr = Some(addr);
                break;
            }
            current_order += 1;
        }

        let block_addr = match block_addr {
            Some(addr) => addr,
            None => return Err("Out of memory"),
        };

        // Remove from free list
        self.remove_from_free_list(block_addr, current_order);

        // Split blocks if necessary
        while current_order > order {
            current_order -= 1;
            let buddy_addr = self.get_buddy_addr(block_addr, current_order);
            self.add_to_free_list(buddy_addr, current_order);
        }

        // Mark as allocated
        self.mark_allocated(block_addr, order);

        self.free_frames -= (1 << order) as SigmaU64;
        self.allocated_frames += (1 << order) as SigmaU64;

        Ok(block_addr)
    }

    /// Free physical frames
    pub unsafe fn free(&mut self, addr: SigmaU64, order: SigmaU8) -> Result<(), &'static str> {
        if !self.initialized {
            return Err("Allocator not initialized");
        }

        if order > self.max_order {
            return Err("Order too large");
        }

        // Check if block is allocated
        if !self.is_allocated(addr, order) {
            return Err("Block not allocated");
        }

        // Mark as free
        self.mark_free(addr, order);

        // Try to merge with buddy
        let mut current_addr = addr;
        let mut current_order = order;

        while current_order < self.max_order {
            let buddy_addr = self.get_buddy_addr(current_addr, current_order);

            if !self.is_free(buddy_addr, current_order) {
                break;
            }

            // Remove buddy from free list
            self.remove_from_free_list(buddy_addr, current_order);

            // Merge blocks
            current_addr = current_addr.min(buddy_addr);
            current_order += 1;
        }

        // Add merged block to free list
        self.add_to_free_list(current_addr, current_order);

        self.free_frames += (1 << order) as SigmaU64;
        self.allocated_frames -= (1 << order) as SigmaU64;

        Ok(())
    }

    /// Get buddy address for a block
    fn get_buddy_addr(&self, addr: SigmaU64, order: SigmaU8) -> SigmaU64 {
        let block_size = (PAGE_SIZE << order) as SigmaU64;
        addr ^ block_size
    }

    /// Add block to free list
    unsafe fn add_to_free_list(&mut self, addr: SigmaU64, order: SigmaU8) {
        let order_usize = order as usize;
        if order_usize <= MAX_ORDER {
            self.free_lists[order_usize] = Some(addr);
        }
    }

    /// Remove block from free list
    unsafe fn remove_from_free_list(&mut self, addr: SigmaU64, order: SigmaU8) {
        let order_usize = order as usize;
        if order_usize <= MAX_ORDER {
            if self.free_lists[order_usize] == Some(addr) {
                self.free_lists[order_usize] = None;
            }
        }
    }

    /// Mark block as allocated
    unsafe fn mark_allocated(&mut self, addr: SigmaU64, order: SigmaU8) {
        // TODO: Implement frame table marking
        let _ = (addr, order);
    }

    /// Mark block as free
    unsafe fn mark_free(&mut self, addr: SigmaU64, order: SigmaU8) {
        // TODO: Implement frame table marking
        let _ = (addr, order);
    }

    /// Check if block is allocated
    unsafe fn is_allocated(&self, addr: SigmaU64, order: SigmaU8) -> SigmaBool {
        // TODO: Implement frame table check
        let _ = (addr, order);
        false
    }

    /// Check if block is free
    unsafe fn is_free(&self, addr: SigmaU64, order: SigmaU8) -> SigmaBool {
        let order_usize = order as usize;
        if order_usize <= MAX_ORDER {
            self.free_lists[order_usize] == Some(addr)
        } else {
            false
        }
    }

    /// Calculate maximum order for given memory
    fn calculate_max_order(&self, frames: SigmaU64) -> SigmaU8 {
        let mut order = 0;
        let mut size = 1;
        while size < frames {
            size <<= 1;
            order += 1;
        }
        order.min(MAX_ORDER as SigmaU8)
    }

    /// Get total frames
    pub unsafe fn get_total_frames(&mut self) -> SigmaU64 {
        self.total_frames
    }

    /// Get free frames
    pub unsafe fn get_free_frames(&mut self) -> SigmaU64 {
        self.free_frames
    }

    /// Get allocated frames
    pub unsafe fn get_allocated_frames(&mut self) -> SigmaU64 {
        self.allocated_frames
    }

    /// Get max order
    pub unsafe fn get_max_order(&mut self) -> SigmaU8 {
        self.max_order
    }

    /// Print allocator statistics
    pub unsafe fn print_stats(&mut self) {
        // TODO: Implement proper printing
        let _ = (self.total_frames, self.free_frames, self.allocated_frames);
    }
}

// ─── Global Allocator Instance ─────────────────────────────────────────────

static mut ALLOCATOR: BuddyAllocator = BuddyAllocator::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_init(base_addr: SigmaU64, total_mem: SigmaU64) -> SigmaI32 {
    match ALLOCATOR.init(base_addr, total_mem) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_alloc(order: SigmaU8) -> SigmaU64 {
    match ALLOCATOR.alloc(order) {
        Ok(addr) => addr,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_free(addr: SigmaU64, order: SigmaU8) -> SigmaI32 {
    match ALLOCATOR.free(addr, order) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_get_free() -> SigmaU64 {
    ALLOCATOR.get_free_frames()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_get_total() -> SigmaU64 {
    ALLOCATOR.get_total_frames()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_buddy_get_allocated() -> SigmaU64 {
    ALLOCATOR.get_allocated_frames()
}

// ─── VMM Compatibility Functions (BUG-001 Fix) ────────────────────────────────
// These functions are called by sigma_vmm.rs for page frame allocation

#[no_mangle]
pub unsafe extern "C" fn alloc_pages(order: usize) -> usize {
    match ALLOCATOR.alloc(order as SigmaU8) {
        Ok(addr) => addr as usize,
        Err(_) => 0,
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_pages(phys: usize, order: usize) {
    let _ = ALLOCATOR.free(phys as SigmaU64, order as SigmaU8);
}
