#[path = "../../src/storage/block.rs"]
pub mod block;
#[path = "../../src/storage/virtio.rs"]
pub mod virtio;

use block::{
    BlockError, BlockOpCode, BlockOrientedDevice, BoundedBlockDeviceManager,
    DeviceClass, RequestState, SsdBlockDevice,
};
use virtio::VirtIoBlockDevice;

#[test]
fn test_bounded_block_manager_request_exhaustion() {
    let mut manager = BoundedBlockDeviceManager::new(2);
    let ssd = SsdBlockDevice::new(1, 512, 100);
    manager.register_device(Box::new(ssd));

    let req1 = manager
        .submit_request(1, BlockOpCode::Read, 0, 1, vec![], 100)
        .unwrap();
    let req2 = manager
        .submit_request(1, BlockOpCode::Write, 1, 1, vec![0u8; 512], 101)
        .unwrap();

    assert!(req1 > 0);
    assert!(req2 > 0);

    let err = manager.submit_request(1, BlockOpCode::Read, 2, 1, vec![], 102);
    assert_eq!(err, Err(BlockError::RequestExhaustion));
}

#[test]
fn test_exact_once_completion_and_double_completion_prevention() {
    let mut manager = BoundedBlockDeviceManager::new(4);
    let ssd = SsdBlockDevice::new(1, 512, 100);
    manager.register_device(Box::new(ssd));

    let req_id = manager
        .submit_request(1, BlockOpCode::Read, 0, 1, vec![], 100)
        .unwrap();

    let dispatched = manager.dispatch_next().unwrap();
    assert_eq!(dispatched, Some(req_id));

    assert_eq!(manager.complete_request(req_id, true), Ok(()));
    assert_eq!(
        manager.complete_request(req_id, true),
        Err(BlockError::DoubleCompletion)
    );

    let reaped = manager.reap_completed(req_id).unwrap();
    assert_eq!(reaped.state, RequestState::Completed);
}

#[test]
fn test_virtio_block_device_lifecycle() {
    let mut dev = VirtIoBlockDevice::new(10, 512, 2048, 16);
    assert_eq!(dev.device_id(), 10);
    assert_eq!(dev.device_class(), DeviceClass::VirtIoBlock);
    assert_eq!(dev.block_size(), 512);
    assert_eq!(dev.total_blocks(), 2048);

    let write_buf = vec![0xDEu8; 512];
    assert!(dev.write_block(0, &write_buf).is_ok());

    let mut read_buf = vec![0u8; 512];
    assert!(dev.read_block(0, &mut read_buf).is_ok());
    assert_eq!(read_buf, write_buf);
}
