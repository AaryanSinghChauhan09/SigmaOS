// SigmaOS Sovereign XFCE Desktop Integration Engine
// Zero-dependency Rust #![no_std] / std implementation of xfwm4, xfce4-panel, xfconf & Thunar integrations.

#[cfg(not(test))]
use alloc::string::{String, ToString};
#[cfg(not(test))]
use alloc::vec::Vec;
#[cfg(not(test))]
use alloc::format;

#[cfg(test)]
use std::string::String;
#[cfg(test)]
use std::vec::Vec;

/// Xfconf Configuration Setting Value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XfconfValue {
    StringVal(String),
    IntVal(i32),
    BoolVal(bool),
}

/// Xfconf Channel Entry
#[derive(Debug, Clone)]
pub struct XfconfProperty {
    pub channel: String, // "xsettings", "xfwm4", "xfce4-panel", "displays"
    pub property_path: String, // "/Net/ThemeName", "/general/theme"
    pub value: XfconfValue,
}

/// XFCE Desktop Integration Engine
#[derive(Debug, Clone)]
pub struct SovereignXfceDesktopEngine {
    pub xfconf_properties: Vec<XfconfProperty>,
    pub active_panel_applets: Vec<String>,
    pub xfwm4_compositing_enabled: bool,
    pub thunar_custom_actions_count: usize,
}

impl SovereignXfceDesktopEngine {
    pub fn new() -> Self {
        let mut props = Vec::new();

        // Default GTK theme setting
        props.push(XfconfProperty {
            channel: String::from("xsettings"),
            property_path: String::from("/Net/ThemeName"),
            value: XfconfValue::StringVal(String::from("Sigma-Dark")),
        });

        // Default xfwm4 theme setting
        props.push(XfconfProperty {
            channel: String::from("xfwm4"),
            property_path: String::from("/general/theme"),
            value: XfconfValue::StringVal(String::from("Sigma-Dark")),
        });

        let mut applets = Vec::new();
        applets.push(String::from("whiskermenu"));
        applets.push(String::from("tasklist"));
        applets.push(String::from("systray"));
        applets.push(String::from("clock"));
        applets.push(String::from("pulseaudio"));

        Self {
            xfconf_properties: props,
            active_panel_applets: applets,
            xfwm4_compositing_enabled: true,
            thunar_custom_actions_count: 3,
        }
    }

    /// Sets or updates an xfconf channel property
    pub fn set_xfconf_property(&mut self, channel: &str, property_path: &str, value: XfconfValue) {
        if let Some(prop) = self.xfconf_properties.iter_mut().find(|p| p.channel == channel && p.property_path == property_path) {
            prop.value = value;
        } else {
            self.xfconf_properties.push(XfconfProperty {
                channel: String::from(channel),
                property_path: String::from(property_path),
                value,
            });
        }
    }

    /// Retrieves an xfconf channel property
    pub fn get_xfconf_property(&self, channel: &str, property_path: &str) -> Option<&XfconfValue> {
        self.xfconf_properties
            .iter()
            .find(|p| p.channel == channel && p.property_path == property_path)
            .map(|p| &p.value)
    }

    /// Adds an applet to xfce4-panel
    pub fn add_panel_applet(&mut self, applet_name: &str) {
        if !self.active_panel_applets.iter().any(|a| a == applet_name) {
            self.active_panel_applets.push(String::from(applet_name));
        }
    }
}

impl Default for SovereignXfceDesktopEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xfce_desktop_engine() {
        let mut engine = SovereignXfceDesktopEngine::new();

        // Check default xfconf settings
        let theme = engine.get_xfconf_property("xsettings", "/Net/ThemeName");
        assert_eq!(theme, Some(&XfconfValue::StringVal(String::from("Sigma-Dark"))));

        // Update setting
        engine.set_xfconf_property("xsettings", "/Net/ThemeName", XfconfValue::StringVal(String::from("Adwaita-Dark")));
        let updated_theme = engine.get_xfconf_property("xsettings", "/Net/ThemeName");
        assert_eq!(updated_theme, Some(&XfconfValue::StringVal(String::from("Adwaita-Dark"))));

        // Add panel applet
        engine.add_panel_applet("cpugraph");
        assert!(engine.active_panel_applets.contains(&String::from("cpugraph")));
    }
}
