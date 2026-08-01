/// SigmaOS sysfs-like device tree implementation
/// Exposes devices, drivers, and attributes in a structured tree
use crate::klib::HashMap;
use std::string::{String, ToString};

pub struct SysfsAttribute {
    pub name: String,
    pub value: String,
}

pub struct SysfsDeviceNode {
    pub name: String,
    pub attributes: HashMap<String, String>,
}

pub struct SysfsTree {
    devices: HashMap<String, SysfsDeviceNode>,
}

impl SysfsTree {
    pub fn new() -> Self {
        SysfsTree {
            devices: HashMap::new(),
        }
    }

    pub fn register_device(&mut self, class: &str, name: &str) {
        let path = format!("/sys/class/{}/{}", class, name);
        let node = SysfsDeviceNode {
            name: name.to_string(),
            attributes: HashMap::new(),
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
