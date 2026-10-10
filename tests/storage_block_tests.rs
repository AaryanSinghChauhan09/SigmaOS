#[path = "../src/storage/block.rs"]
pub mod block;

#[path = "../src/storage/virtio.rs"]
pub mod virtio;

use block::*;
use virtio::*;

#[test]
fn test_block_manager_request_exhaustion() {
    let mut mgr = BoundedBlockDeviceManager::new(3);
    let ssd = SsdBlockDevice::new(1, 512, 100);
    mgr.register_device(Box::new(ssd));

    assert!(mgr.submit_request(1, BlockOpCode::Read, 0, 1, vec![], 100).is_ok());
    assert!(mgr.submit_request(1, BlockOpCode::Read, 1, 1, vec![], 101).is_ok());
    assert!(mgr.submit_request(1, BlockOpCode::Read, 2, 1, vec![], 102).is_ok());

    // 4th request must fail with RequestExhaustion
    let err = mgr.submit_request(1, BlockOpCode::Read, 3, 1, vec![], 103);
    assert_eq!(err, Err(BlockError::RequestExhaustion));
}

#[test]
fn test_exact_once_completion_and_double_completion_prevention() {
    let mut mgr = BoundedBlockDeviceManager::new(10);
    let ssd = SsdBlockDevice::new(1, 512, 100);
    mgr.register_device(Box::new(ssd));

    let req_id = mgr.submit_request(1, BlockOpCode::Write, 5, 1, vec![0xAA; 512], 200).unwrap();
    let dispatched = mgr.dispatch_next().unwrap().unwrap();
    assert_eq!(dispatched, req_id);

    // First completion must succeed
    assert!(mgr.complete_request(req_id, true).is_ok());

    // Duplicate completion must fail
    assert_eq!(mgr.complete_request(req_id, true), Err(BlockError::DoubleCompletion));
}

#[test]
fn test_virtio_ring_queue_submission() {
    let mut virtio = VirtIoBlockDevice::new(5, 512, 1000, 32);
    assert_eq!(virtio.device_class(), DeviceClass::VirtIoBlock);

    let desc = virtio.submit_virtqueue_request(VIRTIO_BLK_T_OUT, 10, 1024).unwrap();
    assert_eq!(desc, 0);
    assert_eq!(virtio.process_used_ring(), 1);
}
