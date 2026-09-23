use std::string::String;
use std::vec;
use std::vec::Vec;
// OpenStack Murano Inspired Application Catalogue for SigmaOS


/// Application Topology / Environment Requirements
#[derive(Debug, Clone)]
pub struct AppTopology {
    pub required_ram_mb: usize,
    pub required_cpu_cores: usize,
    pub required_storage_gb: usize,
    pub network_ports: Vec<u16>,
}

/// Murano Application Package
#[derive(Debug, Clone)]
pub struct ApplicationPackage {
    pub app_id: usize,
    pub name: String,
    pub version: String,
    pub description: String,
    pub category: String,
    pub author: String,
    pub topology: AppTopology,
    pub is_installed: bool,
}

/// Murano-style Application Catalogue Manager
pub struct MuranoApplicationCatalogueManager {
    pub catalogue: Vec<ApplicationPackage>,
    pub next_app_id: usize,
}

impl MuranoApplicationCatalogueManager {
    pub fn new() -> Self {
        let mut manager = MuranoApplicationCatalogueManager {
            catalogue: Vec::new(),
            next_app_id: 1,
        };

        // Seed default system application catalog packages
        manager.register_package(
            "SigmaOS Zenith Desktop",
            "2.4.0",
            "High-performance compositor desktop user environment",
            "Desktop",
            "SigmaOS Core Team",
            AppTopology {
                required_ram_mb: 512,
                required_cpu_cores: 1,
                required_storage_gb: 2,
                network_ports: vec![80, 443],
            },
        );

        manager.register_package(
            "PostgreSQL Database Server",
            "16.1",
            "Enterprise relational database engine",
            "Database",
            "PostgreSQL Global Development Group",
            AppTopology {
                required_ram_mb: 1024,
                required_cpu_cores: 2,
                required_storage_gb: 10,
                network_ports: vec![5432],
            },
        );

        manager
    }

    pub fn register_package(
        &mut self,
        name: &str,
        version: &str,
        description: &str,
        category: &str,
        author: &str,
        topology: AppTopology,
    ) -> usize {
        let id = self.next_app_id;
        self.next_app_id += 1;
        let pkg = ApplicationPackage {
            app_id: id,
            name: String::from(name),
            version: String::from(version),
            description: String::from(description),
            category: String::from(category),
            author: String::from(author),
            topology,
            is_installed: false,
        };
        self.catalogue.push(pkg);
        id
    }

    pub fn search_by_category(&self, category: &str) -> Vec<&ApplicationPackage> {
        self.catalogue
            .iter()
            .filter(|p| p.category.eq_ignore_ascii_case(category))
            .collect()
    }

    pub fn install_package(&mut self, app_id: usize) -> Result<(), &'static str> {
        if let Some(pkg) = self.catalogue.iter_mut().find(|p| p.app_id == app_id) {
            pkg.is_installed = true;
            Ok(())
        } else {
            Err("Application package not found in catalogue")
        }
    }
}

impl Default for MuranoApplicationCatalogueManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux Flatpak & Freedesktop AppStream Metadata Management Engine
/// Inspired by Flathub, GNOME Software, and KDE Discover AppStream XML/AppData spec
#[derive(Debug, Clone)]
pub struct AppStreamMetadata {
    pub app_id: String,
    pub name: String,
    pub summary: String,
    pub developer_name: String,
    pub project_license: String,
    pub appstream_version: String,
    pub categories: Vec<String>,
    pub sandbox_permissions: Vec<String>,
}

#[derive(Debug, Default)]
pub struct LinuxFlatpakAppStreamManagerEngine;

impl LinuxFlatpakAppStreamManagerEngine {
    pub fn new() -> Self {
        Self
    }

    /// Parses Flatpak AppStream appdata entry
    pub fn parse_appdata(&self, app_id: &str, name: &str, summary: &str, license: &str, permissions: &[&str]) -> AppStreamMetadata {
        AppStreamMetadata {
            app_id: String::from(app_id),
            name: String::from(name),
            summary: String::from(summary),
            developer_name: String::from("SigmaOS Community"),
            project_license: String::from(license),
            appstream_version: String::from("1.0"),
            categories: vec![String::from("Utility"), String::from("System")],
            sandbox_permissions: permissions.iter().map(|s| String::from(*s)).collect(),
        }
    }

