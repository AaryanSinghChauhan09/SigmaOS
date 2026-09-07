// use crate::drivers::virtio::VirtioTransport;
use std::vec::Vec;

pub struct VirtioBlk {
    // transport: VirtioTransport,
    capacity: u64,
}

impl VirtioBlk {
    // pub fn new(transport: VirtioTransport) -> Self {
    //     Self { transport, capacity: 0 }
    // }
    
    pub fn new() -> Self {
        Self { capacity: 0 }
    }
    
    pub fn read_block(&self, sector: u64, data: &mut [u8]) -> Result<(), &'static str> {
        Ok(())
    }
    
    pub fn write_block(&mut self, sector: u64, data: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
}
