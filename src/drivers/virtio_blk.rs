#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
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
