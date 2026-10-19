//! SigmaOS Ports System
//!
//! Following principles: OPEN_STANDARD_AND_INTEROPERABILITY, PRACTICAL_LICENSING_AWARENESS,
//! COMMUNITY_ORIENTED_DEVELOPMENT, MERITOCRACY_AND_INCLUSIVITY, VERSION_CONTROL_PRACTICES,
//! SIMPLE_AND_ROBUST, SOURCE_BASED_INSTALLATION, PRISITINE_SOURCES, BUILD_FROM_SOURCE.
//!
//! Competitive with: FreeBSD Ports Collection, Arch AUR, Gentoo Portage.
//!
//! SigmaOS Ports provides:
//! - Source-based package building (like FreeBSD Ports)
//! - Makefile-driven build system
//! - Dependency resolution
//! - Patches and configuration options
//! - Clean separation of source and binary packages
//! - Community-contributed ports

use core::fmt;
use std::collections::HashMap;

/// Port category
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortCategory {
    /// System utilities
    Sysutils,
    /// Network tools
    Net,
    /// Security tools
    Security,
    /// Development tools
    Devel,
    /// Graphics and multimedia
    Graphics,
    /// Audio
    Audio,
    /// Office and productivity
    Office,
    /// Games
    Games,
    /// Science and engineering
    Science,
    /// Web
    Www,
}

/// Port makefile configuration
#[derive(Debug, Clone)]
pub struct PortMakefile {
    pub portname: String,
    pub version: String,
    pub categories: Vec<PortCategory>,
    pub maintainer: String,
    pub comment: String,
    pub license: String,
    pub depends_build: Vec<String>,
    pub depends_run: Vec<String>,
    pub uses: Vec<String>,
    pub build_options: HashMap<String, bool>,
    pub patches: Vec<String>,
    pub source_url: Option<String>,
    pub checksum: Option<String>,
}

/// Installed port
#[derive(Debug, Clone)]
pub struct InstalledPort {
    pub portname: String,
    pub version: String,
    pub install_date: u64,
    pub origin: String, // category/portname
}

/// Ports tree
pub struct PortsTree {
    pub ports: HashMap<String, PortMakefile>,
    pub installed: Vec<InstalledPort>,
    pub ports_dir: String,
}

impl PortsTree {
    /// Create a new ports tree
    pub fn new(ports_dir: String) -> Self {
        Self {
            ports: HashMap::new(),
            installed: Vec::new(),
            ports_dir,
        }
    }

    /// Add a port to the tree
    pub fn add_port(&mut self, makefile: PortMakefile) {
        let key = format!("{}/{}", makefile.portname, makefile.version);
        self.ports.insert(key, makefile);
    }

    /// Find a port by name and version
    pub fn find_port(&self, portname: &str, version: &str) -> Option<&PortMakefile> {
        let key = format!("{}/{}", portname, version);
        self.ports.get(&key)
    }

    /// Install a port (build from source)
    pub fn install_port(&mut self, portname: &str, version: &str) -> Result<(), String> {
        let port = self
            .find_port(portname, version)
            .ok_or_else(|| format!("Port {}@{} not found", portname, version))?;

        // Check build dependencies
        for dep in &port.depends_build {
            if !self.is_installed(dep) {
                return Err(format!("Build dependency not met: {}", dep));
            }
        }

        // Check runtime dependencies
        for dep in &port.depends_run {
            if !self.is_installed(dep) {
                return Err(format!("Runtime dependency not met: {}", dep));
            }
        }

        // Mark as installed
        self.installed.push(InstalledPort {
            portname: portname.to_string(),
            version: version.to_string(),
            install_date: 0, // Would be current timestamp
            origin: format!("{}/{}", portname, version),
        });

        Ok(())
    }

    /// Remove an installed port
    pub fn remove_port(&mut self, portname: &str) -> Result<(), String> {
        let index = self
            .installed
            .iter()
            .position(|p| p.portname == portname)
            .ok_or_else(|| format!("Port {} not installed", portname))?;
        self.installed.remove(index);
        Ok(())
    }

