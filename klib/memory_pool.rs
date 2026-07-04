// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: sigma-memory-pool - Pre-allocated memory pools per shard category
//! Hand-rolled zero-dependency implementation, no_std, no pre-defined libraries/functions
//! =========================================================================

#![no_std]

/// Memory pool block metadata
#[derive(Debug, Clone, Copy)]
struct BlockHeader {
    next: Option<usize>,
    size: usize,
    free: bool,
}

/// Memory pool for fixed-size blocks
pub struct FixedSizePool {
    data: *mut u8,
    blocks: &'static mut [BlockHeader],
    free_list_head: Option<usize>,
    block_size: usize,
    num_blocks: usize,
}

impl FixedSizePool {
    /// Create a new fixed-size memory pool
    pub const fn new(buffer: &'static mut [u8], block_size: usize) -> Self {
        let num_blocks = buffer.len() / block_size;
        let mut blocks = unsafe {
            core::slice::from_raw_parts_mut(
                buffer.as_mut_ptr() as *mut BlockHeader,
                num_blocks,
            )
        };
        
        let mut free_list_head = Some(0);
        let mut i = 0;
        while i < num_blocks - 1 {
            blocks[i].next = Some(i + 1);
            blocks[i].size = block_size;
            blocks[i].free = true;
            i += 1;
        }
        blocks[i].next = None;
        blocks[i].size = block_size;
        blocks[i].free = true;
        
        Self {
            data: buffer.as_mut_ptr(),
            blocks,
            free_list_head,
            block_size,
            num_blocks,
        }
    }

    /// Allocate a block from the pool
    pub fn allocate(&mut self) -> Option<*mut u8> {
        let idx = self.free_list_head?;
        self.free_list_head = self.blocks[idx].next;
        self.blocks[idx].free = false;
        Some(unsafe { self.data.add(idx * self.block_size) })
    }

    /// Deallocate a block back to the pool
    pub fn deallocate(&mut self, ptr: *mut u8) {
        let offset = unsafe { ptr.offset_from(self.data) as usize };
        let idx = offset / self.block_size;
        if idx < self.num_blocks && !self.blocks[idx].free {
            self.blocks[idx].free = true;
            self.blocks[idx].next = self.free_list_head;
            self.free_list_head = Some(idx);
        }
    }

    /// Get number of free blocks
    pub fn free_count(&self) -> usize {
        let mut count = 0;
        let mut curr = self.free_list_head;
        while let Some(idx) = curr {
            count += 1;
            curr = self.blocks[idx].next;
        }
        count
    }
}

/// sigma-memory-pool main registry
pub struct SigmaMemoryPoolRegistry {
    pools: [Option<&'static mut FixedSizePool>; 8],
}

impl SigmaMemoryPoolRegistry {
    pub const fn new() -> Self {
        Self { pools: [None; 8] }
    }

    pub fn register_pool(&mut self, idx: usize, pool: &'static mut FixedSizePool) {
        if idx < 8 {
            self.pools[idx] = Some(pool);
        }
    }
}
