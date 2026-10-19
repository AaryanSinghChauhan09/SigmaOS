//! SigmaAudioConfig - Audio Configuration Manager
//!
//! Inspired by PipeWire audio configuration and pro audio tools, this module provides
//! comprehensive audio configuration including device selection, latency tuning, and
//! audio routing.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Audio device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDeviceType {
    Input,
    Output,
    Duplex,
    Unknown,
}

/// Audio profile enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioProfile {
    LowLatency,
    Balanced,
    HighQuality,
    ProAudio,
    Unknown,
}

/// Audio sample rate enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleRate {
    Hz44100,
    Hz48000,
    Hz96000,
    Hz192000,
    Unknown,
}

/// Audio bit depth enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    Bit16,
    Bit24,
    Bit32,
    Float32,
    Unknown,
}

/// Audio device data structure
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub device_id: String,
    pub device_name: String,
    pub device_type: AudioDeviceType,
    pub profile: AudioProfile,
    pub sample_rate: SampleRate,
    pub bit_depth: BitDepth,
    pub latency_ms: u32,
    pub buffer_size: u32,
    pub enabled: bool,
    pub default_device: bool,
}

impl AudioDevice {
    /// Create a new audio device
    pub fn new(
        device_id: String,
        device_name: String,
        device_type: AudioDeviceType,
    ) -> Self {
        Self {
            device_id,
            device_name,
            device_type,
            profile: AudioProfile::Balanced,
            sample_rate: SampleRate::Hz48000,
            bit_depth: BitDepth::Bit16,
            latency_ms: 20,
            buffer_size: 1024,
            enabled: true,
            default_device: false,
        }
    }

    /// Set audio profile
    pub fn set_profile(&mut self, profile: AudioProfile) {
        self.profile = profile;
        // Adjust settings based on profile
        match profile {
            AudioProfile::LowLatency => {
                self.latency_ms = 5;
                self.buffer_size = 256;
            }
            AudioProfile::ProAudio => {
                self.latency_ms = 2;
                self.buffer_size = 128;
                self.sample_rate = SampleRate::Hz96000;
                self.bit_depth = BitDepth::Float32;
            }
            AudioProfile::HighQuality => {
                self.latency_ms = 50;
                self.buffer_size = 4096;
                self.sample_rate = SampleRate::Hz192000;
                self.bit_depth = BitDepth::Bit32;
            }
            _ => {
                self.latency_ms = 20;
                self.buffer_size = 1024;
            }
        }
    }

    /// Set sample rate
    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        self.sample_rate = sample_rate;
    }

    /// Set bit depth
    pub fn set_bit_depth(&mut self, bit_depth: BitDepth) {
        self.bit_depth = bit_depth;
    }

    /// Set latency
    pub fn set_latency(&mut self, latency_ms: u32) {
        self.latency_ms = latency_ms;
    }

    /// Set buffer size
    pub fn set_buffer_size(&mut self, buffer_size: u32) {
        self.buffer_size = buffer_size;
    }

    /// Enable device
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable device
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set as default device
    pub fn set_default(&mut self, is_default: bool) {
        self.default_device = is_default;
    }

    /// Check if device is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if device is default
    pub fn is_default(&self) -> bool {
        self.default_device
    }
}

/// Audio configuration manager
pub struct SigmaAudioConfig {
    devices: BTreeMap<String, AudioDevice>,
    running: AtomicBool,
}

impl SigmaAudioConfig {
    /// Create a new audio configuration manager
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the audio configuration manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the audio configuration manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add an audio device
    pub fn add_device(&mut self, device: AudioDevice) -> bool {
        let device_id = device.device_id.clone();
        self.devices.insert(device_id, device);
        true
    }

    /// Remove an audio device
    pub fn remove_device(&mut self, device_id: &str) -> Option<AudioDevice> {
        self.devices.remove(device_id)
    }

