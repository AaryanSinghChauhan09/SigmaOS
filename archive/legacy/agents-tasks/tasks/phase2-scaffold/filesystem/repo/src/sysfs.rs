// SPDX-License-Identifier: MIT
// SigmaOS - sysfs (Kernel Object Hierarchy) Implementation
// Inspired by Linux sysfs (fs/sysfs/) providing kernel object representation
// in a virtual filesystem mounted at /sys

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Sysfs Attribute (kobject attribute - readable/writable kernel data)
#[derive(Debug, Clone)]
pub struct SysfsAttribute {
    pub name: String,
    pub mode: u32, // File permissions (e.g., 0644)
    pub value: String,
    pub is_writable: bool,
}

/// Sysfs Kobject (Kernel Object - represents device, driver, or kernel subsystem)
#[derive(Debug, Clone)]
pub struct SysfsKobject {
    pub name: String,
    pub ktype: String, // kobject type (e.g., "device", "driver", "module")
    pub parent: Option<String>,
    pub attributes: BTreeMap<String, SysfsAttribute>,
    pub children: BTreeMap<String, SysfsKobject>,
}

impl SysfsKobject {
    pub fn new(name: &str, ktype: &str) -> Self {
        Self {
            name: name.to_string(),
            ktype: ktype.to_string(),
            parent: None,
            attributes: BTreeMap::new(),
            children: BTreeMap::new(),
        }
    }

    pub fn add_attribute(&mut self, name: &str, value: &str, is_writable: bool) {
        self.attributes.insert(
            name.to_string(),
            SysfsAttribute {
                name: name.to_string(),
                mode: if is_writable { 0o666 } else { 0o444 },
                value: value.to_string(),
                is_writable,
            },
        );
    }

    pub fn add_child(&mut self, child: SysfsKobject) {
        let child_name = child.name.clone();
        self.children.insert(child_name, child);
    }
}

/// Sysfs Virtual Filesystem Root
pub struct SysFs {
    pub root: SysfsKobject,
}

impl SysFs {
    pub fn new() -> Self {
        let mut root = SysfsKobject::new("", "sysfs_root");

        // Initialize standard sysfs hierarchy
        self.initialize_kernel_subsystem(&mut root);
        self.initialize_devices_subsystem(&mut root);
        self.initialize_class_subsystem(&mut root);
        self.initialize_block_subsystem(&mut root);
        self.initialize_power_subsystem(&mut root);
        self.initialize_bus_subsystem(&mut root);
        self.initialize_firmware_subsystem(&mut root);
        self.initialize_module_subsystem(&mut root);

        Self { root }
    }

    fn initialize_kernel_subsystem(&self, root: &mut SysfsKobject) {
        let mut kernel = SysfsKobject::new("kernel", "subsystem");
        kernel.add_attribute("uevent_seqnum", "1234", false);
        kernel.add_attribute("profiling", "0", true);
        kernel.add_attribute("fscaps", "0", false);
        kernel.add_attribute("rcu_expedited", "0", true);
        kernel.add_attribute("rcu_normal", "1", true);
        kernel.add_attribute("seccomp", "1", false);
        kernel.add_attribute("suid_dumpable", "0", true);
        root.add_child(kernel);
    }

    fn initialize_devices_subsystem(&self, root: &mut SysfsKobject) {
        let mut devices = SysfsKobject::new("devices", "subsystem");

        // Platform devices (simplified)
        let mut platform = SysfsKobject::new("platform", "bus");
        let mut serial8250 = SysfsKobject::new("serial8250", "device");
        serial8250.add_attribute("modalias", "platform:serial8250", false);
        serial8250.add_attribute("driver_override", "", true);
        platform.add_child(serial8250);
        devices.add_child(platform);

        // PCI devices (simplified example)
        let mut pci0000_00 = SysfsKobject::new("pci0000:00", "device");
        pci0000_00.add_attribute("local_cpus", "0-3", false);
        pci0000_00.add_attribute("local_cpulist", "0-3", false);
        devices.add_child(pci0000_00);

        root.add_child(devices);
    }

    fn initialize_class_subsystem(&self, root: &mut SysfsKobject) {
        let mut class = SysfsKobject::new("class", "subsystem");

        // Network class
        let mut net = SysfsKobject::new("net", "class");
        let mut eth0 = SysfsKobject::new("eth0", "device");
        eth0.add_attribute("address", "52:54:00:12:34:56", false);
        eth0.add_attribute("broadcast", "ff:ff:ff:ff:ff:ff", false);
        eth0.add_attribute("carrier", "1", false);
        eth0.add_attribute("duplex", "full", false);
        eth0.add_attribute("mtu", "1500", true);
        eth0.add_attribute("operstate", "up", false);
        eth0.add_attribute("speed", "1000", false);
        net.add_child(eth0);
        class.add_child(net);

        // Misc class
        let mut misc = SysfsKobject::new("misc", "class");
        class.add_child(misc);

        root.add_child(class);
    }

    fn initialize_block_subsystem(&self, root: &mut SysfsKobject) {
        let mut block = SysfsKobject::new("block", "subsystem");

        let mut sda = SysfsKobject::new("sda", "device");
        sda.add_attribute("size", "20971520", false);
        sda.add_attribute("device", "8:0", false);
        sda.add_attribute("removable", "0", false);
        sda.add_attribute("ro", "0", false);
        sda.add_attribute("range", "16", false);
        sda.add_attribute("ext_range", "256", false);
        sda.add_attribute("discard_alignment", "0", false);
        sda.add_attribute("cache_type", "write back", false);
        block.add_child(sda);

        root.add_child(block);
    }