    /// Calculates security compliance score (0-100) based on sandbox permissions
    pub fn evaluate_sandbox_rating(&self, meta: &AppStreamMetadata) -> u8 {
        let mut score: u8 = 100;
        for perm in &meta.sandbox_permissions {
            if perm.contains("host") || perm.contains("filesystem=home") {
                score = score.saturating_sub(25);
            }
            if perm.contains("device=all") {
                score = score.saturating_sub(30);
            }
            if perm.contains("socket=pulseaudio") || perm.contains("socket=wayland") {
                score = score.saturating_sub(5);
            }
        }
        score
    }
}

/// FreeBSD pkg (+ OpenBSD Ports) Application Manifest & Dependency Engine
/// Inspired by FreeBSD `/usr/ports`, `pkg` UCL manifest format, and OpenBSD pledge/unveil application isolation
#[derive(Debug, Clone)]
pub struct BsdPkgManifest {
    pub name: String,
    pub origin: String,
    pub version: String,
    pub comment: String,
    pub maintainer: String,
    pub website: String,
    pub dependencies: Vec<(String, String)>,
    pub categories: Vec<String>,
    pub pledge_isolated: bool,
}

#[derive(Debug, Default)]
pub struct FreeBsdPkgAppManifestEngine;

impl FreeBsdPkgAppManifestEngine {
    pub fn new() -> Self {
        Self
    }

    /// Creates a FreeBSD/OpenBSD inspired application package manifest
    pub fn create_manifest(
        &self,
        name: &str,
        origin: &str,
        version: &str,
        comment: &str,
        deps: &[(&str, &str)],
        pledge_isolated: bool,
    ) -> BsdPkgManifest {
        BsdPkgManifest {
            name: String::from(name),
            origin: String::from(origin),
            version: String::from(version),
            comment: String::from(comment),
            maintainer: String::from("ports@sigmaos.org"),
            website: String::from("https://sigmaos.org/ports"),
            dependencies: deps.iter().map(|(n, v)| (String::from(*n), String::from(*v))).collect(),
            categories: vec![String::from("sysutils")],
            pledge_isolated,
        }
    }

    /// Validates manifest structural completeness
    pub fn validate_manifest(&self, manifest: &BsdPkgManifest) -> Result<bool, &'static str> {
        if manifest.name.is_empty() {
            return Err("Manifest missing package name");
        }
        if manifest.origin.is_empty() {
            return Err("Manifest missing origin category/port path");
        }
        if manifest.version.is_empty() {
            return Err("Manifest missing package version");
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murano_catalogue() {
        let mut manager = MuranoApplicationCatalogueManager::new();
        let db_pkgs = manager.search_by_category("Database");
        assert_eq!(db_pkgs.len(), 1);
        let id = db_pkgs[0].app_id;

        manager.install_package(id).unwrap();
        let pkg = manager.catalogue.iter().find(|p| p.app_id == id).unwrap();
        assert!(pkg.is_installed);
    }

    #[test]
    fn test_linux_flatpak_appstream_manager() {
        let engine = LinuxFlatpakAppStreamManagerEngine::new();
        let meta = engine.parse_appdata(
            "org.sigmaos.TextEditor",
            "Sigma Edit",
            "Fast native text editor",
            "GPL-3.0-or-later",
            &["socket=wayland", "filesystem=home"],
        );
        assert_eq!(meta.app_id, "org.sigmaos.TextEditor");
        let score = engine.evaluate_sandbox_rating(&meta);
        assert!(score < 100);
    }

    #[test]
    fn test_freebsd_pkg_manifest_engine() {
        let engine = FreeBsdPkgAppManifestEngine::new();
        let manifest = engine.create_manifest(
            "sigma-tools",
            "sysutils/sigma-tools",
            "1.0.0",
            "SigmaOS system utility suite",
            &[("glibc", ">=2.35"), ("openssl", ">=3.0")],
            true,
        );
        assert_eq!(manifest.dependencies.len(), 2);
        assert!(engine.validate_manifest(&manifest).is_ok());
    }
}
