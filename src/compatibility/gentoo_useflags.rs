//! Gentoo-Inspired USE Flags and Source Compilation System
extern crate alloc;



use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct UseFlag {
    pub name: String,
    pub description: String,
    pub default_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct UseFlagManager {
    global_flags: BTreeSet<String>,
    package_flags: BTreeMap<String, BTreeSet<String>>,
}

impl UseFlagManager {
    pub fn new() -> Self {
        let mut manager = Self {
            global_flags: BTreeSet::new(),
            package_flags: BTreeMap::new(),
        };
        manager.initialize_defaults();
        manager
    }

    fn initialize_defaults(&mut self) {
        self.global_flags.insert("ssl".to_string());
        self.global_flags.insert("ipv6".to_string());
        self.global_flags.insert("unicode".to_string());
    }

    pub fn is_flag_enabled(&self, package: &str, flag: &str) -> bool {
        if let Some(pkg_flags) = self.package_flags.get(package) {
            if pkg_flags.contains(flag) {
                return true;
            }
        }
        self.global_flags.contains(flag)
    }

    pub fn enable_flag(&mut self, flag: String) {
        self.global_flags.insert(flag);
    }
}
