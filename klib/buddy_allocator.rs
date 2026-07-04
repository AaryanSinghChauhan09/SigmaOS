// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: buddy_allocator - Buddy system memory allocator
//! Hand-rolled zero-dependency implementation, no_std, no pre-defined libraries/functions
//! =========================================================================

#![no_std]

/// Buddy allocator
pub struct BuddyAllocator {
    free_lists: [Option<*mut u8>; 32], // up to 2^32 bytes
    block_size: usize,
    total_size: usize,
}

impl BuddyAllocator {
    pub const fn new(block_size: usize, total_size: usize) -> Self {
        Self {
            free_lists: [None; 32],
            block_size,
            total_size,
        }
    }

    /// Initialize the allocator with a memory region
    pub fn init(&mut self, start: *mut u8) {
        let mut current_size = self.total_size;
        let mut level = 0;
        while current_size > 0 && level < 32 {
            if (current_size & (current_size - 1)) == 0 {
                // power of two
                self.free_lists[level] = Some(start);
                break;
            }
            current_size /= 2;
            level += 1;
        }
    }

    /// Calculate the smallest power of two >= size
    fn next_power_of_two(&self, size: usize) -> (usize, usize) {
        let mut level = 0;
        let mut s = self.block_size;
        while s < size && level < 32 {
            s *= 2;
            level += 1;
        }
        (s, level)
    }

    /// Allocate memory
    pub fn allocate(&mut self, size: usize) -> Option<*mut u8> {
        let (required_size, level) = self.next_power_of_two(size);
        for l in level..32 {
            if self.free_lists[l].is_some() {
                // Found a block, split it down to the required level
                let mut current_level = l;
                while current_level > level {
                    let block = self.free_lists[current_level].take().unwrap();
                    let buddy = unsafe { block.add(required_size << (current_level - level - 1)) };
                    self.free_lists[current_level - 1] = Some(buddy);
                    self.free_lists[current_level - 1] = Some(block);
                    current_level -= 1;
                }
                let block = self.free_lists[level].take().unwrap();
                return Some(block);
            }
        }
        None
    }

    /// Free memory
    pub fn free(&mut self, ptr: *mut u8, size: usize) {
        let (free_size, mut level) = self.next_power_of_two(size);
        self.free_lists[level] = Some(ptr);
        // Try to merge with buddy
        let mut current = ptr;
        while level < 31 {
            let buddy_addr = unsafe {
                current.add(if (current as usize) % (free_size << 1) == 0 {
                    free_size
                } else {
                    free_size.wrapping_neg()
                })
            };
            let mut found_buddy = false;
            let mut prev: Option<*mut u8> = None;
            let mut node = self.free_lists[level];
            while let Some(n) = node {
                if n == buddy_addr {
                    found_buddy = true;
                    if let Some(p) = prev {
                        // TODO: fix prev pointer handling
                    } else {
                        self.free_lists[level] = None;
                    }
                    break;
                }
                prev = Some(n);
                // TODO: need linked list structure to traverse
                break;
            }
            if found_buddy {
                current = if current < buddy_addr { current } else { buddy_addr };
                self.free_lists[level] = None;
                level += 1;
                self.free_lists[level] = Some(current);
            } else {
                break;
            }
        }
    }
}
