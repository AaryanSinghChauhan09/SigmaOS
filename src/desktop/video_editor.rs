// SPDX-License-Identifier: MIT
// SigmaOS - Video Editor
// Inspired by Kdenlive and OpenShot
// Provides video editing capabilities

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

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

/// Resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    SD480p,
    HD720p,
    FullHD1080p,
    QHD1440p,
    UHD4K,
}

/// Frame rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    Fps24,
    Fps30,
    Fps60,
}

/// Clip type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipType {
    Video,
    Audio,
    Image,
    Text,
}

/// Clip
#[derive(Debug, Clone)]
pub struct Clip {
    pub id: String,
    pub name: String,
    pub clip_type: ClipType,
    pub file_path: String,
    pub start_time: f64,
    pub duration: f64,
    pub in_point: f64,
    pub out_point: f64,
    pub track: u32,
    pub position: f64,
}

/// Track
#[derive(Debug, Clone)]
pub struct Track {
    pub id: u32,
    pub name: String,
    pub track_type: ClipType,
    pub clips: Vec<String>,
    pub locked: bool,
    pub visible: bool,
    pub volume: f32,
}

/// Transition
#[derive(Debug, Clone)]
pub struct Transition {
    pub id: String,
    pub name: String,
    pub duration: f64,
    pub from_clip: String,
    pub to_clip: String,
}

/// Effect
#[derive(Debug, Clone)]
pub struct Effect {
    pub id: String,
    pub name: String,
    pub effect_type: String,
    pub enabled: bool,
    pub parameters: Vec<(String, String)>,
}

/// Video editor
pub struct VideoEditor {
    pub project_name: String,
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub video_codec: VideoCodec,
    pub audio_codec: AudioCodec,
    pub tracks: Vec<Track>,
    pub clips: Vec<Clip>,
    pub transitions: Vec<Transition>,
    pub effects: Vec<Effect>,
    pub current_time: f64,
    pub duration: f64,
    pub playing: bool,
    pub zoom: f32,
}

impl VideoEditor {
    pub fn new() -> Self {
        Self {
            project_name: "Untitled Project".to_string(),
            resolution: Resolution::FullHD1080p,
            frame_rate: FrameRate::Fps30,
            video_codec: VideoCodec::H264,
            audio_codec: AudioCodec::AAC,
            tracks: Vec::new(),
            clips: Vec::new(),
            transitions: Vec::new(),
            effects: Vec::new(),
            current_time: 0.0,
            duration: 0.0,
            playing: false,
            zoom: 1.0,
        }
    }

    /// Set project name
    pub fn set_project_name(&mut self, name: String) {
        self.project_name = name;
    }

    /// Set resolution
    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.resolution = resolution;
    }

    /// Set frame rate
    pub fn set_frame_rate(&mut self, frame_rate: FrameRate) {
        self.frame_rate = frame_rate;
    }

    /// Add track
    pub fn add_track(&mut self, track_type: ClipType) -> u32 {
        let track_id = self.tracks.len() as u32;
        let track = Track {
            id: track_id,
            name: format!("Track {}", track_id),
            track_type,
            clips: Vec::new(),
            locked: false,
            visible: true,
            volume: 1.0,
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

    /// Add clip
    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
    }

    /// Remove clip
    pub fn remove_clip(&mut self, clip_id: String) -> Result<(), &'static str> {
        let index = self
            .clips
            .iter()
            .position(|c| c.id == clip_id)
            .ok_or("Clip not found")?;
        self.clips.remove(index);
        Ok(())
    }

    /// Add transition
    pub fn add_transition(&mut self, transition: Transition) {
        self.transitions.push(transition);
    }

    /// Add effect
    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
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
        self.current_time = 0.0;
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
    pub fn get_tracks(&self) -> &[Track] {
        &self.tracks
    }

    /// Get clips
    pub fn get_clips(&self) -> &[Clip] {
        &self.clips
    }

    /// Track count
    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Clip count
    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }
}

/// Global video editor
static mut VIDEO_EDITOR: Option<VideoEditor> = None;

/// Initialize video editor
pub fn init_video_editor() {
    unsafe {
        VIDEO_EDITOR = Some(VideoEditor::new());
    }
}

/// Get video editor
pub fn get_video_editor() -> &'static mut VideoEditor {
    unsafe { VIDEO_EDITOR.as_mut().expect("Video editor not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_editor_creation() {
        let ve = VideoEditor::new();
        assert_eq!(ve.project_name, "Untitled Project");
    }

    #[test]
    fn test_add_track() {
        let mut ve = VideoEditor::new();
        ve.add_track(ClipType::Video);
        assert_eq!(ve.track_count(), 1);
    }

    #[test]
    fn test_play_pause() {
        let mut ve = VideoEditor::new();
        ve.play();
        assert!(ve.playing);
        ve.pause();
        assert!(!ve.playing);
    }
}
