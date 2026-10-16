// SPDX-License-Identifier: MIT
// SigmaOS - Waypaper Wallpaper Manager
// Inspired by Waypaper (https://github.com/copycat-killer/waypaper)
// Provides wallpaper management for Wayland

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Wallpaper
#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub path: String,
    pub name: String,
    pub favorite: bool,
}

/// Waypaper instance
pub struct WaypaperInstance {
    pub wallpapers: Vec<Wallpaper>,
    pub current_wallpaper: Option<String>,
    pub active: AtomicBool,
}

impl WaypaperInstance {
    pub fn new() -> Self {
        Self {
            wallpapers: Vec::new(),
            current_wallpaper: None,
            active: AtomicBool::new(false),
        }
    }

    /// Initialize Waypaper
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
        self.load_wallpapers();
    }

    /// Load wallpapers
    pub fn load_wallpapers(&mut self) {
        // In real implementation, would scan wallpaper directories
        // ~/.local/share/wallpapers/
        // /usr/share/wallpapers/

        // Add some sample wallpapers
        self.wallpapers.push(Wallpaper {
            path: "/usr/share/wallpapers/default.jpg".to_string(),
            name: "Default".to_string(),
            favorite: true,
        });
    }

    /// Add wallpaper
    pub fn add_wallpaper(&mut self, path: String, name: String) {
        let wallpaper = Wallpaper {
            path,
            name,
            favorite: false,
        };
        self.wallpapers.push(wallpaper);
    }

    /// Set wallpaper
    pub fn set_wallpaper(&mut self, path: String) -> Result<(), &'static str> {
        // In real implementation, would use swww or hyprpaper:
        // swww img {path}
        // hyprctl hyprpaper wallpaper {path}

        self.current_wallpaper = Some(path);
        Ok(())
    }

    /// Get current wallpaper
    pub fn get_current_wallpaper(&self) -> Option<&String> {
        self.current_wallpaper.as_ref()
    }

    /// Get all wallpapers
    pub fn get_wallpapers(&self) -> &Vec<Wallpaper> {
        &self.wallpapers
    }

    /// Get favorite wallpapers
    pub fn get_favorites(&self) -> Vec<&Wallpaper> {
        self.wallpapers.iter().filter(|w| w.favorite).collect()
    }

    /// Set favorite
    pub fn set_favorite(&mut self, path: String, favorite: bool) -> Result<(), &'static str> {
        for wallpaper in &mut self.wallpapers {
            if wallpaper.path == path {
                wallpaper.favorite = favorite;
                return Ok(());
            }
        }
        Err("Wallpaper not found")
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for WaypaperInstance {
    fn default() -> Self {
        Self::new()
    }
}

/// Global Waypaper instance
static mut WAYPAPER_INSTANCE: Option<WaypaperInstance> = None;

/// Initialize Waypaper
pub fn init_waypaper() {
    unsafe {
        WAYPAPER_INSTANCE = Some(WaypaperInstance::new());
    }
}

/// Get global Waypaper instance
pub fn get_waypaper() -> Option<&'static mut WaypaperInstance> {
    unsafe { WAYPAPER_INSTANCE.as_mut() }
}

/// Set wallpaper (convenience function)
pub fn set_wallpaper(path: String) -> Result<(), &'static str> {
    if let Some(waypaper) = get_waypaper() {
        waypaper.set_wallpaper(path)
    } else {
        Err("Waypaper not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypaper_init() {
        let mut waypaper = WaypaperInstance::new();
        waypaper.init();
        assert!(waypaper.is_active());
        assert!(waypaper.wallpapers.len() > 0);
    }

    #[test]
    fn test_add_wallpaper() {
        let mut waypaper = WaypaperInstance::new();
        waypaper.add_wallpaper("/path/to/wallpaper.jpg".to_string(), "Test".to_string());
        assert_eq!(waypaper.wallpapers.len(), 1);
    }

    #[test]
    fn test_set_wallpaper() {
        let mut waypaper = WaypaperInstance::new();
        waypaper
            .set_wallpaper("/path/to/wallpaper.jpg".to_string())
            .unwrap();
        assert!(waypaper.get_current_wallpaper().is_some());
    }

    #[test]
    fn test_set_favorite() {
        let mut waypaper = WaypaperInstance::new();
        waypaper.add_wallpaper("/path/to/wallpaper.jpg".to_string(), "Test".to_string());
        waypaper
            .set_favorite("/path/to/wallpaper.jpg".to_string(), true)
            .unwrap();
        assert!(waypaper.get_favorites().len() > 0);
    }
}
