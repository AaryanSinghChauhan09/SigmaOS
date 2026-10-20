// SPDX-License-Identifier: MIT
// SigmaOS - Toggle Manager
// Toggle management inspired by Omarchy toggle-* commands and Linux feature toggles

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Toggle state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    On,
    Off,
}

/// Toggle
#[derive(Debug, Clone)]
pub struct Toggle {
    pub id: String,
    pub name: String,
    pub state: ToggleState,
    pub description: String,
}

/// Toggle manager
pub struct SigmaToggleManager {
    toggles: Vec<Toggle>,
}

impl SigmaToggleManager {
    pub fn new() -> Self {
        Self {
            toggles: Vec::new(),
        }
    }

    pub fn add_toggle(&mut self, toggle: Toggle) {
        self.toggles.push(toggle);
    }

    pub fn get_toggles(&self) -> &[Toggle] {
        &self.toggles
    }

    pub fn get_toggle(&self, id: &str) -> Option<&Toggle> {
        self.toggles.iter().find(|t| t.id == id)
    }

    pub fn set_toggle(&mut self, id: String, state: ToggleState) -> Result<(), String> {
        if let Some(toggle) = self.toggles.iter_mut().find(|t| t.id == id) {
            toggle.state = state;
            Ok(())
        } else {
            Err(format!("Toggle {} not found", id))
        }
    }

    pub fn toggle(&mut self, id: String) -> Result<(), String> {
        if let Some(toggle) = self.toggles.iter_mut().find(|t| t.id == id) {
            toggle.state = match toggle.state {
                ToggleState::On => ToggleState::Off,
                ToggleState::Off => ToggleState::On,
            };
            Ok(())
        } else {
            Err(format!("Toggle {} not found", id))
        }
    }

    pub fn get_on_toggles(&self) -> Vec<&Toggle> {
        self.toggles
            .iter()
            .filter(|t| t.state == ToggleState::On)
            .collect()
    }

    pub fn get_off_toggles(&self) -> Vec<&Toggle> {
        self.toggles
            .iter()
            .filter(|t| t.state == ToggleState::Off)
            .collect()
    }
}

impl Default for SigmaToggleManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_manager_creation() {
        let manager = SigmaToggleManager::new();
        assert_eq!(manager.get_toggles().len(), 0);
    }

    #[test]
    fn test_add_toggle() {
        let mut manager = SigmaToggleManager::new();
        let toggle = Toggle {
            id: "wifi".to_string(),
            name: "WiFi".to_string(),
            state: ToggleState::Off,
            description: "WiFi toggle".to_string(),
        };
        manager.add_toggle(toggle);
        assert_eq!(manager.get_toggles().len(), 1);
    }

    #[test]
    fn test_set_toggle() {
        let mut manager = SigmaToggleManager::new();
        let toggle = Toggle {
            id: "wifi".to_string(),
            name: "WiFi".to_string(),
            state: ToggleState::Off,
            description: "WiFi toggle".to_string(),
        };
        manager.add_toggle(toggle);
        manager
            .set_toggle("wifi".to_string(), ToggleState::On)
            .unwrap();
        assert_eq!(manager.get_toggles()[0].state, ToggleState::On);
    }

    #[test]
    fn test_toggle() {
        let mut manager = SigmaToggleManager::new();
        let toggle = Toggle {
            id: "wifi".to_string(),
            name: "WiFi".to_string(),
            state: ToggleState::Off,
            description: "WiFi toggle".to_string(),
        };
        manager.add_toggle(toggle);
        manager.toggle("wifi".to_string()).unwrap();
        assert_eq!(manager.get_toggles()[0].state, ToggleState::On);
    }

    #[test]
    fn test_on_toggles() {
        let mut manager = SigmaToggleManager::new();
        let toggle1 = Toggle {
            id: "wifi".to_string(),
            name: "WiFi".to_string(),
            state: ToggleState::On,
            description: "WiFi toggle".to_string(),
        };
        let toggle2 = Toggle {
            id: "bluetooth".to_string(),
            name: "Bluetooth".to_string(),
            state: ToggleState::Off,
            description: "Bluetooth toggle".to_string(),
        };
        manager.add_toggle(toggle1);
        manager.add_toggle(toggle2);
        assert_eq!(manager.get_on_toggles().len(), 1);
    }
}
