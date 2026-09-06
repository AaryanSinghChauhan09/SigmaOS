#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::vec::Vec;
use std::string::String;

pub struct SysFs {
    nodes: Vec<String>,
}

impl SysFs {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
    
    pub fn read_device_tree(&self) -> String {
        String::from("/sys/devices/pci0000:00\n")
    }
    
    pub fn read_class(&self) -> String {
        String::from("/sys/class/net\n")
    }
    
    pub fn read_block(&self) -> String {
        String::from("/sys/block/sda\n")
    }
    
    pub fn read_kernel(&self) -> String {
        String::from("/sys/kernel/profiling\n")
    }
    
    pub fn read_power(&self) -> String {
        String::from("/sys/power/state\n")
    }
}
