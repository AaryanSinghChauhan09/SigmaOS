//! SigmaVoice - Voice and Speech System
//!
//! Provides comprehensive voice and speech management inspired by Voice Craft.
//! Supports speech recognition, text-to-speech, voice commands, and audio processing.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Voice command
#[derive(Debug, Clone)]
pub struct VoiceCommand {
    pub id: String,
    pub phrase: String,
    pub action: String,
    pub enabled: bool,
}

impl VoiceCommand {
    pub fn new(id: String, phrase: String, action: String) -> Self {
        Self {
            id,
            phrase,
            action,
            enabled: true,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn matches(&self, input: &str) -> bool {
        self.enabled && input.to_lowercase().contains(&self.phrase.to_lowercase())
    }
}

/// Speech recognition result
#[derive(Debug, Clone)]
pub struct SpeechResult {
    pub id: String,
    pub text: String,
    pub confidence: f32,
    pub language: String,
}

impl SpeechResult {
    pub fn new(id: String, text: String, confidence: f32, language: String) -> Self {
        Self {
            id,
            text,
            confidence,
            language,
        }
    }

    pub fn is_confident(&self) -> bool {
        self.confidence >= 0.7
    }
}

/// Voice profile
#[derive(Debug, Clone)]
pub struct VoiceProfile {
    pub id: String,
    pub name: String,
    pub language: String,
    pub voice_id: String,
    pub pitch: f32,
    pub rate: f32,
    pub volume: f32,
}

impl VoiceProfile {
    pub fn new(id: String, name: String, language: String, voice_id: String) -> Self {
        Self {
            id,
            name,
            language,
            voice_id,
            pitch: 1.0,
            rate: 1.0,
            volume: 1.0,
        }
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    pub fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
}

/// SigmaVoice - Voice and speech system
#[derive(Debug)]
pub struct SigmaVoice {
    voice_commands: BTreeMap<String, VoiceCommand>,
    speech_results: Vec<SpeechResult>,
    voice_profiles: BTreeMap<String, VoiceProfile>,
    current_profile: Option<String>,
    listening: bool,
}

impl SigmaVoice {
    pub fn new() -> Self {
        Self {
            voice_commands: BTreeMap::new(),
            speech_results: Vec::new(),
            voice_profiles: BTreeMap::new(),
            current_profile: None,
            listening: false,
        }
    }

    pub fn add_voice_command(&mut self, command: VoiceCommand) {
        self.voice_commands.insert(command.id.clone(), command);
    }

    pub fn remove_voice_command(&mut self, id: &str) -> bool {
        self.voice_commands.remove(id).is_some()
    }

    pub fn get_voice_command(&self, id: &str) -> Option<&VoiceCommand> {
        self.voice_commands.get(id)
    }

    pub fn match_command(&self, input: &str) -> Option<&VoiceCommand> {
        self.voice_commands.values()
            .find(|cmd| cmd.matches(input))
    }

    pub fn add_speech_result(&mut self, result: SpeechResult) {
        self.speech_results.push(result);
    }

    pub fn clear_speech_results(&mut self) {
        self.speech_results.clear();
    }

    pub fn get_latest_result(&self) -> Option<&SpeechResult> {
        self.speech_results.last()
    }

    pub fn add_voice_profile(&mut self, profile: VoiceProfile) {
        self.voice_profiles.insert(profile.id.clone(), profile);
    }

    pub fn remove_voice_profile(&mut self, id: &str) -> bool {
        if self.current_profile.as_ref() == Some(&id.to_string()) {
            self.current_profile = None;
        }
        self.voice_profiles.remove(id).is_some()
    }

    pub fn get_voice_profile(&self, id: &str) -> Option<&VoiceProfile> {
        self.voice_profiles.get(id)
    }

    pub fn set_current_profile(&mut self, id: &str) -> bool {
        if self.voice_profiles.contains_key(id) {
            self.current_profile = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_profile(&self) -> Option<&VoiceProfile> {
        self.current_profile.as_ref()
            .and_then(|id| self.voice_profiles.get(id))
    }

    pub fn start_listening(&mut self) {
        self.listening = true;
    }

    pub fn stop_listening(&mut self) {
        self.listening = false;
    }

    pub fn is_listening(&self) -> bool {
        self.listening
    }

    pub fn command_count(&self) -> usize {
        self.voice_commands.len()
    }

    pub fn profile_count(&self) -> usize {
        self.voice_profiles.len()
    }
}

impl Default for SigmaVoice {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaVoice instance
static mut SIGMAVOICE_INSTANCE: Option<SigmaVoice> = None;

/// Initialize the global SigmaVoice instance
pub fn init_sigmavoice() {
    unsafe {
        SIGMAVOICE_INSTANCE = Some(SigmaVoice::new());
    }
}

/// Get the global SigmaVoice instance
pub fn get_sigmavoice() -> Option<&'static SigmaVoice> {
    unsafe {
        SIGMAVOICE_INSTANCE.as_ref()
    }
}

/// Get the global SigmaVoice instance (mutable)
pub fn get_sigmavoice_mut() -> Option<&'static mut SigmaVoice> {
    unsafe {
        SIGMAVOICE_INSTANCE.as_mut()
    }
}

/// Convenience function to add a voice command
pub fn add_voice_command(command: VoiceCommand) -> bool {
    if let Some(voice) = get_sigmavoice_mut() {
        voice.add_voice_command(command);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voice_command_creation() {
        let command = VoiceCommand::new("cmd1".to_string(), "hello".to_string(), "greet".to_string());
        assert_eq!(command.id, "cmd1");
        assert!(command.enabled);
    }

    #[test]
    fn test_voice_command_matching() {
        let command = VoiceCommand::new("cmd1".to_string(), "hello".to_string(), "greet".to_string());
        assert!(command.matches("hello world"));
        assert!(!command.matches("goodbye"));
    }

    #[test]
    fn test_speech_result_creation() {
        let result = SpeechResult::new("result1".to_string(), "Hello world".to_string(), 0.95, "en-US".to_string());
        assert_eq!(result.text, "Hello world");
        assert!(result.is_confident());
    }

    #[test]
    fn test_voice_profile_creation() {
        let profile = VoiceProfile::new("profile1".to_string(), "Default".to_string(), "en-US".to_string(), "voice1".to_string());
        assert_eq!(profile.id, "profile1");
        assert_eq!(profile.pitch, 1.0);
    }

    #[test]
    fn test_voice_profile_modification() {
        let mut profile = VoiceProfile::new("profile1".to_string(), "Test".to_string(), "en-US".to_string(), "voice1".to_string());
        profile.set_pitch(1.2);
        profile.set_rate(1.5);
        assert_eq!(profile.pitch, 1.2);
        assert_eq!(profile.rate, 1.5);
    }

    #[test]
    fn test_sigmavoice_creation() {
        let voice = SigmaVoice::new();
        assert_eq!(voice.command_count(), 0);
        assert!(!voice.is_listening());
    }

    #[test]
    fn test_sigmavoice_management() {
        let mut voice = SigmaVoice::new();
        let command = VoiceCommand::new("cmd1".to_string(), "hello".to_string(), "greet".to_string());
        voice.add_voice_command(command);
        assert_eq!(voice.command_count(), 1);
    }

    #[test]
    fn test_listening_state() {
        let mut voice = SigmaVoice::new();
        voice.start_listening();
        assert!(voice.is_listening());
        voice.stop_listening();
        assert!(!voice.is_listening());
    }
}
