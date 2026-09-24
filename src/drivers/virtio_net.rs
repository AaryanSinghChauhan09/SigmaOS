/// SigmaOS VirtIO Network Device Driver (Phase 2 Networking)
/// Inspired by Linux virtio_net.c
/// Implements RX/TX VirtQueues for network packet transmission.

use std::vec::Vec;
use crate::drivers::virtio_blk::{VirtioError, VirtioQueue};

#[derive(Debug)]
pub struct MacAddress(pub [u8; 6]);

pub struct VirtioNet {
    pub mac: MacAddress,
    pub rx_queue: VirtioQueue,
    pub tx_queue: VirtioQueue,
    pub device_ready: bool,
}

impl VirtioNet {
    pub fn new() -> Self {
        Self {
            mac: MacAddress([0; 6]),
            rx_queue: VirtioQueue::new(256),
            tx_queue: VirtioQueue::new(256),
            device_ready: false,
        }
    }

    pub fn init(&mut self) -> Result<(), VirtioError> {
        // Driver handshake sequence
        self.device_ready = true;
        self.mac = MacAddress([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]); // Mock QEMU MAC
        Ok(())
    }

    pub fn transmit(&mut self, packet: &[u8]) -> Result<(), VirtioError> {
        if !self.device_ready {
            return Err(VirtioError::DeviceNotReady);
        }
        if self.tx_queue.num_free < 2 {
            return Err(VirtioError::QueueFull);
        }
        
        self.tx_queue.num_free -= 2;
        // Mock hardware TX complete
        self.tx_queue.num_free += 2;
        Ok(())
    }

    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize, VirtioError> {
        if !self.device_ready {
            return Err(VirtioError::DeviceNotReady);
        }
        // Mock returning 0 bytes read
        Ok(0)
    }
}
