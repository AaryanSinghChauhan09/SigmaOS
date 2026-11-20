//! Sound settings panel for SigmaOS-settings.

use crate::{SettingsError, SettingsPanel, SettingsStore};

/// Audio device entry.
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

/// Sound settings panel.
#[derive(Debug)]
pub struct SoundPanel {
    pub output_volume: u8,  // 0-100
    pub input_volume: u8,   // 0-100
    pub muted: bool,
    pub output_devices: Vec<AudioDevice>,
    pub input_devices: Vec<AudioDevice>,
    pub default_output_id: Option<String>,
    pub default_input_id: Option<String>,
}

impl Default for SoundPanel {
    fn default() -> Self {
        Self {
            output_volume: 75,
            input_volume: 80,
            muted: false,
            output_devices: Vec::new(),
            input_devices: Vec::new(),
            default_output_id: None,
            default_input_id: None,
        }
    }
}

impl SoundPanel {
    pub fn add_output_device(&mut self, id: impl Into<String>, name: impl Into<String>) {
        let id = id.into();
        let is_default = self.default_output_id.as_deref() == Some(&id);
        self.output_devices.push(AudioDevice { id, name: name.into(), is_default });
    }

    pub fn set_default_output(&mut self, id: impl Into<String>) {
        let id = id.into();
        for dev in &mut self.output_devices {
            dev.is_default = dev.id == id;
        }
        self.default_output_id = Some(id);
    }
}

impl SettingsPanel for SoundPanel {
    fn id(&self) -> &str { "sound" }
    fn title(&self) -> &str { "Sound" }

    fn apply(&self, store: &mut SettingsStore) -> Result<(), SettingsError> {
        if self.output_volume > 100 || self.input_volume > 100 {
            return Err(SettingsError::ValidationError("volume must be 0-100".into()));
        }
        store.set("sound.output_volume", self.output_volume.to_string());
        store.set("sound.input_volume", self.input_volume.to_string());
        store.set("sound.muted", self.muted.to_string());
        if let Some(id) = &self.default_output_id {
            store.set("sound.default_output", id.clone());
        }
        Ok(())
    }
}
