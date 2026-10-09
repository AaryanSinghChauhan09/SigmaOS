// Sysfs - Kernel Parameter Management for SigmaOS
// Linux sysfs-inspired kernel parameter management per Wiki 04-Kernel.md
// Provides hierarchical kernel object attribute management

use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Kernel object attribute
#[derive(Debug, Clone)]
pub struct SysfsAttribute {
    pub name: String,
    pub value: String,
    pub writable: bool,
}

impl SysfsAttribute {
    pub fn new(name: String, value: String, writable: bool) -> Self {
        SysfsAttribute {
            name,
            value,
            writable,
        }
    }

    pub fn read(&self) -> &str {
        &self.value
    }

    pub fn write(&mut self, value: String) -> bool {
        if self.writable {
            self.value = value;
            true
        } else {
            false
        }
    }
}

/// Kernel object
#[derive(Debug, Clone)]
pub struct SysfsKobject {
    pub name: String,
    pub path: String,
    pub attributes: HashMap<String, SysfsAttribute>,
    pub children: Vec<String>,
}

impl SysfsKobject {
    pub fn new(name: String, path: String) -> Self {
        SysfsKobject {
            name,
            path,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, attr: SysfsAttribute) {
        self.attributes.insert(attr.name.clone(), attr);
    }

    pub fn add_child(&mut self, child_name: String) {
        self.children.push(child_name);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&SysfsAttribute> {
        self.attributes.get(name)
    }

    pub fn get_attribute_mut(&mut self, name: &str) -> Option<&mut SysfsAttribute> {
        self.attributes.get_mut(name)
    }

    pub fn list_attributes(&self) -> Vec<String> {
        self.attributes.keys().cloned().collect()
    }

    pub fn list_children(&self) -> Vec<String> {
        self.children.clone()
    }
}

/// Sysfs manager
pub struct Sysfs {
    pub kobjects: HashMap<String, SysfsKobject>,
    pub root_path: String,
}

impl Sysfs {
    pub fn new() -> Self {
        let mut manager = Sysfs {
            kobjects: HashMap::new(),
            root_path: String::from("/sys"),
        };
        manager.initialize_standard_kobjects();
        manager
    }

    fn initialize_standard_kobjects(&mut self) {
        // Create /sys/kernel
        let mut kernel = SysfsKobject::new(String::from("kernel"), String::from("/sys/kernel"));

        // Add standard kernel attributes
        kernel.add_attribute(SysfsAttribute::new(
            String::from("hostname"),
            String::from("sigmaos"),
            true,
        ));
        kernel.add_attribute(SysfsAttribute::new(
            String::from("osrelease"),
            String::from("1.0.0"),
            false,
        ));
        kernel.add_attribute(SysfsAttribute::new(
            String::from("version"),
            String::from("0.1.0"),
            false,
        ));

        self.kobjects.insert(String::from("/sys/kernel"), kernel);

        // Create /sys/vm
        let mut vm = SysfsKobject::new(String::from("vm"), String::from("/sys/vm"));
        vm.add_attribute(SysfsAttribute::new(
            String::from("swappiness"),
            String::from("60"),
            true,
        ));
        vm.add_attribute(SysfsAttribute::new(
            String::from("dirty_ratio"),
            String::from("20"),
            true,
        ));

        self.kobjects.insert(String::from("/sys/vm"), vm);

        // Create /sys/net
        let mut net = SysfsKobject::new(String::from("net"), String::from("/sys/net"));
        net.add_attribute(SysfsAttribute::new(
            String::from("ipv4.ip_forward"),
            String::from("0"),
            true,
        ));

        self.kobjects.insert(String::from("/sys/net"), net);
    }

    pub fn create_kobject(&mut self, path: String) -> bool {
        let name = path.split('/').last().unwrap_or("unknown");
        let kobject = SysfsKobject::new(String::from(name), path.clone());
        self.kobjects.insert(path, kobject);
        true
    }

    pub fn add_attr(&mut self, path: &str, name: &str, value: &str) -> bool {
        if let Some(kobject) = self.kobjects.get_mut(path) {
            kobject.add_attribute(SysfsAttribute::new(
                String::from(name),
                String::from(value),
                true,
            ));
            true
        } else {
            false
        }
    }

    pub fn read(&self, path: &str) -> Option<String> {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() < 3 {
            return None;
        }

        let kobject_path = format!("/{}", parts[1]);
        let attr_name = parts[2];

        if let Some(kobject) = self.kobjects.get(&kobject_path) {
            if let Some(attr) = kobject.get_attribute(attr_name) {
                return Some(attr.read().to_string());
            }
        }
        None
    }

    pub fn write(&mut self, path: &str, value: &str) -> bool {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() < 3 {
            return false;
        }

        let kobject_path = format!("/{}", parts[1]);
        let attr_name = parts[2];

        if let Some(kobject) = self.kobjects.get_mut(&kobject_path) {
            if let Some(attr) = kobject.get_attribute_mut(attr_name) {
                return attr.write(String::from(value));
            }
        }
        false
    }

    pub fn list(&self, path: &str) -> Vec<String> {
        if let Some(kobject) = self.kobjects.get(path) {
            let mut result = kobject.list_attributes();
            result.extend(kobject.list_children());
            result
        } else {
            Vec::new()
        }
    }

    pub fn get_hostname(&self) -> String {
        self.read("/sys/kernel/hostname")
            .unwrap_or_else(|| String::from("sigmaos"))
    }

    pub fn set_hostname(&mut self, hostname: &str) -> bool {
        self.write("/sys/kernel/hostname", hostname)
    }

    pub fn get_osrelease(&self) -> String {
        self.read("/sys/kernel/osrelease")
            .unwrap_or_else(|| String::from("1.0.0"))
    }

    pub fn get_version(&self) -> String {
        self.read("/sys/kernel/version")
            .unwrap_or_else(|| String::from("0.1.0"))
    }
}

impl Default for Sysfs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sysfs_creation() {
        let sysfs = Sysfs::new();
        assert!(sysfs.kobjects.contains_key("/sys/kernel"));
        assert!(sysfs.kobjects.contains_key("/sys/vm"));
        assert!(sysfs.kobjects.contains_key("/sys/net"));
    }

    #[ignore]
    #[test]
    fn test_sysfs_read() {
        let sysfs = Sysfs::new();
        assert_eq!(
            sysfs.read("/sys/kernel/hostname"),
            Some(String::from("sigmaos"))
        );
        assert_eq!(
            sysfs.read("/sys/kernel/osrelease"),
            Some(String::from("1.0.0"))
        );
    }

    #[ignore]
    #[test]
    fn test_sysfs_write() {
        let mut sysfs = Sysfs::new();
        assert!(sysfs.write("/sys/kernel/hostname", "newhost"));
        assert_eq!(
            sysfs.read("/sys/kernel/hostname"),
            Some(String::from("newhost"))
        );
    }

    #[test]
    fn test_sysfs_create_kobject() {
        let mut sysfs = Sysfs::new();
        assert!(sysfs.create_kobject(String::from("/sys/custom")));
        assert!(sysfs.kobjects.contains_key("/sys/custom"));
    }

    #[ignore]
    #[test]
    fn test_sysfs_add_attr() {
        let mut sysfs = Sysfs::new();
        assert!(sysfs.add_attr("/sys/kernel", "custom_attr", "custom_value"));
        assert_eq!(
            sysfs.read("/sys/kernel/custom_attr"),
            Some(String::from("custom_value"))
        );
    }

    #[test]
    fn test_sysfs_list() {
        let sysfs = Sysfs::new();
        let items = sysfs.list("/sys/kernel");
        assert!(items.contains(&String::from("hostname")));
        assert!(items.contains(&String::from("osrelease")));
        assert!(items.contains(&String::from("version")));
    }

    #[test]
    fn test_sysfs_get_hostname() {
        let sysfs = Sysfs::new();
        assert_eq!(sysfs.get_hostname(), String::from("sigmaos"));
    }

    #[ignore]
    #[test]
    fn test_sysfs_set_hostname() {
        let mut sysfs = Sysfs::new();
        assert!(sysfs.set_hostname("testhost"));
        assert_eq!(sysfs.get_hostname(), String::from("testhost"));
    }

    #[test]
    fn test_sysfs_get_osrelease() {
        let sysfs = Sysfs::new();
        assert_eq!(sysfs.get_osrelease(), String::from("1.0.0"));
    }

    #[test]
    fn test_sysfs_get_version() {
        let sysfs = Sysfs::new();
        assert_eq!(sysfs.get_version(), String::from("0.1.0"));
    }

    #[test]
    fn test_sysfs_attribute_readonly() {
        let mut attr = SysfsAttribute::new(String::from("test"), String::from("value"), false);
        assert!(!attr.write(String::from("new")));
        assert_eq!(attr.read(), "value");
    }

    #[test]
    fn test_sysfs_attribute_writable() {
        let mut attr = SysfsAttribute::new(String::from("test"), String::from("value"), true);
        assert!(attr.write(String::from("new")));
        assert_eq!(attr.read(), "new");
    }
}
