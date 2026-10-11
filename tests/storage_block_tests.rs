// SPDX-License-Identifier: MIT
//! Standalone Storage Block Layer Unit Tests for SigmaOS.

#[path = "../src/storage/block.rs"]
mod block;

#[path = "../src/storage/virtio.rs"]
mod virtio;

use block::{BoundedBlockDeviceManager, BoundedBlockError};
use virtio::{RequestType, VirtIoBlockDevice, VirtIoBlockError};

#[test]
fn test_bounded_block_device_manager_queue_exhaustion() {
    let mut mgr = BoundedBlockDeviceManager::new(1, 2);
    assert!(mgr.submit_request(101, 1, vec![1, 2, 3], true).is_ok());
    assert!(mgr.submit_request(102, 2, vec![4, 5, 6], false).is_ok());

    // Queue full error on exceeding capacity
    let res = mgr.submit_request(103, 3, vec![7, 8, 9], true);
    assert_eq!(res, Err(BoundedBlockError::QueueFull));
}

#[test]
fn test_bounded_block_device_manager_completion_lifecycle() {
    let mut mgr = BoundedBlockDeviceManager::new(1, 5);
    mgr.submit_request(101, 1, vec![10, 20], true).unwrap();

    // Request not processed yet
    assert_eq!(mgr.complete_request(101), Err(BoundedBlockError::NotCompleted));

    // Process queue
    let processed = mgr.process_requests();
    assert_eq!(processed, 1);

    // Complete successfully
    let req = mgr.complete_request(101).unwrap();
    assert_eq!(req.req_id, 101);
    assert!(req.completed);

    // Request not found on double-completion
    assert_eq!(mgr.complete_request(101), Err(BoundedBlockError::RequestNotFound));
}

#[test]
fn test_virtio_block_device_operations() {
    let mut dev = VirtIoBlockDevice::new(1, 100, 2);

    assert!(dev.enqueue_request(201, RequestType::Read, 10, vec![]).is_ok());
    assert!(dev.enqueue_request(202, RequestType::Write, 20, vec![1, 2, 3]).is_ok());

    // Queue full
    assert_eq!(dev.enqueue_request(203, RequestType::Flush, 0, vec![]), Err(VirtIoBlockError::QueueFull));

    // Process queue
    assert_eq!(dev.process_queue(), 2);

    // Dequeue
    let req = dev.dequeue_completed(201).unwrap();
    assert_eq!(req.req_id, 201);

    // Double completion
    assert_eq!(dev.dequeue_completed(201), Err(VirtIoBlockError::DoubleCompletion));
}
