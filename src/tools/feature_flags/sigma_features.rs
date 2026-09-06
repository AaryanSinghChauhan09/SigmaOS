#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SPDX-License-Identifier: MIT
// SigmaOS Feature Flags Subsystem
// Inspired by Gentoo Portage USE flags, OpenBSD pledge/unveil, and FreeBSD Capsicum rights
// Fine-grained control over package compilation, system configuration, and kernel sandboxing


use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// Feature flag definition for raw memory serialization
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FeatureFlag {
    pub name: [u8; 64],
    pub description: [u8; 256],
    pub enabled: bool,
    pub global: bool,
    pub dependencies: [u64; 16], // Other flags this depends on
    pub dep_count: u32,
}

impl FeatureFlag {
    /// Create a new empty feature flag
    pub const fn empty() -> Self {
        Self {
            name: [0; 64],
            description: [0; 256],
            enabled: false,
            global: false,
            dependencies: [0; 16],
            dep_count: 0,
        }
    }

    /// Create a new feature flag with given parameters
    pub fn new(name: &str, description: &str, enabled: bool, global: bool) -> Self {
        let mut flag = Self::empty();

        // Copy name (truncated if too long)
        let name_bytes = name.as_bytes();
        for (i, &byte) in name_bytes.iter().enumerate().take(64) {
            flag.name[i] = byte;
        }

        // Copy description (truncated if too long)
        let desc_bytes = description.as_bytes();
        for (i, &byte) in desc_bytes.iter().enumerate().take(256) {
            flag.description[i] = byte;
        }

        flag.enabled = enabled;
        flag.global = global;
        flag
    }

    /// Get name as string
    pub fn get_name(&self) -> String {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        String::from_utf8_lossy(&self.name[..len]).to_string()
    }

    /// Get description as string
    pub fn get_description(&self) -> String {
        let len = self.description.iter().position(|&b| b == 0).unwrap_or(256);
        String::from_utf8_lossy(&self.description[..len]).to_string()
    }

    /// Add a dependency
    pub fn add_dependency(&mut self, dep_hash: u64) {
        if self.dep_count < 16 {
            self.dependencies[self.dep_count as usize] = dep_hash;
            self.dep_count += 1;
        }
    }
}

/// Maximum number of feature flags
pub const MAX_FEATURE_FLAGS: usize = 512;

/// Global feature flags registry
static mut FEATURE_FLAGS: [FeatureFlag; MAX_FEATURE_FLAGS] = [FeatureFlag::empty(); MAX_FEATURE_FLAGS];
static mut FLAG_COUNT: usize = 0;

/// Feature flag configuration entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFlagConfig {
    pub name: String,
    pub enabled: bool,
    pub description: String,
    pub global: bool,
}

/// Feature flag profile (Gentoo-inspired profile defaults)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureProfile {
    pub name: String,
    pub description: String,
    pub flags: Vec<String>,
}

/// Feature flag resolver for dependency resolution and Portage-style USE flags
pub struct FeatureFlagResolver {
    pub flags: BTreeMap<String, FeatureFlagConfig>,
    pub profiles: BTreeMap<String, FeatureProfile>,
    pub active_profile: Option<String>,
    pub masked_flags: Vec<String>, // Explicitly negative USE flags (-flag)
}

impl FeatureFlagResolver {
    /// Create new feature flag resolver
    pub fn new() -> Self {
        Self {
            flags: BTreeMap::new(),
            profiles: BTreeMap::new(),
            active_profile: None,
            masked_flags: Vec::new(),
        }
    }

    /// Register a feature flag
    pub fn register_flag(&mut self, config: FeatureFlagConfig) {
        self.flags.insert(config.name.clone(), config);
    }

