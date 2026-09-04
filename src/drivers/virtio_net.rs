// use crate::drivers::virtio::VirtioTransport;
use std::vec::Vec;

pub struct VirtioNet {
    // transport: VirtioTransport,
    mac_address: [u8; 6],
}

impl VirtioNet {
    // pub fn new(transport: VirtioTransport) -> Self {
    //     Self { transport, mac_address: [0; 6] }
    // }
    
    pub fn new() -> Self {
        Self { mac_address: [0; 6] }
    }
    
    pub fn transmit(&mut self, packet: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }
    
    pub fn receive(&mut self) -> Option<Vec<u8>> {
        None
    }
    
    pub fn enable_checksum_offload(&mut self) {}
    pub fn enable_mergeable_rx_buffers(&mut self) {}
}
