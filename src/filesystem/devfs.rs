use std::vec::Vec;
use crate::filesystem::vfs::{Inode, FileType, FilePermissions};

pub struct DevFs {
    devices: Vec<DeviceNode>,
}

pub struct DeviceNode {
    pub name: &'static str,
    pub major: u32,
    pub minor: u32,
    pub is_block: bool,
}

impl DevFs {
    pub fn new() -> Self {
        let mut devfs = Self { devices: Vec::new() };
        devfs.register_device("null", 1, 3, false);
        devfs.register_device("zero", 1, 5, false);
        devfs.register_device("full", 1, 7, false);
        devfs.register_device("random", 1, 8, false);
        devfs.register_device("urandom", 1, 9, false);
        devfs.register_device("tty", 5, 0, false);
        devfs.register_device("console", 5, 1, false);
        devfs.register_device("stdin", 0, 0, false);
        devfs.register_device("stdout", 0, 1, false);
        devfs.register_device("stderr", 0, 2, false);
        devfs.register_device("loop0", 7, 0, true);
        devfs.register_device("sda", 8, 0, true);
        devfs.register_device("nvme0n1", 259, 0, true);
        devfs
    }

    pub fn register_device(&mut self, name: &'static str, major: u32, minor: u32, is_block: bool) {
        self.devices.push(DeviceNode { name, major, minor, is_block });
    }
}
