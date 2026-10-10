#[path = "../src/fs/cache.rs"]
mod cache;

use cache::*;

#[test]
fn test_cache_eviction_lru_and_double_eviction() {
    let mut mgr = BlockCacheManager::new(3);

    assert!(mgr.insert_page(1, 1, vec![1; 512], false, 100).is_ok());
    assert!(mgr.insert_page(1, 2, vec![2; 512], true, 200).is_ok());
    assert!(mgr.insert_page(1, 3, vec![3; 512], false, 300).is_ok());

    // Access page 1 to make page 2 the LRU
    assert!(mgr.get_page(1, 1, 400).is_some());

    // 4th insert causes eviction of LRU page 2
    assert!(mgr.insert_page(1, 4, vec![4; 512], false, 500).is_ok());

    // Verify page 2 was evicted and dirty count updated
    assert_eq!(mgr.get_page(1, 2, 510), None);

    // Verify double eviction returns NotCached
    assert_eq!(mgr.evict_page(1, 2), Err(CacheError::NotCached));
}

#[test]
fn test_cache_watermark_oom_metrics() {
    let mut mgr = BlockCacheManager::new(10);
    for i in 0..9 {
        assert!(mgr.insert_page(1, i, vec![0; 512], false, 100 + i).is_ok());
    }

    assert_eq!(mgr.metrics.watermark_level, WatermarkLevel::OomCritical);
}
