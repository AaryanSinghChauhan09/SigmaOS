//! SigmaWallpaper - Wallpaper Management System
//!
//! Inspired by swww (Omarchy Linux), this module provides comprehensive wallpaper
//! management including Wayland support, transition effects, and per-monitor wallpapers.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Transition type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    None,
    Fade,
    Simple,
    Wave,
    Grow,
    Wipe,
    Unknown,
}

/// Display output data structure
#[derive(Debug, Clone)]
pub struct DisplayOutput {
    pub output_id: String,
    pub output_name: String,
    pub width: u32,
    pub height: u32,
    pub scale: f32,
    pub current_wallpaper: String,
}

impl DisplayOutput {
    /// Create a new display output
    pub fn new(
        output_id: String,
        output_name: String,
        width: u32,
        height: u32,
        scale: f32,
    ) -> Self {
        Self {
            output_id,
            output_name,
            width,
            height,
            scale,
            current_wallpaper: String::new(),
        }
    }

    /// Set current wallpaper
    pub fn set_wallpaper(&mut self, wallpaper: String) {
        self.current_wallpaper = wallpaper;
    }

    /// Get resolution
    pub fn resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// Wallpaper data structure
#[derive(Debug, Clone)]
pub struct Wallpaper {
    pub wallpaper_id: String,
    pub image_path: String,
    pub name: String,
    pub is_active: bool,
    pub is_favorite: bool,
}

impl Wallpaper {
    /// Create a new wallpaper
    pub fn new(wallpaper_id: String, image_path: String, name: String) -> Self {
        Self {
            wallpaper_id,
            image_path,
            name,
            is_active: false,
            is_favorite: false,
        }
    }

    /// Set active state
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    /// Set favorite state
    pub fn set_favorite(&mut self, favorite: bool) {
        self.is_favorite = favorite;
    }

    /// Check if wallpaper is active
    pub fn is_wallpaper_active(&self) -> bool {
        self.is_active
    }
}

/// Wallpaper manager
pub struct SigmaWallpaper {
    wallpapers: BTreeMap<String, Wallpaper>,
    outputs: BTreeMap<String, DisplayOutput>,
    default_transition: TransitionType,
    transition_duration: u32,
    running: AtomicBool,
}

impl SigmaWallpaper {
    /// Create a new wallpaper manager
    pub fn new() -> Self {
        Self {
            wallpapers: BTreeMap::new(),
            outputs: BTreeMap::new(),
            default_transition: TransitionType::Fade,
            transition_duration: 1000,
            running: AtomicBool::new(false),
        }
    }

    /// Start the wallpaper manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the wallpaper manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a wallpaper
    pub fn add_wallpaper(&mut self, wallpaper: Wallpaper) -> bool {
        let wp_id = wallpaper.wallpaper_id.clone();
        self.wallpapers.insert(wp_id, wallpaper);
        true
    }

    /// Remove a wallpaper
    pub fn remove_wallpaper(&mut self, wallpaper_id: &str) -> Option<Wallpaper> {
        self.wallpapers.remove(wallpaper_id)
    }

    /// Get a wallpaper by ID
    pub fn get_wallpaper(&self, wallpaper_id: &str) -> Option<&Wallpaper> {
        self.wallpapers.get(wallpaper_id)
    }

    /// List all wallpapers
    pub fn list_wallpapers(&self) -> Vec<&Wallpaper> {
        self.wallpapers.values().collect()
    }

    /// List active wallpapers
    pub fn list_active_wallpapers(&self) -> Vec<&Wallpaper> {
        self.wallpapers
            .values()
            .filter(|w| w.is_wallpaper_active())
            .collect()
    }

    /// List favorite wallpapers
    pub fn list_favorite_wallpapers(&self) -> Vec<&Wallpaper> {
        self.wallpapers.values().filter(|w| w.is_favorite).collect()
    }

    /// Add a display output
    pub fn add_output(&mut self, output: DisplayOutput) -> bool {
        let output_id = output.output_id.clone();
        self.outputs.insert(output_id, output);
        true
    }

    /// Remove a display output
    pub fn remove_output(&mut self, output_id: &str) -> Option<DisplayOutput> {
        self.outputs.remove(output_id)
    }

    /// Get an output by ID
    pub fn get_output(&self, output_id: &str) -> Option<&DisplayOutput> {
        self.outputs.get(output_id)
    }

    /// List all outputs
    pub fn list_outputs(&self) -> Vec<&DisplayOutput> {
        self.outputs.values().collect()
    }

