// SigmaOS Sound/Audio Manager
// Inspired by Linux Mint's Sound settings and Omarchy's audio utilities

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Audio device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DesktopAudioDeviceType {
    Speaker,
    Microphone,
    Headphones,
    Headset,
    LineOut,
    LineIn,
    SPDIF,
    HDMI,
    Bluetooth,
}

impl DesktopAudioDeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopAudioDeviceType::Speaker => "Speaker",
            DesktopAudioDeviceType::Microphone => "Microphone",
            DesktopAudioDeviceType::Headphones => "Headphones",
            DesktopAudioDeviceType::Headset => "Headset",
            DesktopAudioDeviceType::LineOut => "Line Out",
            DesktopAudioDeviceType::LineIn => "Line In",
            DesktopAudioDeviceType::SPDIF => "SPDIF",
            DesktopAudioDeviceType::HDMI => "HDMI",
            DesktopAudioDeviceType::Bluetooth => "Bluetooth",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "speaker" => Some(DesktopAudioDeviceType::Speaker),
            "microphone" => Some(DesktopAudioDeviceType::Microphone),
            "headphones" => Some(DesktopAudioDeviceType::Headphones),
            "headset" => Some(DesktopAudioDeviceType::Headset),
            "lineout" | "line out" => Some(DesktopAudioDeviceType::LineOut),
            "linein" | "line in" => Some(DesktopAudioDeviceType::LineIn),
            "spdif" => Some(DesktopAudioDeviceType::SPDIF),
            "hdmi" => Some(DesktopAudioDeviceType::HDMI),
            "bluetooth" => Some(DesktopAudioDeviceType::Bluetooth),
            _ => None,
        }
    }
}

/// Audio device status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopAudioDeviceStatus {
    Active,
    Inactive,
    Unplugged,
    Error,
}

impl DesktopAudioDeviceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopAudioDeviceStatus::Active => "Active",
            DesktopAudioDeviceStatus::Inactive => "Inactive",
            DesktopAudioDeviceStatus::Unplugged => "Unplugged",
            DesktopAudioDeviceStatus::Error => "Error",
        }
    }
}

/// Audio device
#[derive(Debug, Clone)]
pub struct DesktopAudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: DesktopAudioDeviceType,
    pub status: DesktopAudioDeviceStatus,
    pub volume: u8, // 0-100
    pub is_muted: bool,
    pub is_default: bool,
    pub channels: u8,
    pub sample_rate: u32,
}

impl DesktopAudioDevice {
    pub fn new(
        id: String,
        name: String,
        device_type: DesktopAudioDeviceType,
        status: DesktopAudioDeviceStatus,
    ) -> Self {
        DesktopAudioDevice {
            id,
            name,
            device_type,
            status,
            volume: 50,
            is_muted: false,
            is_default: false,
            channels: 2,
            sample_rate: 48000,
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.is_muted = muted;
    }

    pub fn set_default(&mut self, default: bool) {
        self.is_default = default;
    }
}

/// Sound application
#[derive(Debug, Clone)]
pub struct DesktopSoundApplication {
    pub id: String,
    pub name: String,
    pub volume: u8,
    pub is_muted: bool,
}

impl DesktopSoundApplication {
    pub fn new(id: String, name: String) -> Self {
        DesktopSoundApplication {
            id,
            name,
            volume: 100,
            is_muted: false,
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.is_muted = muted;
    }
}

/// Sound output profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopSoundProfile {
    AnalogStereo,
    AnalogSurround51,
    AnalogSurround71,
    DigitalStereo,
    DigitalSurround51,
    DigitalSurround71,
}

impl DesktopSoundProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopSoundProfile::AnalogStereo => "Analog Stereo",
            DesktopSoundProfile::AnalogSurround51 => "Analog Surround 5.1",
            DesktopSoundProfile::AnalogSurround71 => "Analog Surround 7.1",
            DesktopSoundProfile::DigitalStereo => "Digital Stereo",
            DesktopSoundProfile::DigitalSurround51 => "Digital Surround 5.1",
            DesktopSoundProfile::DigitalSurround71 => "Digital Surround 7.1",
        }
    }
}

/// Sound/Audio Manager
pub struct DesktopSoundManager {
    devices: HashMap<String, DesktopAudioDevice>,
    applications: HashMap<String, DesktopSoundApplication>,
    output_profile: DesktopSoundProfile,
    master_volume: u8,
    master_muted: bool,
    next_device_id: u32,
    next_app_id: u32,
}

impl DesktopSoundManager {
    pub fn new() -> Self {
        let mut manager = DesktopSoundManager {
            devices: HashMap::new(),
            applications: HashMap::new(),
            output_profile: DesktopSoundProfile::AnalogStereo,
            master_volume: 50,
            master_muted: false,
            next_device_id: 1,
            next_app_id: 1,
        };

        // Add default devices
        manager.add_default_devices();

        manager
    }

