// SPDX-License-Identifier: MIT
// SigmaOS - Waybar Status Bar
// Inspired by Waybar (https://github.com/Alexays/Waybar)
// Provides status bar for Wayland with module system

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Waybar module type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaybarModuleType {
    Clock,
    Battery,
    CPU,
    Memory,
    Network,
    Audio,
    Workspace,
    Tray,
    Custom,
}

/// Waybar module
#[derive(Debug)]
pub struct WaybarModule {
    pub id: String,
    pub module_type: WaybarModuleType,
    pub label: String,
    pub icon: String,
    pub enabled: AtomicBool,
    pub position: u32, // Position in bar
}

impl WaybarModule {
    pub fn new(id: String, module_type: WaybarModuleType, label: String, icon: String) -> Self {
        Self {
            id,
            module_type,
            label,
            icon,
            enabled: AtomicBool::new(true),
            position: 0,
        }
    }

    /// Enable module
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    /// Disable module
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    /// Is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Set position
    pub fn set_position(&mut self, position: u32) {
        self.position = position;
    }
}

/// Waybar configuration
#[derive(Debug, Clone, Copy)]
pub struct WaybarConfig {
    pub position: BarPosition,
    pub height: u32,
    pub width: u32,
    pub background_color: u32, // ARGB
    pub text_color: u32,       // ARGB
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for WaybarConfig {
    fn default() -> Self {
        Self {
            position: BarPosition::Top,
            height: 30,
            width: 1920,
            background_color: 0xFF000000, // Black
            text_color: 0xFFFFFFFF,       // White
        }
    }
}

/// Waybar instance
pub struct WaybarInstance {
    pub config: WaybarConfig,
    pub modules: BTreeMap<String, WaybarModule>,
    pub active: AtomicBool,
}

impl WaybarInstance {
    pub fn new(config: WaybarConfig) -> Self {
        Self {
            config,
            modules: BTreeMap::new(),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize waybar
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.active.store(true, Ordering::Relaxed);

        // Add default modules
        self.add_default_modules();

        Ok(())
    }

    /// Add default modules
    fn add_default_modules(&mut self) {
        self.add_module(WaybarModule::new(
            "clock".to_string(),
            WaybarModuleType::Clock,
            "Clock".to_string(),
            "".to_string(),
        ));

        self.add_module(WaybarModule::new(
            "battery".to_string(),
            WaybarModuleType::Battery,
            "Battery".to_string(),
            "".to_string(),
        ));

        self.add_module(WaybarModule::new(
            "cpu".to_string(),
            WaybarModuleType::CPU,
            "CPU".to_string(),
            "".to_string(),
        ));

        self.add_module(WaybarModule::new(
            "memory".to_string(),
            WaybarModuleType::Memory,
            "Memory".to_string(),
            "".to_string(),
        ));

        self.add_module(WaybarModule::new(
            "network".to_string(),
            WaybarModuleType::Network,
            "Network".to_string(),
            "".to_string(),
        ));
    }

    /// Add module
    pub fn add_module(&mut self, module: WaybarModule) {
        self.modules.insert(module.id.clone(), module);
    }

    /// Get module
    pub fn get_module(&self, id: &str) -> Option<&WaybarModule> {
        self.modules.get(id)
    }

    /// Get mutable module
    pub fn get_module_mut(&mut self, id: &str) -> Option<&mut WaybarModule> {
        self.modules.get_mut(id)
    }

    /// Remove module
    pub fn remove_module(&mut self, id: &str) -> Result<(), &'static str> {
        self.modules.remove(id).ok_or("Module not found")?;
        Ok(())
    }

    /// Update module label
    pub fn update_module_label(&mut self, id: &str, label: String) -> Result<(), &'static str> {
        let module = self.modules.get_mut(id).ok_or("Module not found")?;
        module.label = label;
        Ok(())
    }

    /// Get visible modules
    pub fn get_visible_modules(&self) -> Vec<&WaybarModule> {
        let mut modules: Vec<_> = self.modules.values().collect();
        modules.sort_by(|a, b| a.position.cmp(&b.position));
        modules.retain(|m| m.is_enabled());
        modules
    }

    /// Set bar position
    pub fn set_position(&mut self, position: BarPosition) {
        self.config.position = position;
    }

    /// Set bar height
    pub fn set_height(&mut self, height: u32) {
        self.config.height = height;
    }

    /// Render bar (in real implementation, would render to Wayland surface)
    pub fn render(&self) -> String {
        let mut output = String::new();

        for module in self.get_visible_modules() {
            output.push_str(&module.label);
            output.push_str("  ");
        }

        output
    }
}

impl Default for WaybarInstance {
    fn default() -> Self {
        Self::new(WaybarConfig::default())
    }
}

/// Global waybar instance
static mut WAYBAR_INSTANCE: Option<WaybarInstance> = None;

/// Initialize waybar
pub fn init_waybar() {
    unsafe {
        WAYBAR_INSTANCE = Some(WaybarInstance::new(WaybarConfig::default()));
    }
}

/// Get global waybar instance
pub fn get_waybar() -> Option<&'static mut WaybarInstance> {
    unsafe { WAYBAR_INSTANCE.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waybar_init() {
        let mut waybar = WaybarInstance::new(WaybarConfig::default());
        waybar.init().unwrap();
        assert!(waybar.active.load(Ordering::Relaxed));
        assert!(waybar.get_module("clock").is_some());
    }

    #[test]
    fn test_add_module() {
        let mut waybar = WaybarInstance::new(WaybarConfig::default());
        let module = WaybarModule::new(
            "test".to_string(),
            WaybarModuleType::Custom,
            "Test".to_string(),
            "".to_string(),
        );
        waybar.add_module(module);
        assert!(waybar.get_module("test").is_some());
    }

    #[test]
    fn test_remove_module() {
        let mut waybar = WaybarInstance::new(WaybarConfig::default());
        waybar.init().unwrap();
        waybar.remove_module("clock").unwrap();
        assert!(waybar.get_module("clock").is_none());
    }

    #[test]
    fn test_update_module_label() {
        let mut waybar = WaybarInstance::new(WaybarConfig::default());
        waybar.init().unwrap();
        waybar
            .update_module_label("clock", "12:00".to_string())
            .unwrap();
        assert_eq!(waybar.get_module("clock").unwrap().label, "12:00");
    }

    #[test]
    fn test_get_visible_modules() {
        let mut waybar = WaybarInstance::new(WaybarConfig::default());
        waybar.init().unwrap();
        let visible = waybar.get_visible_modules();
        assert!(visible.len() > 0);
    }

    #[test]
    fn test_waybar_config() {
        let config = WaybarConfig::default();
        assert_eq!(config.position, BarPosition::Top);
        assert_eq!(config.height, 30);
    }
}
