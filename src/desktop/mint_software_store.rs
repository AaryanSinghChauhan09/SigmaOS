//! SigmaOS Software Store & Flatpak AppStream Engine (`mintinstall` counterpart)
//!
//! Inspired by Linux Mint's Software Manager:
//! - Dual package origin support: Native (sigpkg) and Sandboxed (Flatpak / OCI)
//! - Fine-grained sandbox permission auditing (Network, Home folder, D-Bus, GPU acceleration)
//! - Categorized app catalog with community star ratings, reviews, and verified badges

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Packaging origin
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageFormat {
    NativeSigpkg,
    FlatpakFlathub,
    AppImage,
}

/// Sandbox security permissions required by an application
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandboxPermissions {
    pub network_access: bool,
    pub full_filesystem_access: bool,
    pub home_directory_access: bool,
    pub dbus_session_bus: bool,
    pub gpu_hardware_acceleration: bool,
    pub audio_pipewire_playback: bool,
    pub camera_microphone_access: bool,
}

impl SandboxPermissions {
    pub fn strict_sandbox() -> Self {
        Self {
            network_access: false,
            full_filesystem_access: false,
            home_directory_access: false,
            dbus_session_bus: false,
            gpu_hardware_acceleration: false,
            audio_pipewire_playback: false,
            camera_microphone_access: false,
        }
    }

    pub fn is_fully_sandboxed(&self) -> bool {
        !self.full_filesystem_access && !self.home_directory_access
    }
}

/// Application listing in the software store
#[derive(Debug, Clone)]
pub struct StoreAppListing {
    pub app_id: String,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub category: String,
    pub version: String,
    pub format: PackageFormat,
    pub permissions: SandboxPermissions,
    pub download_size_bytes: u64,
    pub installed_size_bytes: u64,
    pub rating_score: f32, // 1.0 .. 5.0
    pub total_reviews_count: u32,
    pub is_verified_developer: bool,
    pub is_installed: bool,
}

/// Linux Mint-Inspired Software Store Subsystem
pub struct MintSoftwareStoreEngine {
    pub catalog: BTreeMap<String, StoreAppListing>,
    pub installed_apps: Vec<String>,
}

impl MintSoftwareStoreEngine {
    pub fn new() -> Self {
        let mut store = Self {
            catalog: BTreeMap::new(),
            installed_apps: Vec::new(),
        };
        store.seed_curated_catalog();
        store
    }

    fn seed_curated_catalog(&mut self) {
        self.catalog.insert(
            "org.mozilla.firefox".into(),
            StoreAppListing {
                app_id: "org.mozilla.firefox".into(),
                name: "Firefox Web Browser".into(),
                summary: "Fast, private, and secure web browsing".into(),
                description: "The free and open-source web browser developed by Mozilla.".into(),
                category: "Internet".into(),
                version: "unknown".into(),
                format: PackageFormat::FlatpakFlathub,
                permissions: SandboxPermissions {
                    network_access: true,
                    full_filesystem_access: false,
                    home_directory_access: true,
                    dbus_session_bus: true,
                    gpu_hardware_acceleration: true,
                    audio_pipewire_playback: true,
                    camera_microphone_access: true,
                },
                download_size_bytes: 0,
                installed_size_bytes: 0,
                rating_score: 0.0,
                total_reviews_count: 0,
                is_verified_developer: false,
                is_installed: false,
            },
        );
        self.catalog.insert(
            "org.gimp.GIMP".into(),
            StoreAppListing {
                app_id: "org.gimp.GIMP".into(),
                name: "GIMP Image Editor".into(),
                summary: "Create and edit professional graphics".into(),
                description: "GNU Image Manipulation Program for photo retouching and authoring."
                    .into(),
                category: "Graphics".into(),
                version: "unknown".into(),
                format: PackageFormat::FlatpakFlathub,
                permissions: SandboxPermissions {
                    network_access: false,
                    full_filesystem_access: false,
                    home_directory_access: true,
                    dbus_session_bus: false,
                    gpu_hardware_acceleration: true,
                    audio_pipewire_playback: false,
                    camera_microphone_access: false,
                },
                download_size_bytes: 0,
                installed_size_bytes: 0,
                rating_score: 0.0,
                total_reviews_count: 0,
                is_verified_developer: false,
                is_installed: false,
            },
        );

        self.catalog.insert(
            "com.visualstudio.code".into(),
            StoreAppListing {
                app_id: "com.visualstudio.code".into(),
                name: "Visual Studio Code".into(),
                summary: "Extensible code editor and IDE".into(),
                description:
                    "Lightweight but powerful source code editor with built-in Git and debugging."
                        .into(),
                category: "Development".into(),
                version: "unknown".into(),
                format: PackageFormat::NativeSigpkg,
                permissions: SandboxPermissions {
                    network_access: true,
                    full_filesystem_access: true,
                    home_directory_access: true,
                    dbus_session_bus: true,
                    gpu_hardware_acceleration: true,
                    audio_pipewire_playback: false,
                    camera_microphone_access: false,
                },
                download_size_bytes: 0,
                installed_size_bytes: 0,
                rating_score: 0.0,
                total_reviews_count: 0,
                is_verified_developer: false,
                is_installed: false,
            },
        );
    }

