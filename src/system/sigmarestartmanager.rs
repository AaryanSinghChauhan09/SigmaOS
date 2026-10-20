// SPDX-License-Identifier: MIT
// SigmaOS - Restart Manager
// Component restart management inspired by Omarchy restart-* commands and Linux service restart

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Component state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    Running,
    Stopped,
    Restarting,
    Failed,
}

/// Component
#[derive(Debug, Clone)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub state: ComponentState,
    pub pid: Option<u32>,
}

/// Restart manager
pub struct SigmaRestartManager {
    components: Vec<Component>,
}

impl SigmaRestartManager {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn get_components(&self) -> &[Component] {
        &self.components
    }

    pub fn get_component(&self, id: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.id == id)
    }

    pub fn restart_component(&mut self, id: String) -> Result<(), String> {
        if let Some(component) = self.components.iter_mut().find(|c| c.id == id) {
            component.state = ComponentState::Restarting;
            component.state = ComponentState::Running;
            Ok(())
        } else {
            Err(format!("Component {} not found", id))
        }
    }

    pub fn stop_component(&mut self, id: String) -> Result<(), String> {
        if let Some(component) = self.components.iter_mut().find(|c| c.id == id) {
            component.state = ComponentState::Stopped;
            component.pid = None;
            Ok(())
        } else {
            Err(format!("Component {} not found", id))
        }
    }

    pub fn start_component(&mut self, id: String) -> Result<(), String> {
        if let Some(component) = self.components.iter_mut().find(|c| c.id == id) {
            component.state = ComponentState::Running;
            component.pid = Some(1000);
            Ok(())
        } else {
            Err(format!("Component {} not found", id))
        }
    }

    pub fn get_running_components(&self) -> Vec<&Component> {
        self.components.iter().filter(|c| c.state == ComponentState::Running).collect()
    }
}

impl Default for SigmaRestartManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restart_manager_creation() {
        let manager = SigmaRestartManager::new();
        assert_eq!(manager.get_components().len(), 0);
    }

    #[test]
    fn test_add_component() {
        let mut manager = SigmaRestartManager::new();
        let component = Component {
            id: "waybar".to_string(),
            name: "Waybar".to_string(),
            state: ComponentState::Running,
            pid: Some(1000),
        };
        manager.add_component(component);
        assert_eq!(manager.get_components().len(), 1);
    }

    #[test]
    fn test_restart_component() {
        let mut manager = SigmaRestartManager::new();
        let component = Component {
            id: "waybar".to_string(),
            name: "Waybar".to_string(),
            state: ComponentState::Running,
            pid: Some(1000),
        };
        manager.add_component(component);
        manager.restart_component("waybar".to_string()).unwrap();
        assert_eq!(manager.get_components()[0].state, ComponentState::Running);
    }

    #[test]
    fn test_stop_component() {
        let mut manager = SigmaRestartManager::new();
        let component = Component {
            id: "waybar".to_string(),
            name: "Waybar".to_string(),
            state: ComponentState::Running,
            pid: Some(1000),
        };
        manager.add_component(component);
        manager.stop_component("waybar".to_string()).unwrap();
        assert_eq!(manager.get_components()[0].state, ComponentState::Stopped);
    }

    #[test]
    fn test_start_component() {
        let mut manager = SigmaRestartManager::new();
        let component = Component {
            id: "waybar".to_string(),
            name: "Waybar".to_string(),
            state: ComponentState::Stopped,
            pid: None,
        };
        manager.add_component(component);
        manager.start_component("waybar".to_string()).unwrap();
        assert_eq!(manager.get_components()[0].state, ComponentState::Running);
    }

    #[test]
    fn test_running_components() {
        let mut manager = SigmaRestartManager::new();
        let component1 = Component {
            id: "waybar".to_string(),
            name: "Waybar".to_string(),
            state: ComponentState::Running,
            pid: Some(1000),
        };
        let component2 = Component {
            id: "terminal".to_string(),
            name: "Terminal".to_string(),
            state: ComponentState::Stopped,
            pid: None,
        };
        manager.add_component(component1);
        manager.add_component(component2);
        assert_eq!(manager.get_running_components().len(), 1);
    }
}
