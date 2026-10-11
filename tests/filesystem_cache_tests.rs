// SPDX-License-Identifier: MIT
//! Block Cache Manager Unit and Integration Tests for SigmaOS.

#[path = "../src/fs/cache.rs"]
mod cache;

use cache::{BlockCacheManager, CacheError};

#[test]
fn test_block_cache_write_read_and_lru_eviction() {
    let mut cache = BlockCacheManager::new(2);

    assert!(cache.write_block(1, vec![10, 20]).is_ok());
    assert!(cache.write_block(2, vec![30, 40]).is_ok());

    // Read block 1 to update its LRU timestamp
    assert_eq!(cache.read_block(1), Some(vec![10, 20]));

    // Write block 3 -> triggers eviction of block 2 (least recently used)
    assert!(cache.write_block(3, vec![50, 60]).is_ok());
    assert_eq!(cache.eviction_count, 1);

    // Block 2 should be gone
    assert_eq!(cache.read_block(2), None);
    // Block 1 should still exist
    assert_eq!(cache.read_block(1), Some(vec![10, 20]));
}

#[test]
fn test_block_cache_dirty_flush() {
    let mut cache = BlockCacheManager::new(5);

    cache.write_block(1, vec![1, 2, 3]).unwrap();
    cache.write_block(2, vec![4, 5, 6]).unwrap();

    assert_eq!(cache.flush_dirty_blocks(), 2);
    assert_eq!(cache.dirty_writebacks, 2);

    // Subsequent flush finds no dirty blocks
    assert_eq!(cache.flush_dirty_blocks(), 0);
}
