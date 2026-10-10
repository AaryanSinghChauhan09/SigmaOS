#[cfg(feature = "standalone_test_direct")]
#[path = "block.rs"]
mod block;

#[cfg(feature = "standalone_test_direct")]
use block::{
    BlockDeviceID, BlockError, BlockNumber, BlockOrientedDevice, DeviceClass,
};

#[cfg(not(feature = "standalone_test_direct"))]
use super::block::{
    BlockDeviceID, BlockError, BlockNumber, BlockOrientedDevice, DeviceClass,
};

use std::vec::Vec;

pub const VIRTIO_BLK_T_IN: u32 = 0;
pub const VIRTIO_BLK_T_OUT: u32 = 1;
pub const VIRTIO_BLK_T_FLUSH: u32 = 4;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct VirtQueueDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[derive(Debug, Clone)]
pub struct VirtIoBlockHeader {
    pub req_type: u32,
    pub reserved: u32,
    pub sector: u64,
}

pub struct VirtIoBlockDevice {
    pub id: BlockDeviceID,
    pub block_size: usize,
    pub total_blocks: u64,
    pub queue_size: usize,
    pub write_blocked: bool,
    pub storage: Vec<u8>,
    pub pending_descriptors: Vec<VirtQueueDesc>,
}

impl VirtIoBlockDevice {
    pub fn new(id: BlockDeviceID, block_size: usize, total_blocks: u64, queue_size: usize) -> Self {
        let total_bytes = (block_size as u64 * total_blocks) as usize;
        Self {
            id,
            block_size,
            total_blocks,
            queue_size,
            write_blocked: false,
            storage: std::vec![0u8; total_bytes],
            pending_descriptors: Vec::new(),
        }
    }

    pub fn submit_virtqueue_request(
        &mut self,
        req_type: u32,
        sector: u64,
        data_len: u32,
    ) -> Result<u16, BlockError> {
        if self.pending_descriptors.len() >= self.queue_size {
            return Err(BlockError::QueueFull);
        }

        let desc_id = self.pending_descriptors.len() as u16;
        let desc = VirtQueueDesc {
            addr: sector * self.block_size as u64,
            len: data_len,
            flags: if req_type == VIRTIO_BLK_T_IN { 1 } else { 0 },
            next: 0,
        };
        self.pending_descriptors.push(desc);
        Ok(desc_id)
    }

    pub fn process_used_ring(&mut self) -> usize {
        let count = self.pending_descriptors.len();
        self.pending_descriptors.clear();
        count
    }
}

impl BlockOrientedDevice for VirtIoBlockDevice {
    fn device_id(&self) -> BlockDeviceID {
        self.id
    }

    fn device_class(&self) -> DeviceClass {
        DeviceClass::VirtIoBlock
    }

    fn block_size(&self) -> usize {
        self.block_size
    }

    fn total_blocks(&self) -> u64 {
        self.total_blocks
    }

    fn is_write_blocked(&self) -> bool {
        self.write_blocked
    }

    fn set_write_blocked(&mut self, blocked: bool) {
        self.write_blocked = blocked;
    }

    fn read_block(&self, block_num: BlockNumber, buffer: &mut [u8]) -> Result<(), BlockError> {
        if block_num >= self.total_blocks {
            return Err(BlockError::OutOfBounds);
        }
        let offset = (block_num as usize) * self.block_size;
        let len = self.block_size.min(buffer.len());
        buffer[..len].copy_from_slice(&self.storage[offset..offset + len]);
        Ok(())
    }

    fn write_block(&mut self, block_num: BlockNumber, data: &[u8]) -> Result<(), BlockError> {
        if self.write_blocked {
            return Err(BlockError::WriteBlocked);
        }
        if block_num >= self.total_blocks {
            return Err(BlockError::OutOfBounds);
        }
        let offset = (block_num as usize) * self.block_size;
        let len = self.block_size.min(data.len());
        self.storage[offset..offset + len].copy_from_slice(&data[..len]);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), BlockError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtio_block_device_lifecycle() {
        let mut virtio = VirtIoBlockDevice::new(10, 512, 100, 16);
        assert_eq!(virtio.device_class(), DeviceClass::VirtIoBlock);

        let write_data = std::vec![0xA5u8; 512];
        assert!(virtio.write_block(2, &write_data).is_ok());

        let mut read_buf = std::vec![0u8; 512];
        assert!(virtio.read_block(2, &mut read_buf).is_ok());
        assert_eq!(read_buf, write_data);

        let desc_id = virtio.submit_virtqueue_request(VIRTIO_BLK_T_IN, 2, 512).unwrap();
        assert_eq!(desc_id, 0);

        let processed = virtio.process_used_ring();
        assert_eq!(processed, 1);
    }
}
