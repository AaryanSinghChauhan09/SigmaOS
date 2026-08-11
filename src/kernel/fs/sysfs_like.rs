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

/// SigmaOS sysfs-like device tree implementation
/// Exposes devices, drivers, and attributes in a structured tree
use crate::klib::BTreeMap;
use std::string::{String, ToString};

pub struct SysfsAttribute {
    pub name: String,
    pub value: String,
}

pub struct SysfsDeviceNode {
    pub name: String,
    pub attributes: BTreeMap<String, String>,
}

pub struct SysfsTree {
    devices: BTreeMap<String, SysfsDeviceNode>,
}

impl SysfsTree {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SysfsTree {
            devices: BTreeMap::new(),
        }
    }

    pub fn register_device(&mut self, class: &str, name: &str) {
        let path = format!("/sys/class/{}/{}", class, name);
        let node = SysfsDeviceNode {
            name: name.to_string(),
            attributes: BTreeMap::new(),
        };
        self.devices.insert(path, node);
    }

    pub fn set_attribute(
        &mut self,
        class: &str,
        name: &str,
        attr: &str,
        value: &str,
    ) -> Result<(), &'static str> {
        let path = format!("/sys/class/{}/{}", class, name);
        let node = self
            .devices
            .get_mut(&path)
            .ok_or("Device not found in sysfs")?;
        node.attributes.insert(attr.to_string(), value.to_string());
        Ok(())
    }

    pub fn read_attribute(
        &self,
        class: &str,
        name: &str,
        attr: &str,
    ) -> Result<String, &'static str> {
        let path = format!("/sys/class/{}/{}", class, name);
        let node = self.devices.get(&path).ok_or("Device not found in sysfs")?;
        let value = node.attributes.get(attr).ok_or("Attribute not found")?;
        Ok(value.clone())
    }
}

impl Default for SysfsTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_tree() {
        let mut sys = SysfsTree::new();
        sys.register_device("net", "eth0");
        sys.set_attribute("net", "eth0", "speed", "1000").unwrap();

        assert_eq!(sys.read_attribute("net", "eth0", "speed").unwrap(), "1000");
    }
}
