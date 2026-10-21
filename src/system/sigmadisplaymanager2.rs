// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Display state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayState {
    On,
    Off,
    Suspended,
}

/// Display mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    Normal,
    Mirror,
    Extend,
}

/// Display output
#[derive(Debug, Clone)]
pub struct DisplayOutput {
    pub id: String,
    pub name: String,
    pub resolution: String,
    pub refresh_rate: u32,
    pub state: DisplayState,
    pub is_primary: bool,
}

/// Display manager for managing display outputs
/// Inspired by Linux display management and Omarchy display commands
#[derive(Debug, Clone)]
pub struct SigmaDisplayManager {
    outputs: Vec<DisplayOutput>,
    brightness: u8,
}

impl SigmaDisplayManager {
    pub fn new() -> Self {
        Self {
            outputs: Vec::new(),
            brightness: 100,
        }
    }

    /// Add a display output
    pub fn add_output(&mut self, output: DisplayOutput) {
        self.outputs.push(output);
    }

    /// Get output by ID
    pub fn get_output(&self, id: &str) -> Option<&DisplayOutput> {
        self.outputs.iter().find(|o| o.id == id)
    }

    /// Get all outputs
    pub fn get_outputs(&self) -> &[DisplayOutput] {
        &self.outputs
    }

    /// Turn on display
    pub fn turn_on(&mut self, id: &str) -> Result<String, String> {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.id == id) {
            output.state = DisplayState::On;
            Ok(format!("Turned on display: {}", output.name))
        } else {
            Err(format!("Output not found: {}", id))
        }
    }

    /// Turn off display
    pub fn turn_off(&mut self, id: &str) -> Result<String, String> {
        if let Some(output) = self.outputs.iter_mut().find(|o| o.id == id) {
            output.state = DisplayState::Off;
            Ok(format!("Turned off display: {}", output.name))
        } else {
            Err(format!("Output not found: {}", id))
        }
    }

    /// Set brightness
    pub fn set_brightness(&mut self, level: u8) -> Result<String, String> {
        if level <= 100 {
            self.brightness = level;
            Ok(format!("Brightness set to: {}%", level))
        } else {
            Err("Invalid brightness level".to_string())
        }
    }

    /// Get brightness
    pub fn get_brightness(&self) -> u8 {
        self.brightness
    }

    /// Set primary display
    pub fn set_primary(&mut self, id: &str) -> Result<String, String> {
        for output in &mut self.outputs {
            output.is_primary = false;
        }
        if let Some(output) = self.outputs.iter_mut().find(|o| o.id == id) {
            output.is_primary = true;
            Ok(format!("Set primary display: {}", output.name))
        } else {
            Err(format!("Output not found: {}", id))
        }
    }

    /// Get primary display
    pub fn get_primary(&self) -> Option<&DisplayOutput> {
        self.outputs.iter().find(|o| o.is_primary)
    }

    /// Get active displays
    pub fn get_active_displays(&self) -> Vec<&DisplayOutput> {
        self.outputs
            .iter()
            .filter(|o| o.state == DisplayState::On)
            .collect()
    }
}

impl Default for SigmaDisplayManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_output() {
        let mut manager = SigmaDisplayManager::new();
        let output = DisplayOutput {
            id: "eDP-1".to_string(),
            name: "Internal Display".to_string(),
            resolution: "1920x1080".to_string(),
            refresh_rate: 60,
            state: DisplayState::On,
            is_primary: true,
        };
        manager.add_output(output);
        assert_eq!(manager.get_outputs().len(), 1);
    }

    #[test]
    fn test_turn_off() {
        let mut manager = SigmaDisplayManager::new();
        let output = DisplayOutput {
            id: "eDP-1".to_string(),
            name: "Internal Display".to_string(),
            resolution: "1920x1080".to_string(),
            refresh_rate: 60,
            state: DisplayState::On,
            is_primary: true,
        };
        manager.add_output(output);
        manager.turn_off("eDP-1").unwrap();
        assert_eq!(
            manager.get_output("eDP-1").unwrap().state,
            DisplayState::Off
        );
    }

    #[test]
    fn test_set_brightness() {
        let mut manager = SigmaDisplayManager::new();
        manager.set_brightness(50).unwrap();
        assert_eq!(manager.get_brightness(), 50);
    }

    #[test]
    fn test_set_primary() {
        let mut manager = SigmaDisplayManager::new();
        let output = DisplayOutput {
            id: "eDP-1".to_string(),
            name: "Internal Display".to_string(),
            resolution: "1920x1080".to_string(),
            refresh_rate: 60,
            state: DisplayState::On,
            is_primary: false,
        };
        manager.add_output(output);
        manager.set_primary("eDP-1").unwrap();
        assert!(manager.get_output("eDP-1").unwrap().is_primary);
    }

    #[test]
    fn test_get_active_displays() {
        let mut manager = SigmaDisplayManager::new();
        let output1 = DisplayOutput {
            id: "eDP-1".to_string(),
            name: "Internal Display".to_string(),
            resolution: "1920x1080".to_string(),
            refresh_rate: 60,
            state: DisplayState::On,
            is_primary: true,
        };
        let output2 = DisplayOutput {
            id: "HDMI-1".to_string(),
            name: "External Display".to_string(),
            resolution: "2560x1440".to_string(),
            refresh_rate: 144,
            state: DisplayState::Off,
            is_primary: false,
        };
        manager.add_output(output1);
        manager.add_output(output2);
        assert_eq!(manager.get_active_displays().len(), 1);
    }
}