    fn add_default_devices(&mut self) {
        let mut speakers = DesktopAudioDevice::new(
            format!("dev_{}", self.next_device_id),
            "Built-in Speakers".to_string(),
            DesktopAudioDeviceType::Speaker,
            DesktopAudioDeviceStatus::Active,
        );
        speakers.set_default(true);
        self.devices.insert(speakers.id.clone(), speakers);
        self.next_device_id += 1;

        let mut mic = DesktopAudioDevice::new(
            format!("dev_{}", self.next_device_id),
            "Built-in Microphone".to_string(),
            DesktopAudioDeviceType::Microphone,
            DesktopAudioDeviceStatus::Active,
        );
        self.devices.insert(mic.id.clone(), mic);
        self.next_device_id += 1;

        let headphones = DesktopAudioDevice::new(
            format!("dev_{}", self.next_device_id),
            "Headphones".to_string(),
            DesktopAudioDeviceType::Headphones,
            DesktopAudioDeviceStatus::Inactive,
        );
        self.devices.insert(headphones.id.clone(), headphones);
        self.next_device_id += 1;
    }

    pub fn add_device(
        &mut self,
        name: String,
        device_type: DesktopAudioDeviceType,
        status: DesktopAudioDeviceStatus,
    ) -> String {
        let id = format!("dev_{}", self.next_device_id);
        let device = DesktopAudioDevice::new(id.clone(), name, device_type, status);
        self.devices.insert(id.clone(), device);
        self.next_device_id += 1;
        id
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        if let Some(device) = self.devices.get(id) {
            if device.is_default {
                return false; // Cannot remove default device
            }
        }
        self.devices.remove(id).is_some()
    }

    pub fn get_device(&self, id: &str) -> Option<&DesktopAudioDevice> {
        self.devices.get(id)
    }

    pub fn get_devices(&self) -> Vec<&DesktopAudioDevice> {
        self.devices.values().collect()
    }

    pub fn get_devices_by_type(
        &self,
        device_type: DesktopAudioDeviceType,
    ) -> Vec<&DesktopAudioDevice> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    pub fn get_default_output_device(&self) -> Option<&DesktopAudioDevice> {
        self.devices.values().find(|d| {
            d.is_default
                && matches!(
                    d.device_type,
                    DesktopAudioDeviceType::Speaker
                        | DesktopAudioDeviceType::Headphones
                        | DesktopAudioDeviceType::Headset
                        | DesktopAudioDeviceType::HDMI
                        | DesktopAudioDeviceType::SPDIF
                        | DesktopAudioDeviceType::LineOut
                        | DesktopAudioDeviceType::Bluetooth
                )
        })
    }

    pub fn get_default_input_device(&self) -> Option<&DesktopAudioDevice> {
        self.devices.values().find(|d| {
            d.is_default
                && matches!(
                    d.device_type,
                    DesktopAudioDeviceType::Microphone
                        | DesktopAudioDeviceType::Headset
                        | DesktopAudioDeviceType::LineIn
                )
        })
    }

    pub fn set_default_device(&mut self, id: &str) -> bool {
        if !self.devices.contains_key(id) {
            return false;
        }

        // Clear default from all devices of same type
        let device_type = self.devices.get(id).map(|d| d.device_type);
        if let Some(dt) = device_type {
            for device in self.devices.values_mut() {
                if device.device_type == dt {
                    device.set_default(false);
                }
            }
        }

        // Set new default
        if let Some(device) = self.devices.get_mut(id) {
            device.set_default(true);
            true
        } else {
            false
        }
    }

    pub fn set_device_volume(&mut self, id: &str, volume: u8) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_volume(volume);
            true
        } else {
            false
        }
    }

    pub fn set_device_muted(&mut self, id: &str, muted: bool) -> bool {
        if let Some(device) = self.devices.get_mut(id) {
            device.set_muted(muted);
            true
        } else {
            false
        }
    }

    pub fn add_application(&mut self, name: String) -> String {
        let id = format!("app_{}", self.next_app_id);
        let app = DesktopSoundApplication::new(id.clone(), name);
        self.applications.insert(id.clone(), app);
        self.next_app_id += 1;
        id
    }

    pub fn remove_application(&mut self, id: &str) -> bool {
        self.applications.remove(id).is_some()
    }

    pub fn get_application(&self, id: &str) -> Option<&DesktopSoundApplication> {
        self.applications.get(id)
    }

    pub fn get_applications(&self) -> Vec<&DesktopSoundApplication> {
        self.applications.values().collect()
    }

    pub fn set_application_volume(&mut self, id: &str, volume: u8) -> bool {
        if let Some(app) = self.applications.get_mut(id) {
            app.set_volume(volume);
            true
        } else {
            false
        }
    }

    pub fn set_application_muted(&mut self, id: &str, muted: bool) -> bool {
        if let Some(app) = self.applications.get_mut(id) {
            app.set_muted(muted);
            true
        } else {
            false
        }
    }

    pub fn get_master_volume(&self) -> u8 {
        self.master_volume
    }

    pub fn set_master_volume(&mut self, volume: u8) {
        self.master_volume = volume.min(100);
    }

    pub fn is_master_muted(&self) -> bool {
        self.master_muted
    }

    pub fn set_master_muted(&mut self, muted: bool) {
        self.master_muted = muted;
    }

    pub fn get_output_profile(&self) -> DesktopSoundProfile {
        self.output_profile
    }

    pub fn set_output_profile(&mut self, profile: DesktopSoundProfile) {
        self.output_profile = profile;
    }

    pub fn get_statistics(&self) -> DesktopSoundStatistics {
        DesktopSoundStatistics {
            device_count: self.devices.len(),
            application_count: self.applications.len(),
            master_volume: self.master_volume,
            master_muted: self.master_muted,
            output_profile: self.output_profile,
        }
    }
}

