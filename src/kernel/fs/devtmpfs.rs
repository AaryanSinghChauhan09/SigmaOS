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
/// Automatically registers and creates device files when drivers boot.
/// Improved with Linux-inspired udev rules, permissions, and symlink mappings.

use crate::klib::BTreeMap;
use crate::klib::Vec;
use std::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Char,
    Block,
}

#[derive(Debug, Clone)]
pub struct DeviceNode {
    pub name: String,
    pub class: DeviceClass,
    pub major: u32,
    pub minor: u32,
    pub mode: u32,       // Unix permission mode (e.g., 0o666)
    pub uid: u32,        // Owner User ID
    pub gid: u32,        // Owner Group ID
    pub symlinks: Vec<String>, // Symlink aliases inside /dev
}

/// A udev-style rule that maps matched devices to permissions and symlinks
#[derive(Debug, Clone)]
pub struct UdevRule {
    pub match_name_prefix: Option<String>,
    pub match_class: Option<DeviceClass>,
    pub set_mode: Option<u32>,
    pub set_uid: Option<u32>,
    pub set_gid: Option<u32>,
    pub create_symlink: Option<String>,
}

pub struct DevTmpFs {
    devices: BTreeMap<String, DeviceNode>,
    symlinks: BTreeMap<String, String>, // symlink -> target node name
    udev_rules: Vec<UdevRule>,
}

impl DevTmpFs {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DevTmpFs {
            devices: BTreeMap::new(),
            symlinks: BTreeMap::new(),
            udev_rules: Vec::new(),
        }
    }

    /// Add a dynamic udev rule to the system
    pub fn add_udev_rule(&mut self, rule: UdevRule) {
        self.udev_rules.push(rule);
    }

    /// Registers a device file in /dev and applies matching udev rules.
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

        // Default Linux-style permissions
        let mut mode = if class == DeviceClass::Char { 0o600 } else { 0o660 };
        let mut uid = 0; // root
        let mut gid = 0; // root
        let mut symlink_aliases = Vec::new();

        // Apply matching udev rules
        for rule in self.udev_rules.iter() {
            let mut matches = true;

            if let Some(ref prefix) = rule.match_name_prefix {
                if !name.starts_with(prefix) {
                    matches = false;
                }
            }

            if let Some(ref r_class) = rule.match_class {
                if *r_class != class {
                    matches = false;
                }
            }

            if matches {
                if let Some(m) = rule.set_mode {
                    mode = m;
                }
                if let Some(u) = rule.set_uid {
                    uid = u;
                }
                if let Some(g) = rule.set_gid {
                    gid = g;
                }
                if let Some(ref sym) = rule.create_symlink {
                    let formatted_sym = sym.replace("%k", name);
                    symlink_aliases.push(formatted_sym.clone());
                    self.symlinks.insert(formatted_sym, name.to_string());
                }
            }
        }

        let dev = DeviceNode {
            name: name.to_string(),
            class,
            major,
            minor,
            mode,
            uid,
            gid,
            symlinks: symlink_aliases,
        };

        self.devices.insert(name.to_string(), dev);
        Ok(())
    }

    /// Unregisters a device file and its associated symlinks.
    pub fn unregister_device(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(dev) = self.devices.remove(name) {
            for sym in dev.symlinks.iter() {
                self.symlinks.remove(sym);
            }
            Ok(())
        } else {
            Err("Device not found in /dev")
        }
    }

    /// Retrieves a device node by direct name or symlink alias.
    pub fn get_device(&self, name: &str) -> Option<&DeviceNode> {
        if let Some(target) = self.devices.get(name) {
            return Some(target);
        }
        if let Some(target_name) = self.symlinks.get(name) {
            return self.devices.get(target_name);
        }
        None
    }

    /// Populates standard Linux device nodes during initialization
    pub fn init_standard_devices(&mut self) {
        // null, zero, random, urandom, tty should be world-writable character devices (0o666)
        let _ = self.register_device_with_perms("null", DeviceClass::Char, 1, 3, 0o666, 0, 0);
        let _ = self.register_device_with_perms("zero", DeviceClass::Char, 1, 5, 0o666, 0, 0);
        let _ = self.register_device_with_perms("random", DeviceClass::Char, 1, 8, 0o666, 0, 0);
        let _ = self.register_device_with_perms("urandom", DeviceClass::Char, 1, 9, 0o666, 0, 0);
        let _ = self.register_device_with_perms("tty", DeviceClass::Char, 5, 0, 0o666, 0, 0);

        // Disk sda should be root/disk group accessible (0o660)
        let _ = self.register_device_with_perms("sda", DeviceClass::Block, 8, 0, 0o660, 0, 6); // 6 = disk group id
    }

    /// Helper to register a device node with specific permissions directly
    pub fn register_device_with_perms(
        &mut self,
        name: &str,
        class: DeviceClass,
        major: u32,
        minor: u32,
        mode: u32,
        uid: u32,
        gid: u32,
    ) -> Result<(), &'static str> {
        if self.devices.contains_key(name) {
            return Err("Device already registered in /dev");
        }

        let dev = DeviceNode {
            name: name.to_string(),
            class,
            major,
            minor,
            mode,
            uid,
            gid,
            symlinks: Vec::new(),
        };

        self.devices.insert(name.to_string(), dev);
        Ok(())
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
    fn test_devtmpfs_basic() {
        let mut dev = DevTmpFs::new();
        dev.register_device("null", DeviceClass::Char, 1, 3).unwrap();
        dev.register_device("sda", DeviceClass::Block, 8, 0).unwrap();

        assert_eq!(dev.get_device("null").unwrap().major, 1);
        assert_eq!(dev.get_device("sda").unwrap().class, DeviceClass::Block);

        dev.unregister_device("null").unwrap();
        assert!(dev.get_device("null").is_none());
    }

    #[test]
    fn test_devtmpfs_udev_rules_and_symlinks() {
        let mut dev = DevTmpFs::new();

        // Add a udev rule to match sda and sdb disks and create 'disk/by-label/%k' symlinks
        dev.add_udev_rule(UdevRule {
            match_name_prefix: Some("sd".to_string()),
            match_class: Some(DeviceClass::Block),
            set_mode: Some(0o664),
            set_uid: Some(1000),
            set_gid: Some(1001),
            create_symlink: Some("disk/by-label/%k".to_string()),
        });

        dev.register_device("sda", DeviceClass::Block, 8, 0).unwrap();

        // Verify that sda node has updated permissions and symlink alias
        let sda_node = dev.get_device("sda").unwrap();
        assert_eq!(sda_node.mode, 0o664);
        assert_eq!(sda_node.uid, 1000);
        assert_eq!(sda_node.gid, 1001);
        assert_eq!(sda_node.symlinks[0], "disk/by-label/sda");

        // Verify lookup by symlink works flawlessly
        let aliased_node = dev.get_device("disk/by-label/sda").unwrap();
        assert_eq!(aliased_node.name, "sda");
        assert_eq!(aliased_node.major, 8);
    }

    #[test]
    fn test_devtmpfs_standard_devices() {
        let mut dev = DevTmpFs::new();
        dev.init_standard_devices();

        assert!(dev.get_device("null").is_some());
        assert_eq!(dev.get_device("null").unwrap().mode, 0o666);
        assert_eq!(dev.get_device("zero").unwrap().mode, 0o666);
        assert_eq!(dev.get_device("random").unwrap().mode, 0o666);
        assert_eq!(dev.get_device("sda").unwrap().mode, 0o660);
        assert_eq!(dev.get_device("sda").unwrap().gid, 6);
    }
}
