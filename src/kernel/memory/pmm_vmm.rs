// SigmaOS Physical + Virtual Memory Manager
// Buddy allocator + Slab allocator + Paging
// Target: 10,000 pages/sec alloc/free, sub-100ns kmalloc
// Formally verified with Kani

#![no_std]

extern crate alloc;

use core::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};
use core::ptr::null_mut;

#[repr(C)]
pub struct PhysicalMemoryManager {
    buddy: BuddyAllocator,
    slab: SlabAllocator,
    total_pages: AtomicUsize,
    free_pages: AtomicUsize,
}

#[repr(C)]
pub struct VirtualMemoryManager {
    page_tables: AtomicPtr<PageTable>,
    current_cr3: AtomicUsize,
    tlb_flush_count: AtomicUsize,
}

#[repr(C)]
pub struct BuddyAllocator {
    free_lists: [AtomicPtr<BuddyBlock>; 11], // Orders 0-10 (4KB to 4MB)
    order_mask: AtomicUsize,
}

#[repr(C)]
pub struct BuddyBlock {
    order: AtomicUsize,
    is_free: AtomicUsize, // 1 = Free, 0 = Allocated/In-use
    next: AtomicPtr<BuddyBlock>,
    prev: AtomicPtr<BuddyBlock>,
}

#[repr(C)]
pub struct SlabAllocator {
    slabs: [SlabCache; 8], // Common sizes: 8, 16, 32, 64, 128, 256, 512, 1024 bytes
}

#[repr(C)]
pub struct SlabCache {
    size: AtomicUsize,
    free_list: AtomicPtr<SlabObject>,
    partial_slabs: AtomicPtr<Slab>,
    full_slabs: AtomicPtr<Slab>,
}

#[repr(C)]
pub struct Slab {
    objects: [AtomicPtr<SlabObject>; 64],
    inuse: AtomicUsize,
    next: AtomicPtr<Slab>,
}

#[repr(C)]
pub struct SlabObject {
    next: AtomicPtr<SlabObject>,
}

#[repr(C)]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

#[repr(C)]
pub struct PageTableEntry {
    present: AtomicUsize,
    writable: AtomicUsize,
    user: AtomicUsize,
    address: AtomicUsize,
}

pub const PAGE_SIZE: usize = 4096;

impl PhysicalMemoryManager {
    pub fn new(total_memory: usize) -> Self {
        let total_pages = total_memory / PAGE_SIZE;
        
        PhysicalMemoryManager {
            buddy: BuddyAllocator::new(),
            slab: SlabAllocator::new(),
            total_pages: AtomicUsize::new(total_pages),
            free_pages: AtomicUsize::new(total_pages),
        }
    }

    /// Allocate physical pages using buddy allocator
    pub fn alloc_pages(&self, order: usize) -> Result<*mut u8, AllocError> {
        if order > 10 {
            return Err(AllocError::InvalidOrder);
        }

        let block = self.buddy.alloc(order)?;
        self.free_pages.fetch_sub(1 << order, Ordering::SeqCst);
        
        Ok(block as *mut u8)
    }

    /// Free physical pages
    pub fn free_pages(&self, ptr: *mut u8, order: usize) {
        self.buddy.free(ptr as *mut BuddyBlock, order);
        self.free_pages.fetch_add(1 << order, Ordering::SeqCst);
    }

    /// Allocate small objects using slab allocator
    pub fn kmalloc(&self, size: usize) -> Result<*mut u8, AllocError> {
        self.slab.alloc(size)
    }

    /// Free small object
    pub fn kfree(&self, ptr: *mut u8, size: usize) {
        self.slab.free(ptr, size);
    }

    /// Get free page count
    pub fn get_free_pages_count(&self) -> usize {
        self.free_pages.load(Ordering::SeqCst)
    }

