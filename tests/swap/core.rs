// Standalone unit tests for Swap Subsystem

#[path = "../../src/swap/core.rs"]
pub mod core;
#[path = "../../src/swap/enqueue.rs"]
pub mod enqueue;

use core::{SwapError, SwapManager};
use enqueue::{SwapPriority, SwapQueueManager};

#[test]
fn test_swap_manager_swap_out_in_basic() {
    let sm = SwapManager::new(10);
    assert_eq!(sm.total_slots(), 10);
    assert_eq!(sm.free_slots(), 10);

    let slot = sm.swap_out_page(0x1000, 0x20000).expect("Swap out should succeed");
    assert_eq!(sm.free_slots(), 9);

    // Double swap out of same virtual address returns AlreadySwapped
    let err = sm.swap_out_page(0x1000, 0x30000);
    assert_eq!(err, Err(SwapError::AlreadySwapped(0x1000)));

    // Swap in
    let (phys, freed_slot) = sm.swap_in_page(0x1000).expect("Swap in should succeed");
    assert_eq!(phys, 0x20000);
    assert_eq!(freed_slot, slot);
    assert_eq!(sm.free_slots(), 10);

    // Swap in non-existent page returns NotSwapped
    let not_swapped = sm.swap_in_page(0x1000);
    assert_eq!(not_swapped, Err(SwapError::NotSwapped(0x1000)));
}

#[test]
fn test_swap_manager_oom_on_exhaustion() {
    let sm = SwapManager::new(2); // Only 2 slots
    sm.swap_out_page(0x1000, 0x10000).unwrap();
    sm.swap_out_page(0x2000, 0x20000).unwrap();

    assert!(!sm.is_oom_triggered());

    // Exhaustion attempt
    let oom = sm.swap_out_page(0x3000, 0x30000);
    assert_eq!(oom, Err(SwapError::OutOfSwapSlots));
    assert!(sm.is_oom_triggered());
}

#[test]
fn test_swap_queue_priority_and_processing() {
    let sm = SwapManager::new(10);
    let queue = SwapQueueManager::new(5); // Queue depth 5

    queue.enqueue_request(0x1000, 0x10000, SwapPriority::Normal, true).unwrap();
    queue.enqueue_request(0x2000, 0x20000, SwapPriority::High, true).unwrap();

    assert_eq!(queue.pending_count(), 2);

    // High priority request should be processed first
    let first = queue.process_next(&sm).unwrap().unwrap();
    assert_eq!(first.0, 0x2000);

    let second = queue.process_next(&sm).unwrap().unwrap();
    assert_eq!(second.0, 0x1000);

    assert_eq!(queue.pending_count(), 0);
}
