#[path = "../../src/fs/cache.rs"]
mod cache;

use cache::{BlockCacheManager, CacheError, WatermarkLevel};

#[test]
fn test_cache_lru_eviction_and_dirty_writeback() {
    let mut cache_mgr = BlockCacheManager::new(2);

    cache_mgr
        .insert_page(1, 100, vec![1, 2, 3, 4], false, 100)
        .unwrap();
    cache_mgr
        .insert_page(1, 101, vec![5, 6, 7, 8], false, 200)
        .unwrap();
    cache_mgr.mark_dirty(1, 100).unwrap();

    // Access 101 at timestamp 300 to make 100 LRU (timestamp 100)
    assert!(cache_mgr.get_page(1, 101, 300).is_some());

    // Insert block 102 -> should evict LRU block 100 and write back dirty data
    cache_mgr
        .insert_page(1, 102, vec![9, 10, 11, 12], false, 400)
        .unwrap();

    assert!(cache_mgr.get_page(1, 100, 500).is_none());
    assert!(cache_mgr.get_page(1, 101, 500).is_some());
    assert!(cache_mgr.get_page(1, 102, 500).is_some());
}

#[test]
fn test_cache_double_eviction_prevention() {
    let mut cache_mgr = BlockCacheManager::new(2);

    cache_mgr
        .insert_page(1, 10, vec![0xAA; 16], false, 100)
        .unwrap();
    assert!(cache_mgr.evict_page(1, 10).is_ok());
    assert_eq!(cache_mgr.evict_page(1, 10), Err(CacheError::NotCached));
}

#[test]
fn test_cache_oom_watermark_metrics() {
    let mut cache_mgr = BlockCacheManager::new(10);
    for i in 0..10 {
        cache_mgr
            .insert_page(1, i as u64, vec![i as u8; 32], false, 100 + i)
            .unwrap();
    }

    let metrics = &cache_mgr.metrics;
    assert_eq!(metrics.total_pages, 10);
    assert_eq!(metrics.max_pages, 10);
    assert_eq!(metrics.watermark_level, WatermarkLevel::OomCritical);
}
