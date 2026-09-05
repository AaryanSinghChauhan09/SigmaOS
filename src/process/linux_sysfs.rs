use std::string::{String, ToString};
// Sovereign Sysfs (/sys) and Loopback Device Subsystem
// Dynamic hardware attribute representation and loopback block overlay mounting inspired by Linux.

use crate::klib::HashMap;

#[derive(Debug, Clone)]
pub struct SysfsAttribute {
    pub path: String,
    pub value: String,
    pub is_writable: bool,
}

#[derive(Debug, Clone)]
pub struct LoopDevice {
    pub dev_index: usize,          // e.g. /dev/loop0
    pub backing_file_path: String, // backing image file path
    pub offset: u64,
    pub size_limit: u64,
    pub is_read_only: bool,
}

impl LoopDevice {
    pub fn new(dev_index: usize, backing_file_path: &str) -> Self {
        Self {
            dev_index,
            backing_file_path: backing_file_path.to_string(),
            offset: 0,
            size_limit: 0,
            is_read_only: false,
        }
    }
}

pub struct SysfsRegistry {
    pub attributes: HashMap<String, SysfsAttribute>,
    pub loop_devices: HashMap<usize, LoopDevice>,
}

impl SysfsRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            attributes: HashMap::new(),
            loop_devices: HashMap::new(),
        };

        registry.seed_sysfs_defaults();
        registry
    }

    fn seed_sysfs_defaults(&mut self) {
        // 1. CPU online states
        self.attributes.insert(
            "sys/devices/system/cpu/cpu0/online".to_string(),
            SysfsAttribute {
                path: "sys/devices/system/cpu/cpu0/online".to_string(),
                value: "1".to_string(),
                is_writable: false,
            },
        );
        self.attributes.insert(
            "sys/devices/system/cpu/cpu1/online".to_string(),
            SysfsAttribute {
                path: "sys/devices/system/cpu/cpu1/online".to_string(),
                value: "1".to_string(),
                is_writable: true, // Writable to hot-unplug cores!
            },
        );

        // 2. Battery status (Power supply class)
        self.attributes.insert(
            "sys/class/power_supply/BAT0/capacity".to_string(),
            SysfsAttribute {
                path: "sys/class/power_supply/BAT0/capacity".to_string(),
                value: "84".to_string(), // 84% battery
                is_writable: false,
            },
        );
        self.attributes.insert(
            "sys/class/power_supply/BAT0/status".to_string(),
            SysfsAttribute {
                path: "sys/class/power_supply/BAT0/status".to_string(),
                value: "Discharging".to_string(),
                is_writable: false,
            },
        );
    }

    /// Read dynamic hardware attribute from sysfs
    pub fn read_attribute(&self, path: &str) -> Result<String, &'static str> {
        let clean_path = path.trim_start_matches('/');
        self.attributes
            .get(clean_path)
            .map(|attr| attr.value.clone())
            .ok_or("sysfs attribute not found")
    }

    /// Write and update sysfs hardware parameters (e.g. hot-unplugging a CPU)
    pub fn write_attribute(&mut self, path: &str, new_value: &str) -> Result<(), &'static str> {
        let clean_path = path.trim_start_matches('/');
        let attr = self
            .attributes
            .get_mut(clean_path)
            .ok_or("sysfs attribute not found")?;
        if !attr.is_writable {
            return Err("Permission denied: sysfs attribute is read-only");
        }
        attr.value = new_value.to_string();
        Ok(())
    }

    /// Configure and mount a new loopback device /dev/loopX
    pub fn mount_loop_device(
        &mut self,
        dev_index: usize,
        backing_file_path: &str,
    ) -> Result<(), &'static str> {
        if self.loop_devices.contains_key(&dev_index) {
            return Err("Loopback device index already in use");
        }
        let device = LoopDevice::new(dev_index, backing_file_path);
        self.loop_devices.insert(dev_index, device);
        Ok(())
    }

    /// Detach loopback device
    pub fn detach_loop_device(&mut self, dev_index: usize) -> Result<(), &'static str> {
        self.loop_devices
            .remove(&dev_index)
            .map(|_| ())
            .ok_or("Loopback device not found")
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_read_write() {
        let mut registry = SysfsRegistry::new();

        // 1. Read battery status
        let bat = registry
            .read_attribute("/sys/class/power_supply/BAT0/capacity")
            .unwrap();
        assert_eq!(bat, "84");

        // 2. Write to read-only attribute -> fails
        assert!(registry
            .write_attribute("/sys/class/power_supply/BAT0/capacity", "90")
            .is_err());

        // 3. Write to writable attribute -> succeeds
        assert!(registry
            .write_attribute("/sys/devices/system/cpu/cpu1/online", "0")
            .is_ok());
        let cpu_state = registry
            .read_attribute("/sys/devices/system/cpu/cpu1/online")
            .unwrap();
        assert_eq!(cpu_state, "0");
    }

    #[test]
    fn test_loop_device_mount_lifecycle() {
        let mut registry = SysfsRegistry::new();

        // Mount a custom system overlay image
        assert!(registry
            .mount_loop_device(0, "/var/images/ubuntu_root.img")
            .is_ok());
        // Duplicate mount index -> fails
        assert!(registry
            .mount_loop_device(0, "/var/images/other.img")
            .is_err());

        let dev = registry.loop_devices.get(&0).unwrap();
        assert_eq!(dev.backing_file_path, "/var/images/ubuntu_root.img");
        assert!(!dev.is_read_only);

        // Detach
        assert!(registry.detach_loop_device(0).is_ok());
        assert!(registry.loop_devices.get(&0).is_none());
    }
}