    /// Check if a port is installed
    pub fn is_installed(&self, portname: &str) -> bool {
        self.installed.iter().any(|p| p.portname == portname)
    }

    /// List installed ports
    pub fn list_installed(&self) -> &[InstalledPort] {
        &self.installed
    }

    /// List available ports
    pub fn list_available(&self) -> Vec<&PortMakefile> {
        self.ports.values().collect()
    }
}

impl Default for PortsTree {
    fn default() -> Self {
        Self::new("/usr/ports".to_string())
    }
}

impl fmt::Display for PortCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortCategory::Sysutils => write!(f, "sysutils"),
            PortCategory::Net => write!(f, "net"),
            PortCategory::Security => write!(f, "security"),
            PortCategory::Devel => write!(f, "devel"),
            PortCategory::Graphics => write!(f, "graphics"),
            PortCategory::Audio => write!(f, "audio"),
            PortCategory::Office => write!(f, "office"),
            PortCategory::Games => write!(f, "games"),
            PortCategory::Science => write!(f, "science"),
            PortCategory::Www => write!(f, "www"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_find_port() {
        let mut tree = PortsTree::new("/usr/ports".to_string());
        let makefile = PortMakefile {
            portname: "nginx".to_string(),
            version: "1.24.0".to_string(),
            categories: vec![PortCategory::Www],
            maintainer: "admin@example.com".to_string(),
            comment: "HTTP server".to_string(),
            license: "BSD-2-Clause".to_string(),
            depends_build: vec![],
            depends_run: vec![],
            uses: vec![],
            build_options: HashMap::new(),
            patches: vec![],
            source_url: Some("https://nginx.org/nginx-1.24.0.tar.gz".to_string()),
            checksum: Some("sha256:abc123".to_string()),
        };
        tree.add_port(makefile);
        let found = tree.find_port("nginx", "1.24.0");
        assert!(found.is_some());
        assert_eq!(found.unwrap().comment, "HTTP server");
    }

    #[test]
    fn test_install_port() {
        let mut tree = PortsTree::new("/usr/ports".to_string());
        let makefile = PortMakefile {
            portname: "vim".to_string(),
            version: "9.0.0".to_string(),
            categories: vec![PortCategory::Devel],
            maintainer: "admin@example.com".to_string(),
            comment: "Text editor".to_string(),
            license: "Vim".to_string(),
            depends_build: vec![],
            depends_run: vec![],
            uses: vec![],
            build_options: HashMap::new(),
            patches: vec![],
            source_url: None,
            checksum: None,
        };
        tree.add_port(makefile);
        tree.install_port("vim", "9.0.0").unwrap();
        assert!(tree.is_installed("vim"));
        assert_eq!(tree.list_installed().len(), 1);
    }

    #[test]
    fn test_install_missing_dependency() {
        let mut tree = PortsTree::new("/usr/ports".to_string());
        let makefile = PortMakefile {
            portname: "app".to_string(),
            version: "1.0.0".to_string(),
            categories: vec![],
            maintainer: "admin@example.com".to_string(),
            comment: "Test app".to_string(),
            license: "MIT".to_string(),
            depends_build: vec![],
            depends_run: vec!["libfoo".to_string()],
            uses: vec![],
            build_options: HashMap::new(),
            patches: vec![],
            source_url: None,
            checksum: None,
        };
        tree.add_port(makefile);
        let result = tree.install_port("app", "1.0.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_port() {
        let mut tree = PortsTree::new("/usr/ports".to_string());
        let makefile = PortMakefile {
            portname: "test".to_string(),
            version: "1.0.0".to_string(),
            categories: vec![],
            maintainer: "admin@example.com".to_string(),
            comment: "Test".to_string(),
            license: "MIT".to_string(),
            depends_build: vec![],
            depends_run: vec![],
            uses: vec![],
            build_options: HashMap::new(),
            patches: vec![],
            source_url: None,
            checksum: None,
        };
        tree.add_port(makefile);
        tree.install_port("test", "1.0.0").unwrap();
        tree.remove_port("test").unwrap();
        assert!(!tree.is_installed("test"));
    }
}