//! SigmaPower - Power Management Settings System
//!
//! Provides comprehensive power management settings inspired by Linux power management tools.
//! Supports power profiles, screen brightness, and power saving modes.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Power profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
    Unknown,
;

/// Screen brightness
#[derive(Debug, Clone)]
pub struct Brightness {
    pub level: u8,
    pub min: u8,
    pub max: u8,
;

impl Brightness {
    pub fn new(level: u8) -> Self {
        Self {
            level,
            min: 0,
            max: 100,
        }
    }

    pub fn with_range(mut self, min: u8, max: u8) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level.clamp(self.min, self.max);
    }

    pub fn percentage(&self) -> u8 {
        if self.max == self.min {
            0
        } else {
            ((self.level - self.min) * 100) / (self.max - self.min)
        }
    }
;

/// Power settings
#[derive(Debug, Clone)]
pub struct PowerSettings {
    pub profile: PowerProfile,
    pub screen_brightness: Brightness,
    pub idle_timeout: Option<u32>,
    pub sleep_timeout: Option<u32>,
    pub hibernate_timeout: Option<u32>,
    pub screen_off_timeout: Option<u32>,
;

impl PowerSettings {
    pub fn new() -> Self {
        Self {
            profile: PowerProfile::Balanced,
            screen_brightness: Brightness::new(80),
            idle_timeout: Some(300),
            sleep_timeout: Some(900),
            hibernate_timeout: None,
            screen_off_timeout: Some(120),
        }
    }

    pub fn with_profile(mut self, profile: PowerProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn with_brightness(mut self, level: u8) -> Self {
        self.screen_brightness = Brightness::new(level);
        self
    }

    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.profile = profile;
    }

    pub fn set_brightness(&mut self, level: u8) {
        self.screen_brightness.set_level(level);
    }

    pub fn set_idle_timeout(&mut self, timeout: u32) {
        self.idle_timeout = Some(timeout);
    }

    pub fn set_sleep_timeout(&mut self, timeout: u32) {
        self.sleep_timeout = Some(timeout);
    }

    pub fn set_hibernate_timeout(&mut self, timeout: u32) {
        self.hibernate_timeout = Some(timeout);
    }

    pub fn set_screen_off_timeout(&mut self, timeout: u32) {
        self.screen_off_timeout = Some(timeout);
    }
;

impl Default for PowerSettings {
    fn default() -> Self {
        Self::new()
    }
;

/// Power manager
pub struct SigmaPower {
    settings: PowerSettings,
;

impl SigmaPower {
    pub fn new() -> Self {
        Self {
            settings: PowerSettings::new(),
        }
    }

    pub fn get_settings(&self) -> &PowerSettings {
        &self.settings
    }

    pub fn get_settings_mut(&mut self) -> &mut PowerSettings {
        &mut self.settings
    }

    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.settings.set_profile(profile);
    }

    pub fn set_brightness(&mut self, level: u8) {
        self.settings.set_brightness(level);
    }

    pub fn set_idle_timeout(&mut self, timeout: u32) {
        self.settings.set_idle_timeout(timeout);
    }

    pub fn set_sleep_timeout(&mut self, timeout: u32) {
        self.settings.set_sleep_timeout(timeout);
    }

    pub fn set_hibernate_timeout(&mut self, timeout: u32) {
        self.settings.set_hibernate_timeout(timeout);
    }

    pub fn set_screen_off_timeout(&mut self, timeout: u32) {
        self.settings.set_screen_off_timeout(timeout);
    }

    pub fn get_profile(&self) -> PowerProfile {
        self.settings.profile
    }

    pub fn get_brightness(&self) -> u8 {
        self.settings.screen_brightness.level
    }

    pub fn get_brightness_percentage(&self) -> u8 {
        self.settings.screen_brightness.percentage()
    }
;

impl Default for SigmaPower {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAPOWER_INSTANCE: Option<SigmaPower> = None;

pub fn init_sigmapower() {
    unsafe {
        SIGMAPOWER_INSTANCE = Some(SigmaPower::new());
    }
;

pub fn get_sigmapower() -> &'static SigmaPower {
    unsafe {
        SIGMAPOWER_INSTANCE
            .as_ref()
            .expect("SigmaPower not initialized")
    }
;

pub fn get_sigmapower_mut() -> &'static mut SigmaPower {
    unsafe {
        SIGMAPOWER_INSTANCE
            .as_mut()
            .expect("SigmaPower not initialized")
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brightness_creation() {
        let brightness = Brightness::new(80);
        assert_eq!(brightness.level, 80);
        assert_eq!(brightness.min, 0);
        assert_eq!(brightness.max, 100);
    }

    #[test]
    fn test_brightness_range() {
        let brightness = Brightness::new(50).with_range(10, 90);
        assert_eq!(brightness.min, 10);
        assert_eq!(brightness.max, 90);
    }

    #[test]
    fn test_brightness_clamping() {
        let mut brightness = Brightness::new(50).with_range(10, 90);
        brightness.set_level(5);
        assert_eq!(brightness.level, 10);

        brightness.set_level(100);
        assert_eq!(brightness.level, 90);
    }

    #[test]
    fn test_brightness_percentage() {
        let brightness = Brightness::new(50);
        assert_eq!(brightness.percentage(), 50);
    }

    #[test]
    fn test_power_settings_creation() {
        let settings = PowerSettings::new();
        assert_eq!(settings.profile, PowerProfile::Balanced);
        assert_eq!(settings.screen_brightness.level, 80);
    }

    #[test]
    fn test_power_settings_builder() {
        let settings = PowerSettings::new()
            .with_profile(PowerProfile::Performance)
            .with_brightness(100);

        assert_eq!(settings.profile, PowerProfile::Performance);
        assert_eq!(settings.screen_brightness.level, 100);
    }

    #[test]
    fn test_power_manager() {
        let manager = SigmaPower::new();
        assert_eq!(manager.get_profile(), PowerProfile::Balanced);
    }

    #[test]
    fn test_set_profile() {
        let mut manager = SigmaPower::new();
        manager.set_profile(PowerProfile::Performance);
        assert_eq!(manager.get_profile(), PowerProfile::Performance);
    }

    #[test]
    fn test_set_brightness() {
        let mut manager = SigmaPower::new();
        manager.set_brightness(90);
        assert_eq!(manager.get_brightness(), 90);
    }

    #[test]
    fn test_set_timeouts() {
        let mut manager = SigmaPower::new();
        manager.set_idle_timeout(600);
        manager.set_sleep_timeout(1800);
        manager.set_hibernate_timeout(3600);
        manager.set_screen_off_timeout(300);

        assert_eq!(manager.get_settings().idle_timeout, Some(600));
    }

    #[test]
    fn test_global_instance() {
        init_sigmapower();
        let manager = get_sigmapower();
        assert_eq!(manager.get_profile(), PowerProfile::Balanced);

        let manager_mut = get_sigmapower_mut();
        manager_mut.set_profile(PowerProfile::Performance);
        assert_eq!(manager.get_profile(), PowerProfile::Performance);
    }
;