    /// Search the catalog by query
    pub fn search(&self, query: &str) -> Vec<&StoreAppListing> {
        let q = query.to_lowercase();
        self.catalog
            .values()
            .filter(|app| {
                app.name.to_lowercase().contains(&q)
                    || app.summary.to_lowercase().contains(&q)
                    || app.category.to_lowercase().contains(&q)
            })
            .collect()
    }

    /// Install only when a transactional package backend is available.
    /// Catalog metadata alone must never be reported as an installed package.
    pub fn install_app(&mut self, app_id: &str) -> Result<String, &'static str> {
        let app = self
            .catalog
            .get(app_id)
            .ok_or("Application ID not found in store")?;
        if app.is_installed {
            return Err("Application is already installed");
        }
        Err("Package installation backend is unavailable")
    }

    /// Uninstall only when a transactional package backend is available.
    pub fn uninstall_app(&mut self, app_id: &str) -> Result<String, &'static str> {
        if !self
            .catalog
            .get(app_id)
            .ok_or("Application ID not found in store")?
            .is_installed
        {
            return Err("Application is not installed");
        }
        Err("Package removal backend is unavailable")
    }

    /// Audit security and calculate safety score (0 - 100)
    pub fn audit_app_security(&self, app_id: &str) -> Result<u32, &'static str> {
        let app = self.catalog.get(app_id).ok_or("Application ID not found")?;
        let mut score: i32 = 100;
        if app.permissions.full_filesystem_access {
            score -= 35;
        }
        if app.permissions.camera_microphone_access && app.category != "Communications" {
            score -= 20;
        }
        if app.permissions.home_directory_access {
            score -= 10;
        }
        if !app.is_verified_developer {
            score -= 15;
        }
        Ok(score.max(10) as u32)
    }

    /// Return a measured delta size when package-diff metadata is available.
    pub fn estimate_delta_download_bytes(&self, app_id: &str) -> Result<u64, &'static str> {
        self.catalog.get(app_id).ok_or("Application ID not found")?;
        Err("Measured package delta metadata is unavailable")
    }

    /// Create hardened sandbox permissions by revoking high-risk privileges
    pub fn harden_permissions(&mut self, app_id: &str) -> Result<SandboxPermissions, &'static str> {
        let app = self
            .catalog
            .get_mut(app_id)
            .ok_or("Application ID not found")?;
        app.permissions.full_filesystem_access = false;
        if app.category != "Communications" {
            app.permissions.camera_microphone_access = false;
        }
        Ok(app.permissions.clone())
    }

    /// Install a batch only through a transactional package backend.
    pub fn batch_install(&mut self, app_ids: &[&str]) -> Result<usize, &'static str> {
        for id in app_ids {
            if !self.catalog.contains_key(*id) {
                return Err("One or more apps in batch not found");
            }
        }
        for (index, id) in app_ids.iter().enumerate() {
            if self.catalog[*id].is_installed {
                return Err("One or more apps in batch is already installed");
            }
            if app_ids[..index].contains(id) {
                return Err("Duplicate application in install batch");
            }
        }
        if app_ids.is_empty() {
            return Ok(0);
        }
        Err("Transactional package installation backend is unavailable")
    }
}

impl Default for MintSoftwareStoreEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_search_and_install_fails_closed_without_backend() {
        let mut store = MintSoftwareStoreEngine::new();
        let search_results = store.search("graphics");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "GIMP Image Editor");

        assert_eq!(
            store.install_app("org.gimp.GIMP"),
            Err("Package installation backend is unavailable")
        );
        assert_eq!(
            store.uninstall_app("org.mozilla.firefox"),
            Err("Application is not installed")
        );
        assert!(store.installed_apps.is_empty());
        assert!(!store.catalog["org.gimp.GIMP"].is_installed);
        assert!(!store.installed_apps.contains(&"org.gimp.GIMP".to_string()));
    }

    #[test]
    fn failed_batch_install_leaves_catalog_and_installed_state_unchanged() {
        let mut store = MintSoftwareStoreEngine::new();
        let result = store.batch_install(&["org.gimp.GIMP", "com.visualstudio.code"]);
        assert_eq!(
            result,
            Err("Transactional package installation backend is unavailable")
        );
        assert!(store.installed_apps.is_empty());
        assert!(store.catalog.values().all(|app| !app.is_installed));
    }

    #[test]
    fn test_sandbox_permissions() {
        let store = MintSoftwareStoreEngine::new();
        let firefox = store.catalog.get("org.mozilla.firefox").unwrap();
        assert!(firefox.permissions.network_access);
        assert!(!firefox.permissions.full_filesystem_access);
    }

    #[test]
    fn test_audit_and_hardening() {
        let mut store = MintSoftwareStoreEngine::new();
        let score = store.audit_app_security("com.visualstudio.code").unwrap();
        assert!(score < 100);

        assert_eq!(
            store.estimate_delta_download_bytes("org.mozilla.firefox"),
            Err("Measured package delta metadata is unavailable")
        );

        let hardened = store.harden_permissions("com.visualstudio.code").unwrap();
        assert!(!hardened.full_filesystem_access);

        let batch_res = store.batch_install(&["org.gimp.GIMP"]);
        assert_eq!(
            batch_res,
            Err("Transactional package installation backend is unavailable")
        );
    }
}
