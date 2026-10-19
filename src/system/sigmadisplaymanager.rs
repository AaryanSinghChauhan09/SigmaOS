//! SigmaDisplayManager - Display and Login Management System
//!
//! Provides comprehensive display and login management inspired by Linux Mint and Omarchy.
//! Supports display configuration, resolution management, and login screen settings.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Display resolution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32, // Hz
;

impl Resolution {
    pub fn new(width: u32, height: u32, refresh_rate: u32) -> Self {
        Self {
            width,
            height,
            refresh_rate,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}x{}@{}Hz", self.width, self.height, self.refresh_rate)
    }
;

/// Display output
#[derive(Debug, Clone)]
pub struct DisplayOutput {
    pub id: String,
    pub name: String,
    pub is_primary: bool,
    pub is_connected: bool,
    pub enabled: bool,
    pub current_resolution: Option<Resolution>,
    pub supported_resolutions: Vec<Resolution>,
    pub brightness: u8, // 0-100
;

impl DisplayOutput {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            is_primary: false,
            is_connected: false,
            enabled: true,
            current_resolution: None,
            supported_resolutions: Vec::new(),
            brightness: 100,
        }
    }

    pub fn set_primary(&mut self, primary: bool) {
        self.is_primary = primary;
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.current_resolution = Some(resolution);
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        self.brightness = brightness.min(100);
    }

    pub fn add_supported_resolution(&mut self, resolution: Resolution) {
        self.supported_resolutions.push(resolution);
    }
;

/// Login screen settings
#[derive(Debug, Clone)]
pub struct LoginSettings {
    pub id: String,
    pub auto_login: bool,
    pub auto_login_user: Option<String>,
    pub show_user_list: bool,
    pub allow_guest_login: bool,
    pub theme: String,
    pub background_image: String,
    pub logo_image: String,
;

impl LoginSettings {
    pub fn new(id: String) -> Self {
        Self {
            id,
            auto_login: false,
            auto_login_user: None,
            show_user_list: true,
            allow_guest_login: false,
            theme: "default".to_string(),
            background_image: String::new(),
            logo_image: String::new(),
        }
    }

    pub fn set_auto_login(&mut self, enabled: bool, username: Option<String>) {
        self.auto_login = enabled;
        self.auto_login_user = username;
    }

    pub fn set_show_user_list(&mut self, show: bool) {
        self.show_user_list = show;
    }

    pub fn set_allow_guest_login(&mut self, allow: bool) {
        self.allow_guest_login = allow;
    }

    pub fn set_theme(&mut self, theme: String) {
        self.theme = theme;
    }

    pub fn set_background(&mut self, image: String) {
        self.background_image = image;
    }

    pub fn set_logo(&mut self, image: String) {
        self.logo_image = image;
    }
;

/// SigmaDisplayManager - Display and login management
#[derive(Debug)]
pub struct SigmaDisplayManager {
    outputs: BTreeMap<String, DisplayOutput>,
    login_settings: LoginSettings,
;

impl SigmaDisplayManager {
    pub fn new() -> Self {
        Self {
            outputs: BTreeMap::new(),
            login_settings: LoginSettings::new("default".to_string()),
        }
    }

    pub fn add_output(&mut self, output: DisplayOutput) {
        self.outputs.insert(output.id.clone(), output);
    }

    pub fn remove_output(&mut self, id: &str) -> bool {
        self.outputs.remove(id).is_some()
    }

    pub fn get_output(&self, id: &str) -> Option<&DisplayOutput> {
        self.outputs.get(id)
    }

    pub fn get_output_mut(&mut self, id: &str) -> Option<&mut DisplayOutput> {
        self.outputs.get_mut(id)
    }

