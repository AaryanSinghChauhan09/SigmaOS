// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Audio device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDeviceType {
    Sink,
    Source,
}

/// Audio device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDeviceState {
    Active,
    Idle,
    Suspended,
}

/// Audio device
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub state: AudioDeviceState,
    pub volume: u8,
    pub is_muted: bool,
    pub is_default: bool,
}

/// Audio manager for managing audio devices
/// Inspired by Omarchy audio- commands and Linux audio management
#[derive(Debug, Clone)]
pub struct SigmaAudioManager2 {
    devices: Vec<AudioDevice>,
}

impl SigmaAudioManager2 {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Add an audio device
    pub fn add_device(&mut self, device: AudioDevice) {
        self.devices.push(device);
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&AudioDevice> {
        self.devices.iter().find(|d| d.id == id)
    }

    /// Get all devices
    pub fn get_devices(&self) -> &[AudioDevice] {
        &self.devices
    }

    /// Set volume
    pub fn set_volume(&mut self, id: &str, volume: u8) -> Result<String, String> {
        if volume <= 100 {
            if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
                device.volume = volume;
                Ok(format!("Volume set to: {}%", volume))
            } else {
                Err(format!("Device not found: {}", id))
            }
        } else {
            Err("Invalid volume level".to_string())
        }
    }

    /// Mute device
    pub fn mute(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.is_muted = true;
            Ok(format!("Muted device: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Unmute device
    pub fn unmute(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.is_muted = false;
            Ok(format!("Unmuted device: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Set default device
    pub fn set_default(&mut self, id: &str) -> Result<String, String> {
        for device in &mut self.devices {
            device.is_default = false;
        }
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.is_default = true;
            Ok(format!("Set default device: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Get default device
    pub fn get_default(&self) -> Option<&AudioDevice> {
        self.devices.iter().find(|d| d.is_default)
    }

    /// Get devices by type
    pub fn get_devices_by_type(&self, device_type: AudioDeviceType) -> Vec<&AudioDevice> {
        self.devices
            .iter()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    /// Get active devices
    pub fn get_active_devices(&self) -> Vec<&AudioDevice> {
        self.devices
            .iter()
            .filter(|d| d.state == AudioDeviceState::Active)
            .collect()
    }
}

impl Default for SigmaAudioManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_device() {
        let mut manager = SigmaAudioManager2::new();
        let device = AudioDevice {
            id: "sink-0".to_string(),
            name: "Speakers".to_string(),
            device_type: AudioDeviceType::Sink,
            state: AudioDeviceState::Active,
            volume: 100,
            is_muted: false,
            is_default: true,
        };
        manager.add_device(device);
        assert_eq!(manager.get_devices().len(), 1);
    }

    #[test]
    fn test_set_volume() {
        let mut manager = SigmaAudioManager2::new();
        let device = AudioDevice {
            id: "sink-0".to_string(),
            name: "Speakers".to_string(),
            device_type: AudioDeviceType::Sink,
            state: AudioDeviceState::Active,
            volume: 100,
            is_muted: false,
            is_default: true,
        };
        manager.add_device(device);
        manager.set_volume("sink-0", 50).unwrap();
        assert_eq!(manager.get_device("sink-0").unwrap().volume, 50);
    }

    #[test]
    fn test_mute() {
        let mut manager = SigmaAudioManager2::new();
        let device = AudioDevice {
            id: "sink-0".to_string(),
            name: "Speakers".to_string(),
            device_type: AudioDeviceType::Sink,
            state: AudioDeviceState::Active,
            volume: 100,
            is_muted: false,
            is_default: true,
        };
        manager.add_device(device);
        manager.mute("sink-0").unwrap();
        assert!(manager.get_device("sink-0").unwrap().is_muted);
    }

    #[test]
    fn test_set_default() {
        let mut manager = SigmaAudioManager2::new();
        let device = AudioDevice {
            id: "sink-0".to_string(),
            name: "Speakers".to_string(),
            device_type: AudioDeviceType::Sink,
            state: AudioDeviceState::Active,
            volume: 100,
            is_muted: false,
            is_default: false,
        };
        manager.add_device(device);
        manager.set_default("sink-0").unwrap();
        assert!(manager.get_device("sink-0").unwrap().is_default);
    }

    #[test]
    fn test_get_devices_by_type() {
        let mut manager = SigmaAudioManager2::new();
        let device1 = AudioDevice {
            id: "sink-0".to_string(),
            name: "Speakers".to_string(),
            device_type: AudioDeviceType::Sink,
            state: AudioDeviceState::Active,
            volume: 100,
            is_muted: false,
            is_default: true,
        };
        let device2 = AudioDevice {
            id: "source-0".to_string(),
            name: "Microphone".to_string(),
            device_type: AudioDeviceType::Source,
            state: AudioDeviceState::Active,
            volume: 100,
            is_muted: false,
            is_default: false,
        };
        manager.add_device(device1);
        manager.add_device(device2);
        assert_eq!(manager.get_devices_by_type(AudioDeviceType::Sink).len(), 1);
    }
}
