//! SigmaMicrophone - Microphone Management System
//!
//! Provides comprehensive microphone management inspired by Linux ALSA and PulseAudio.
//! Supports microphone discovery, audio capture, and input device control.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Microphone state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrophoneState {
    Idle,
    Recording,
    Error,
    Unknown,
}

/// Sample rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleRate {
    Hz8000,
    Hz16000,
    Hz22050,
    Hz44100,
    Hz48000,
    Hz96000,
    Hz192000,
    Unknown,
}

/// Sample format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleFormat {
    S16LE,
    S16BE,
    S24LE,
    S24BE,
    S32LE,
    S32BE,
    Float32LE,
    Float32BE,
    Unknown,
}

/// Channel count
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelCount {
    Mono,
    Stereo,
    Surround51,
    Surround71,
    Unknown,
}

/// Microphone
#[derive(Debug, Clone)]
pub struct Microphone {
    pub id: String,
    pub name: String,
    pub state: MicrophoneState,
    pub device: String,
    pub sample_rates: Vec<SampleRate>,
    pub formats: Vec<SampleFormat>,
    pub channels: Vec<ChannelCount>,
}

impl Microphone {
    pub fn new(id: String, name: String, device: String) -> Self {
        Self {
            id,
            name,
            state: MicrophoneState::Idle,
            device,
            sample_rates: Vec::new(),
            formats: Vec::new(),
            channels: Vec::new(),
        }
    }

    pub fn with_sample_rate(mut self, rate: SampleRate) -> Self {
        self.sample_rates.push(rate);
        self
    }

    pub fn with_format(mut self, format: SampleFormat) -> Self {
        self.formats.push(format);
        self
    }

    pub fn with_channels(mut self, channels: ChannelCount) -> Self {
        self.channels.push(channels);
        self
    }

    pub fn start_recording(&mut self) {
        self.state = MicrophoneState::Recording;
    }

    pub fn stop_recording(&mut self) {
        self.state = MicrophoneState::Idle;
    }
}

/// Microphone manager
pub struct SigmaMicrophone {
    microphones: BTreeMap<String, Microphone>,
    next_microphone_id: u64,
}

impl SigmaMicrophone {
    pub fn new() -> Self {
        Self {
            microphones: BTreeMap::new(),
            next_microphone_id: 1,
        }
    }

    pub fn add_microphone(&mut self, name: String, device: String) -> String {
        let id = format!("mic-{}", self.next_microphone_id);
        self.next_microphone_id += 1;
        let microphone = Microphone::new(id.clone(), name, device);
        self.microphones.insert(id.clone(), microphone);
        id
    }

    pub fn get_microphone(&self, id: &str) -> Option<&Microphone> {
        self.microphones.get(id)
    }

    pub fn get_microphone_mut(&mut self, id: &str) -> Option<&mut Microphone> {
        self.microphones.get_mut(id)
    }

    pub fn list_microphones(&self) -> Vec<&Microphone> {
        self.microphones.values().collect()
    }

    pub fn remove_microphone(&mut self, id: &str) -> bool {
        self.microphones.remove(id).is_some()
    }

    pub fn microphone_count(&self) -> usize {
        self.microphones.len()
    }
}

impl Default for SigmaMicrophone {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMAMICROPHONE_INSTANCE: Option<SigmaMicrophone> = None;

pub fn init_sigmamicrophone() {
    unsafe {
        SIGMAMICROPHONE_INSTANCE = Some(SigmaMicrophone::new());
    }
}

pub fn get_sigmamicrophone() -> &'static SigmaMicrophone {
    unsafe {
        SIGMAMICROPHONE_INSTANCE
            .as_ref()
            .expect("SigmaMicrophone not initialized")
    }
}

pub fn get_sigmamicrophone_mut() -> &'static mut SigmaMicrophone {
    unsafe {
        SIGMAMICROPHONE_INSTANCE
            .as_mut()
            .expect("SigmaMicrophone not initialized")
    }
}

