// Zenith Compositor Configuration for SigmaOS
// Zenith compositor configuration per Wiki 08-Desktop.md
// Provides configuration parsing and management for the Zenith desktop environment

use std::string::{String, ToString};
use std::vec::Vec;

/// Compositor backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorBackend {
    Drm,
    Wayland,
    X11,
}

impl CompositorBackend {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "drm" => CompositorBackend::Drm,
            "wayland" => CompositorBackend::Wayland,
            "x11" => CompositorBackend::X11,
            _ => CompositorBackend::Drm,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            CompositorBackend::Drm => "drm",
            CompositorBackend::Wayland => "wayland",
            CompositorBackend::X11 => "x11",
        }
    }
}

/// Output scale mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputScale {
    Auto,
    Scale1x,
    Scale2x,
    Scale3x,
}

impl OutputScale {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "auto" => OutputScale::Auto,
            "1x" => OutputScale::Scale1x,
            "2x" => OutputScale::Scale2x,
            "3x" => OutputScale::Scale3x,
            _ => OutputScale::Auto,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            OutputScale::Auto => "auto",
            OutputScale::Scale1x => "1x",
            OutputScale::Scale2x => "2x",
            OutputScale::Scale3x => "3x",
        }
    }
}

/// Theme type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    Auto,
}

impl Theme {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "auto" => Theme::Auto,
            _ => Theme::Dark,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::Auto => "auto",
        }
    }
}

/// Mouse acceleration mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseAcceleration {
    None,
    Adaptive,
    Flat,
}

impl MouseAcceleration {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => MouseAcceleration::None,
            "adaptive" => MouseAcceleration::Adaptive,
            "flat" => MouseAcceleration::Flat,
            _ => MouseAcceleration::Adaptive,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            MouseAcceleration::None => "none",
            MouseAcceleration::Adaptive => "adaptive",
            MouseAcceleration::Flat => "flat",
        }
    }
}

/// Compositor configuration
#[derive(Debug, Clone)]
pub struct CompositorConfig {
    pub backend: CompositorBackend,
    pub output_scale: OutputScale,
    pub vsync: bool,
}

impl Default for CompositorConfig {
    fn default() -> Self {
        CompositorConfig {
            backend: CompositorBackend::Drm,
            output_scale: OutputScale::Auto,
            vsync: true,
        }
    }
}

/// Input configuration
#[derive(Debug, Clone)]
pub struct InputConfig {
    pub keyboard_layout: String,
    pub mouse_acceleration: MouseAcceleration,
}

impl Default for InputConfig {
    fn default() -> Self {
        InputConfig {
            keyboard_layout: String::from("us"),
            mouse_acceleration: MouseAcceleration::Adaptive,
        }
    }
}

/// Appearance configuration
#[derive(Debug, Clone)]
pub struct AppearanceConfig {
    pub theme: Theme,
    pub font: String,
    pub icon_theme: String,
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        AppearanceConfig {
            theme: Theme::Dark,
            font: String::from("system-ui"),
            icon_theme: String::from("sigma-icons"),
        }
    }
}

/// Zenith compositor configuration
#[derive(Debug, Clone)]
pub struct ZenithConfig {
    pub compositor: CompositorConfig,
    pub input: InputConfig,
    pub appearance: AppearanceConfig,
}

impl ZenithConfig {
    pub fn new() -> Self {
        ZenithConfig {
            compositor: CompositorConfig::default(),
            input: InputConfig::default(),
            appearance: AppearanceConfig::default(),
        }
    }

    pub fn parse_config(config_str: &str) -> Self {
        let mut config = ZenithConfig::new();

        for line in config_str.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim().trim_matches('"');

            match key {
                "backend" => config.compositor.backend = CompositorBackend::from_str(value),
                "output_scale" => config.compositor.output_scale = OutputScale::from_str(value),
                "vsync" => config.compositor.vsync = value == "true",
                "keyboard_layout" => config.input.keyboard_layout = String::from(value),
                "mouse_acceleration" => {
                    config.input.mouse_acceleration = MouseAcceleration::from_str(value)
                }
                "theme" => config.appearance.theme = Theme::from_str(value),
                "font" => config.appearance.font = String::from(value),
                "icon_theme" => config.appearance.icon_theme = String::from(value),
                _ => {}
            }
        }

