//! SigmaSound - Audio Management System
//!
//! Provides comprehensive audio management inspired by Linux PulseAudio and PipeWire.
//! Supports audio devices, volume control, and audio routing.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Audio device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDeviceType {
    Sink,
    Source,
    Unknown,
;

/// Audio device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioDeviceState {
    Running,
    Idle,
    Suspended,
    Unknown,
;

/// Audio device
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: AudioDeviceType,
    pub state: AudioDeviceState,
    pub volume: u8,
    pub muted: bool,
    pub description: String,
;

impl AudioDevice {
    pub fn new(id: String, name: String, device_type: AudioDeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            state: AudioDeviceState::Idle,
            volume: 100,
            muted: false,
            description: String::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    pub fn mute(&mut self) {
        self.muted = true;
    }

    pub fn unmute(&mut self) {
        self.muted = false;
    }

    pub fn run(&mut self) {
        self.state = AudioDeviceState::Running;
    }

    pub fn suspend(&mut self) {
        self.state = AudioDeviceState::Suspended;
    }
;

/// Audio manager
pub struct SigmaSound {
    devices: BTreeMap<String, AudioDevice>,
    next_device_id: u64,
    default_sink: Option<String>,
    default_source: Option<String>,
;

impl SigmaSound {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            next_device_id: 1,
            default_sink: None,
            default_source: None,
        }
    }

    pub fn add_device(&mut self, name: String, device_type: AudioDeviceType) -> String {
        let id = format!("device-{}", self.next_device_id);
        self.next_device_id += 1;
        let device = AudioDevice::new(id.clone(), name, device_type);
        self.devices.insert(id.clone(), device);
        id
    }

    pub fn get_device(&self, id: &str) -> Option<&AudioDevice> {
        self.devices.get(id)
    }

    pub fn get_device_mut(&mut self, id: &str) -> Option<&mut AudioDevice> {
        self.devices.get_mut(id)
    }

    pub fn list_devices(&self) -> Vec<&AudioDevice> {
        self.devices.values().collect()
    }

    pub fn list_devices_by_type(&self, device_type: AudioDeviceType) -> Vec<&AudioDevice> {
        self.devices
            .values()
            .filter(|d| d.device_type == device_type)
            .collect()
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        self.devices.remove(id).is_some()
    }

    pub fn set_default_sink(&mut self, id: String) {
        self.default_sink = Some(id);
    }

    pub fn get_default_sink(&self) -> Option<&String> {
        self.default_sink.as_ref()
    }

    pub fn set_default_source(&mut self, id: String) {
        self.default_source = Some(id);
    }

    pub fn get_default_source(&self) -> Option<&String> {
        self.default_source.as_ref()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
;

impl Default for SigmaSound {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMASOUND_INSTANCE: Option<SigmaSound> = None;

pub fn init_sigmasound() {
    unsafe {
        SIGMASOUND_INSTANCE = Some(SigmaSound::new());
    }
;

pub fn get_sigmasound() -> &'static SigmaSound {
    unsafe {
        SIGMASOUND_INSTANCE
            .as_ref()
            .expect("SigmaSound not initialized")
    }
;

pub fn get_sigmasound_mut() -> &'static mut SigmaSound {
    unsafe {
        SIGMASOUND_INSTANCE
            .as_mut()
            .expect("SigmaSound not initialized")
    }
;

pub fn add_device(name: String, device_type: AudioDeviceType) -> String {
    let manager = get_sigmasound_mut();
    manager.add_device(name, device_type)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_device_creation() {
        let device = AudioDevice::new(
            "device-1".to_string(),
            "Speakers".to_string(),
            AudioDeviceType::Sink,
        );
        assert_eq!(device.id, "device-1");
        assert_eq!(device.name, "Speakers");
        assert_eq!(device.device_type, AudioDeviceType::Sink);
        assert_eq!(device.volume, 100);
    }

    #[test]
    fn test_audio_device_volume() {
        let mut device = AudioDevice::new(
            "device-1".to_string(),
            "Speakers".to_string(),
            AudioDeviceType::Sink,
        );
        device.set_volume(80);
        assert_eq!(device.volume, 80);

        device.set_volume(150);
        assert_eq!(device.volume, 100);
    }

    #[test]
    fn test_audio_device_mute() {
        let mut device = AudioDevice::new(
            "device-1".to_string(),
            "Speakers".to_string(),
            AudioDeviceType::Sink,
        );
        assert!(!device.muted);

        device.mute();
        assert!(device.muted);

        device.unmute();
        assert!(!device.muted);
    }

    #[test]
    fn test_audio_device_state() {
        let mut device = AudioDevice::new(
            "device-1".to_string(),
            "Speakers".to_string(),
            AudioDeviceType::Sink,
        );
        assert_eq!(device.state, AudioDeviceState::Idle);

        device.run();
        assert_eq!(device.state, AudioDeviceState::Running);

        device.suspend();
        assert_eq!(device.state, AudioDeviceState::Suspended);
    }

    #[test]
    fn test_sound_manager() {
        let manager = SigmaSound::new();
        assert_eq!(manager.device_count(), 0);
    }

    #[test]
    fn test_add_device() {
        let mut manager = SigmaSound::new();
        let id = manager.add_device("Speakers".to_string(), AudioDeviceType::Sink);
        assert_eq!(id, "device-1");
        assert_eq!(manager.device_count(), 1);
    }

    #[test]
    fn test_list_devices_by_type() {
        let mut manager = SigmaSound::new();
        manager.add_device("Speakers".to_string(), AudioDeviceType::Sink);
        manager.add_device("Microphone".to_string(), AudioDeviceType::Source);
        manager.add_device("Headphones".to_string(), AudioDeviceType::Sink);

        let sinks = manager.list_devices_by_type(AudioDeviceType::Sink);
        assert_eq!(sinks.len(), 2);
    }

    #[test]
    fn test_default_devices() {
        let mut manager = SigmaSound::new();
        let sink_id = manager.add_device("Speakers".to_string(), AudioDeviceType::Sink);
        let source_id = manager.add_device("Microphone".to_string(), AudioDeviceType::Source);

        manager.set_default_sink(sink_id.clone());
        manager.set_default_source(source_id.clone());

        assert_eq!(manager.get_default_sink(), Some(&sink_id));
        assert_eq!(manager.get_default_source(), Some(&source_id));
    }

    #[test]
    fn test_global_instance() {
        init_sigmasound();
        let manager = get_sigmasound();
        assert_eq!(manager.device_count(), 0);

        let manager_mut = get_sigmasound_mut();
        manager_mut.add_device("Speakers".to_string(), AudioDeviceType::Sink);
        assert_eq!(manager.device_count(), 1);
    }

    #[test]
    fn test_add_device_convenience() {
        init_sigmasound();
        let id = add_device("Speakers".to_string(), AudioDeviceType::Sink);
        assert_eq!(id, "device-1");

        let manager = get_sigmasound();
        assert!(manager.get_device(&id).is_some());
    }
;
