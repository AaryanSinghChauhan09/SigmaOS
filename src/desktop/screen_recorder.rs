// SPDX-License-Identifier: MIT
// SigmaOS - Screen Recorder
// Inspired by OBS Studio and SimpleScreenRecorder
// Provides screen recording functionality

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Recording area
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingArea {
    Fullscreen,
    Window,
    Selection,
    Custom {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
}

/// Video codec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodec {
    H264,
    H265,
    VP9,
    AV1,
    ProRes,
}

/// Audio codec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCodec {
    AAC,
    Opus,
    Vorbis,
    MP3,
    FLAC,
}

/// Container format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerFormat {
    MP4,
    MKV,
    WebM,
    MOV,
    AVI,
}

/// Recording quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Recording state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Idle,
    Recording,
    Paused,
    Stopping,
}

/// Recording configuration
#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub area: RecordingArea,
    pub video_codec: VideoCodec,
    pub audio_codec: AudioCodec,
    pub container: ContainerFormat,
    pub quality: RecordingQuality,
    pub frame_rate: u32,
    pub bitrate: u32,
    pub record_audio: bool,
    pub record_cursor: bool,
    pub output_path: String,
}

/// Screen recorder
pub struct ScreenRecorder {
    pub state: RecordingState,
    pub config: RecordingConfig,
    pub duration: u32,
    pub file_size: u64,
}

impl ScreenRecorder {
    pub fn new() -> Self {
        Self {
            state: RecordingState::Idle,
            config: RecordingConfig {
                area: RecordingArea::Fullscreen,
                video_codec: VideoCodec::H264,
                audio_codec: AudioCodec::AAC,
                container: ContainerFormat::MP4,
                quality: RecordingQuality::High,
                frame_rate: 30,
                bitrate: 5000,
                record_audio: true,
                record_cursor: true,
                output_path: String::new(),
            },
            duration: 0,
            file_size: 0,
        }
    }

    /// Set recording area
    pub fn set_area(&mut self, area: RecordingArea) {
        self.config.area = area;
    }

    /// Set video codec
    pub fn set_video_codec(&mut self, codec: VideoCodec) {
        self.config.video_codec = codec;
    }

    /// Set audio codec
    pub fn set_audio_codec(&mut self, codec: AudioCodec) {
        self.config.audio_codec = codec;
    }

    /// Set container format
    pub fn set_container(&mut self, container: ContainerFormat) {
        self.config.container = container;
    }

    /// Set quality
    pub fn set_quality(&mut self, quality: RecordingQuality) {
        self.config.quality = quality;
    }

    /// Set frame rate
    pub fn set_frame_rate(&mut self, frame_rate: u32) {
        self.config.frame_rate = frame_rate;
    }

    /// Set bitrate
    pub fn set_bitrate(&mut self, bitrate: u32) {
        self.config.bitrate = bitrate;
    }

    /// Set output path
    pub fn set_output_path(&mut self, path: String) {
        self.config.output_path = path;
    }

    /// Toggle audio recording
    pub fn toggle_audio(&mut self) {
        self.config.record_audio = !self.config.record_audio;
    }

    /// Toggle cursor recording
    pub fn toggle_cursor(&mut self) {
        self.config.record_cursor = !self.config.record_cursor;
    }

    /// Start recording
    pub fn start_recording(&mut self) -> Result<(), &'static str> {
        if self.state != RecordingState::Idle {
            return Err("Already recording or paused");
        }
        if self.config.output_path.is_empty() {
            return Err("Output path not set");
        }
        self.state = RecordingState::Recording;
        Ok(())
    }

    /// Pause recording
    pub fn pause_recording(&mut self) -> Result<(), &'static str> {
        if self.state != RecordingState::Recording {
            return Err("Not recording");
        }
        self.state = RecordingState::Paused;
        Ok(())
    }

    /// Resume recording
    pub fn resume_recording(&mut self) -> Result<(), &'static str> {
        if self.state != RecordingState::Paused {
            return Err("Not paused");
        }
        self.state = RecordingState::Recording;
        Ok(())
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> Result<(), &'static str> {
        if self.state != RecordingState::Recording && self.state != RecordingState::Paused {
            return Err("Not recording");
        }
        self.state = RecordingState::Stopping;
        self.state = RecordingState::Idle;
        Ok(())
    }

    /// Get state
    pub fn get_state(&self) -> RecordingState {
        self.state
    }

    /// Get duration
    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    /// Get file size
    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }
}

/// Global screen recorder
static mut SCREEN_RECORDER: Option<ScreenRecorder> = None;

/// Initialize screen recorder
pub fn init_screen_recorder() {
    unsafe {
        SCREEN_RECORDER = Some(ScreenRecorder::new());
    }
}

/// Get screen recorder
pub fn get_screen_recorder() -> &'static mut ScreenRecorder {
    unsafe {
        SCREEN_RECORDER
            .as_mut()
            .expect("Screen recorder not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_recorder_creation() {
        let sr = ScreenRecorder::new();
        assert_eq!(sr.state, RecordingState::Idle);
    }

    #[test]
    fn test_start_recording() {
        let mut sr = ScreenRecorder::new();
        sr.set_output_path("/tmp/recording.mp4".to_string());
        sr.start_recording().unwrap();
        assert_eq!(sr.state, RecordingState::Recording);
    }

    #[test]
    fn test_pause_resume() {
        let mut sr = ScreenRecorder::new();
        sr.set_output_path("/tmp/recording.mp4".to_string());
        sr.start_recording().unwrap();
        sr.pause_recording().unwrap();
        assert_eq!(sr.state, RecordingState::Paused);
        sr.resume_recording().unwrap();
        assert_eq!(sr.state, RecordingState::Recording);
    }
}