    /// Get a device by ID
    pub fn get_device(&self, device_id: &str) -> Option<&AudioDevice> {
        self.devices.get(device_id)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&AudioDevice> {
        self.devices.values().collect()
    }

    /// List devices by type
    pub fn list_devices_by_type(&self, device_type: AudioDeviceType) -> Vec<&AudioDevice> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    /// List enabled devices
    pub fn list_enabled_devices(&self) -> Vec<&AudioDevice> {
        self.devices
            .values()
            .filter(|d| d.enabled)
            .collect()
    }

    /// Get default device by type
    pub fn get_default_device(&self, device_type: AudioDeviceType) -> Option<&AudioDevice> {
        self.devices
            .values()
            .find(|d| d.device_type == device_type && d.default_device)
    }

    /// Set default device
    pub fn set_default_device(&mut self, device_id: &str) -> bool {
        // Clear default flag for devices of same type
        let device_type = if let Some(device) = self.devices.get(device_id) {
            device.device_type
        } else {
            return false;
        };

        for device in self.devices.values_mut() {
            if device.device_type == device_type {
                device.set_default(false);
            }
        }

        // Set new default
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_default(true);
            true
        } else {
            false
        }
    }

    /// Enable a device
    pub fn enable_device(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.enable();
            true
        } else {
            false
        }
    }

    /// Disable a device
    pub fn disable_device(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.disable();
            true
        } else {
            false
        }
    }

    /// Configure device profile
    pub fn configure_profile(&mut self, device_id: &str, profile: AudioProfile) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_profile(profile);
            true
        } else {
            false
        }
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for SigmaAudioConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaAudioConfig instance (placeholder for actual global state)
static unsafe mutable SIGMAAUDIOCONFIG_INSTANCE: Option<SigmaAudioConfig> = None;

/// Initialize the global SigmaAudioConfig instance
pub fn init_sigmaaudioconfig() {
    unsafe {
        SIGMAAUDIOCONFIG_INSTANCE = Some(SigmaAudioConfig::new());
    }
}

/// Get the global SigmaAudioConfig instance
pub fn get_sigmaaudioconfig() -> Option<&'static SigmaAudioConfig> {
    unsafe {
        SIGMAAUDIOCONFIG_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaAudioConfig instance
pub fn get_sigmaaudioconfig_mut() -> Option<&'static mut SigmaAudioConfig> {
    unsafe {
        SIGMAAUDIOCONFIG_INSTANCE.as_mut()
    }
}

/// Convenience function to add a device
pub fn add_audio_device(device: AudioDevice) -> bool {
    if let Some(audio_mgr) = get_sigmaaudioconfig_mut() {
        audio_mgr.add_device(device)
    } else {
        false
    }
}

