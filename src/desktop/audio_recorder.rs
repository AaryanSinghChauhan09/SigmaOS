// SPDX-License-Identifier: MIT
// SigmaOS - Audio Recorder
// Inspired by Audacity and GNOME Sound Recorder
// Provides audio recording functionality

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Recording state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Stopped,
    Recording,
    Paused,
}

/// Audio format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    WAV,
    MP3,
    OGG,
    FLAC,
    AAC,
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
}

/// Bit depth
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BitDepth {
    Bit8,
    Bit16,
    Bit24,
    Bit32,
}

/// Recording
#[derive(Debug, Clone)]
pub struct Recording {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub duration: f32,
    pub format: AudioFormat,
    pub sample_rate: SampleRate,
    pub bit_depth: BitDepth,
    pub channels: u32,
    pub file_size: u64,
    pub created: i64,
}

/// Audio recorder
pub struct AudioRecorder {
    pub recordings: Vec<Recording>,
    pub current_recording: Option<Recording>,
    pub recording_state: RecordingState,
    pub default_format: AudioFormat,
    pub default_sample_rate: SampleRate,
    pub default_bit_depth: BitDepth,
    pub default_channels: u32,
    pub input_device: String,
    pub output_device: String,
    pub input_gain: f32,
    pub output_gain: f32,
}

impl AudioRecorder {
    pub fn new() -> Self {
        Self {
            recordings: Vec::new(),
            current_recording: None,
            recording_state: RecordingState::Stopped,
            default_format: AudioFormat::WAV,
            default_sample_rate: SampleRate::Hz44100,
            default_bit_depth: BitDepth::Bit16,
            default_channels: 2,
            input_device: "default".to_string(),
            output_device: "default".to_string(),
            input_gain: 1.0,
            output_gain: 1.0,
        }
    }

    /// Start recording
    pub fn start_recording(&mut self, name: String) -> Result<(), &'static str> {
        if self.recording_state == RecordingState::Recording {
            return Err("Already recording");
        }

        let recording = Recording {
            id: self.recordings.len() as u32,
            name: name.clone(),
            path: format!("/home/user/Recordings/{}.wav", name),
            duration: 0.0,
            format: self.default_format,
            sample_rate: self.default_sample_rate,
            bit_depth: self.default_bit_depth,
            channels: self.default_channels,
            file_size: 0,
            created: 0, // Simplified
        };

        self.current_recording = Some(recording);
        self.recording_state = RecordingState::Recording;
        Ok(())
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> Result<(), &'static str> {
        if self.recording_state == RecordingState::Stopped {
            return Err("Not recording");
        }

        if let Some(recording) = self.current_recording.take() {
            self.recordings.push(recording);
        }

        self.recording_state = RecordingState::Stopped;
        Ok(())
    }

    /// Pause recording
    pub fn pause_recording(&mut self) -> Result<(), &'static str> {
        if self.recording_state != RecordingState::Recording {
            return Err("Not recording");
        }

        self.recording_state = RecordingState::Paused;
        Ok(())
    }

    /// Resume recording
    pub fn resume_recording(&mut self) -> Result<(), &'static str> {
        if self.recording_state != RecordingState::Paused {
            return Err("Not paused");
        }

        self.recording_state = RecordingState::Recording;
        Ok(())
    }

    /// Set input device
    pub fn set_input_device(&mut self, device: String) {
        self.input_device = device;
    }

    /// Set output device
    pub fn set_output_device(&mut self, device: String) {
        self.output_device = device;
    }

    /// Set input gain
    pub fn set_input_gain(&mut self, gain: f32) {
        self.input_gain = gain.clamp(0.0, 2.0);
    }

    /// Set output gain
    pub fn set_output_gain(&mut self, gain: f32) {
        self.output_gain = gain.clamp(0.0, 2.0);
    }

    /// Set default format
    pub fn set_default_format(&mut self, format: AudioFormat) {
        self.default_format = format;
    }

    /// Set default sample rate
    pub fn set_default_sample_rate(&mut self, rate: SampleRate) {
        self.default_sample_rate = rate;
    }

    /// Set default bit depth
    pub fn set_default_bit_depth(&mut self, depth: BitDepth) {
        self.default_bit_depth = depth;
    }

    /// Set default channels
    pub fn set_default_channels(&mut self, channels: u32) {
        self.default_channels = channels.clamp(1, 8);
    }

    /// Get current recording
    pub fn get_current_recording(&self) -> Option<&Recording> {
        self.current_recording.as_ref()
    }

    /// Get all recordings
    pub fn get_all_recordings(&self) -> &[Recording] {
        &self.recordings
    }

    /// Remove recording
    pub fn remove_recording(&mut self, id: u32) -> Result<(), &'static str> {
        let index = self
            .recordings
            .iter()
            .position(|r| r.id == id)
            .ok_or("Recording not found")?;
        self.recordings.remove(index);
        Ok(())
    }

    /// Recording count
    pub fn recording_count(&self) -> usize {
        self.recordings.len()
    }

    /// Format duration
    pub fn format_duration(duration: f32) -> String {
        let hours = (duration / 3600.0) as u32;
        let minutes = ((duration % 3600.0) / 60.0) as u32;
        let seconds = (duration % 60.0) as u32;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
}

/// Global audio recorder
static mut AUDIO_RECORDER: Option<AudioRecorder> = None;

/// Initialize audio recorder
pub fn init_audio_recorder() {
    unsafe {
        AUDIO_RECORDER = Some(AudioRecorder::new());
    }
}

/// Get audio recorder
pub fn get_audio_recorder() -> &'static mut AudioRecorder {
    unsafe {
        AUDIO_RECORDER
            .as_mut()
            .expect("Audio recorder not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_recorder_creation() {
        let recorder = AudioRecorder::new();
        assert_eq!(recorder.recording_state, RecordingState::Stopped);
    }

    #[test]
    fn test_start_recording() {
        let mut recorder = AudioRecorder::new();
        recorder.start_recording("Test".to_string()).unwrap();
        assert_eq!(recorder.recording_state, RecordingState::Recording);
    }

    #[test]
    fn test_stop_recording() {
        let mut recorder = AudioRecorder::new();
        recorder.start_recording("Test".to_string()).unwrap();
        recorder.stop_recording().unwrap();
        assert_eq!(recorder.recording_count(), 1);
    }

    #[test]
    fn test_pause_resume() {
        let mut recorder = AudioRecorder::new();
        recorder.start_recording("Test".to_string()).unwrap();
        recorder.pause_recording().unwrap();
        assert_eq!(recorder.recording_state, RecordingState::Paused);
        recorder.resume_recording().unwrap();
        assert_eq!(recorder.recording_state, RecordingState::Recording);
    }

    #[test]
    fn test_set_input_gain() {
        let mut recorder = AudioRecorder::new();
        recorder.set_input_gain(1.5);
        assert_eq!(recorder.input_gain, 1.5);
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(AudioRecorder::format_duration(3661.0), "01:01:01");
        assert_eq!(AudioRecorder::format_duration(125.0), "02:05");
    }
}
