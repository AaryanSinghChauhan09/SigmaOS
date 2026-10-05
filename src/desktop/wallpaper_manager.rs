// SigmaOS Wallpaper Manager
// Inspired by Linux Mint's wallpaper settings and Omarchy's wallpaper utilities

use std::collections::HashMap;
use std::path::PathBuf;

/// Wallpaper mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperMode {
    Stretch,
    Fit,
    Fill,
    Center,
    Tile,
    Span,
}

impl WallpaperMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            WallpaperMode::Stretch => "Stretch",
            WallpaperMode::Fit => "Fit",
            WallpaperMode::Fill => "Fill",
            WallpaperMode::Center => "Center",
            WallpaperMode::Tile => "Tile",
            WallpaperMode::Span => "Span",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "stretch" => Some(WallpaperMode::Stretch),
            "fit" => Some(WallpaperMode::Fit),
            "fill" => Some(WallpaperMode::Fill),
            "center" => Some(WallpaperMode::Center),
            "tile" => Some(WallpaperMode::Tile),
            "span" => Some(WallpaperMode::Span),
            _ => None,
        }
    }
}

/// Wallpaper source
#[derive(Debug, Clone)]
pub struct WallpaperSource {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub thumbnail: Option<PathBuf>,
    pub is_default: bool,
}

impl WallpaperSource {
    pub fn new(id: String, name: String, path: PathBuf) -> Self {
        WallpaperSource {
            id,
            name,
            path,
            thumbnail: None,
            is_default: false,
        }
    }

    pub fn set_thumbnail(&mut self, thumbnail: PathBuf) {
        self.thumbnail = Some(thumbnail);
    }

    pub fn set_default(&mut self, default: bool) {
        self.is_default = default;
    }
}

/// Wallpaper profile
#[derive(Debug, Clone)]
pub struct WallpaperProfile {
    pub id: String,
    pub name: String,
    pub wallpaper_id: String,
    pub mode: WallpaperMode,
    pub is_dark: bool,
}

impl WallpaperProfile {
    pub fn new(id: String, name: String, wallpaper_id: String, mode: WallpaperMode) -> Self {
        WallpaperProfile {
            id,
            name,
            wallpaper_id,
            mode,
            is_dark: false,
        }
    }

    pub fn set_dark(&mut self, dark: bool) {
        self.is_dark = dark;
    }
}

/// Wallpaper Manager
pub struct WallpaperManager {
    sources: HashMap<String, WallpaperSource>,
    profiles: HashMap<String, WallpaperProfile>,
    current_wallpaper: Option<String>,
    current_profile: Option<String>,
    auto_change_enabled: bool,
    auto_change_interval: u32, // minutes
    next_source_id: u32,
    next_profile_id: u32,
}

impl WallpaperManager {
    pub fn new() -> Self {
        let mut manager = WallpaperManager {
            sources: HashMap::new(),
            profiles: HashMap::new(),
            current_wallpaper: None,
            current_profile: None,
            auto_change_enabled: false,
            auto_change_interval: 30,
            next_source_id: 1,
            next_profile_id: 1,
        };

        // Add default wallpapers
        manager.add_default_sources();

        // Add default profile
        manager.add_default_profile();

        manager
    }

    fn add_default_sources(&mut self) {
        let mut sigma_blue = WallpaperSource::new(
            format!("src_{}", self.next_source_id),
            "Sigma Blue".to_string(),
            PathBuf::from("/usr/share/wallpapers/sigma-blue.jpg"),
        );
        sigma_blue.set_default(true);
        self.sources.insert(sigma_blue.id.clone(), sigma_blue);
        self.next_source_id += 1;

        let sigma_dark = WallpaperSource::new(
            format!("src_{}", self.next_source_id),
            "Sigma Dark".to_string(),
            PathBuf::from("/usr/share/wallpapers/sigma-dark.jpg"),
        );
        self.sources.insert(sigma_dark.id.clone(), sigma_dark);
        self.next_source_id += 1;

        let nature = WallpaperSource::new(
            format!("src_{}", self.next_source_id),
            "Nature".to_string(),
            PathBuf::from("/usr/share/wallpapers/nature.jpg"),
        );
        self.sources.insert(nature.id.clone(), nature);
        self.next_source_id += 1;
    }

