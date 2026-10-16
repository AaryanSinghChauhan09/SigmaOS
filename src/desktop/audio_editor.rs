// SPDX-License-Identifier: MIT
// SigmaOS - Audio Editor
// Inspired by Audacity and Ardour
// Provides audio editing capabilities

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

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
    Float32,
}

/// Channel count
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelCount {
    Mono,
    Stereo,
    Surround51,
    Surround71,
}

/// Tool type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioTool {
    Selection,
    Move,
    Cut,
    Copy,
    Paste,
    Trim,
    Silence,
    Zoom,
}

/// Audio region
#[derive(Debug, Clone)]
pub struct AudioRegion {
    pub id: String,
    pub name: String,
    pub start_time: f64,
    pub duration: f64,
    pub offset: f64,
    pub track: u32,
    pub muted: bool,
    pub volume: f32,
    pub pan: f32,
}

/// Track
#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: u32,
    pub name: String,
    pub track_type: String,
    pub regions: Vec<String>,
    pub muted: bool,
    pub solo: bool,
    pub volume: f32,
    pub pan: f32,
    pub armed: bool,
}

/// Effect
#[derive(Debug, Clone)]
pub struct AudioEffect {
    pub id: String,
    pub name: String,
    pub effect_type: String,
    pub enabled: bool,
    pub parameters: Vec<(String, f32)>,
}

/// Audio editor
pub struct AudioEditor {
    pub project_name: String,
    pub sample_rate: SampleRate,
    pub bit_depth: BitDepth,
    pub channel_count: ChannelCount,
    pub format: AudioFormat,
    pub tracks: Vec<AudioTrack>,
    pub regions: Vec<AudioRegion>,
    pub effects: Vec<AudioEffect>,
    pub current_time: f64,
    pub duration: f64,
    pub playing: bool,
    pub recording: bool,
    pub selection_start: f64,
    pub selection_end: f64,
    pub zoom: f32,
}

impl AudioEditor {
    pub fn new() -> Self {
        Self {
            project_name: "Untitled Project".to_string(),
            sample_rate: SampleRate::Hz44100,
            bit_depth: BitDepth::Bit24,
            channel_count: ChannelCount::Stereo,
            format: AudioFormat::WAV,
            tracks: Vec::new(),
            regions: Vec::new(),
            effects: Vec::new(),
            current_time: 0.0,
            duration: 0.0,
            playing: false,
            recording: false,
            selection_start: 0.0,
            selection_end: 0.0,
            zoom: 1.0,
        }
    }

    /// Set project name
    pub fn set_project_name(&mut self, name: String) {
        self.project_name = name;
    }

    /// Set sample rate
    pub fn set_sample_rate(&mut self, sample_rate: SampleRate) {
        self.sample_rate = sample_rate;
    }

    /// Set bit depth
    pub fn set_bit_depth(&mut self, bit_depth: BitDepth) {
        self.bit_depth = bit_depth;
    }

    /// Add track
    pub fn add_track(&mut self, track_type: String) -> u32 {
        let track_id = self.tracks.len() as u32;
        let track = AudioTrack {
            id: track_id,
            name: format!("Track {}", track_id),
            track_type,
            regions: Vec::new(),
            muted: false,
            solo: false,
            volume: 1.0,
            pan: 0.0,
            armed: false,
        };
        self.tracks.push(track);
        track_id
    }

    /// Remove track
    pub fn remove_track(&mut self, track_id: u32) -> Result<(), &'static str> {
        let index = self
            .tracks
            .iter()
            .position(|t| t.id == track_id)
            .ok_or("Track not found")?;
        self.tracks.remove(index);
        Ok(())
    }

    /// Add region
    pub fn add_region(&mut self, region: AudioRegion) {
        self.regions.push(region);
    }

    /// Remove region
    pub fn remove_region(&mut self, region_id: String) -> Result<(), &'static str> {
        let index = self
            .regions
            .iter()
            .position(|r| r.id == region_id)
            .ok_or("Region not found")?;
        self.regions.remove(index);
        Ok(())
    }

    /// Add effect
    pub fn add_effect(&mut self, effect: AudioEffect) {
        self.effects.push(effect);
    }

    /// Set selection
    pub fn set_selection(&mut self, start: f64, end: f64) {
        self.selection_start = start;
        self.selection_end = end;
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection_start = 0.0;
        self.selection_end = 0.0;
    }

    /// Set current time
    pub fn set_current_time(&mut self, time: f64) {
        self.current_time = time.max(0.0).min(self.duration);
    }

    /// Play
    pub fn play(&mut self) {
        self.playing = true;
    }

    /// Pause
    pub fn pause(&mut self) {
        self.playing = false;
    }

    /// Stop
    pub fn stop(&mut self) {
        self.playing = false;
        self.recording = false;
        self.current_time = 0.0;
    }

    /// Record
    pub fn record(&mut self) {
        self.recording = true;
        self.playing = true;
    }

    /// Seek
    pub fn seek(&mut self, time: f64) {
        self.current_time = time.max(0.0).min(self.duration);
    }

    /// Set zoom
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.max(0.1).min(10.0);
    }

    /// Zoom in
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom * 1.25).min(10.0);
    }

    /// Zoom out
    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom / 1.25).max(0.1);
    }

    /// Get tracks
    pub fn get_tracks(&self) -> &[AudioTrack] {
        &self.tracks
    }

    /// Get regions
    pub fn get_regions(&self) -> &[AudioRegion] {
        &self.regions
    }

    /// Track count
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Region count
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }
}

/// Global audio editor
static mut AUDIO_EDITOR: Option<AudioEditor> = None;

/// Initialize audio editor
pub fn init_audio_editor() {
    unsafe {
        AUDIO_EDITOR = Some(AudioEditor::new());
    }
}

/// Get audio editor
pub fn get_audio_editor() -> &'static mut AudioEditor {
    unsafe { AUDIO_EDITOR.as_mut().expect("Audio editor not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_editor_creation() {
        let ae = AudioEditor::new();
        assert_eq!(ae.project_name, "Untitled Project");
    }

    #[test]
    fn test_add_track() {
        let mut ae = AudioEditor::new();
        ae.add_track("Audio".to_string());
        assert_eq!(ae.track_count(), 1);
    }

    #[test]
    fn test_play_pause() {
        let mut ae = AudioEditor::new();
        ae.play();
        assert!(ae.playing);
        ae.pause();
        assert!(!ae.playing);
    }
}
