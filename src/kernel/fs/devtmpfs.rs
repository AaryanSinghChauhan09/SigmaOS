#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

/// SigmaOS devtmpfs (/dev) pseudo-filesystem
/// Automatically registers and creates device files when drivers boot
use crate::klib::HashMap;
use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Char,
    Block,
}

pub struct DeviceNode {
    pub name: String,
    pub class: DeviceClass,
    pub major: u32,
    pub minor: u32,
}

pub struct DevTmpFs {
    devices: HashMap<String, DeviceNode>,
}

impl DevTmpFs {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DevTmpFs {
            devices: HashMap::new(),
        }
    }

    pub fn register_device(
        &mut self,
        name: &str,
        class: DeviceClass,
        major: u32,
        minor: u32,
    ) -> Result<(), &'static str> {
        if self.devices.contains_key(name) {
            return Err("Device already registered in /dev");
        }
        let dev = DeviceNode {
            name: name.to_string(),
            class,
            major,
            minor,
        };
        self.devices.insert(name.to_string(), dev);
        Ok(())
    }

    pub fn unregister_device(&mut self, name: &str) -> Result<(), &'static str> {
        if self.devices.remove(name).is_some() {
            Ok(())
        } else {
            Err("Device not found in /dev")
        }
    }

    pub fn get_device(&self, name: &str) -> Option<&DeviceNode> {
        self.devices.get(name)
    }
}

impl Default for DevTmpFs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devtmpfs() {
        let mut dev = DevTmpFs::new();
        dev.register_device("null", DeviceClass::Char, 1, 3)
            .unwrap();
        dev.register_device("sda", DeviceClass::Block, 8, 0)
            .unwrap();

        assert_eq!(dev.get_device("null").unwrap().major, 1);
        assert_eq!(dev.get_device("sda").unwrap().class, DeviceClass::Block);

        dev.unregister_device("null").unwrap();
        assert!(dev.get_device("null").is_none());
    }
}