/// Convenience function to set default device
pub fn set_default_audio_device(device_id: &str) -> bool {
    if let Some(audio_mgr) = get_sigmaaudioconfig_mut() {
        audio_mgr.set_default_device(device_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_device_creation() {
        let device = AudioDevice::new(
            "default".to_string(),
            "Default Audio Device".to_string(),
            AudioDeviceType::Output,
        );

        assert_eq!(device.device_id, "default");
        assert_eq!(device.device_type, AudioDeviceType::Output);
        assert_eq!(device.profile, AudioProfile::Balanced);
        assert!(device.is_enabled());
    }

    #[test]
    fn test_audio_device_profile() {
        let mut device = AudioDevice::new(
            "pro".to_string(),
            "Pro Audio Device".to_string(),
            AudioDeviceType::Duplex,
        );

        device.set_profile(AudioProfile::ProAudio);
        assert_eq!(device.profile, AudioProfile::ProAudio);
        assert_eq!(device.latency_ms, 2);
        assert_eq!(device.buffer_size, 128);
    }

    #[test]
    fn test_audio_device_enable_disable() {
        let mut device = AudioDevice::new(
            "test".to_string(),
            "Test Device".to_string(),
            AudioDeviceType::Input,
        );

        assert!(device.is_enabled());
        device.disable();
        assert!(!device.is_enabled());
        device.enable();
        assert!(device.is_enabled());
    }

    #[test]
    fn test_sigmaaudioconfig_creation() {
        let audio_mgr = SigmaAudioConfig::new();
        assert!(!audio_mgr.is_running());
        assert_eq!(audio_mgr.device_count(), 0);
    }

    #[test]
    fn test_sigmaaudioconfig_start_stop() {
        let mut audio_mgr = SigmaAudioConfig::new();
        audio_mgr.start();
        assert!(audio_mgr.is_running());
        audio_mgr.stop();
        assert!(!audio_mgr.is_running());
    }

    #[test]
    fn test_sigmaaudioconfig_add_device() {
        let mut audio_mgr = SigmaAudioConfig::new();
        let device = AudioDevice::new(
            "default".to_string(),
            "Default Device".to_string(),
            AudioDeviceType::Output,
        );

        let added = audio_mgr.add_device(device);
        assert!(added);
        assert_eq!(audio_mgr.device_count(), 1);
    }

    #[test]
    fn test_sigmaaudioconfig_remove_device() {
        let mut audio_mgr = SigmaAudioConfig::new();
        let device = AudioDevice::new(
            "test".to_string(),
            "Test Device".to_string(),
            AudioDeviceType::Input,
        );

        audio_mgr.add_device(device);
        assert_eq!(audio_mgr.device_count(), 1);

        let removed = audio_mgr.remove_device("test");
        assert!(removed.is_some());
        assert_eq!(audio_mgr.device_count(), 0);
    }

    #[test]
    fn test_sigmaaudioconfig_list_by_type() {
        let mut audio_mgr = SigmaAudioConfig::new();

        audio_mgr.add_device(AudioDevice::new(
            "out1".to_string(),
            "Output 1".to_string(),
            AudioDeviceType::Output,
        ));

        audio_mgr.add_device(AudioDevice::new(
            "out2".to_string(),
            "Output 2".to_string(),
            AudioDeviceType::Output,
        ));

        audio_mgr.add_device(AudioDevice::new(
            "in1".to_string(),
            "Input 1".to_string(),
            AudioDeviceType::Input,
        ));

        let output_devices = audio_mgr.list_devices_by_type(AudioDeviceType::Output);
        assert_eq!(output_devices.len(), 2);

        let input_devices = audio_mgr.list_devices_by_type(AudioDeviceType::Input);
        assert_eq!(input_devices.len(), 1);
    }

    #[test]
    fn test_sigmaaudioconfig_set_default() {
        let mut audio_mgr = SigmaAudioConfig::new();

        audio_mgr.add_device(AudioDevice::new(
            "out1".to_string(),
            "Output 1".to_string(),
            AudioDeviceType::Output,
        ));

        audio_mgr.add_device(AudioDevice::new(
            "out2".to_string(),
            "Output 2".to_string(),
            AudioDeviceType::Output,
        ));

        let set = audio_mgr.set_default_device("out1");
        assert!(set);

        let default = audio_mgr.get_default_device(AudioDeviceType::Output);
        assert!(default.is_some());
        assert_eq!(default.unwrap().device_id, "out1");
    }

    #[test]
    fn test_sigmaaudioconfig_configure_profile() {
        let mut audio_mgr = SigmaAudioConfig::new();
        let device = AudioDevice::new(
            "pro".to_string(),
            "Pro Device".to_string(),
            AudioDeviceType::Duplex,
        );

        audio_mgr.add_device(device);
        let configured = audio_mgr.configure_profile("pro", AudioProfile::ProAudio);
        assert!(configured);

        let device = audio_mgr.get_device("pro").unwrap();
        assert_eq!(device.profile, AudioProfile::ProAudio);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaaudioconfig();

        let audio_mgr = get_sigmaaudioconfig();
        assert!(audio_mgr.is_some());

        let mut audio_mgr_mut = get_sigmaaudioconfig_mut();
        assert!(audio_mgr_mut.is_some());

        if let Some(audio_mgr) = audio_mgr_mut {
            audio_mgr.start();
            assert!(audio_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_add_device() {
        init_sigmaaudioconfig();

        let device = AudioDevice::new(
            "default".to_string(),
            "Default Device".to_string(),
            AudioDeviceType::Output,
        );

        let added = add_audio_device(device);
        assert!(added);

        let audio_mgr = get_sigmaaudioconfig().unwrap();
        assert_eq!(audio_mgr.device_count(), 1);
    }

    #[test]
    fn test_convenience_set_default() {
        init_sigmaaudioconfig();

        let device = AudioDevice::new(
            "default".to_string(),
            "Default Device".to_string(),
            AudioDeviceType::Output,
        );

        add_audio_device(device);
        let set = set_default_audio_device("default");
        assert!(set);

        let audio_mgr = get_sigmaaudioconfig().unwrap();
        let default = audio_mgr.get_default_device(AudioDeviceType::Output);
        assert!(default.is_some());
    }
}
