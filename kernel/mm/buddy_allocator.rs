/// SigmaOS: Buddy Physical Memory Allocator
/// Phase G Blocker #2: Buddy physical allocator
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.

#[allow(dead_code)]

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
pub const MAX_FRAMES: usize = 262144; // Maximum frames (1GB memory)

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
    pub next: SigmaU64,  // Next free block in linked list
    pub prev: SigmaU64,  // Previous free block in linked list
}

// ─── Buddy Allocator ─────────────────────────────────────────────────────

pub struct BuddyAllocator {
    initialized: SigmaBool,
    total_frames: SigmaU64,
    free_frames: SigmaU64,
    allocated_frames: SigmaU64,
    max_order: SigmaU8,
    free_lists: [Option<SigmaU64>; MAX_ORDER + 1],
    frame_table: [PhysicalFrame; MAX_FRAMES], // BUG-001 Fix: Static frame table
    block_table: [BuddyBlock; MAX_FRAMES], // BUG-001 Fix: Static block table
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
            frame_table: [PhysicalFrame {
                pfn: 0,
                order: 0,
                allocated: false,
                reserved: false,
            }; MAX_FRAMES],
            block_table: [BuddyBlock {
                order: 0,
                free: false,
                split: false,
                next: 0,
                prev: 0,
            }; MAX_FRAMES],
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

        // BUG-001 Fix: Initialize frame table with actual frame data
        let actual_frames = if self.total_frames as usize > MAX_FRAMES {
            MAX_FRAMES as SigmaU64
        } else {
            self.total_frames
        };

        for i in 0..actual_frames as usize {
            self.frame_table[i] = PhysicalFrame {
                pfn: i as SigmaU64,
                order: 0,
                allocated: false,
                reserved: false,
            };
            self.block_table[i] = BuddyBlock {
                order: 0,
                free: true,
                split: false,
                next: 0,
                prev: 0,
            };
        }

        // Clear free lists
        for i in 0..=MAX_ORDER {
            self.free_lists[i] = None;
        }

        // Add all memory to free list at max order
        self.add_to_free_list(base_addr, self.max_order);

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

    /// Add block to free list (linked list implementation)
    unsafe fn add_to_free_list(&mut self, addr: SigmaU64, order: SigmaU8) {
        let order_usize = order as usize;
        if order_usize > MAX_ORDER {
            return;
        }

        let pfn = self.addr_to_pfn(addr);
        if pfn >= MAX_FRAMES as SigmaU64 {
            return;
        }

        let pfn_usize = pfn as usize;

        // Insert at head of free list for this order
        if let Some(head_addr) = self.free_lists[order_usize] {
            let head_pfn = self.addr_to_pfn(head_addr);
            if head_pfn < MAX_FRAMES as SigmaU64 {
                self.block_table[head_pfn as usize].prev = addr;
            }
        }

        self.block_table[pfn_usize].next = self.free_lists[order_usize].unwrap_or(0);
        self.block_table[pfn_usize].prev = 0;
        self.free_lists[order_usize] = Some(addr);
    }

    /// Remove block from free list (linked list implementation)
    unsafe fn remove_from_free_list(&mut self, addr: SigmaU64, order: SigmaU8) {
        let order_usize = order as usize;
        if order_usize > MAX_ORDER {
            return;
        }

        let pfn = self.addr_to_pfn(addr);
        if pfn >= MAX_FRAMES as SigmaU64 {
            return;
        }

        let pfn_usize = pfn as usize;
        let block = self.block_table[pfn_usize];

        // Update previous block's next pointer
        if block.prev != 0 {
            let prev_pfn = self.addr_to_pfn(block.prev);
            if prev_pfn < MAX_FRAMES as SigmaU64 {
                self.block_table[prev_pfn as usize].next = block.next;
            }
        } else {
            // Block was head of list
            self.free_lists[order_usize] = if block.next != 0 { Some(block.next) } else { None };
        }

        // Update next block's prev pointer
        if block.next != 0 {
            let next_pfn = self.addr_to_pfn(block.next);
            if next_pfn < MAX_FRAMES as SigmaU64 {
                self.block_table[next_pfn as usize].prev = block.prev;
            }
        }

        // Clear block's links
        self.block_table[pfn_usize].next = 0;
        self.block_table[pfn_usize].prev = 0;
    }

    /// Mark block as allocated (BUG-001 Fix)
    unsafe fn mark_allocated(&mut self, addr: SigmaU64, order: SigmaU8) {
        let pfn = self.addr_to_pfn(addr);
        if pfn < MAX_FRAMES as SigmaU64 {
            self.frame_table[pfn as usize].allocated = true;
            self.frame_table[pfn as usize].order = order;
            self.block_table[pfn as usize].free = false;
            
            // Mark all frames in the block
            let num_frames = 1 << order;
            for i in 0..num_frames {
                let frame_pfn = pfn + i as SigmaU64;
                if frame_pfn < MAX_FRAMES as SigmaU64 {
                    self.frame_table[frame_pfn as usize].allocated = true;
                    self.frame_table[frame_pfn as usize].order = order;
                }
            }
        }
    }

    /// Mark block as free (BUG-001 Fix)
    unsafe fn mark_free(&mut self, addr: SigmaU64, order: SigmaU8) {
        let pfn = self.addr_to_pfn(addr);
        if pfn < MAX_FRAMES as SigmaU64 {
            self.frame_table[pfn as usize].allocated = false;
            self.frame_table[pfn as usize].order = 0;
            self.block_table[pfn as usize].free = true;
            
            // Mark all frames in the block
            let num_frames = 1 << order;
            for i in 0..num_frames {
                let frame_pfn = pfn + i as SigmaU64;
                if frame_pfn < MAX_FRAMES as SigmaU64 {
                    self.frame_table[frame_pfn as usize].allocated = false;
                    self.frame_table[frame_pfn as usize].order = 0;
                }
            }
        }
    }

    /// Check if block is allocated (BUG-001 Fix)
    unsafe fn is_allocated(&self, addr: SigmaU64, order: SigmaU8) -> SigmaBool {
        let pfn = self.addr_to_pfn(addr);
        if pfn < MAX_FRAMES as SigmaU64 {
            self.frame_table[pfn as usize].allocated
        } else {
            false
        }
    }

    /// Check if block is free (BUG-001 Fix)
    unsafe fn is_free(&self, addr: SigmaU64, order: SigmaU8) -> SigmaBool {
        let pfn = self.addr_to_pfn(addr);
        if pfn < MAX_FRAMES as SigmaU64 {
            !self.frame_table[pfn as usize].allocated
        } else {
            false
        }
    }
    
    /// Convert physical address to frame number (BUG-001 Fix)
    fn addr_to_pfn(&self, addr: SigmaU64) -> SigmaU64 {
        (addr - self.base_address) / PAGE_SIZE as SigmaU64
    }
    
    /// Convert frame number to physical address (BUG-001 Fix)
    fn pfn_to_addr(&self, pfn: SigmaU64) -> SigmaU64 {
        self.base_address + (pfn * PAGE_SIZE as SigmaU64)
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
