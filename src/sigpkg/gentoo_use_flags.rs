// SPDX-License-Identifier: MIT
// SigmaOS Gentoo Linux USE Flags Engine
// Implements USE flag system for conditional compilation and feature selection

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use crate::klib::collections::HashMap;

/// USE flag definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseFlag {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

/// USE flag profile configuration
#[derive(Debug, Clone)]
pub struct UseProfile {
    pub name: String,
    pub flags: Vec<UseFlag>,
}

/// Gentoo-style USE flag manager
pub struct UseFlagManager {
    global_flags: HashMap<String, UseFlag>,
    profiles: Vec<UseProfile>,
    current_profile: Option<String>,
}

impl UseFlagManager {
    pub fn new() -> Self {
        let mut manager = Self {
            global_flags: HashMap::new(),
            profiles: Vec::new(),
            current_profile: None,
        };
        
        // Initialize common USE flags
        manager.initialize_common_flags();
        manager
    }

    fn initialize_common_flags(&mut self) {
        let common_flags = vec![
            UseFlag {
                name: "X".to_string(),
                description: "Enable X11 support".to_string(),
                enabled: true,
            },
            UseFlag {
                name: "gtk".to_string(),
                description: "Enable GTK+ toolkit support".to_string(),
                enabled: true,
            },
            UseFlag {
                name: "qt5".to_string(),
                description: "Enable Qt5 framework support".to_string(),
                enabled: false,
            },
            UseFlag {
                name: "systemd".to_string(),
                description: "Enable systemd integration".to_string(),
                enabled: false,
            },
            UseFlag {
                name: "openssl".to_string(),
                description: "Enable OpenSSL support".to_string(),
                enabled: true,
            },
        ];

        for flag in common_flags {
            self.global_flags.insert(flag.name.clone(), flag);
        }
    }

    /// Set a USE flag
    pub fn set_flag(&mut self, flag_name: &str, enabled: bool) -> Result<(), String> {
        if let Some(flag) = self.global_flags.get_mut(flag_name) {
            flag.enabled = enabled;
            Ok(())
        } else {
            Err(format!("USE flag {} not found", flag_name))
        }
    }

    /// Get USE flag status
    pub fn get_flag(&self, flag_name: &str) -> Option<bool> {
        self.global_flags.get(flag_name).map(|f| f.enabled)
    }

    /// Create a new profile
    pub fn create_profile(&mut self, name: &str, flags: Vec<UseFlag>) {
        let profile = UseProfile {
            name: name.to_string(),
            flags,
        };
        self.profiles.push(profile);
    }

    /// Set active profile
    pub fn set_profile(&mut self, profile_name: &str) -> Result<(), String> {
        if self.profiles.iter().any(|p| p.name == profile_name) {
            self.current_profile = Some(profile_name.to_string());
            Ok(())
        } else {
            Err(format!("Profile {} not found", profile_name))
        }
    }

    /// Get effective flags for current profile
    pub fn get_effective_flags(&self) -> Vec<UseFlag> {
        let mut effective = Vec::new();
        
        // Start with global flags
        for (_, flag) in &self.global_flags {
            effective.push(flag.clone());
        }
        
        // Apply profile-specific overrides
        if let Some(profile_name) = &self.current_profile {
            if let Some(profile) = self.profiles.iter().find(|p| &p.name == profile_name) {
                for profile_flag in &profile.flags {
                    if let Some(global_flag) = effective.iter_mut().find(|f| f.name == profile_flag.name) {
                        global_flag.enabled = profile_flag.enabled;
                    } else {
                        effective.push(profile_flag.clone());
                    }
                }
            }
        }
        
        effective
    }

    /// Parse USE flag string (e.g., "X gtk -qt5 systemd")
    pub fn parse_use_string(&mut self, use_string: &str) -> Result<(), String> {
        for token in use_string.split_whitespace() {
            let (enabled, flag_name) = if token.starts_with('-') {
                (false, &token[1..])
            } else {
                (true, token)
            };
            
            self.set_flag(flag_name, enabled)?;
        }
        
        Ok(())
    }
}

/// Portage-like dependency specification with USE conditions
#[derive(Debug, Clone)]
pub struct ConditionalDependency {
    pub package: String,
    pub use_condition: Option<String>, // e.g., "X? ( x11-libs/libX11 )"
}

impl ConditionalDependency {
    pub fn new(package: &str, use_condition: Option<&str>) -> Self {
        Self {
            package: package.to_string(),
            use_condition: use_condition.map(|s| s.to_string()),
        }
    }

    /// Check if dependency should be included based on USE flags
    pub fn should_include(&self, use_manager: &UseFlagManager) -> bool {
        if let Some(condition) = &self.use_condition {
            // Parse simple condition like "X?" or "!gtk?"
            let flag_name = condition.trim_end_matches('?');
            let negated = flag_name.starts_with('!');
            let actual_flag = if negated { &flag_name[1..] } else { flag_name };
            
            if let Some(enabled) = use_manager.get_flag(actual_flag) {
                negated != enabled
            } else {
                true // Default to including if flag unknown
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_use_flag_manager() {
        let mut manager = UseFlagManager::new();
        
        assert_eq!(manager.get_flag("X"), Some(true));
        assert_eq!(manager.get_flag("qt5"), Some(false));
        
        manager.set_flag("gtk", false).unwrap();
        assert_eq!(manager.get_flag("gtk"), Some(false));
    }

    #[test]
    fn test_use_string_parsing() {
        let mut manager = UseFlagManager::new();
        manager.parse_use_string("X -gtk qt5").unwrap();
        
        assert_eq!(manager.get_flag("X"), Some(true));
        assert_eq!(manager.get_flag("gtk"), Some(false));
        assert_eq!(manager.get_flag("qt5"), Some(true));
    }

    #[test]
    fn test_conditional_dependency() {
        let mut manager = UseFlagManager::new();
        let dep = ConditionalDependency::new("x11-libs/libX11", Some("X?"));
        
        assert!(dep.should_include(&manager));
        
        manager.set_flag("X", false).unwrap();
        assert!(!dep.should_include(&manager));
    }

    #[test]
    fn test_profiles() {
        let mut manager = UseFlagManager::new();
        let desktop_flags = vec![
            UseFlag {
                name: "X".to_string(),
                description: "X11 support".to_string(),
                enabled: true,
            },
            UseFlag {
                name: "gtk".to_string(),
                description: "GTK support".to_string(),
                enabled: true,
            },
        ];
        
        manager.create_profile("desktop", desktop_flags);
        manager.set_profile("desktop").unwrap();
        
        let effective = manager.get_effective_flags();
        assert!(effective.iter().any(|f| f.name == "X" && f.enabled));
    }
}