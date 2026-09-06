#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::vec::Vec;
use std::string::{String, ToString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppReview {
    pub author: String,
    pub rating: u32, // 1 to 5 stars
    pub comment: String,
    pub helpful_votes: u32,
}

impl AppReview {
    pub fn new(author: &str, rating: u32, comment: &str) -> Self {
        Self {
            author: author.to_string(),
            rating: rating.clamp(1, 5),
            comment: comment.to_string(),
            helpful_votes: 0,
        }
    }
}

/// Flathub / GNOME Software Sandbox Permission Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppSandboxPermissions {
    pub network_access: bool,
    pub filesystem_full_access: bool,
    pub audio_access: bool,
    pub camera_access: bool,
    pub gpu_acceleration: bool,
}

impl AppSandboxPermissions {
    pub fn strict_default() -> Self {
        Self {
            network_access: false,
            filesystem_full_access: false,
            audio_access: false,
            camera_access: false,
            gpu_acceleration: true,
        }
    }
}

/// Elementary AppCenter / Pop!_OS Pop_Shop Pay-What-You-Want Monetization Tier
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMonetizationTier {
    Free,
    PayWhatYouWant { suggested_amount_usd: u32 },
    Commercial { price_usd: u32 },
}

/// Arch AUR / FreeBSD Ports Build-from-Source Configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildFromSourceConfig {
    pub enabled: bool,
    pub custom_cflags: String,
    pub make_jobs: u32,
}

/// NixOS / Flatpak Delta Updates & Restore Snapshot
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftwareUpdateSnapshot {
    pub snapshot_id: u64,
    pub app_name: String,
    pub previous_version: String,
    pub delta_size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppStoreItem {
    pub name: String,
    pub version: String,
    pub developer: String,
    pub description: String,
    pub category: String,
    pub size_bytes: u64,
    pub installed: bool,
    pub reviews: Vec<AppReview>,
    pub sandbox_permissions: AppSandboxPermissions,
    pub monetization_tier: AppMonetizationTier,
    pub build_from_source: BuildFromSourceConfig,
}

impl AppStoreItem {
    pub fn new(name: &str, version: &str, developer: &str, description: &str, category: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            developer: developer.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            size_bytes: size,
            installed: false,
            reviews: Vec::new(),
            sandbox_permissions: AppSandboxPermissions::strict_default(),
            monetization_tier: AppMonetizationTier::Free,
            build_from_source: BuildFromSourceConfig {
                enabled: false,
                custom_cflags: "-O2 -pipe".to_string(),
                make_jobs: 4,
            },
        }
    }

    pub fn get_average_rating(&self) -> f32 {
        if self.reviews.is_empty() {
            return 0.0;
        }
        let total_rating: u32 = self.reviews.iter().map(|r| r.rating).sum();
        total_rating as f32 / self.reviews.len() as f32
    }
}

pub struct GuiAppStore {
    pub items: Vec<AppStoreItem>,
}

impl GuiAppStore {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn register_app(&mut self, item: AppStoreItem) -> Result<(), &'static str> {
        if self.items.iter().any(|i| i.name == item.name) {
            return Err("Application is already registered in the store catalog");
        }
        self.items.push(item);
        Ok(())
    }

    pub fn install_app(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == name) {
            if item.installed {
                return Err("Application is already installed");
            }
            item.installed = true;
            Ok(())
        } else {
            Err("Application not found in store catalog")
        }
    }

    pub fn uninstall_app(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == name) {
            if !item.installed {
                return Err("Application is not installed");
            }
            item.installed = false;
            Ok(())
        } else {
            Err("Application not found in store catalog")
        }
    }

    pub fn add_review(&mut self, app_name: &str, review: AppReview) -> Result<(), &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.name == app_name) {
            item.reviews.push(review);
            Ok(())
        } else {
            Err("Application not found in store catalog")
        }
    }

    pub fn search_apps(&self, query: &str) -> Vec<&AppStoreItem> {
        let mut results = Vec::new();
        let query_lower = query.to_string(); // simple mock search
        for item in &self.items {
            if item.name.contains(&query_lower) || item.description.contains(&query_lower) {
                results.push(item);
            }
        }
        results
    }

    pub fn filter_by_category(&self, category: &str) -> Vec<&AppStoreItem> {
        let mut results = Vec::new();
        for item in &self.items {
            if item.category == category {
                results.push(item);
            }
        }
        results
    }
}

impl Default for GuiAppStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_app_sandbox_permissions() {
        let perms = AppSandboxPermissions::strict_default();
        assert!(!perms.network_access);
        assert!(!perms.filesystem_full_access);
        assert!(perms.gpu_acceleration);
    }

    #[test]
    fn test_app_monetization_tier() {
        let app = AppStoreItem::new("GIMP", "2.10", "GIMP Team", "Image Editor", "Graphics", 120_000_000);
        assert_eq!(app.monetization_tier, AppMonetizationTier::Free);
    }

    #[test]
    fn test_build_from_source_config() {
        let app = AppStoreItem::new("Neovim", "0.9", "Neovim Core", "Text Editor", "Development", 15_000_000);
        assert_eq!(app.build_from_source.custom_cflags, "-O2 -pipe");
        assert_eq!(app.build_from_source.make_jobs, 4);
    }

    #[test]
    fn test_software_update_snapshot() {
        let snapshot = SoftwareUpdateSnapshot {
            snapshot_id: 101,
            app_name: "Firefox".to_string(),
            previous_version: "115.0".to_string(),
            delta_size_bytes: 5_400_000,
        };
        assert_eq!(snapshot.snapshot_id, 101);
        assert_eq!(snapshot.app_name, "Firefox");
    }
}