pub fn add_microphone(name: String, device: String) -> String {
    let manager = get_sigmamicrophone_mut();
    manager.add_microphone(name, device)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microphone_creation() {
        let mic = Microphone::new("mic-1".to_string(), "Blue Yeti".to_string(), "/dev/snd/controlC0".to_string());
        assert_eq!(mic.id, "mic-1");
        assert_eq!(mic.name, "Blue Yeti");
        assert_eq!(mic.state, MicrophoneState::Idle);
    }

    #[test]
    fn test_microphone_state() {
        let mut mic = Microphone::new("mic-1".to_string(), "Blue Yeti".to_string(), "/dev/snd/controlC0".to_string());
        assert_eq!(mic.state, MicrophoneState::Idle);

        mic.start_recording();
        assert_eq!(mic.state, MicrophoneState::Recording);

        mic.stop_recording();
        assert_eq!(mic.state, MicrophoneState::Idle);
    }

    #[test]
    fn test_microphone_with_sample_rate() {
        let mic = Microphone::new("mic-1".to_string(), "Blue Yeti".to_string(), "/dev/snd/controlC0".to_string())
            .with_sample_rate(SampleRate::Hz48000)
            .with_sample_rate(SampleRate::Hz44100);
        assert_eq!(mic.sample_rates.len(), 2);
    }

    #[test]
    fn test_microphone_with_format() {
        let mic = Microphone::new("mic-1".to_string(), "Blue Yeti".to_string(), "/dev/snd/controlC0".to_string())
            .with_format(SampleFormat::S16LE)
            .with_format(SampleFormat::Float32LE);
        assert_eq!(mic.formats.len(), 2);
    }

    #[test]
    fn test_microphone_with_channels() {
        let mic = Microphone::new("mic-1".to_string(), "Blue Yeti".to_string(), "/dev/snd/controlC0".to_string())
            .with_channels(ChannelCount::Mono)
            .with_channels(ChannelCount::Stereo);
        assert_eq!(mic.channels.len(), 2);
    }

    #[test]
    fn test_microphone_manager() {
        let manager = SigmaMicrophone::new();
        assert_eq!(manager.microphone_count(), 0);
    }

    #[test]
    fn test_add_microphone() {
        let mut manager = SigmaMicrophone::new();
        let id = manager.add_microphone("Blue Yeti".to_string(), "/dev/snd/controlC0".to_string());
        assert_eq!(id, "mic-1");
        assert_eq!(manager.microphone_count(), 1);
    }

    #[test]
    fn test_list_microphones() {
        let mut manager = SigmaMicrophone::new();
        manager.add_microphone("Blue Yeti".to_string(), "/dev/snd/controlC0".to_string());
        manager.add_microphone("Audio-Technica".to_string(), "/dev/snd/controlC1".to_string());

        let mics = manager.list_microphones();
        assert_eq!(mics.len(), 2);
    }

    #[test]
    fn test_remove_microphone() {
        let mut manager = SigmaMicrophone::new();
        let id = manager.add_microphone("Blue Yeti".to_string(), "/dev/snd/controlC0".to_string());
        assert_eq!(manager.microphone_count(), 1);

        manager.remove_microphone(&id);
        assert_eq!(manager.microphone_count(), 0);
    }

    #[test]
    fn test_global_instance() {
        init_sigmamicrophone();
        let manager = get_sigmamicrophone();
        assert_eq!(manager.microphone_count(), 0);

        let manager_mut = get_sigmamicrophone_mut();
        manager_mut.add_microphone("Blue Yeti".to_string(), "/dev/snd/controlC0".to_string());
        assert_eq!(manager.microphone_count(), 1);
    }

    #[test]
    fn test_add_microphone_convenience() {
        init_sigmamicrophone();
        let id = add_microphone("Blue Yeti".to_string(), "/dev/snd/controlC0".to_string());
        assert_eq!(id, "mic-1");

        let manager = get_sigmamicrophone();
        assert!(manager.get_microphone(&id).is_some());
    }
}