    pub fn set_primary_output(&mut self, id: &str) -> bool {
        if let Some(output) = self.outputs.get_mut(id) {
            output.set_primary(true);
            // Unset other primaries
            for (other_id, other_output) in &mut self.outputs {
                if other_id != id {
                    other_output.set_primary(false);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn get_primary_output(&self) -> Option<&DisplayOutput> {
        self.outputs.values().find(|output| output.is_primary)
    }

    pub fn list_connected(&self) -> Vec<&DisplayOutput> {
        self.outputs.values()
            .filter(|output| output.is_connected)
            .collect()
    }

    pub fn list_enabled(&self) -> Vec<&DisplayOutput> {
        self.outputs.values()
            .filter(|output| output.enabled)
            .collect()
    }

    pub fn get_login_settings(&self) -> &LoginSettings {
        &self.login_settings
    }

    pub fn get_login_settings_mut(&mut self) -> &mut LoginSettings {
        &mut self.login_settings
    }

    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }
;

impl Default for SigmaDisplayManager {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaDisplayManager instance
static mut SIGMADISPLAYMANAGER_INSTANCE: Option<SigmaDisplayManager> = None;

/// Initialize the global SigmaDisplayManager instance
pub fn init_sigmadisplaymanager() {
    unsafe {
        SIGMADISPLAYMANAGER_INSTANCE = Some(SigmaDisplayManager::new());
    }
;

/// Get the global SigmaDisplayManager instance
pub fn get_sigmadisplaymanager() -> Option<&'static SigmaDisplayManager> {
    unsafe {
        SIGMADISPLAYMANAGER_INSTANCE.as_ref()
    }
;

/// Get the global SigmaDisplayManager instance (mutable)
pub fn get_sigmadisplaymanager_mut() -> Option<&'static mut SigmaDisplayManager> {
    unsafe {
        SIGMADISPLAYMANAGER_INSTANCE.as_mut()
    }
;

/// Convenience function to add an output
pub fn add_output(output: DisplayOutput) -> bool {
    if let Some(manager) = get_sigmadisplaymanager_mut() {
        manager.add_output(output);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolution_creation() {
        let res = Resolution::new(1920, 1080, 60);
        assert_eq!(res.to_string(), "1920x1080@60Hz");
    }

    #[test]
    fn test_display_output_creation() {
        let output = DisplayOutput::new("HDMI-1".to_string(), "HDMI-1".to_string());
        assert_eq!(output.id, "HDMI-1");
        assert_eq!(output.brightness, 100);
    }

    #[test]
    fn test_display_output_modification() {
        let mut output = DisplayOutput::new("DP-1".to_string(), "DP-1".to_string());
        output.set_primary(true);
        output.set_connected(true);
        output.set_brightness(75);
        let res = Resolution::new(2560, 1440, 144);
        output.set_resolution(res);
        assert!(output.is_primary);
        assert_eq!(output.brightness, 75);
    }

    #[test]
    fn test_login_settings_creation() {
        let settings = LoginSettings::new("default".to_string());
        assert!(!settings.auto_login);
        assert!(settings.show_user_list);
    }

    #[test]
    fn test_login_settings_modification() {
        let mut settings = LoginSettings::new("default".to_string());
        settings.set_auto_login(true, Some("user1".to_string()));
        settings.set_allow_guest_login(true);
        settings.set_theme("dark".to_string());
        assert!(settings.auto_login);
        assert_eq!(settings.auto_login_user, Some("user1".to_string()));
    }

    #[test]
    fn test_sigmadisplaymanager_creation() {
        let manager = SigmaDisplayManager::new();
        assert_eq!(manager.output_count(), 0);
    }

    #[test]
    fn test_sigmadisplaymanager_management() {
        let mut manager = SigmaDisplayManager::new();
        let output = DisplayOutput::new("HDMI-1".to_string(), "HDMI-1".to_string());
        manager.add_output(output);
        assert_eq!(manager.output_count(), 1);
    }

    #[test]
    fn test_primary_output() {
        let mut manager = SigmaDisplayManager::new();
        let mut output1 = DisplayOutput::new("HDMI-1".to_string(), "HDMI-1".to_string());
        output1.set_primary(true);
        manager.add_output(output1);
        let output2 = DisplayOutput::new("DP-1".to_string(), "DP-1".to_string());
        manager.add_output(output2);
        manager.set_primary_output("DP-1");
        let primary = manager.get_primary_output();
        assert_eq!(primary.unwrap().id, "DP-1");
    }
;
}
