//! SigmaOS Bar Widgets System
//!
//! Inspired by: Omarchy's modular bar with interactive widgets (workspaces, clock, weather, media, battery, etc.)
//! Following principles: PLUG_IN_DRIVER_MODEL, COMPOSITION_OVER_INHERITANCE, MODULARITY,
//! COMPONENT_ISOLATION, SINGLE_RESPONSIBILITY, LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID,
//! DISCOVERABILITY, CLEAR_ABSTRACTION, ENCAPSULATION, CODE_REUSABILITY.

use std::collections::HashMap;

/// Bar widget kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidgetKind {
    Workspaces,
    ActiveWindow,
    Clock,
    Weather,
    Media,
    SystemTray,
    Battery,
    KeyboardLayout,
    Network,
    Audio,
    DND,
    NightLight,
    StayAwake,
    ScreenRecording,
    Dictation,
    Reminders,
    Custom(String),
}

/// Bar widget configuration
#[derive(Debug, Clone)]
pub struct WidgetConfig {
    pub kind: WidgetKind,
    pub enabled: bool,
    pub position: WidgetPosition,
    pub size: u32,
    pub transparency: f32,
    pub config: HashMap<String, String>,
}

/// Bar widget position
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidgetPosition {
    Left,
    Center,
    Right,
}

/// Bar widget
#[derive(Debug, Clone)]
pub struct BarWidget {
    pub id: String,
    pub kind: WidgetKind,
    pub config: WidgetConfig,
    pub visible: bool,
}

/// Bar configuration
#[derive(Debug, Clone)]
pub struct BarConfig {
    pub position: BarPosition,
    pub transparency: f32,
    pub height: u32,
    pub widgets: Vec<BarWidget>,
    pub draggable: bool,
    pub double_click_toggle: bool,
}

/// Bar position
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BarPosition {
    Top,
    Bottom,
    Left,
    Right,
}

/// Bar manager
pub struct BarManager {
    pub config: BarConfig,
    pub widgets: HashMap<String, BarWidget>,
}

impl BarManager {
    pub fn new() -> Self {
        let mut config = BarConfig {
            position: BarPosition::Top,
            transparency: 0.9,
            height: 32,
            widgets: Vec::new(),
            draggable: true,
            double_click_toggle: true,
        };

        // Default widgets like Omarchy
        let default_widgets = vec![
            ("workspaces", WidgetKind::Workspaces, WidgetPosition::Left),
            ("active-window", WidgetKind::ActiveWindow, WidgetPosition::Left),
            ("clock", WidgetKind::Clock, WidgetPosition::Center),
            ("weather", WidgetKind::Weather, WidgetPosition::Right),
            ("media", WidgetKind::Media, WidgetPosition::Right),
            ("system-tray", WidgetKind::SystemTray, WidgetPosition::Right),
            ("battery", WidgetKind::Battery, WidgetPosition::Right),
            ("audio", WidgetKind::Audio, WidgetPosition::Right),
        ];

        let mut widgets = Vec::new();
        for (id, kind, position) in default_widgets {
            widgets.push(BarWidget {
                id: id.to_string(),
                kind,
                config: WidgetConfig {
                    kind: kind.clone(),
                    enabled: true,
                    position,
                    size: 32,
                    transparency: 0.9,
                    config: HashMap::new(),
                },
                visible: true,
            });
        }
        config.widgets = widgets;

        let mut widget_map = HashMap::new();
        for w in &config.widgets {
            widget_map.insert(w.id.clone(), w.clone());
        }

        Self {
            config,
            widgets: widget_map,
        }
    }

    /// Add a widget
    pub fn add_widget(&mut self, id: String, kind: WidgetKind, position: WidgetPosition) {
        let widget = BarWidget {
            id: id.clone(),
            kind: kind.clone(),
            config: WidgetConfig {
                kind,
                enabled: true,
                position,
                size: 32,
                transparency: 0.9,
                config: HashMap::new(),
            },
            visible: true,
        };
        self.config.widgets.push(widget.clone());
        self.widgets.insert(id, widget);
    }

