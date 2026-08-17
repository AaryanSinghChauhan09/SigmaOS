// Gentoo Portage-Inspired USE-Flag Solver & Ebuild Dependency Engine
// Zero-dependency, safe Rust source-based package dependency solver with USE flag profiles

use std::collections::{HashMap, HashSet};

/// USE-Flag configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseFlag {
    pub name: String,
    pub enabled: bool,
    pub description: String,
}

/// Gentoo Ebuild dependency expression (e.g. "ssl? ( >=dev-libs/openssl-1.1 )")
#[derive(Debug, Clone)]
pub struct EbuildDependency {
    pub package_name: String,
    pub required_use_flag: Option<String>, // Conditional on USE flag
    pub min_version: Option<String>,
}

/// Gentoo Portage Ebuild package model
#[derive(Debug, Clone)]
pub struct EbuildPackage {
    pub category: String,
    pub name: String,
    pub version: String,
    pub use_flags: Vec<UseFlag>,
    pub dependencies: Vec<EbuildDependency>,
    pub slot: String,
}

impl EbuildPackage {
    pub fn new(category: &str, name: &str, version: &str) -> Self {
        Self {
            category: category.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            use_flags: Vec::new(),
            dependencies: Vec::new(),
            slot: "0".to_string(),
        }
    }

    pub fn full_atom(&self) -> String {
        format!("{}/{}-{}", self.category, self.name, self.version)
    }

    pub fn add_use_flag(&mut self, name: &str, enabled: bool, desc: &str) {
        self.use_flags.push(UseFlag {
            name: name.to_string(),
            enabled,
            description: desc.to_string(),
        });
    }

    pub fn add_dependency(&mut self, pkg_name: &str, conditional_flag: Option<&str>) {
        self.dependencies.push(EbuildDependency {
            package_name: pkg_name.to_string(),
            required_use_flag: conditional_flag.map(|s| s.to_string()),
            min_version: None,
        });
    }
}

/// Gentoo Portage USE-Flag & Ebuild Resolver Engine
#[derive(Debug, Clone)]
pub struct PortageEngine {
    pub global_use_flags: HashSet<String>,
    pub available_ebuilds: HashMap<String, EbuildPackage>,
    pub installed_packages: HashSet<String>,
}

impl PortageEngine {
    pub fn new() -> Self {
        let mut global_use_flags = HashSet::new();
        global_use_flags.insert("ssl".to_string());
        global_use_flags.insert("X".to_string());
        global_use_flags.insert("wayland".to_string());

        Self {
            global_use_flags,
            available_ebuilds: HashMap::new(),
            installed_packages: HashSet::new(),
        }
    }

    /// Enable or disable a global USE flag
    pub fn set_use_flag(&mut self, flag: &str, enable: bool) {
        if enable {
            self.global_use_flags.insert(flag.to_string());
        } else {
            self.global_use_flags.remove(flag);
        }
    }

    /// Register an ebuild package in Portage tree
    pub fn add_ebuild(&mut self, ebuild: EbuildPackage) {
        self.available_ebuilds.insert(ebuild.name.clone(), ebuild);
    }

    /// Resolve all dependencies for a package atom considering active USE flags
    pub fn resolve_dependencies(&self, pkg_name: &str) -> Result<Vec<String>, &'static str> {
        let ebuild = self
            .available_ebuilds
            .get(pkg_name)
            .ok_or("Ebuild atom not found in Portage tree")?;

        let mut resolved = Vec::new();
        resolved.push(ebuild.full_atom());

        for dep in &ebuild.dependencies {
            let dep_required = match &dep.required_use_flag {
                Some(flag) => self.global_use_flags.contains(flag),
                None => true,
            };

            if dep_required {
                if let Ok(mut child_deps) = self.resolve_dependencies(&dep.package_name) {
                    resolved.append(&mut child_deps);
                } else {
                    resolved.push(dep.package_name.clone());
                }
            }
        }

        // Deduplicate resolved dependencies while preserving install order
        let mut unique = Vec::new();
        for item in resolved {
            if !unique.contains(&item) {
                unique.push(item);
            }
        }

        Ok(unique)
    }

    /// Emerge (compile & install) package atom with active USE flags
    pub fn emerge(&mut self, pkg_name: &str) -> Result<String, &'static str> {
        let deps = self.resolve_dependencies(pkg_name)?;
        for dep in &deps {
            self.installed_packages.insert(dep.clone());
        }
        Ok(format!(
            "Emerging {} with USE flags {:?}. Total atoms compiled: {}",
            pkg_name,
            self.global_use_flags,
            deps.len()
        ))
    }
}

impl Default for PortageEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portage_use_flag_dependency_resolution() {
        let mut engine = PortageEngine::new();

        let mut curl = EbuildPackage::new("net-misc", "curl", "8.2.1");
        curl.add_dependency("openssl", Some("ssl"));
        curl.add_dependency("libssh2", Some("ssh"));

        let openssl = EbuildPackage::new("dev-libs", "openssl", "3.1.2");
        engine.add_ebuild(curl);
        engine.add_ebuild(openssl);

        // 'ssl' flag is enabled globally by default -> openssl should be resolved
        let deps = engine.resolve_dependencies("curl").unwrap();
        assert!(deps.iter().any(|d| d.contains("openssl")));

        // Disable 'ssl' flag -> openssl dependency should no longer be required
        engine.set_use_flag("ssl", false);
        let deps_nossl = engine.resolve_dependencies("curl").unwrap();
        assert!(!deps_nossl.iter().any(|d| d.contains("openssl")));
    }
}