        config
    }

    pub fn to_config_string(&self) -> String {
        let mut result = String::new();

        result.push_str("[compositor]\n");
        result.push_str(&format!(
            "backend = \"{}\"\n",
            self.compositor.backend.as_str()
        ));
        result.push_str(&format!(
            "output_scale = \"{}\"\n",
            self.compositor.output_scale.as_str()
        ));
        result.push_str(&format!("vsync = {}\n\n", self.compositor.vsync));

        result.push_str("[input]\n");
        result.push_str(&format!(
            "keyboard_layout = \"{}\"\n",
            self.input.keyboard_layout
        ));
        result.push_str(&format!(
            "mouse_acceleration = \"{}\"\n\n",
            self.input.mouse_acceleration.as_str()
        ));

        result.push_str("[appearance]\n");
        result.push_str(&format!("theme = \"{}\"\n", self.appearance.theme.as_str()));
        result.push_str(&format!("font = \"{}\"\n", self.appearance.font));
        result.push_str(&format!(
            "icon_theme = \"{}\"\n",
            self.appearance.icon_theme
        ));

        result
    }

    pub fn set_backend(&mut self, backend: CompositorBackend) {
        self.compositor.backend = backend;
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.compositor.vsync = vsync;
    }

    pub fn set_keyboard_layout(&mut self, layout: String) {
        self.input.keyboard_layout = layout;
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.appearance.theme = theme;
    }
}

impl Default for ZenithConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zenith_config_creation() {
        let config = ZenithConfig::new();
        assert_eq!(config.compositor.backend, CompositorBackend::Drm);
        assert_eq!(config.compositor.vsync, true);
        assert_eq!(config.input.keyboard_layout, "us");
        assert_eq!(config.appearance.theme, Theme::Dark);
    }

    #[test]
    fn test_parse_config() {
        let config_str = r#"
backend = "drm"
output_scale = "auto"
vsync = true
keyboard_layout = "us"
mouse_acceleration = "adaptive"
theme = "dark"
font = "system-ui"
icon_theme = "sigma-icons"
"#;

        let config = ZenithConfig::parse_config(config_str);
        assert_eq!(config.compositor.backend, CompositorBackend::Drm);
        assert_eq!(config.compositor.vsync, true);
        assert_eq!(config.input.keyboard_layout, "us");
        assert_eq!(config.appearance.theme, Theme::Dark);
    }

    #[test]
    fn test_to_config_string() {
        let config = ZenithConfig::new();
        let config_str = config.to_config_string();

        assert!(config_str.contains("[compositor]"));
        assert!(config_str.contains("[input]"));
        assert!(config_str.contains("[appearance]"));
        assert!(config_str.contains("backend = \"drm\""));
        assert!(config_str.contains("vsync = true"));
    }

    #[test]
    fn test_set_backend() {
        let mut config = ZenithConfig::new();
        config.set_backend(CompositorBackend::Wayland);
        assert_eq!(config.compositor.backend, CompositorBackend::Wayland);
    }

    #[test]
    fn test_set_vsync() {
        let mut config = ZenithConfig::new();
        config.set_vsync(false);
        assert_eq!(config.compositor.vsync, false);
    }

    #[test]
    fn test_set_keyboard_layout() {
        let mut config = ZenithConfig::new();
        config.set_keyboard_layout(String::from("de"));
        assert_eq!(config.input.keyboard_layout, "de");
    }

    #[test]
    fn test_set_theme() {
        let mut config = ZenithConfig::new();
        config.set_theme(Theme::Light);
        assert_eq!(config.appearance.theme, Theme::Light);
    }

    #[test]
    fn test_compositor_backend_from_str() {
        assert_eq!(CompositorBackend::from_str("drm"), CompositorBackend::Drm);
        assert_eq!(
            CompositorBackend::from_str("wayland"),
            CompositorBackend::Wayland
        );
        assert_eq!(CompositorBackend::from_str("x11"), CompositorBackend::X11);
        assert_eq!(
            CompositorBackend::from_str("unknown"),
            CompositorBackend::Drm
        );
    }

    #[test]
    fn test_output_scale_from_str() {
        assert_eq!(OutputScale::from_str("auto"), OutputScale::Auto);
        assert_eq!(OutputScale::from_str("1x"), OutputScale::Scale1x);
        assert_eq!(OutputScale::from_str("2x"), OutputScale::Scale2x);
        assert_eq!(OutputScale::from_str("3x"), OutputScale::Scale3x);
    }

    #[test]
    fn test_theme_from_str() {
        assert_eq!(Theme::from_str("dark"), Theme::Dark);
        assert_eq!(Theme::from_str("light"), Theme::Light);
        assert_eq!(Theme::from_str("auto"), Theme::Auto);
    }

    #[test]
    fn test_mouse_acceleration_from_str() {
        assert_eq!(MouseAcceleration::from_str("none"), MouseAcceleration::None);
        assert_eq!(
            MouseAcceleration::from_str("adaptive"),
            MouseAcceleration::Adaptive
        );
        assert_eq!(MouseAcceleration::from_str("flat"), MouseAcceleration::Flat);
    }

    #[test]
    fn test_default_configs() {
        assert_eq!(CompositorConfig::default().backend, CompositorBackend::Drm);
        assert_eq!(InputConfig::default().keyboard_layout, "us");
        assert_eq!(AppearanceConfig::default().theme, Theme::Dark);
    }
}
