// SPDX-License-Identifier: MIT OR Apache-2.0
// SigmaOS klib::bitmap - Atomic Bitmap for Resource Tracking (zero external dependencies)
// Inspired by Linux's bitmap.h and the POSIX bitset API
// Uses only core atomics, no std or alloc required

use core::sync::atomic::{AtomicU64, Ordering};

/// Fixed-size atomic bitmap for tracking resources (page frames, PIDs, IRQs, etc.)
/// Each bit represents one resource. Inspired by Linux's `bitmap_find_free_region`.
pub struct AtomicBitmap<const WORDS: usize> {
    bits: [AtomicU64; WORDS],
}

impl<const WORDS: usize> AtomicBitmap<WORDS> {
    /// Create a new all-zero (all free) bitmap
    pub const fn new() -> Self {
        // SAFETY: AtomicU64 is valid with zero initialization
        // We use const-context unsafe for static allocation
        #[allow(clippy::declare_interior_mutable_const)]
        const ZERO: AtomicU64 = AtomicU64::new(0);
        Self {
            bits: [ZERO; WORDS],
        }
    }

    /// Total capacity in bits
    pub const fn capacity(&self) -> usize {
        WORDS * 64
    }

    /// Set bit at position (mark as used). Returns false if already set.
    pub fn set(&self, pos: usize) -> bool {
        if pos >= self.capacity() {
            return false;
        }
        let word = pos / 64;
        let bit = pos % 64;
        let old = self.bits[word].fetch_or(1 << bit, Ordering::AcqRel);
        old & (1 << bit) == 0 // Return true if we set it (was clear before)
    }

    /// Clear bit at position (mark as free). Returns false if already clear.
    pub fn clear(&self, pos: usize) -> bool {
        if pos >= self.capacity() {
            return false;
        }
        let word = pos / 64;
        let bit = pos % 64;
        let old = self.bits[word].fetch_and(!(1u64 << bit), Ordering::AcqRel);
        old & (1 << bit) != 0 // Return true if we cleared it (was set before)
    }

    /// Test if bit is set.
    pub fn test(&self, pos: usize) -> bool {
        if pos >= self.capacity() {
            return false;
        }
        let word = pos / 64;
        let bit = pos % 64;
        self.bits[word].load(Ordering::Acquire) & (1 << bit) != 0
    }

    /// Find and atomically set the first free bit.
    /// Returns `Some(pos)` or `None` if all bits are set.
    /// Inspired by Linux's `bitmap_find_next_zero_area_off`.
    pub fn alloc_one(&self) -> Option<usize> {
        for word_idx in 0..WORDS {
            let word = self.bits[word_idx].load(Ordering::Acquire);
            if word == u64::MAX {
                continue; // All bits set in this word
            }

            // Find first zero bit using trailing ones count
            let bit = (!word).trailing_zeros() as usize;
            let pos = word_idx * 64 + bit;

            // Atomically try to set this bit (CAS loop for concurrency)
            let mask = 1u64 << bit;
            let result = self.bits[word_idx].fetch_or(mask, Ordering::AcqRel);

            if result & mask == 0 {
                // We set the bit successfully
                return Some(pos);
            }
            // Another thread beat us; retry from this word
        }
        None
    }

    /// Count the number of set bits (popcount)
    pub fn count_ones(&self) -> usize {
        self.bits
            .iter()
            .map(|w| w.load(Ordering::Relaxed).count_ones() as usize)
            .sum()
    }

    /// Count the number of clear bits
    pub fn count_zeros(&self) -> usize {
        self.capacity() - self.count_ones()
    }

    /// Find first set bit (like Linux's `find_first_bit`)
    pub fn find_first_set(&self) -> Option<usize> {
        for (word_idx, word) in self.bits.iter().enumerate() {
            let w = word.load(Ordering::Acquire);
            if w != 0 {
                let bit = w.trailing_zeros() as usize;
                return Some(word_idx * 64 + bit);
            }
        }
        None
    }

    /// Find first clear bit (like Linux's `find_first_zero_bit`)
    pub fn find_first_clear(&self) -> Option<usize> {
        for (word_idx, word) in self.bits.iter().enumerate() {
            let w = word.load(Ordering::Acquire);
            if w != u64::MAX {
                let bit = (!w).trailing_zeros() as usize;
                let pos = word_idx * 64 + bit;
                if pos < self.capacity() {
                    return Some(pos);
                }
            }
        }
        None
    }

    /// Iterate over all set bit positions (non-atomic snapshot)
    pub fn iter_set<F>(&self, mut f: F)
    where
        F: FnMut(usize),
    {
        for (word_idx, word) in self.bits.iter().enumerate() {
            let mut w = word.load(Ordering::Relaxed);
            while w != 0 {
                let bit = w.trailing_zeros() as usize;
                f(word_idx * 64 + bit);
                w &= w - 1; // Clear lowest set bit (Brian Kernighan's trick)
            }
        }
    }
}

/// Page frame allocator bitmap - tracks which physical pages are free
/// Each bit = one 4KB physical page frame
pub type PageFrameBitmap = AtomicBitmap<512>; // 512 × 64 = 32768 pages = 128MB

/// PID bitmap - tracks which process IDs are in use
pub type PidBitmap = AtomicBitmap<64>; // 64 × 64 = 4096 PIDs

/// IRQ allocation bitmap - tracks which interrupt vectors are allocated
pub type IrqBitmap = AtomicBitmap<4>; // 4 × 64 = 256 IRQs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_set_clear() {
        static BITMAP: AtomicBitmap<2> = AtomicBitmap::new();

        assert!(!BITMAP.test(0));
        assert!(BITMAP.set(0)); // Set bit 0
        assert!(BITMAP.test(0)); // Should be set
        assert!(!BITMAP.set(0)); // Already set - returns false
        assert!(BITMAP.clear(0)); // Clear bit 0
        assert!(!BITMAP.test(0)); // Should be clear
    }

    #[test]
    fn test_alloc_one() {
        static BITMAP: AtomicBitmap<1> = AtomicBitmap::new();

        let pos1 = BITMAP.alloc_one().expect("Should find free bit");
        let pos2 = BITMAP.alloc_one().expect("Should find another free bit");
        assert_ne!(pos1, pos2);
        assert_eq!(pos1, 0); // First free bit is 0
        assert_eq!(pos2, 1); // Second free bit is 1
    }

    #[test]
    fn test_count() {
        static BITMAP: AtomicBitmap<1> = AtomicBitmap::new();

        assert_eq!(BITMAP.count_ones(), 0);
        BITMAP.set(5);
        BITMAP.set(10);
        BITMAP.set(63);
        assert_eq!(BITMAP.count_ones(), 3);
        assert_eq!(BITMAP.count_zeros(), 64 - 3);
    }

    #[test]
    fn test_iter_set() {
        static BITMAP: AtomicBitmap<1> = AtomicBitmap::new();
        BITMAP.set(1);
        BITMAP.set(7);
        BITMAP.set(42);

        let mut found = [false; 3];
        BITMAP.iter_set(|pos| match pos {
            1 => found[0] = true,
            7 => found[1] = true,
            42 => found[2] = true,
            _ => {}
        });
        assert!(found.iter().all(|&b| b));
    }
}
