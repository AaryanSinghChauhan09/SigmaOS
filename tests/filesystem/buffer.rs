//! Standalone Buffer Cache Integration & Unit Tests

#[path = "../../src/fs/buffer.rs"]
pub mod buffer;

use buffer::{BufferCache, BufferError, BufferState};

#[test]
fn test_buffer_cache_allocation_and_lru() {
    let mut cache = BufferCache::new(2, 512);

    let b1 = cache.get_block(1, 10).unwrap();
    assert_eq!(b1.block_num, 10);
    assert_eq!(b1.dev_id, 1);
    assert_eq!(b1.state, BufferState::Clean);

    let b2 = cache.get_block(1, 20).unwrap();
    assert_eq!(b2.block_num, 20);

    assert_eq!(cache.cached_count(), 2);

    let b3 = cache.get_block(1, 30).unwrap();
    assert_eq!(b3.block_num, 30);
    assert_eq!(cache.cached_count(), 2);
}

#[test]
fn test_buffer_cache_multi_device_isolation() {
    let mut cache = BufferCache::new(4, 512);

    let b1 = cache.get_block(1, 100).unwrap();
    b1.data[0] = 42;

    let b2 = cache.get_block(2, 100).unwrap();
    b2.data[0] = 99;

    assert_eq!(cache.get_block(1, 100).unwrap().data[0], 42);
    assert_eq!(cache.get_block(2, 100).unwrap().data[0], 99);
}

#[test]
fn test_buffer_dirty_flushing() {
    let mut cache = BufferCache::new(4, 512);

    cache.get_block(1, 100).unwrap();
    cache.mark_dirty(1, 100).expect("Mark dirty should succeed");

    let flushed = cache.flush_dirty().expect("Flush should succeed");
    assert_eq!(flushed, 1);
}

#[test]
fn test_buffer_pool_exhaustion_when_locked() {
    let mut cache = BufferCache::new(1, 512);
    let buf = cache.get_block(1, 5).unwrap();
    buf.state = BufferState::Locked;

    let res = cache.get_block(1, 6);
    assert_eq!(res.err(), Some(BufferError::PoolExhausted));
}
