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
                version: "130.0".into(),
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
                download_size_bytes: 85_000_000,
                installed_size_bytes: 240_000_000,
                rating_score: 4.8,
                total_reviews_count: 12450,
                is_verified_developer: true,
                is_installed: true,
            },
        );
        self.installed_apps.push("org.mozilla.firefox".into());

        self.catalog.insert(
            "org.gimp.GIMP".into(),
            StoreAppListing {
                app_id: "org.gimp.GIMP".into(),
                name: "GIMP Image Editor".into(),
                summary: "Create and edit professional graphics".into(),
                description: "GNU Image Manipulation Program for photo retouching and authoring.".into(),
                category: "Graphics".into(),
                version: "3.0.0-RC1".into(),
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
                download_size_bytes: 140_000_000,
                installed_size_bytes: 380_000_000,
                rating_score: 4.6,
                total_reviews_count: 8320,
                is_verified_developer: true,
                is_installed: false,
            },
        );

        self.catalog.insert(
            "com.visualstudio.code".into(),
            StoreAppListing {
                app_id: "com.visualstudio.code".into(),
                name: "Visual Studio Code".into(),
                summary: "Extensible code editor and IDE".into(),
                description: "Lightweight but powerful source code editor with built-in Git and debugging.".into(),
                category: "Development".into(),
                version: "1.93.0".into(),
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
                download_size_bytes: 98_000_000,
                installed_size_bytes: 310_000_000,
                rating_score: 4.9,
                total_reviews_count: 34100,
                is_verified_developer: true,
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

    /// Install an application from the store
    pub fn install_app(&mut self, app_id: &str) -> Result<String, &'static str> {
        let app = self.catalog.get_mut(app_id).ok_or("Application ID not found in store")?;
        if app.is_installed {
            return Err("Application is already installed");
        }
        app.is_installed = true;
        self.installed_apps.push(app_id.to_string());
        Ok(format!("Successfully installed '{}' via {:?}", app.name, app.format))
    }

    /// Uninstall an application
    pub fn uninstall_app(&mut self, app_id: &str) -> Result<String, &'static str> {
        let app = self.catalog.get_mut(app_id).ok_or("Application ID not found in store")?;
        if !app.is_installed {
            return Err("Application is not installed");
        }
        app.is_installed = false;
        self.installed_apps.retain(|id| id != app_id);
        Ok(format!("Successfully uninstalled '{}'", app.name))
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
    fn test_store_search_and_install() {
        let mut store = MintSoftwareStoreEngine::new();
        let search_results = store.search("graphics");
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].name, "GIMP Image Editor");

        let res = store.install_app("org.gimp.GIMP");
        assert!(res.is_ok());
        assert!(store.installed_apps.contains(&"org.gimp.GIMP".to_string()));

        let uninst = store.uninstall_app("org.gimp.GIMP");
        assert!(uninst.is_ok());
        assert!(!store.installed_apps.contains(&"org.gimp.GIMP".to_string()));
    }

    #[test]
    fn test_sandbox_permissions() {
        let store = MintSoftwareStoreEngine::new();
        let firefox = store.catalog.get("org.mozilla.firefox").unwrap();
        assert!(firefox.permissions.network_access);
        assert!(!firefox.permissions.full_filesystem_access);
    }
}
