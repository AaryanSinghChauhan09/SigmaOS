//! SigmaOS Settings — system settings UI.
//!
//! Upstream inspiration:
//! - cinnamon-control-center:   <https://github.com/linuxmint/cinnamon-control-center>
//! - cinnamon-settings-daemon:  <https://github.com/linuxmint/cinnamon-settings-daemon>

pub mod display;
pub mod network;
pub mod sound;

use std::{collections::HashMap, fmt};

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum SettingsError {
    PanelNotFound(String),
    PersistenceFailed(String),
    ValidationError(String),
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsError::PanelNotFound(n) => write!(f, "panel not found: {n}"),
            SettingsError::PersistenceFailed(msg) => write!(f, "persistence failed: {msg}"),
            SettingsError::ValidationError(msg) => write!(f, "validation error: {msg}"),
        }
    }
}

impl std::error::Error for SettingsError {}

// ---------------------------------------------------------------------------
// Settings store
// ---------------------------------------------------------------------------

/// Flat key-value settings store.
#[derive(Debug, Default, Clone)]
pub struct SettingsStore {
    data: HashMap<String, String>,
}

impl SettingsStore {
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(String::as_str)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Settings panel trait
// ---------------------------------------------------------------------------

pub trait SettingsPanel: fmt::Debug {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn apply(&self, store: &mut SettingsStore) -> Result<(), SettingsError>;
}

// ---------------------------------------------------------------------------
// Network panel
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct NetworkPanel {
    pub wifi_enabled: bool,
    pub ethernet_enabled: bool,
}

impl SettingsPanel for NetworkPanel {
    fn id(&self) -> &str { "network" }
    fn title(&self) -> &str { "Network" }

    fn apply(&self, store: &mut SettingsStore) -> Result<(), SettingsError> {
        store.set("network.wifi_enabled", self.wifi_enabled.to_string());
        store.set("network.ethernet_enabled", self.ethernet_enabled.to_string());
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Display panel
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct DisplayPanel {
    pub brightness: u8,
    pub night_light: bool,
    pub resolution: (u32, u32),
}

impl Default for DisplayPanel {
    fn default() -> Self {
        Self {
            brightness: 100,
            night_light: false,
            resolution: (1920, 1080),
        }
    }
}

impl SettingsPanel for DisplayPanel {
    fn id(&self) -> &str { "display" }
    fn title(&self) -> &str { "Display" }

    fn apply(&self, store: &mut SettingsStore) -> Result<(), SettingsError> {
        if self.brightness > 100 {
            return Err(SettingsError::ValidationError(
                "brightness must be 0-100".into(),
            ));
        }
        store.set("display.brightness", self.brightness.to_string());
        store.set("display.night_light", self.night_light.to_string());
        store.set(
            "display.resolution",
            format!("{}x{}", self.resolution.0, self.resolution.1),
        );
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Settings app
// ---------------------------------------------------------------------------

pub struct SettingsApp {
    panels: Vec<Box<dyn SettingsPanel>>,
    pub store: SettingsStore,
}

impl SettingsApp {
    fn new() -> Self {
        Self {
            panels: Vec::new(),
            store: SettingsStore::default(),
        }
    }

    pub fn register_panel(&mut self, panel: Box<dyn SettingsPanel>) {
        self.panels.push(panel);
    }

    pub fn apply_all(&mut self) -> Result<(), SettingsError> {
        let results: Vec<Result<(), SettingsError>> = self
            .panels
            .iter()
            .map(|p| p.apply(&mut self.store))
            .collect();
        for r in results {
            r?;
        }
        Ok(())
    }

    pub fn panel(&self, id: &str) -> Option<&dyn SettingsPanel> {
        self.panels.iter().find(|p| p.id() == id).map(|p| p.as_ref())
    }

    pub fn panel_count(&self) -> usize {
        self.panels.len()
    }
}

// ---------------------------------------------------------------------------
// Public entry-point
// ---------------------------------------------------------------------------

pub fn launch_settings() -> SettingsApp {
    let mut app = SettingsApp::new();
    app.register_panel(Box::new(NetworkPanel::default()));
    app.register_panel(Box::new(DisplayPanel::default()));
    app
}

// ---------------------------------------------------------------------------
// Inline tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_app_launches_with_default_panels() {
        let app = launch_settings();
        assert_eq!(app.panel_count(), 2);
    }

    #[test]
    fn display_panel_validates_brightness() {
        let mut panel = DisplayPanel::default();
        panel.brightness = 200;
        let mut store = SettingsStore::default();
        let result = panel.apply(&mut store);
        assert!(result.is_err());
    }
}