    fn initialize_power_subsystem(&self, root: &mut SysfsKobject) {
        let mut power = SysfsKobject::new("power", "subsystem");
        power.add_attribute("state", "mem", true);
        power.add_attribute("control", "auto", true);
        power.add_attribute("wakeup_count", "0", false);
        power.add_attribute("autosuspend_delay_ms", "2000", true);
        power.add_attribute("runtime_active_time", "0", false);
        power.add_attribute("runtime_suspended_time", "0", false);
        power.add_attribute("runtime_status", "unsupported", false);
        root.add_child(power);
    }

    fn initialize_bus_subsystem(&self, root: &mut SysfsKobject) {
        let mut bus = SysfsKobject::new("bus", "subsystem");

        // PCI bus
        let mut pci = SysfsKobject::new("pci", "bus");
        pci.add_attribute("devices_autoprobe", "1", true);
        pci.add_attribute("rescan", "", true);
        bus.add_child(pci);

        // Platform bus
        let mut platform = SysfsKobject::new("platform", "bus");
        bus.add_child(platform);

        root.add_child(bus);
    }

    fn initialize_firmware_subsystem(&self, root: &mut SysfsKobject) {
        let mut firmware = SysfsKobject::new("firmware", "subsystem");

        let mut acpi = SysfsKobject::new("acpi", "subsystem");
        acpi.add_attribute("fadt", "1", false);
        acpi.add_attribute("dsdt", "1", false);
        firmware.add_child(acpi);

        let mut memmap = SysfsKobject::new("memmap", "subsystem");
        firmware.add_child(memmap);

        root.add_child(firmware);
    }

    fn initialize_module_subsystem(&self, root: &mut SysfsKobject) {
        let mut module = SysfsKobject::new("module", "subsystem");

        // Example loaded modules
        let mut kernel_module = SysfsKobject::new("kernel", "module");
        kernel_module.add_attribute("refcnt", "1", false);
        kernel_module.add_attribute("taint", "", false);
        kernel_module.add_attribute("coresize", "16384", false);
        kernel_module.add_attribute("initsize", "0", false);
        module.add_child(kernel_module);

        root.add_child(module);
    }

    /// Navigate sysfs hierarchy and read attribute value
    pub fn read_attribute(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return None;
        }

        let mut current = &self.root;
        let mut i = 0;

        // Navigate through kobjects
        while i < parts.len() - 1 {
            if let Some(child) = current.children.get(parts[i]) {
                current = child;
                i += 1;
            } else {
                return None;
            }
        }

        // Try to read as attribute
        if let Some(attr) = current.attributes.get(parts[i]) {
            Some(attr.value.clone())
        } else {
            None
        }
    }

    /// Write to a writable sysfs attribute
    pub fn write_attribute(&mut self, path: &str, value: &str) -> Result<(), String> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return Err("Invalid path".to_string());
        }

        let current = self.navigate_to_parent(&parts, parts.len() - 1)?;
        let attr_name = parts[parts.len() - 1];

        if let Some(attr) = current.attributes.get_mut(attr_name) {
            if attr.is_writable {
                attr.value = value.to_string();
                Ok(())
            } else {
                Err(format!("Attribute {} is not writable", attr_name))
            }
        } else {
            Err(format!("Attribute {} not found", attr_name))
        }
    }

    /// Helper to navigate to parent kobject
    fn navigate_to_parent(&mut self, parts: &[&str], depth: usize) -> Result<&mut SysfsKobject, String> {
        // This is a simplified implementation - in a real VFS, we'd have proper path resolution
        Ok(&mut self.root)
    }

    /// List children of a kobject (for directory listing)
    pub fn list_children(&self, path: &str) -> Vec<String> {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current = &self.root;

        for part in parts {
            if let Some(child) = current.children.get(part) {
                current = child;
            } else {
                return Vec::new();
            }
        }

        let mut result = Vec::new();
        for name in current.children.keys() {
            result.push(name.clone());
        }
        for name in current.attributes.keys() {
            result.push(name.clone());
        }
        result
    }
}

impl Default for SysFs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_initialization() {
        let sysfs = SysFs::new();
        assert!(sysfs.root.children.contains_key("kernel"));
        assert!(sysfs.root.children.contains_key("devices"));
        assert!(sysfs.root.children.contains_key("class"));
        assert!(sysfs.root.children.contains_key("block"));
        assert!(sysfs.root.children.contains_key("power"));
    }

    #[test]
    fn test_sysfs_read_attribute() {
        let sysfs = SysFs::new();
        let profiling = sysfs.read_attribute("kernel/profiling");
        assert_eq!(profiling, Some("0".to_string()));

        let state = sysfs.read_attribute("power/state");
        assert_eq!(state, Some("mem".to_string()));
    }

    #[test]
    fn test_sysfs_list_children() {
        let sysfs = SysFs::new();
        let kernel_children = sysfs.list_children("kernel");
        assert!(kernel_children.contains(&"uevent_seqnum".to_string()));
        assert!(kernel_children.contains(&"profiling".to_string()));
    }

    #[test]
    fn test_sysfs_write_attribute() {
        let mut sysfs = SysFs::new();
        let result = sysfs.write_attribute("kernel/profiling", "1");
        assert!(result.is_ok());

        let value = sysfs.read_attribute("kernel/profiling");
        assert_eq!(value, Some("1".to_string()));
    }

    #[test]
    fn test_sysfs_write_readonly_fails() {
        let mut sysfs = SysFs::new();
        let result = sysfs.write_attribute("kernel/uevent_seqnum", "9999");
        assert!(result.is_err());
    }
}