    fn add_default_profile(&mut self) {
        if let Some(default_source) = self.sources.values().find(|s| s.is_default) {
            let profile_id = format!("prof_{}", self.next_profile_id);
            let profile = WallpaperProfile::new(
                profile_id.clone(),
                "Default".to_string(),
                default_source.id.clone(),
                WallpaperMode::Fill,
            );
            self.profiles.insert(profile_id.clone(), profile);
            self.next_profile_id += 1;
            self.current_profile = Some(profile_id);
            self.current_wallpaper = Some(default_source.id.clone());
        }
    }

    pub fn add_source(&mut self, name: String, path: PathBuf) -> String {
        let id = format!("src_{}", self.next_source_id);
        let source = WallpaperSource::new(id.clone(), name, path);
        self.sources.insert(id.clone(), source);
        self.next_source_id += 1;
        id
    }

    pub fn remove_source(&mut self, id: &str) -> bool {
        if let Some(source) = self.sources.get(id) {
            if source.is_default {
                return false; // Cannot remove default source
            }
        }
        self.sources.remove(id).is_some()
    }

    pub fn get_source(&self, id: &str) -> Option<&WallpaperSource> {
        self.sources.get(id)
    }

    pub fn get_sources(&self) -> Vec<&WallpaperSource> {
        self.sources.values().collect()
    }

    pub fn set_default_source(&mut self, id: &str) -> bool {
        if !self.sources.contains_key(id) {
            return false;
        }

        // Clear default from all sources
        for source in self.sources.values_mut() {
            source.set_default(false);
        }

        // Set new default
        if let Some(source) = self.sources.get_mut(id) {
            source.set_default(true);
            true
        } else {
            false
        }
    }

    pub fn add_profile(&mut self, name: String, wallpaper_id: String, mode: WallpaperMode) -> String {
        let id = format!("prof_{}", self.next_profile_id);
        let profile = WallpaperProfile::new(id.clone(), name, wallpaper_id, mode);
        self.profiles.insert(id.clone(), profile);
        self.next_profile_id += 1;
        id
    }

    pub fn remove_profile(&mut self, id: &str) -> bool {
        self.profiles.remove(id).is_some()
    }

    pub fn get_profile(&self, id: &str) -> Option<&WallpaperProfile> {
        self.profiles.get(id)
    }

    pub fn get_profiles(&self) -> Vec<&WallpaperProfile> {
        self.profiles.values().collect()
    }

    pub fn set_wallpaper(&mut self, source_id: &str) -> bool {
        if !self.sources.contains_key(source_id) {
            return false;
        }
        self.current_wallpaper = Some(source_id.to_string());
        true
    }

    pub fn get_current_wallpaper(&self) -> Option<&WallpaperSource> {
        if let Some(current_id) = &self.current_wallpaper {
            self.sources.get(current_id)
        } else {
            None
        }
    }

    pub fn set_profile(&mut self, profile_id: &str) -> bool {
        if let Some(profile) = self.profiles.get(profile_id) {
            self.current_profile = Some(profile_id.to_string());
            self.current_wallpaper = Some(profile.wallpaper_id.clone());
            true
        } else {
            false
        }
    }

    pub fn get_current_profile(&self) -> Option<&WallpaperProfile> {
        if let Some(current_id) = &self.current_profile {
            self.profiles.get(current_id)
        } else {
            None
        }
    }

    pub fn is_auto_change_enabled(&self) -> bool {
        self.auto_change_enabled
    }

    pub fn set_auto_change_enabled(&mut self, enabled: bool) {
        self.auto_change_enabled = enabled;
    }

    pub fn get_auto_change_interval(&self) -> u32 {
        self.auto_change_interval
    }

    pub fn set_auto_change_interval(&mut self, interval: u32) {
        self.auto_change_interval = interval.max(1);
    }

    pub fn cycle_wallpaper(&mut self) -> bool {
        let source_ids: Vec<String> = self.sources.keys().cloned().collect();
        if source_ids.is_empty() {
            return false;
        }

        if let Some(current) = &self.current_wallpaper {
            if let Some(pos) = source_ids.iter().position(|id| id == current) {
                let next_pos = (pos + 1) % source_ids.len();
                self.set_wallpaper(&source_ids[next_pos])
            } else {
                self.set_wallpaper(&source_ids[0])
            }
        } else {
            self.set_wallpaper(&source_ids[0])
        }
    }