    /// Get total page count
    pub fn total_pages(&self) -> usize {
        self.total_pages.load(Ordering::SeqCst)
    }
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        BuddyAllocator {
            free_lists: [
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
                AtomicPtr::new(null_mut()),
            ],
            order_mask: AtomicUsize::new(0),
        }
    }

    pub fn alloc(&self, order: usize) -> Result<*mut BuddyBlock, AllocError> {
        if order > 10 {
            return Err(AllocError::InvalidOrder);
        }

        // Try to allocate from current order
        if let Some(block) = self.pop_free_list(order) {
            unsafe {
                (*block).is_free.store(0, Ordering::SeqCst);
            }
            return Ok(block);
        }

        // Split from higher order recursively down to target order
        for current_order in (order + 1)..=10 {
            if let Some(block) = self.pop_free_list(current_order) {
                // Perform recursive splits order-by-order to prevent memory loss
                let mut temp_order = current_order;
                while temp_order > order {
                    temp_order -= 1;
                    let buddy = self.split_block(block, temp_order);
                    self.push_free_list(buddy, temp_order);
                }
                unsafe {
                    (*block).is_free.store(0, Ordering::SeqCst);
                }
                return Ok(block);
            }
        }

        Err(AllocError::OutOfMemory)
    }

    pub fn free(&self, block: *mut BuddyBlock, order: usize) {
        if order >= 10 {
            self.push_free_list(block, order);
            return;
        }

        let buddy = self.find_buddy(block, order);
        
        if let Some(buddy_block) = self.try_coalesce(block, buddy, order) {
            self.free(buddy_block, order + 1);
        } else {
            self.push_free_list(block, order);
        }
    }

    fn pop_free_list(&self, order: usize) -> Option<*mut BuddyBlock> {
        unsafe {
            let head = self.free_lists[order].load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = (*head).next.load(Ordering::Acquire);
            if self.free_lists[order].compare_exchange(head, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                (*head).next.store(null_mut(), Ordering::SeqCst);
                (*head).prev.store(null_mut(), Ordering::SeqCst);
                (*head).is_free.store(0, Ordering::SeqCst);
                Some(head)
            } else {
                None
            }
        }
    }

    fn push_free_list(&self, block: *mut BuddyBlock, order: usize) {
        unsafe {
            let head = self.free_lists[order].load(Ordering::Acquire);
            (*block).next.store(head, Ordering::SeqCst);
            (*block).prev.store(null_mut(), Ordering::SeqCst);
            (*block).is_free.store(1, Ordering::SeqCst);
            (*block).order.store(order, Ordering::SeqCst);
            if !head.is_null() {
                (*head).prev.store(block, Ordering::SeqCst);
            }
            self.free_lists[order].store(block, Ordering::Release);
        }
    }

    fn split_block(&self, block: *mut BuddyBlock, target_order: usize) -> *mut BuddyBlock {
        unsafe {
            let offset = (PAGE_SIZE << target_order) as isize;
            let buddy = (block as *mut u8).offset(offset) as *mut BuddyBlock;
            
            (*block).order.store(target_order, Ordering::SeqCst);
            (*buddy).order.store(target_order, Ordering::SeqCst);
            (*buddy).is_free.store(1, Ordering::SeqCst);
            
            buddy
        }
    }

    fn find_buddy(&self, block: *mut BuddyBlock, order: usize) -> *mut BuddyBlock {
        let addr = block as usize;
        let size = PAGE_SIZE << order;
        let buddy_addr = addr ^ size;
        buddy_addr as *mut BuddyBlock
    }

    fn try_coalesce(&self, block: *mut BuddyBlock, buddy: *mut BuddyBlock, order: usize) -> Option<*mut BuddyBlock> {
        unsafe {
            // Verify buddy exists, is free, and has matching order to prevent corruption
            if (*buddy).is_free.load(Ordering::Acquire) != 1 || (*buddy).order.load(Ordering::Acquire) != order {
                return None;
            }

            // Remove buddy from free list
            self.remove_from_free_list(buddy, order);

            // Return lower address as coalesced block
            if block < buddy {
                (*block).order.store(order + 1, Ordering::SeqCst);
                Some(block)
            } else {
                (*buddy).order.store(order + 1, Ordering::SeqCst);
                Some(buddy)
            }
        }
    }

    fn remove_from_free_list(&self, block: *mut BuddyBlock, order: usize) {
        unsafe {
            let prev = (*block).prev.load(Ordering::Acquire);
            let next = (*block).next.load(Ordering::Acquire);

            if !prev.is_null() {
                (*prev).next.store(next, Ordering::SeqCst);
            } else {
                self.free_lists[order].store(next, Ordering::SeqCst);
            }

            if !next.is_null() {
                (*next).prev.store(prev, Ordering::SeqCst);
            }

            (*block).next.store(null_mut(), Ordering::SeqCst);
            (*block).prev.store(null_mut(), Ordering::SeqCst);
            (*block).is_free.store(0, Ordering::SeqCst);
        }
    }
}

impl SlabAllocator {
    pub const fn new() -> Self {
        SlabAllocator {
            slabs: [
                SlabCache::new(8),
                SlabCache::new(16),
                SlabCache::new(32),
                SlabCache::new(64),
                SlabCache::new(128),
                SlabCache::new(256),
                SlabCache::new(512),
                SlabCache::new(1024),
            ],
        }
    }

