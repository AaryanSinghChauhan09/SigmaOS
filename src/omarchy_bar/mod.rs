// SPDX-License-Identifier: MIT
// SigmaOS Omarchy Bar Widgets
// Omarchy Quickshell-inspired bar widgets for status and control

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

/// Bar widget type
#[derive(Debug, Clone, PartialEq)]
pub enum BarWidgetType {
    Clock,
    Network,
    Audio,
    Bluetooth,
    Power,
    Weather,
    Media,
    SystemUpdate,
    Indicators,
    Workspaces,
    Custom(String),
}

/// Bar widget position
#[derive(Debug, Clone, PartialEq)]
pub enum BarWidgetPosition {
    Left,
    Center,
    Right,
}

/// Bar widget
#[derive(Debug, Clone)]
pub struct BarWidget {
    pub id: String,
    pub name: String,
    pub widget_type: BarWidgetType,
    pub position: BarWidgetPosition,
    pub enabled: bool,
    pub config: BTreeMap<String, String>,
    pub visible: bool,
}

impl BarWidget {
    pub fn new(id: String, name: String, widget_type: BarWidgetType) -> Self {
        Self {
            id,
            name,
            widget_type,
            position: BarWidgetPosition::Center,
            enabled: true,
            config: BTreeMap::new(),
            visible: true,
        }
    }

    /// Set position
    pub fn set_position(&mut self, position: BarWidgetPosition) {
        self.position = position;
    }

    /// Enable widget
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable widget
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set config
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }
}

/// Bar widget manager
#[derive(Debug, Clone)]
pub struct BarWidgetManager {
    pub widgets: BTreeMap<String, BarWidget>,
    pub left_widgets: Vec<String>,
    pub center_widgets: Vec<String>,
    pub right_widgets: Vec<String>,
}

impl BarWidgetManager {
    pub fn new() -> Self {
        Self {
            widgets: BTreeMap::new(),
            left_widgets: Vec::new(),
            center_widgets: Vec::new(),
            right_widgets: Vec::new(),
        }
    }

    /// Add widget
    pub fn add_widget(&mut self, widget: BarWidget) {
        let id = widget.id.clone();
        let position = widget.position.clone();

        match position {
            BarWidgetPosition::Left => self.left_widgets.push(id.clone()),
            BarWidgetPosition::Center => self.center_widgets.push(id.clone()),
            BarWidgetPosition::Right => self.right_widgets.push(id.clone()),
        }

        self.widgets.insert(id, widget);
    }

    /// Remove widget
    pub fn remove_widget(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(widget) = self.widgets.remove(&id) {
            match widget.position {
                BarWidgetPosition::Left => self.left_widgets.retain(|w| w != &id),
                BarWidgetPosition::Center => self.center_widgets.retain(|w| w != &id),
                BarWidgetPosition::Right => self.right_widgets.retain(|w| w != &id),
            }
            Ok(())
        } else {
            Err("Widget not found")
        }
    }

    /// Get widget by ID
    pub fn get_widget(&self, id: &str) -> Option<&BarWidget> {
        self.widgets.get(id)
    }

    /// Get widgets by position
    pub fn get_by_position(&self, position: &BarWidgetPosition) -> Vec<&BarWidget> {
        let ids = match position {
            BarWidgetPosition::Left => &self.left_widgets,
            BarWidgetPosition::Center => &self.center_widgets,
            BarWidgetPosition::Right => &self.right_widgets,
        };

        ids.iter()
            .filter_map(|id| self.widgets.get(id))
            .collect()
    }

    /// Enable widget
    pub fn enable_widget(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(widget) = self.widgets.get_mut(&id) {
            widget.enable();
            Ok(())
        } else {
            Err("Widget not found")
        }
    }

    /// Disable widget
    pub fn disable_widget(&mut self, id: String) -> Result<(), &'static str> {
        if let Some(widget) = self.widgets.get_mut(&id) {
            widget.disable();
            Ok(())
        } else {
            Err("Widget not found")
        }
    }

    /// List all widgets
    pub fn list_widgets(&self) -> Vec<String> {
        self.widgets.keys().cloned().collect()
    }

    /// Get visible widgets
    pub fn get_visible(&self) -> Vec<&BarWidget> {
        self.widgets
            .values()
            .filter(|w| w.visible && w.enabled)
            .collect()
    }
}

impl Default for BarWidgetManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_widget() {
        let widget = BarWidget::new(
            String::from("clock"),
            String::from("Clock"),
            BarWidgetType::Clock
        );
        
        assert_eq!(widget.widget_type, BarWidgetType::Clock);
        assert!(widget.enabled);
    }

    #[test]
    fn test_bar_widget_manager() {
        let mut manager = BarWidgetManager::new();
        let widget = BarWidget::new(
            String::from("network"),
            String::from("Network"),
            BarWidgetType::Network
        );
        
        manager.add_widget(widget);
        assert_eq!(manager.widgets.len(), 1);
    }

    #[test]
    fn test_widget_positioning() {
        let mut manager = BarWidgetManager::new();
        let mut widget = BarWidget::new(
            String::from("clock"),
            String::from("Clock"),
            BarWidgetType::Clock
        );
        widget.set_position(BarWidgetPosition::Left);
        
        manager.add_widget(widget);
        assert_eq!(manager.left_widgets.len(), 1);
    }

    #[test]
    fn test_enable_disable() {
        let mut manager = BarWidgetManager::new();
        let widget = BarWidget::new(
            String::from("audio"),
            String::from("Audio"),
            BarWidgetType::Audio
        );
        
        manager.add_widget(widget);
        assert!(manager.enable_widget(String::from("audio")).is_ok());
        assert!(manager.disable_widget(String::from("audio")).is_ok());
    }

    #[test]
    fn test_visible_widgets() {
        let mut manager = BarWidgetManager::new();
        let widget = BarWidget::new(
            String::from("power"),
            String::from("Power"),
            BarWidgetType::Power
        );
        
        manager.add_widget(widget);
        let visible = manager.get_visible();
        assert_eq!(visible.len(), 1);
    }
}