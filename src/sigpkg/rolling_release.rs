// SPDX-License-Identifier: MIT
// Rolling Release System - Arch Linux Inspired
// Provides continuous updates without major version bumps

use crate::klib::{hashmap::HashMap, string::SigmaString, vec::Vec};
use crate::sigpkg::{Version, VersionConstraint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateChannel {
    Stable,
    Testing,
    Unstable,
}

#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub package_name: SigmaString,
    pub old_version: Version,
    pub new_version: Version,
    pub update_type: UpdateType,
    pub security_critical: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    Security,
}

#[derive(Debug, Clone)]
pub struct RollingReleaseManager {
    current_version: Version,
    update_channel: UpdateChannel,
    auto_update_enabled: bool,
    package_versions: HashMap<SigmaString, Version>,
    update_history: Vec<PackageUpdate>,
}

impl RollingReleaseManager {
    pub fn new(initial_version: Version, channel: UpdateChannel) -> Self {
        RollingReleaseManager {
            current_version: initial_version,
            update_channel: channel,
            auto_update_enabled: false,
            package_versions: HashMap::new(),
            update_history: Vec::new(),
        }
    }

    pub fn enable_auto_updates(&mut self) {
        self.auto_update_enabled = true;
    }

    pub fn disable_auto_updates(&mut self) {
        self.auto_update_enabled = false;
    }

    pub fn set_update_channel(&mut self, channel: UpdateChannel) {
        self.update_channel = channel;
    }

    pub fn register_package(&mut self, name: SigmaString, version: Version) {
        self.package_versions.insert(name, version);
    }

    pub fn check_for_updates(&self, available_packages: &HashMap<SigmaString, Version>) -> Vec<PackageUpdate> {
        let mut updates = Vec::new();
        
        for (package_name, current_version) in &self.package_versions {
            if let Some(available_version) = available_packages.get(package_name) {
                if available_version > current_version {
                    let update_type = self.determine_update_type(current_version, available_version);
                    let security_critical = self.is_security_critical(package_name);
                    
                    updates.push(PackageUpdate {
                        package_name: package_name.clone(),
                        old_version: *current_version,
                        new_version: *available_version,
                        update_type,
                        security_critical,
                    });
                }
            }
        }
        
        updates.sort_by(|a, b| {
            // Security updates first, then by version
            if a.security_critical && !b.security_critical {
                return core::cmp::Ordering::Less;
            }
            if !a.security_critical && b.security_critical {
                return core::cmp::Ordering::Greater;
            }
            b.new_version.cmp(&a.new_version)
        });
        
        updates
    }

    pub fn apply_update(&mut self, update: PackageUpdate) -> Result<(), &'static str> {
        if let Some(current_version) = self.package_versions.get(&update.package_name) {
            if *current_version != update.old_version {
                return Err("Version mismatch - package already updated");
            }
        }
        
        self.package_versions.insert(update.package_name.clone(), update.new_version);
        self.update_history.push(update.clone());
        
        Ok(())
    }

    pub fn rollback_to_version(&mut self, package_name: &SigmaString, target_version: Version) -> Result<(), &'static str> {
        if let Some(current_version) = self.package_versions.get(package_name) {
            if *current_version == target_version {
                return Ok(()); // Already at target version
            }
            
            // Find the update history entry
            for update in &self.update_history {
                if update.package_name == *package_name && update.old_version == target_version {
                    self.package_versions.insert(package_name.clone(), target_version);
                    return Ok(());
                }
            }
            
            Err("Target version not found in update history")
        } else {
            Err("Package not found")
        }
    }

    pub fn get_system_version(&self) -> Version {
        self.current_version
    }

    pub fn get_update_history(&self) -> &[PackageUpdate] {
        &self.update_history
    }

    fn determine_update_type(&self, old: &Version, new: &Version) -> UpdateType {
        if new.major > old.major {
            UpdateType::Major
        } else if new.minor > old.minor {
            UpdateType::Minor
        } else if new.patch > old.patch {
            UpdateType::Patch
        } else {
            UpdateType::Security // Assume security if same version but different
        }
    }

    fn is_security_critical(&self, package_name: &SigmaString) -> bool {
        // Common security-critical packages
        let security_packages = [
            "kernel", "openssl", "gnutls", "libssl", "crypto",
            "sudo", "openssh", "glibc", "systemd", "bash"
        ];
        
        let name_str = package_name.as_str();
        security_packages.iter().any(|&pkg| name_str.contains(pkg))
    }
}

impl Default for RollingReleaseManager {
    fn default() -> Self {
        Self::new(
            Version { major: 0, minor: 1, patch: 0 },
            UpdateChannel::Stable
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_release_manager_creation() {
        let manager = RollingReleaseManager::new(
            Version { major: 1, minor: 0, patch: 0 },
            UpdateChannel::Stable
        );
        
        assert_eq!(manager.get_system_version().major, 1);
        assert!(!manager.auto_update_enabled);
    }

    #[test]
    fn test_package_registration() {
        let mut manager = RollingReleaseManager::default();
        manager.register_package(SigmaString::from("test-pkg"), Version { major: 1, minor: 0, patch: 0 });
        
        assert!(manager.package_versions.contains_key(&SigmaString::from("test-pkg")));
    }

    #[test]
    fn test_update_detection() {
        let mut manager = RollingReleaseManager::default();
        manager.register_package(SigmaString::from("test-pkg"), Version { major: 1, minor: 0, patch: 0 });
        
        let mut available = HashMap::new();
        available.insert(SigmaString::from("test-pkg"), Version { major: 1, minor: 1, patch: 0 });
        
        let updates = manager.check_for_updates(&available);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].new_version.minor, 1);
    }

    #[test]
    fn test_update_application() {
        let mut manager = RollingReleaseManager::default();
        manager.register_package(SigmaString::from("test-pkg"), Version { major: 1, minor: 0, patch: 0 });
        
        let update = PackageUpdate {
            package_name: SigmaString::from("test-pkg"),
            old_version: Version { major: 1, minor: 0, patch: 0 },
            new_version: Version { major: 1, minor: 1, patch: 0 },
            update_type: UpdateType::Minor,
            security_critical: false,
        };
        
        assert!(manager.apply_update(update).is_ok());
        assert_eq!(manager.package_versions.get(&SigmaString::from("test-pkg")).unwrap().minor, 1);
    }

    #[test]
    fn test_security_critical_detection() {
        let manager = RollingReleaseManager::default();
        assert!(manager.is_security_critical(&SigmaString::from("kernel")));
        assert!(manager.is_security_critical(&SigmaString::from("openssl")));
        assert!(!manager.is_security_critical(&SigmaString::from("text-editor")));
    }
}