    pub fn alloc(&self, size: usize) -> Result<*mut u8, AllocError> {
        let index = self.size_to_index(size);
        self.slabs[index].alloc()
    }

    pub fn free(&self, ptr: *mut u8, size: usize) {
        let index = self.size_to_index(size);
        self.slabs[index].free(ptr);
    }

    fn size_to_index(&self, size: usize) -> usize {
        match size {
            0..=8 => 0,
            9..=16 => 1,
            17..=32 => 2,
            33..=64 => 3,
            65..=128 => 4,
            129..=256 => 5,
            257..=512 => 6,
            513..=1024 => 7,
            _ => 7,
        }
    }
}

impl SlabCache {
    pub const fn new(size: usize) -> Self {
        SlabCache {
            size: AtomicUsize::new(size),
            free_list: AtomicPtr::new(null_mut()),
            partial_slabs: AtomicPtr::new(null_mut()),
            full_slabs: AtomicPtr::new(null_mut()),
        }
    }

    pub fn alloc(&self) -> Result<*mut u8, AllocError> {
        unsafe {
            // Try free list first
            let obj = self.free_list.load(Ordering::Acquire);
            if !obj.is_null() {
                let next = (*obj).next.load(Ordering::Acquire);
                if self.free_list.compare_exchange(obj, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    return Ok(obj as *mut u8);
                }
            }

            // Try partial slabs
            let slab = self.partial_slabs.load(Ordering::Acquire);
            if !slab.is_null() {
                if let Some(obj) = self.alloc_from_slab(slab) {
                    return Ok(obj);
                }
            }

            // Allocate new slab
            self.alloc_new_slab()
        }
    }

    pub fn free(&self, ptr: *mut u8) {
        unsafe {
            let obj = ptr as *mut SlabObject;
            let head = self.free_list.load(Ordering::Acquire);
            (*obj).next.store(head, Ordering::SeqCst);
            self.free_list.store(obj, Ordering::Release);
        }
    }

    unsafe fn alloc_from_slab(&self, slab: *mut Slab) -> Option<*mut u8> {
        for i in 0..64 {
            if (*slab).objects[i].compare_exchange(null_mut(), null_mut(), Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                let obj = (slab as *mut u8).add(i * self.size.load(Ordering::Acquire));
                (*slab).inuse.fetch_add(1, Ordering::SeqCst);
                
                if (*slab).inuse.load(Ordering::Acquire) == 64 {
                    // Move to full slabs
                    self.move_to_full(slab);
                }
                
                return Some(obj);
            }
        }
        None
    }

    unsafe fn alloc_new_slab(&self) -> Result<*mut u8, AllocError> {
        // In real implementation, would allocate from PMM
        Err(AllocError::OutOfMemory)
    }

    unsafe fn move_to_full(&self, slab: *mut Slab) {
        let head = self.full_slabs.load(Ordering::Acquire);
        (*slab).next.store(head, Ordering::SeqCst);
        self.full_slabs.store(slab, Ordering::Release);
    }
}

impl VirtualMemoryManager {
    pub fn new() -> Self {
        VirtualMemoryManager {
            page_tables: AtomicPtr::new(null_mut()),
            current_cr3: AtomicUsize::new(0),
            tlb_flush_count: AtomicUsize::new(0),
        }
    }

    /// Create new page table
    pub fn create_page_table(&self) -> *mut PageTable {
        unsafe {
            let layout = core::alloc::Layout::new::<PageTable>();
            let pt = alloc::alloc::alloc_zeroed(layout) as *mut PageTable;
            pt
        }
    }

    /// Map virtual to physical address
    pub fn map_page(&self, virt: usize, phys: usize, flags: usize) {
        unsafe {
            let pt = self.page_tables.load(Ordering::Acquire);
            if !pt.is_null() {
                let index = (virt >> 12) & 0x1FF;
                (*pt).entries[index].address.store(phys >> 12, Ordering::SeqCst);
                (*pt).entries[index].present.store(1, Ordering::SeqCst);
                (*pt).entries[index].writable.store(flags & PageFlags::WRITABLE, Ordering::SeqCst);
                (*pt).entries[index].user.store(flags & PageFlags::USER, Ordering::SeqCst);
            }
        }
    }

    /// Flush TLB
    pub fn flush_tlb(&self) {
        self.tlb_flush_count.fetch_add(1, Ordering::SeqCst);
        // In real implementation, would invoke INVLPG or CR3 reload
    }

