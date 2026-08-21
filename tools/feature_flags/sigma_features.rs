// Gentoo USE-inspired Feature Flag Definition System for SigmaOS
// Location: tools/feature_flags/sigma_features.rs

#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatureFlag {
    pub name: [u8; 64],
    pub description: [u8; 256],
    pub enabled: bool,
    pub global: bool,
    pub dependencies: [u64; 16], // Hash or IDs of flags this depends on
    pub dep_count: u32,
}

impl FeatureFlag {
    pub const fn empty() -> Self {
        FeatureFlag {
            name: [0; 64],
            description: [0; 256],
            enabled: false,
            global: false,
            dependencies: [0; 16],
            dep_count: 0,
        }
    }

    pub fn new(name_str: &str, desc_str: &str, enabled: bool, global: bool) -> Self {
        let mut flag = Self::empty();
        let name_bytes = name_str.as_bytes();
        let name_len = name_bytes.len().min(64);
        flag.name[..name_len].copy_from_slice(&name_bytes[..name_len]);

        let desc_bytes = desc_str.as_bytes();
        let desc_len = desc_bytes.len().min(256);
        flag.description[..desc_len].copy_from_slice(&desc_bytes[..desc_len]);

        flag.enabled = enabled;
        flag.global = global;
        flag
    }

    pub fn get_name(&self) -> &str {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..len]).unwrap_or("")
    }

    pub fn get_description(&self) -> &str {
        let len = self.description.iter().position(|&b| b == 0).unwrap_or(self.description.len());
        core::str::from_utf8(&self.description[..len]).unwrap_or("")
    }

    pub fn add_dependency(&mut self, dep_id: u64) -> bool {
        if (self.dep_count as usize) < self.dependencies.len() {
            self.dependencies[self.dep_count as usize] = dep_id;
            self.dep_count += 1;
            true
        } else {
            false
        }
    }
}

pub const MAX_FEATURE_FLAGS: usize = 512;
pub static mut FEATURE_FLAGS: [FeatureFlag; MAX_FEATURE_FLAGS] = [FeatureFlag::empty(); MAX_FEATURE_FLAGS];

pub struct FeatureFlagRegistry {
    flags: Vec<FeatureFlag>,
}

impl FeatureFlagRegistry {
    pub fn new() -> Self {
        FeatureFlagRegistry { flags: Vec::new() }
    }

    pub fn register_flag(&mut self, flag: FeatureFlag) {
        self.flags.push(flag);
    }

    pub fn find_flag(&self, name: &str) -> Option<&FeatureFlag> {
        self.flags.iter().find(|f| f.get_name() == name)
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) -> bool {
        if let Some(flag) = self.flags.iter_mut().find(|f| f.get_name() == name) {
            flag.enabled = enabled;
            true
        } else {
            false
        }
    }

    pub fn list_global_flags(&self) -> Vec<&FeatureFlag> {
        self.flags.iter().filter(|f| f.global).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_flag_creation() {
        let mut flag = FeatureFlag::new("bluetooth", "Bluetooth wireless protocol support", false, true);
        assert_eq!(flag.get_name(), "bluetooth");
        assert_eq!(flag.get_description(), "Bluetooth wireless protocol support");
        assert!(!flag.enabled);
        assert!(flag.global);

        assert!(flag.add_dependency(1001));
        assert_eq!(flag.dep_count, 1);
        assert_eq!(flag.dependencies[0], 1001);
    }

    #[test]
    fn test_feature_registry() {
        let mut registry = FeatureFlagRegistry::new();
        registry.register_flag(FeatureFlag::new("dbus", "D-Bus IPC system", true, true));
        registry.register_flag(FeatureFlag::new("wayland", "Wayland display server protocol", true, false));

        assert!(registry.find_flag("dbus").unwrap().enabled);
        assert!(registry.set_enabled("dbus", false));
        assert!(!registry.find_flag("dbus").unwrap().enabled);

        let globals = registry.list_global_flags();
        assert_eq!(globals.len(), 1);
        assert_eq!(globals[0].get_name(), "dbus");
    }
}