impl Default for DesktopSoundManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Sound statistics
#[derive(Debug, Clone, Copy)]
pub struct DesktopSoundStatistics {
    pub device_count: usize,
    pub application_count: usize,
    pub master_volume: u8,
    pub master_muted: bool,
    pub output_profile: DesktopSoundProfile,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_device_type_from_str() {
        assert_eq!(
            DesktopAudioDeviceType::from_str("speaker"),
            Some(DesktopAudioDeviceType::Speaker)
        );
        assert_eq!(
            DesktopAudioDeviceType::from_str("microphone"),
            Some(DesktopAudioDeviceType::Microphone)
        );
        assert_eq!(
            DesktopAudioDeviceType::from_str("headphones"),
            Some(DesktopAudioDeviceType::Headphones)
        );
        assert_eq!(DesktopAudioDeviceType::from_str("invalid"), None);
    }

    #[test]
    fn test_sound_manager_initialization() {
        let manager = DesktopSoundManager::new();
        assert_eq!(manager.get_devices().len(), 3);
        assert_eq!(manager.get_master_volume(), 50);
        assert!(!manager.is_master_muted());
    }

    #[test]
    fn test_add_device() {
        let mut manager = DesktopSoundManager::new();
        let id = manager.add_device(
            "USB Speaker".to_string(),
            DesktopAudioDeviceType::Speaker,
            DesktopAudioDeviceStatus::Active,
        );
        assert!(manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), 4);
    }

    #[test]
    fn test_remove_device() {
        let mut manager = DesktopSoundManager::new();
        let id = manager.add_device(
            "USB Speaker".to_string(),
            DesktopAudioDeviceType::Speaker,
            DesktopAudioDeviceStatus::Active,
        );
        assert!(manager.remove_device(&id));
        assert!(!manager.get_device(&id).is_some());
        assert_eq!(manager.get_devices().len(), 3);
    }

    #[test]
    fn test_set_default_device() {
        let mut manager = DesktopSoundManager::new();
        let id = manager.add_device(
            "USB Speaker".to_string(),
            DesktopAudioDeviceType::Speaker,
            DesktopAudioDeviceStatus::Active,
        );
        assert!(manager.set_default_device(&id));
        assert!(manager.get_device(&id).unwrap().is_default);
    }

    #[test]
    fn test_set_device_volume() {
        let mut manager = DesktopSoundManager::new();
        let id = manager.add_device(
            "USB Speaker".to_string(),
            DesktopAudioDeviceType::Speaker,
            DesktopAudioDeviceStatus::Active,
        );
        assert!(manager.set_device_volume(&id, 75));
        assert_eq!(manager.get_device(&id).unwrap().volume, 75);
    }

    #[test]
    fn test_set_device_muted() {
        let mut manager = DesktopSoundManager::new();
        let id = manager.add_device(
            "USB Speaker".to_string(),
            DesktopAudioDeviceType::Speaker,
            DesktopAudioDeviceStatus::Active,
        );
        assert!(manager.set_device_muted(&id, true));
        assert!(manager.get_device(&id).unwrap().is_muted);
    }

    #[test]
    fn test_add_application() {
        let mut manager = DesktopSoundManager::new();
        let id = manager.add_application("Firefox".to_string());
        assert!(manager.get_application(&id).is_some());
        assert_eq!(manager.get_applications().len(), 1);
    }

    #[test]
    fn test_set_application_volume() {
        let mut manager = DesktopSoundManager::new();
        let id = manager.add_application("Firefox".to_string());
        assert!(manager.set_application_volume(&id, 50));
        assert_eq!(manager.get_application(&id).unwrap().volume, 50);
    }

    #[test]
    fn test_master_volume() {
        let mut manager = DesktopSoundManager::new();
        manager.set_master_volume(80);
        assert_eq!(manager.get_master_volume(), 80);
    }

    #[test]
    fn test_master_muted() {
        let mut manager = DesktopSoundManager::new();
        manager.set_master_muted(true);
        assert!(manager.is_master_muted());
    }

    #[test]
    fn test_output_profile() {
        let mut manager = DesktopSoundManager::new();
        manager.set_output_profile(DesktopSoundProfile::DigitalSurround51);
        assert_eq!(
            manager.get_output_profile(),
            DesktopSoundProfile::DigitalSurround51
        );
    }

    #[test]
    fn test_statistics() {
        let manager = DesktopSoundManager::new();
        let stats = manager.get_statistics();
        assert_eq!(stats.device_count, 3);
        assert_eq!(stats.application_count, 0);
        assert_eq!(stats.master_volume, 50);
    }
}