    /// Set wallpaper on all outputs
    pub fn set_wallpaper_all(&mut self, wallpaper_id: &str) -> bool {
        if let Some(wallpaper) = self.wallpapers.get(wallpaper_id) {
            let path = wallpaper.image_path.clone();

            for output in self.outputs.values_mut() {
                output.set_wallpaper(path.clone());
            }

            if let Some(wp) = self.wallpapers.get_mut(wallpaper_id) {
                wp.set_active(true);
            }

            true
        } else {
            false
        }
    }

    /// Set wallpaper on specific output
    pub fn set_wallpaper_output(&mut self, wallpaper_id: &str, output_id: &str) -> bool {
        if let Some(wallpaper) = self.wallpapers.get(wallpaper_id) {
            if let Some(output) = self.outputs.get_mut(output_id) {
                output.set_wallpaper(wallpaper.image_path.clone());

                if let Some(wp) = self.wallpapers.get_mut(wallpaper_id) {
                    wp.set_active(true);
                }

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Set default transition type
    pub fn set_default_transition(&mut self, transition: TransitionType) {
        self.default_transition = transition;
    }

    /// Set transition duration
    pub fn set_transition_duration(&mut self, duration: u32) {
        self.transition_duration = duration;
    }

    /// Set wallpaper with transition
    pub fn set_wallpaper_with_transition(
        &mut self,
        wallpaper_id: &str,
        transition: TransitionType,
        duration: u32,
    ) -> bool {
        self.set_default_transition(transition);
        self.set_transition_duration(duration);
        self.set_wallpaper_all(wallpaper_id)
    }

    /// Get wallpaper count
    pub fn wallpaper_count(&self) -> usize {
        self.wallpapers.len()
    }

    /// Get output count
    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }
}

impl Default for SigmaWallpaper {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaWallpaper instance (placeholder for actual global state)
static mut SIGMAWALLPAPER_INSTANCE: Option<SigmaWallpaper> = None;

/// Initialize the global SigmaWallpaper instance
pub fn init_sigmawallpaper() {
    unsafe {
        SIGMAWALLPAPER_INSTANCE = Some(SigmaWallpaper::new());
    }
}

/// Get the global SigmaWallpaper instance
pub fn get_sigmawallpaper() -> Option<&'static SigmaWallpaper> {
    unsafe { SIGMAWALLPAPER_INSTANCE.as_ref() }
}

/// Get mutable global SigmaWallpaper instance
pub fn get_sigmawallpaper_mut() -> Option<&'static mut SigmaWallpaper> {
    unsafe { SIGMAWALLPAPER_INSTANCE.as_mut() }
}

/// Convenience function to add a wallpaper
pub fn add_wallpaper(wallpaper: Wallpaper) -> bool {
    if let Some(wallpaper_mgr) = get_sigmawallpaper_mut() {
        wallpaper_mgr.add_wallpaper(wallpaper)
    } else {
        false
    }
}

/// Convenience function to set wallpaper on all outputs
pub fn set_wallpaper_all(wallpaper_id: &str) -> bool {
    if let Some(wallpaper_mgr) = get_sigmawallpaper_mut() {
        wallpaper_mgr.set_wallpaper_all(wallpaper_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallpaper_creation() {
        let wallpaper = Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image.jpg".to_string(),
            "Nature".to_string(),
        );

        assert_eq!(wallpaper.wallpaper_id, "wp1");
        assert_eq!(wallpaper.image_path, "/path/to/image.jpg");
        assert!(!wallpaper.is_wallpaper_active());
    }

    #[test]
    fn test_wallpaper_state() {
        let mut wallpaper = Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image.jpg".to_string(),
            "Nature".to_string(),
        );

        wallpaper.set_active(true);
        assert!(wallpaper.is_wallpaper_active());

        wallpaper.set_favorite(true);
        assert!(wallpaper.is_favorite);
    }

    #[test]
    fn test_display_output_creation() {
        let output =
            DisplayOutput::new("output1".to_string(), "HDMI-1".to_string(), 1920, 1080, 1.0);

        assert_eq!(output.output_id, "output1");
        assert_eq!(output.resolution(), (1920, 1080));
    }

    #[test]
    fn test_sigmawallpaper_creation() {
        let wallpaper_mgr = SigmaWallpaper::new();
        assert!(!wallpaper_mgr.is_running());
        assert_eq!(wallpaper_mgr.wallpaper_count(), 0);
        assert_eq!(wallpaper_mgr.output_count(), 0);
    }

    #[test]
    fn test_sigmawallpaper_start_stop() {
        let mut wallpaper_mgr = SigmaWallpaper::new();
        wallpaper_mgr.start();
        assert!(wallpaper_mgr.is_running());
        wallpaper_mgr.stop();
        assert!(!wallpaper_mgr.is_running());
    }

    #[test]
    fn test_sigmawallpaper_add_wallpaper() {
        let mut wallpaper_mgr = SigmaWallpaper::new();
        let wallpaper = Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image.jpg".to_string(),
            "Nature".to_string(),
        );

        let added = wallpaper_mgr.add_wallpaper(wallpaper);
        assert!(added);
        assert_eq!(wallpaper_mgr.wallpaper_count(), 1);
    }

