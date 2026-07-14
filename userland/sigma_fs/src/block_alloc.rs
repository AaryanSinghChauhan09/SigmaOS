/// Sovereign block allocator — absorbs XFS block group and ext4 buddy allocator principles.
/// Hand-rolled bitmap, no external allocator crates.
pub struct BlockAllocator {
    pub block_size: usize,
    pub total_blocks: usize,
    // Bitmap: bit-i = 1 means block i is used
    bitmap: Vec<u64>,
}

impl BlockAllocator {
    pub fn new(total_blocks: usize, block_size: usize) -> Self {
        let words = (total_blocks + 63) / 64;
        Self { block_size, total_blocks, bitmap: vec![0u64; words] }
    }

    pub fn alloc(&mut self) -> Option<usize> {
        for (word_idx, word) in self.bitmap.iter_mut().enumerate() {
            if *word != u64::MAX {
                for bit in 0..64 {
                    if (*word & (1u64 << bit)) == 0 {
                        *word |= 1u64 << bit;
                        let block = word_idx * 64 + bit;
                        if block < self.total_blocks {
                            return Some(block);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn free(&mut self, block: usize) {
        let word_idx = block / 64;
        let bit = block % 64;
        if word_idx < self.bitmap.len() {
            self.bitmap[word_idx] &= !(1u64 << bit);
        }
    }
}
