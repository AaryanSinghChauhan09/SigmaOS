//! Display settings panel for SigmaOS-settings.

use crate::{SettingsError, SettingsPanel, SettingsStore};

/// Display orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Normal,
    Left,
    Right,
    Inverted,
}

impl Default for Orientation { fn default() -> Self { Self::Normal } }

impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Normal => "normal",
            Self::Left => "left",
            Self::Right => "right",
            Self::Inverted => "inverted",
        };
        write!(f, "{}", s)
    }
}

/// Display settings panel.
#[derive(Debug)]
pub struct DisplaySettingsPanel {
    pub resolution: (u32, u32),
    pub refresh_hz: u32,
    pub orientation: Orientation,
    pub brightness: u8,
    pub night_light: bool,
    pub scale: f64,
}

impl Default for DisplaySettingsPanel {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            refresh_hz: 60,
            orientation: Orientation::Normal,
            brightness: 100,
            night_light: false,
            scale: 1.0,
        }
    }
}

impl SettingsPanel for DisplaySettingsPanel {
    fn id(&self) -> &str { "display" }
    fn title(&self) -> &str { "Display" }

    fn apply(&self, store: &mut SettingsStore) -> Result<(), SettingsError> {
        if self.brightness > 100 {
            return Err(SettingsError::ValidationError("brightness must be 0-100".into()));
        }
        if self.scale <= 0.0 {
            return Err(SettingsError::ValidationError("scale must be > 0".into()));
        }
        store.set("display.resolution", format!("{}x{}", self.resolution.0, self.resolution.1));
        store.set("display.refresh_hz", self.refresh_hz.to_string());
        store.set("display.orientation", self.orientation.to_string());
        store.set("display.brightness", self.brightness.to_string());
        store.set("display.night_light", self.night_light.to_string());
        store.set("display.scale", self.scale.to_string());
        Ok(())
    }
}
