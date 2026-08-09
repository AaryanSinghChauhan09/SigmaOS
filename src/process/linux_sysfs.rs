/// Linux-grade Sysfs (/sys) Pseudo-Filesystem and Device Parameter Subsystem for SigmaOS
/// Replicates structured system class, bus, kernel, and power supply telemetry in plain-text format.

use crate::klib::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysfsPermission {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// Represents a plain-text virtual attribute file in the sysfs hierarchy (e.g. /sys/kernel/secureboot)
pub struct SysfsAttribute {
    pub path: [u8; 64],
    pub value: [u8; 32],
    pub permission: SysfsPermission,
}

impl SysfsAttribute {
    pub fn new(path: &[u8], initial_val: &[u8], permission: SysfsPermission) -> Self {
        let mut path_arr = [0u8; 64];
        let mut val_arr = [0u8; 32];

        path_arr[..path.len().min(63)].copy_from_slice(&path[..path.len().min(63)]);
        val_arr[..initial_val.len().min(31)].copy_from_slice(&initial_val[..initial_val.len().min(31)]);

        SysfsAttribute {
            path: path_arr,
            value: val_arr,
            permission,
        }
    }

    pub fn path_len(&self) -> usize {
        self.path.iter().position(|&b| b == 0).unwrap_or(64)
    }

    pub fn value_len(&self) -> usize {
        self.value.iter().position(|&b| b == 0).unwrap_or(32)
    }

    pub fn read_value(&self) -> Result<&[u8], &'static str> {
        if self.permission == SysfsPermission::WriteOnly {
            return Err("Permission Denied: Write-only attribute");
        }
        Ok(&self.value[..self.value_len()])
    }

    pub fn write_value(&mut self, new_val: &[u8]) -> Result<(), &'static str> {
        if self.permission == SysfsPermission::ReadOnly {
            return Err("Permission Denied: Read-only attribute");
        }
        let len = new_val.len().min(31);
        self.value = [0u8; 32];
        self.value[..len].copy_from_slice(&new_val[..len]);
        Ok(())
    }
}

/// Centralized manager for the virtual /sys directory hierarchy
pub struct SysfsManager {
    pub attributes: Vec<SysfsAttribute>,
}

impl Default for SysfsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SysfsManager {
    pub fn new() -> Self {
        let mut manager = SysfsManager {
            attributes: Vec::new(),
        };
        manager.initialize_default_sys_tree();
        manager
    }

    /// Pre-populate structured virtual endpoints mimicking common Linux sysfs structures
    fn initialize_default_sys_tree(&mut self) {
        // CPU status
        self.register_attribute(b"/sys/devices/system/cpu/cpu0/online", b"1", SysfsPermission::ReadOnly);
        self.register_attribute(b"/sys/devices/system/cpu/cpu0/microcode/version", b"0xdeadbeef", SysfsPermission::ReadOnly);

        // Power supply (battery telemetry)
        self.register_attribute(b"/sys/class/power_supply/BAT0/capacity", b"98", SysfsPermission::ReadOnly);
        self.register_attribute(b"/sys/class/power_supply/BAT0/status", b"Charging", SysfsPermission::ReadOnly);

        // Kernel parameters
        self.register_attribute(b"/sys/kernel/security/secureboot", b"1", SysfsPermission::ReadOnly);
        self.register_attribute(b"/sys/module/sigma_kernel/parameters/debug_level", b"3", SysfsPermission::ReadWrite);
    }

    pub fn register_attribute(&mut self, path: &[u8], val: &[u8], perm: SysfsPermission) {
        self.attributes.push(SysfsAttribute::new(path, val, perm));
    }

    pub fn read_attribute(&self, path: &[u8]) -> Result<&[u8], &'static str> {
        for i in 0..self.attributes.len() {
            let attr = &self.attributes[i];
            let len = attr.path_len();
            if len == path.len() && &attr.path[..len] == path {
                return attr.read_value();
            }
        }
        Err("Attribute path not found")
    }

    pub fn write_attribute(&mut self, path: &[u8], new_val: &[u8]) -> Result<(), &'static str> {
        for i in 0..self.attributes.len() {
            let attr = &mut self.attributes[i];
            let len = attr.path_len();
            if len == path.len() && &attr.path[..len] == path {
                return attr.write_value(new_val);
            }
        }
        Err("Attribute path not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_attribute_read_write() {
        let mut attr = SysfsAttribute::new(b"/sys/kernel/test", b"hello", SysfsPermission::ReadWrite);
        assert_eq!(attr.read_value().unwrap(), b"hello");

        assert!(attr.write_value(b"world_mod").is_ok());
        assert_eq!(attr.read_value().unwrap(), b"world_mod");
    }

    #[test]
    fn test_sysfs_manager_tree_queries() {
        let mut manager = SysfsManager::new();

        // Query default pre-populated CPU online attribute
        let cpu_online = manager.read_attribute(b"/sys/devices/system/cpu/cpu0/online").unwrap();
        assert_eq!(cpu_online, b"1");

        // Query default Battery capacity
        let bat_cap = manager.read_attribute(b"/sys/class/power_supply/BAT0/capacity").unwrap();
        assert_eq!(bat_cap, b"98");

        // Write and Read mutable kernel parameter
        assert!(manager.write_attribute(b"/sys/module/sigma_kernel/parameters/debug_level", b"5").is_ok());
        let debug_level = manager.read_attribute(b"/sys/module/sigma_kernel/parameters/debug_level").unwrap();
        assert_eq!(debug_level, b"5");

        // Attempting to write to read-only attribute must fail
        assert!(manager.write_attribute(b"/sys/class/power_supply/BAT0/capacity", b"50").is_err());
    }
}