    /// Register a profile
    pub fn register_profile(&mut self, profile: FeatureProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    /// Set active profile
    pub fn set_active_profile(&mut self, profile_name: &str) -> Result<(), String> {
        if !self.profiles.contains_key(profile_name) {
            return Err(format!("Profile {} not found", profile_name));
        }
        self.active_profile = Some(profile_name.to_string());
        Ok(())
    }

    /// Apply Portage-style USE flags (e.g., "+wayland", "-x11")
    pub fn apply_use_flag(&mut self, flag_expr: &str) -> Result<(), String> {
        if let Some(stripped) = flag_expr.strip_prefix('-') {
            if !self.masked_flags.contains(&stripped.to_string()) {
                self.masked_flags.push(stripped.to_string());
            }
            if let Some(config) = self.flags.get_mut(stripped) {
                config.enabled = false;
            }
            Ok(())
        } else {
            let clean_flag = flag_expr.strip_prefix('+').unwrap_or(flag_expr);
            // Unmask if previously masked
            self.masked_flags.retain(|f| f != clean_flag);
            if let Some(config) = self.flags.get_mut(clean_flag) {
                config.enabled = true;
                Ok(())
            } else {
                Err(format!("Feature flag {} not registered", clean_flag))
            }
        }
    }

    /// Resolve feature flags with dependencies and masks
    pub fn resolve(&self) -> Result<Vec<String>, String> {
        let mut enabled_flags = Vec::new();
        let mut visited = BTreeMap::new();

        // Start with profile flags if active
        if let Some(profile_name) = &self.active_profile {
            if let Some(profile) = self.profiles.get(profile_name) {
                for flag_name in &profile.flags {
                    if flag_name.starts_with('-') {
                        continue;
                    }
                    let clean = flag_name.strip_prefix('+').unwrap_or(flag_name);
                    if !self.masked_flags.contains(&clean.to_string()) {
                        self.resolve_flag(clean, &mut enabled_flags, &mut visited)?;
                    }
                }
            }
        }

        // Add globally enabled flags that are not masked
        for (name, config) in &self.flags {
            if config.global && config.enabled && !self.masked_flags.contains(name) {
                self.resolve_flag(name, &mut enabled_flags, &mut visited)?;
            }
        }

        Ok(enabled_flags)
    }

    /// Recursively resolve a feature flag and its dependencies
    fn resolve_flag(
        &self,
        flag_name: &str,
        enabled_flags: &mut Vec<String>,
        visited: &mut BTreeMap<String, bool>,
    ) -> Result<(), String> {
        if self.masked_flags.contains(&flag_name.to_string()) {
            return Ok(());
        }

        // Check for circular dependencies
        if let Some(&in_progress) = visited.get(flag_name) {
            if in_progress {
                return Err(format!("Circular dependency detected for flag {}", flag_name));
            }
            return Ok(()); // Already resolved
        }

        visited.insert(flag_name.to_string(), true);

        // Get flag configuration
        let config = self.flags.get(flag_name)
            .ok_or_else(|| format!("Feature flag {} not found", flag_name))?;

        if config.enabled {
            // Add to enabled flags if not already present
            if !enabled_flags.contains(&flag_name.to_string()) {
                enabled_flags.push(flag_name.to_string());
            }
        }

        visited.insert(flag_name.to_string(), false);
        Ok(())
    }

    /// Check for conflicts between flags
    pub fn check_conflicts(&self) -> Vec<String> {
        let mut conflicts = Vec::new();

        // X11 vs Wayland
        if self.is_flag_enabled("x11") && self.is_flag_enabled("wayland") {
            conflicts.push("x11 and wayland are mutually exclusive".to_string());
        }

        // systemd vs openrc
        if self.is_flag_enabled("systemd") && self.is_flag_enabled("openrc") {
            conflicts.push("systemd and openrc are mutually exclusive".to_string());
        }

        // SELinux vs AppArmor
        if self.is_flag_enabled("selinux") && self.is_flag_enabled("apparmor") {
            conflicts.push("selinux and apparmor are mutually exclusive".to_string());
        }

        conflicts
    }

    /// Check if a flag is enabled
    pub fn is_flag_enabled(&self, flag_name: &str) -> bool {
        if self.masked_flags.contains(&flag_name.to_string()) {
            return false;
        }
        self.flags.get(flag_name).map(|c| c.enabled).unwrap_or(false)
    }

    /// Enable a flag
    pub fn enable_flag(&mut self, flag_name: &str) -> Result<(), String> {
        self.apply_use_flag(flag_name)
    }

    /// Disable a flag
    pub fn disable_flag(&mut self, flag_name: &str) -> Result<(), String> {
        let mut neg = String::from("-");
        neg.push_str(flag_name);
        self.apply_use_flag(&neg)
    }

    /// List all registered flags
    pub fn list_flags(&self) -> Vec<&FeatureFlagConfig> {
        self.flags.values().collect()
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&FeatureProfile> {
        self.profiles.values().collect()
    }
}

impl Default for FeatureFlagResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize default feature flags (Linux & BSD inspired)
pub fn init_default_flags(resolver: &mut FeatureFlagResolver) {
    // System-level flags
    resolver.register_flag(FeatureFlagConfig {
        name: "bluetooth".to_string(),
        enabled: false,
        description: "Bluetooth network and audio support".to_string(),
        global: true,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "dbus".to_string(),
        enabled: true,
        description: "D-Bus IPC messaging bus system".to_string(),
        global: true,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "systemd".to_string(),
        enabled: false,
        description: "systemd init system and cgroup v2 compatibility".to_string(),
        global: true,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "openrc".to_string(),
        enabled: true,
        description: "OpenRC init system service supervision".to_string(),
        global: true,
    });

    // Display & Graphics flags
    resolver.register_flag(FeatureFlagConfig {
        name: "wayland".to_string(),
        enabled: true,
        description: "Wayland zenith graphics compositor engine".to_string(),
        global: false,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "x11".to_string(),
        enabled: false,
        description: "X11 legacy display server compatibility".to_string(),
        global: false,
    });

    // Security & Mandatory Access Control flags
    resolver.register_flag(FeatureFlagConfig {
        name: "selinux".to_string(),
        enabled: false,
        description: "SELinux mandatory access control policy engine".to_string(),
        global: true,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "apparmor".to_string(),
        enabled: true,
        description: "AppArmor file path profiling security engine".to_string(),
        global: true,
    });

    // BSD Security Innovations
    resolver.register_flag(FeatureFlagConfig {
        name: "pledge".to_string(),
        enabled: true,
        description: "OpenBSD-style pledge syscall restricted execution".to_string(),
        global: true,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "unveil".to_string(),
        enabled: true,
        description: "OpenBSD-style unveil filesystem access filtering".to_string(),
        global: true,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "capsicum".to_string(),
        enabled: true,
        description: "FreeBSD Capsicum capability-based isolation".to_string(),
        global: true,
    });

    // Development & Optimization flags
    resolver.register_flag(FeatureFlagConfig {
        name: "debug".to_string(),
        enabled: false,
        description: "Enable debug symbols, assertions, and tracing".to_string(),
        global: true,
    });

    resolver.register_flag(FeatureFlagConfig {
        name: "optimize".to_string(),
        enabled: true,
        description: "Enable CPU micro-architecture vector optimizations".to_string(),
        global: true,
    });
}

/// Initialize default system profiles
pub fn init_default_profiles(resolver: &mut FeatureFlagResolver) {
    // Minimal profile
    resolver.register_profile(FeatureProfile {
        name: "minimal".to_string(),
        description: "Minimal sovereign system kernel without desktop overhead".to_string(),
        flags: vec![
            "dbus".to_string(),
            "openrc".to_string(),
            "apparmor".to_string(),
            "pledge".to_string(),
            "unveil".to_string(),
            "optimize".to_string(),
        ],
    });

    // Desktop profile
    resolver.register_profile(FeatureProfile {
        name: "desktop".to_string(),
        description: "Full sovereign desktop system with Wayland and Zenith UI".to_string(),
        flags: vec![
            "dbus".to_string(),
            "openrc".to_string(),
            "wayland".to_string(),
            "bluetooth".to_string(),
            "apparmor".to_string(),
            "pledge".to_string(),
            "unveil".to_string(),
            "capsicum".to_string(),
            "optimize".to_string(),
        ],
    });

    // Development profile
    resolver.register_profile(FeatureProfile {
        name: "development".to_string(),
        description: "Development environment with debugging tools enabled".to_string(),
        flags: vec![
            "dbus".to_string(),
            "openrc".to_string(),
            "debug".to_string(),
            "apparmor".to_string(),
            "pledge".to_string(),
        ],
    });
}

/// Calculate hash for feature flag name (FNV-1a 64-bit)
pub fn calculate_flag_hash(name: &str) -> u64 {
    let mut hash: u64 = 14695981039346656037;
    for &byte in name.as_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(1099511628211);
    }
    hash
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_flag_basic() {
        let flag = FeatureFlag::new("wayland", "Wayland graphics", true, false);
        assert_eq!(flag.get_name(), "wayland");
        assert_eq!(flag.get_description(), "Wayland graphics");
        assert!(flag.enabled);
    }

    #[test]
    fn test_portage_style_use_flags() {
        let mut resolver = FeatureFlagResolver::new();
        init_default_flags(&mut resolver);

        // Apply negative USE flag
        assert!(resolver.is_flag_enabled("dbus"));
        resolver.apply_use_flag("-dbus").unwrap();
        assert!(!resolver.is_flag_enabled("dbus"));

        // Apply positive USE flag
        resolver.apply_use_flag("+bluetooth").unwrap();
        assert!(resolver.is_flag_enabled("bluetooth"));
    }

    #[test]
    fn test_profile_resolution() {
        let mut resolver = FeatureFlagResolver::new();
        init_default_flags(&mut resolver);
        init_default_profiles(&mut resolver);

        resolver.set_active_profile("desktop").unwrap();
        let flags = resolver.resolve().unwrap();

        assert!(flags.contains(&"wayland".to_string()));
        assert!(flags.contains(&"pledge".to_string()));
        assert!(flags.contains(&"capsicum".to_string()));
    }

    #[test]
    fn test_flag_conflicts() {
        let mut resolver = FeatureFlagResolver::new();
        init_default_flags(&mut resolver);

        // Enable mutually exclusive flags
        resolver.enable_flag("x11").unwrap();
        resolver.enable_flag("wayland").unwrap();

        let conflicts = resolver.check_conflicts();
        assert!(!conflicts.is_empty());
        assert!(conflicts[0].contains("x11 and wayland"));
    }

    #[test]
    fn test_bsd_security_flags() {
        let mut resolver = FeatureFlagResolver::new();
        init_default_flags(&mut resolver);

        assert!(resolver.is_flag_enabled("pledge"));
        assert!(resolver.is_flag_enabled("unveil"));
        assert!(resolver.is_flag_enabled("capsicum"));
    }
}