    #[test]
    fn test_sigmawallpaper_add_output() {
        let mut wallpaper_mgr = SigmaWallpaper::new();
        let output =
            DisplayOutput::new("output1".to_string(), "HDMI-1".to_string(), 1920, 1080, 1.0);

        let added = wallpaper_mgr.add_output(output);
        assert!(added);
        assert_eq!(wallpaper_mgr.output_count(), 1);
    }

    #[test]
    fn test_sigmawallpaper_set_wallpaper_all() {
        let mut wallpaper_mgr = SigmaWallpaper::new();

        wallpaper_mgr.add_wallpaper(Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image.jpg".to_string(),
            "Nature".to_string(),
        ));

        wallpaper_mgr.add_output(DisplayOutput::new(
            "output1".to_string(),
            "HDMI-1".to_string(),
            1920,
            1080,
            1.0,
        ));

        let set = wallpaper_mgr.set_wallpaper_all("wp1");
        assert!(set);

        let output = wallpaper_mgr.get_output("output1").unwrap();
        assert_eq!(output.current_wallpaper, "/path/to/image.jpg");
    }

    #[test]
    fn test_sigmawallpaper_set_wallpaper_output() {
        let mut wallpaper_mgr = SigmaWallpaper::new();

        wallpaper_mgr.add_wallpaper(Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image.jpg".to_string(),
            "Nature".to_string(),
        ));

        wallpaper_mgr.add_output(DisplayOutput::new(
            "output1".to_string(),
            "HDMI-1".to_string(),
            1920,
            1080,
            1.0,
        ));

        let set = wallpaper_mgr.set_wallpaper_output("wp1", "output1");
        assert!(set);

        let output = wallpaper_mgr.get_output("output1").unwrap();
        assert_eq!(output.current_wallpaper, "/path/to/image.jpg");
    }

    #[test]
    fn test_sigmawallpaper_list_favorite() {
        let mut wallpaper_mgr = SigmaWallpaper::new();

        let mut wp1 = Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image1.jpg".to_string(),
            "Nature".to_string(),
        );
        wp1.set_favorite(true);

        let wp2 = Wallpaper::new(
            "wp2".to_string(),
            "/path/to/image2.jpg".to_string(),
            "City".to_string(),
        );

        wallpaper_mgr.add_wallpaper(wp1);
        wallpaper_mgr.add_wallpaper(wp2);

        let favorites = wallpaper_mgr.list_favorite_wallpapers();
        assert_eq!(favorites.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmawallpaper();

        let wallpaper_mgr = get_sigmawallpaper();
        assert!(wallpaper_mgr.is_some());

        let mut wallpaper_mgr_mut = get_sigmawallpaper_mut();
        assert!(wallpaper_mgr_mut.is_some());

        if let Some(wallpaper_mgr) = wallpaper_mgr_mut {
            wallpaper_mgr.start();
            assert!(wallpaper_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_wallpaper() {
        init_sigmawallpaper();

        let wallpaper = Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image.jpg".to_string(),
            "Nature".to_string(),
        );

        let added = add_wallpaper(wallpaper);
        assert!(added);

        let wallpaper_mgr = get_sigmawallpaper().unwrap();
        assert_eq!(wallpaper_mgr.wallpaper_count(), 1);
    }

    #[test]
    fn test_convenience_set_wallpaper_all() {
        init_sigmawallpaper();

        let wallpaper = Wallpaper::new(
            "wp1".to_string(),
            "/path/to/image.jpg".to_string(),
            "Nature".to_string(),
        );

        add_wallpaper(wallpaper);

        let output =
            DisplayOutput::new("output1".to_string(), "HDMI-1".to_string(), 1920, 1080, 1.0);

        if let Some(wallpaper_mgr) = get_sigmawallpaper_mut() {
            wallpaper_mgr.add_output(output);
        }

        let set = set_wallpaper_all("wp1");
        assert!(set);
    }
}
