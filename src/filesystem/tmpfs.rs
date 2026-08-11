#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use crate::filesystem::vfs::{Inode, FileType, FilePermissions};

pub struct TmpFs {
    files: BTreeMap<String, Vec<u8>>,
    max_size: usize,
    current_size: usize,
    swap_enabled: bool,
}

use alloc::string::String;

impl TmpFs {
    pub fn new(max_size: usize) -> Self {
        Self {
            files: BTreeMap::new(),
            max_size,
            current_size: 0,
            swap_enabled: true,
        }
    }

    pub fn write_file(&mut self, path: &str, data: &[u8]) -> Result<(), &'static str> {
        if self.current_size + data.len() > self.max_size {
            return Err("TmpFs out of memory");
        }
        self.files.insert(String::from(path), data.to_vec());
        self.current_size += data.len();
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Option<&Vec<u8>> {
        self.files.get(path)
    }
}