    pub fn get_statistics(&self) -> WallpaperStatistics {
        WallpaperStatistics {
            source_count: self.sources.len(),
            profile_count: self.profiles.len(),
            auto_change_enabled: self.auto_change_enabled,
            auto_change_interval: self.auto_change_interval,
        }
    }
}

impl Default for WallpaperManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Wallpaper statistics
#[derive(Debug, Clone, Copy)]
pub struct WallpaperStatistics {
    pub source_count: usize,
    pub profile_count: usize,
    pub auto_change_enabled: bool,
    pub auto_change_interval: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallpaper_mode_from_str() {
        assert_eq!(WallpaperMode::from_str("stretch"), Some(WallpaperMode::Stretch));
        assert_eq!(WallpaperMode::from_str("fill"), Some(WallpaperMode::Fill));
        assert_eq!(WallpaperMode::from_str("invalid"), None);
    }

    #[test]
    fn test_wallpaper_manager_initialization() {
        let manager = WallpaperManager::new();
        assert_eq!(manager.get_sources().len(), 3);
        assert_eq!(manager.get_profiles().len(), 1);
        assert!(manager.get_current_wallpaper().is_some());
    }

    #[test]
    fn test_add_source() {
        let mut manager = WallpaperManager::new();
        let id = manager.add_source(
            "Custom".to_string(),
            PathBuf::from("/path/to/wallpaper.jpg"),
        );
        assert!(manager.get_source(&id).is_some());
        assert_eq!(manager.get_sources().len(), 4);
    }

    #[test]
    fn test_remove_source() {
        let mut manager = WallpaperManager::new();
        let id = manager.add_source(
            "Custom".to_string(),
            PathBuf::from("/path/to/wallpaper.jpg"),
        );
        assert!(manager.remove_source(&id));
        assert!(!manager.get_source(&id).is_some());
        assert_eq!(manager.get_sources().len(), 3);
    }

    #[test]
    fn test_set_default_source() {
        let mut manager = WallpaperManager::new();
        let id = manager.add_source(
            "Custom".to_string(),
            PathBuf::from("/path/to/wallpaper.jpg"),
        );
        assert!(manager.set_default_source(&id));
        assert!(manager.get_source(&id).unwrap().is_default);
    }

    #[test]
    fn test_add_profile() {
        let mut manager = WallpaperManager::new();
        let source_id = manager.get_sources()[0].id.clone();
        let id = manager.add_profile(
            "Custom Profile".to_string(),
            source_id,
            WallpaperMode::Fit,
        );
        assert!(manager.get_profile(&id).is_some());
        assert_eq!(manager.get_profiles().len(), 2);
    }

    #[test]
    fn test_set_wallpaper() {
        let mut manager = WallpaperManager::new();
        let id = manager.add_source(
            "Custom".to_string(),
            PathBuf::from("/path/to/wallpaper.jpg"),
        );
        assert!(manager.set_wallpaper(&id));
        assert_eq!(manager.get_current_wallpaper().unwrap().id, id);
    }

    #[test]
    fn test_set_profile() {
        let mut manager = WallpaperManager::new();
        let source_id = manager.get_sources()[0].id.clone();
        let id = manager.add_profile(
            "Custom Profile".to_string(),
            source_id,
            WallpaperMode::Fit,
        );
        assert!(manager.set_profile(&id));
        assert_eq!(manager.get_current_profile().unwrap().id, id);
    }

    #[test]
    fn test_auto_change() {
        let mut manager = WallpaperManager::new();
        manager.set_auto_change_enabled(true);
        manager.set_auto_change_interval(60);
        assert!(manager.is_auto_change_enabled());
        assert_eq!(manager.get_auto_change_interval(), 60);
    }

    #[test]
    fn test_cycle_wallpaper() {
        let mut manager = WallpaperManager::new();
        let current_id = manager.get_current_wallpaper().unwrap().id.clone();
        assert!(manager.cycle_wallpaper());
        let new_id = manager.get_current_wallpaper().unwrap().id.clone();
        assert_ne!(current_id, new_id);
    }

    #[test]
    fn test_statistics() {
        let manager = WallpaperManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.source_count, 3);
        assert_eq!(stats.profile_count, 1);
    }
}