    /// Remove a widget
    pub fn remove_widget(&mut self, id: &str) -> Result<(), String> {
        self.config.widgets.retain(|w| w.id != id);
        self.widgets.remove(id).ok_or_else(|| format!("Widget {} not found", id))?;
        Ok(())
    }

    /// Toggle widget visibility
    pub fn toggle_widget(&mut self, id: &str) -> Result<(), String> {
        let widget = self.widgets.get_mut(id).ok_or_else(|| format!("Widget {} not found", id))?;
        widget.visible = !widget.visible;
        Ok(())
    }

    /// Set bar transparency
    pub fn set_transparency(&mut self, transparency: f32) -> Result<(), String> {
        if transparency < 0.0 || transparency > 1.0 {
            return Err("Transparency must be between 0.0 and 1.0".to_string());
        }
        self.config.transparency = transparency;
        Ok(())
    }

    /// Set bar position
    pub fn set_position(&mut self, position: BarPosition) {
        self.config.position = position;
    }

    /// List visible widgets
    pub fn list_visible(&self) -> Vec<&BarWidget> {
        self.config.widgets.iter().filter(|w| w.visible).collect()
    }

    /// Get widget by ID
    pub fn get_widget(&self, id: &str) -> Option<&BarWidget> {
        self.widgets.get(id)
    }
}

impl Default for BarManager {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for WidgetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetKind::Workspaces => write!(f, "Workspaces"),
            WidgetKind::ActiveWindow => write!(f, "Active Window"),
            WidgetKind::Clock => write!(f, "Clock"),
            WidgetKind::Weather => write!(f, "Weather"),
            WidgetKind::Media => write!(f, "Media"),
            WidgetKind::SystemTray => write!(f, "System Tray"),
            WidgetKind::Battery => write!(f, "Battery"),
            WidgetKind::KeyboardLayout => write!(f, "Keyboard Layout"),
            WidgetKind::Network => write!(f, "Network"),
            WidgetKind::Audio => write!(f, "Audio"),
            WidgetKind::DND => write!(f, "DND"),
            WidgetKind::NightLight => write!(f, "Night Light"),
            WidgetKind::StayAwake => write!(f, "Stay Awake"),
            WidgetKind::ScreenRecording => write!(f, "Screen Recording"),
            WidgetKind::Dictation => write!(f, "Dictation"),
            WidgetKind::Reminders => write!(f, "Reminders"),
            WidgetKind::Custom(name) => write!(f, "Custom ({})", name),
        }
    }
}

impl fmt::Display for BarPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BarPosition::Top => write!(f, "Top"),
            BarPosition::Bottom => write!(f, "Bottom"),
            BarPosition::Left => write!(f, "Left"),
            BarPosition::Right => write!(f, "Right"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bar() {
        let bar = BarManager::default();
        assert_eq!(bar.config.position, BarPosition::Top);
        assert!(!bar.config.widgets.is_empty());
        assert!(bar.widgets.contains_key("workspaces"));
    }

    #[test]
    fn test_add_widget() {
        let mut bar = BarManager::new();
        bar.add_widget("custom-widget".to_string(), WidgetKind::Reminders, WidgetPosition::Right);
        assert!(bar.widgets.contains_key("custom-widget"));
    }

    #[test]
    fn test_remove_widget() {
        let mut bar = BarManager::new();
        bar.add_widget("temp".to_string(), WidgetKind::Reminders, WidgetPosition::Right);
        bar.remove_widget("temp").unwrap();
        assert!(!bar.widgets.contains_key("temp"));
    }

    #[test]
    fn test_toggle_widget() {
        let mut bar = BarManager::new();
        assert!(bar.widgets.get("workspaces").unwrap().visible);
        bar.toggle_widget("workspaces").unwrap();
        assert!(!bar.widgets.get("workspaces").unwrap().visible);
    }

    #[test]
    fn test_set_transparency() {
        let mut bar = BarManager::new();
        assert!(bar.set_transparency(0.5).is_ok());
        assert!(bar.set_transparency(1.5).is_err());
    }

    #[test]
    fn test_list_visible() {
        let bar = BarManager::new();
        let visible = bar.list_visible();
        assert!(!visible.is_empty());
        assert!(visible.iter().all(|w| w.visible));
    }
}