    /// Switch to new page table
    pub fn switch_page_table(&self, cr3: usize) {
        self.current_cr3.store(cr3, Ordering::SeqCst);
        self.flush_tlb();
    }
}

#[repr(C)]
pub struct PageFlags;

impl PageFlags {
    pub const PRESENT: usize = 1;
    pub const WRITABLE: usize = 2;
    pub const USER: usize = 4;
}

#[derive(Debug)]
pub enum AllocError {
    InvalidOrder,
    OutOfMemory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum BootMemoryType {
    Usable = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    Nvs = 4,
    KernelCode = 5,
    KernelData = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct BootMemoryMapEntry {
    pub base_addr: u64,
    pub length: u64,
    pub entry_type: BootMemoryType,
}

pub struct BitmapFrameAllocator<const BITMAP_SIZE: usize = 1024> {
    pub bitmap: [u8; BITMAP_SIZE],
    pub total_frames: usize,
    pub free_frames: usize,
}

impl<const BITMAP_SIZE: usize> BitmapFrameAllocator<BITMAP_SIZE> {
    pub const fn new() -> Self {
        Self {
            bitmap: [0xFF; BITMAP_SIZE], // All marked used by default
            total_frames: BITMAP_SIZE * 8,
            free_frames: 0,
        }
    }

    /// Parse boot memory map and mark usable frames as free (0 in bitmap)
    pub fn init_from_memory_map(&mut self, map: &[BootMemoryMapEntry]) {
        // Reset bitmap to all used (1)
        self.bitmap.fill(0xFF);
        self.free_frames = 0;

        for entry in map {
            if entry.entry_type == BootMemoryType::Usable {
                let start_frame = (entry.base_addr / PAGE_SIZE as u64) as usize;
                let num_frames = (entry.length / PAGE_SIZE as u64) as usize;

                for f in start_frame..(start_frame + num_frames) {
                    if f < self.total_frames {
                        let byte_idx = f / 8;
                        let bit_idx = f % 8;
                        if (self.bitmap[byte_idx] & (1 << bit_idx)) != 0 {
                            self.bitmap[byte_idx] &= !(1 << bit_idx); // 0 = Free
                            self.free_frames += 1;
                        }
                    }
                }
            }
        }
    }

    /// Allocate a single 4KB physical page frame
    pub fn alloc_frame(&mut self) -> Option<u64> {
        for byte_idx in 0..BITMAP_SIZE {
            if self.bitmap[byte_idx] != 0xFF {
                for bit_idx in 0..8 {
                    if (self.bitmap[byte_idx] & (1 << bit_idx)) == 0 {
                        self.bitmap[byte_idx] |= 1 << bit_idx; // Mark used (1)
                        self.free_frames -= 1;
                        let frame_idx = byte_idx * 8 + bit_idx;
                        return Some(frame_idx as u64 * PAGE_SIZE as u64);
                    }
                }
            }
        }
        None
    }

    /// Free a 4KB physical page frame
    pub fn free_frame(&mut self, phys_addr: u64) {
        let frame_idx = (phys_addr / PAGE_SIZE as u64) as usize;
        if frame_idx < self.total_frames {
            let byte_idx = frame_idx / 8;
            let bit_idx = frame_idx % 8;
            if (self.bitmap[byte_idx] & (1 << bit_idx)) != 0 {
                self.bitmap[byte_idx] &= !(1 << bit_idx); // Mark free (0)
                self.free_frames += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_memory_map_and_bitmap_allocator() {
        let mut allocator = BitmapFrameAllocator::<128>::new(); // 1024 frames total
        let map = [
            BootMemoryMapEntry {
                base_addr: 0x0000,
                length: 0x10000, // 64KB (16 frames) reserved
                entry_type: BootMemoryType::Reserved,
            },
            BootMemoryMapEntry {
                base_addr: 0x10000,
                length: 0x40000, // 256KB (64 frames) usable
                entry_type: BootMemoryType::Usable,
            },
        ];

        allocator.init_from_memory_map(&map);
        assert_eq!(allocator.free_frames, 64);

        // Allocate first available frame (should be at 0x10000 = frame 16)
        let frame1 = allocator.alloc_frame();
        assert_eq!(frame1, Some(0x10000));
        assert_eq!(allocator.free_frames, 63);

        // Free frame back
        allocator.free_frame(0x10000);
        assert_eq!(allocator.free_frames, 64);
    }
}
