extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// MATE Desktop & LMDE Betsy Package Category
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MateBetsyCategory {
    MateCore,
    MateApplets,
    MateUtils,
    MateDesktopEnv,
    AptDebianBetsy,
}

/// MATE & Betsy Package Record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MateBetsyPackage {
    pub name: String,
    pub version: String,
    pub category: MateBetsyCategory,
    pub dependencies: Vec<String>,
    pub installed: bool,
    pub pin_priority: i32, // APT pinning priority (e.g., 500 default, 700 LMDE preferred)
}

/// MATE Packages Betsy Management Engine (LMDE 2 Betsy & MATE Desktop parity)
pub struct MatePackagesBetsyEngine {
    pub active_distribution: String,
    pub packages: BTreeMap<String, MateBetsyPackage>,
    pub transaction_history: Vec<String>,
}

impl MatePackagesBetsyEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            active_distribution: "LMDE 2 Betsy (Debian Jessie Base)".to_string(),
            packages: BTreeMap::new(),
            transaction_history: Vec::new(),
        };

        // Seed core MATE Betsy packages
        engine.register_package(MateBetsyPackage {
            name: "mate-desktop-environment".to_string(),
            version: "1.16.0".to_string(),
            category: MateBetsyCategory::MateDesktopEnv,
            dependencies: vec![
                "mate-panel".to_string(),
                "caja".to_string(),
                "marco".to_string(),
            ],
            installed: true,
            pin_priority: 700,
        });

        engine.register_package(MateBetsyPackage {
            name: "mate-panel".to_string(),
            version: "1.16.0".to_string(),
            category: MateBetsyCategory::MateCore,
            dependencies: vec!["libmate-menu".to_string()],
            installed: true,
            pin_priority: 700,
        });

        engine.register_package(MateBetsyPackage {
            name: "mate-applets".to_string(),
            version: "1.16.0".to_string(),
            category: MateBetsyCategory::MateApplets,
            dependencies: vec!["mate-panel".to_string()],
            installed: false,
            pin_priority: 500,
        });

        engine
    }

    pub fn register_package(&mut self, pkg: MateBetsyPackage) {
        self.packages.insert(pkg.name.clone(), pkg);
    }

    pub fn install_package(&mut self, pkg_name: &str) -> Result<usize, &'static str> {
        let mut installed_count = 0;

        if let Some(pkg) = self.packages.get(pkg_name).cloned() {
            // 1. Resolve dependencies recursively
            for dep in &pkg.dependencies {
                if let Some(dep_pkg) = self.packages.get_mut(dep) {
                    if !dep_pkg.installed {
                        dep_pkg.installed = true;
                        installed_count += 1;
                        self.transaction_history
                            .push(format!("Installed dependency: {}", dep));
                    }
                }
            }

            // 2. Install main package
            if let Some(target) = self.packages.get_mut(pkg_name) {
                if !target.installed {
                    target.installed = true;
                    installed_count += 1;
                    self.transaction_history
                        .push(format!("Installed package: {}", pkg_name));
                }
            }

            Ok(installed_count)
        } else {
            Err("Package not found in Betsy repository")
        }
    }

    pub fn set_apt_pin_priority(
        &mut self,
        pkg_name: &str,
        priority: i32,
    ) -> Result<(), &'static str> {
        if let Some(pkg) = self.packages.get_mut(pkg_name) {
            pkg.pin_priority = priority;
            Ok(())
        } else {
            Err("Package not found")
        }
    }

    pub fn query_packages_by_category(
        &self,
        category: MateBetsyCategory,
    ) -> Vec<&MateBetsyPackage> {
        self.packages
            .values()
            .filter(|p| p.category == category)
            .collect()
    }
}

impl Default for MatePackagesBetsyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mate_packages_betsy_engine() {
        let mut engine = MatePackagesBetsyEngine::new();
        assert_eq!(
            engine.active_distribution,
            "LMDE 2 Betsy (Debian Jessie Base)"
        );

        // Test Category Query
        let applets = engine.query_packages_by_category(MateBetsyCategory::MateApplets);
        assert_eq!(applets.len(), 1);
        assert_eq!(applets[0].name, "mate-applets");

        // Test Dependency Resolution & Installation
        let installed_count = engine.install_package("mate-applets").unwrap();
        assert_eq!(installed_count, 1);
        assert!(engine.packages.get("mate-applets").unwrap().installed);

        // Test uninstalled dependency resolution
        engine.register_package(MateBetsyPackage {
            name: "pluma".to_string(),
            version: "1.16.0".to_string(),
            category: MateBetsyCategory::MateUtils,
            dependencies: vec!["libgtksourceview-3.0".to_string()],
            installed: false,
            pin_priority: 500,
        });

        engine.register_package(MateBetsyPackage {
            name: "libgtksourceview-3.0".to_string(),
            version: "3.22.0".to_string(),
            category: MateBetsyCategory::AptDebianBetsy,
            dependencies: vec![],
            installed: false,
            pin_priority: 500,
        });

        let pluma_installed = engine.install_package("pluma").unwrap();
        assert_eq!(pluma_installed, 2);
        assert!(engine.packages.get("pluma").unwrap().installed);
        assert!(
            engine
                .packages
                .get("libgtksourceview-3.0")
                .unwrap()
                .installed
        );

        // Test Non-existent package error handling
        assert!(engine.install_package("non-existent-pkg").is_err());
        assert!(engine
            .set_apt_pin_priority("non-existent-pkg", 100)
            .is_err());

        // Test APT Pin Priority Setting
        assert!(engine.set_apt_pin_priority("mate-applets", 900).is_ok());
        assert_eq!(
            engine.packages.get("mate-applets").unwrap().pin_priority,
            900
        );
    }
}
