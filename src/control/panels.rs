//! SigmaOS Control Panels
//!
//! Inspired by: Omarchy's control panels (Audio, Bluetooth, Network, Display, Power)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, SEPARATION_OF_CONCERNS, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY,
//! USER_FRIENDLY, SIMPLE_USER_INTERFACE, CLEAR_ABSTRACTION, COMPOSITION_OVER_INHERITANCE.

use std::collections::HashMap;

/// Control panel type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlPanel {
    Audio,
    Bluetooth,
    Network,
    Display,
    Power,
    Keyboard,
    Mouse,
    User,
    Security,
    About,
}

/// Control panel entry
#[derive(Debug, Clone)]
pub struct PanelEntry {
    pub panel: ControlPanel,
    pub title: String,
    pub icon: String,
    pub shortcut: Option<String>,
    pub description: String,
}

/// Control panels manager
pub struct ControlPanels {
    pub panels: HashMap<ControlPanel, PanelEntry>,
    pub active_panel: Option<ControlPanel>,
    pub shortcuts: HashMap<String, ControlPanel>, // shortcut -> panel
}

impl ControlPanels {
    pub fn new() -> Self {
        let mut panels = HashMap::new();
        let mut shortcuts = HashMap::new();

        let default_panels = vec![
            (ControlPanel::Audio, "Audio", "🔊", Some("SUPER+CTRL+A"), "Audio controls and volume"),
            (ControlPanel::Bluetooth, "Bluetooth", "📱", Some("SUPER+CTRL+B"), "Bluetooth devices"),
            (ControlPanel::Network, "Network", "🌐", Some("SUPER+CTRL+W"), "Network connections"),
            (ControlPanel::Display, "Display", "🖥️", Some("SUPER+CTRL+D"), "Display settings"),
            (ControlPanel::Power, "Power", "🔋", Some("SUPER+CTRL+P"), "Power management"),
            (ControlPanel::Keyboard, "Keyboard", "⌨️", None, "Keyboard settings"),
            (ControlPanel::Mouse, "Mouse", "🖱️", None, "Mouse settings"),
            (ControlPanel::User, "User", "👤", None, "User accounts"),
            (ControlPanel::Security, "Security", "🔒", None, "Security settings"),
            (ControlPanel::About, "About", "ℹ️", None, "System information"),
        ];

        for (panel, title, icon, shortcut, desc) in default_panels {
            let entry = PanelEntry {
                panel: panel.clone(),
                title: title.to_string(),
                icon: icon.to_string(),
                shortcut: shortcut.map(|s| s.to_string()),
                description: desc.to_string(),
            };
            panels.insert(panel.clone(), entry);
            if let Some(sc) = shortcut {
                shortcuts.insert(sc, panel);
            }
        }

        Self {
            panels,
            active_panel: None,
            shortcuts,
        }
    }

    /// Open a control panel
    pub fn open(&mut self, panel: ControlPanel) {
        self.active_panel = Some(panel);
    }

    /// Close active panel
    pub fn close(&mut self) {
        self.active_panel = None;
    }

    /// Get active panel
    pub fn get_active(&self) -> Option<&PanelEntry> {
        self.active_panel.as_ref().and_then(|p| self.panels.get(p))
    }

    /// Handle keyboard shortcut
    pub fn handle_shortcut(&mut self, shortcut: &str) -> Option<ControlPanel> {
        self.shortcuts.get(shortcut).cloned()
    }

    /// List all panels
    pub fn list_panels(&self) -> Vec<&PanelEntry> {
        self.panels.values().collect()
    }

    /// Get panel by type
    pub fn get_panel(&self, panel: ControlPanel) -> Option<&PanelEntry> {
        self.panels.get(&panel)
    }
}

impl Default for ControlPanels {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ControlPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControlPanel::Audio => write!(f, "Audio"),
            ControlPanel::Bluetooth => write!(f, "Bluetooth"),
            ControlPanel::Network => write!(f, "Network"),
            ControlPanel::Display => write!(f, "Display"),
            ControlPanel::Power => write!(f, "Power"),
            ControlPanel::Keyboard => write!(f, "Keyboard"),
            ControlPanel::Mouse => write!(f, "Mouse"),
            ControlPanel::User => write!(f, "User"),
            ControlPanel::Security => write!(f, "Security"),
            ControlPanel::About => write!(f, "About"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_panels() {
        let panels = ControlPanels::default();
        assert_eq!(panels.list_panels().len(), 10);
        assert!(panels.get_panel(ControlPanel::Audio).is_some());
    }

    #[test]
    fn test_open_close_panel() {
        let mut panels = ControlPanels::new();
        panels.open(ControlPanel::Audio);
        assert!(panels.get_active().is_some());
        assert_eq!(panels.get_active().unwrap().panel, ControlPanel::Audio);
        panels.close();
        assert!(panels.get_active().is_none());
    }

    #[test]
    fn test_shortcut_handling() {
        let mut panels = ControlPanels::new();
        let panel = panels.handle_shortcut("SUPER+CTRL+A");
        assert_eq!(panel, Some(ControlPanel::Audio));
        let panel = panels.handle_shortcut("SUPER+CTRL+B");
        assert_eq!(panel, Some(ControlPanel::Bluetooth));
        let panel = panels.handle_shortcut("INVALID");
        assert_eq!(panel, None);
    }

    #[test]
    fn test_panel_entry() {
        let panels = ControlPanels::default();
        let audio = panels.get_panel(ControlPanel::Audio).unwrap();
        assert_eq!(audio.title, "Audio");
        assert_eq!(audio.shortcut, Some("SUPER+CTRL+A".to_string()));
    }
}