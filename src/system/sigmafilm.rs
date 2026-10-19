//! SigmaFilm - Film and Video Editing System
//!
//! Provides comprehensive film and video editing management inspired by Film Craft.
//! Supports video clips, timeline editing, and media management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoResolution {
    SD480,
    HD720,
    FHD1080,
    QHD1440,
    UHD2160,
    UHD4320,
}

/// Video frame rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    Fps24,
    Fps30,
    Fps60,
    Fps120,
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
    MP3,
    Opus,
    FLAC,
    PCM,
}

/// Media clip
#[derive(Debug, Clone)]
pub struct MediaClip {
    pub id: String,
    pub name: String,
    pub duration: u64, // in milliseconds
    pub start_time: u64,
    pub end_time: u64,
    pub video_codec: Option<VideoCodec>,
    pub audio_codec: Option<AudioCodec>,
    pub resolution: Option<VideoResolution>,
    pub frame_rate: Option<FrameRate>,
}

impl MediaClip {
    pub fn new(id: String, name: String, duration: u64) -> Self {
        Self {
            id,
            name,
            duration,
            start_time: 0,
            end_time: duration,
            video_codec: None,
            audio_codec: None,
            resolution: None,
            frame_rate: None,
        }
    }

    pub fn set_time_range(&mut self, start: u64, end: u64) {
        self.start_time = start;
        self.end_time = end;
    }

    pub fn set_video_codec(&mut self, codec: VideoCodec) {
        self.video_codec = Some(codec);
    }

    pub fn set_audio_codec(&mut self, codec: AudioCodec) {
        self.audio_codec = Some(codec);
    }

    pub fn set_resolution(&mut self, resolution: VideoResolution) {
        self.resolution = Some(resolution);
    }

    pub fn set_frame_rate(&mut self, frame_rate: FrameRate) {
        self.frame_rate = Some(frame_rate);
    }

    pub fn get_trimmed_duration(&self) -> u64 {
        self.end_time - self.start_time
    }
}

/// Timeline track
#[derive(Debug, Clone)]
pub struct TimelineTrack {
    pub id: String,
    pub name: String,
    pub track_type: String, // "video", "audio", "subtitle"
    pub clips: Vec<String>, // clip IDs
    pub locked: bool,
    pub muted: bool,
}

impl TimelineTrack {
    pub fn new(id: String, name: String, track_type: String) -> Self {
        Self {
            id,
            name,
            track_type,
            clips: Vec::new(),
            locked: false,
            muted: false,
        }
    }

    pub fn add_clip(&mut self, clip_id: String) {
        self.clips.push(clip_id);
    }

    pub fn remove_clip(&mut self, index: usize) -> bool {
        if index < self.clips.len() {
            self.clips.remove(index);
            true
        } else {
            false
        }
    }

    pub fn set_locked(&mut self, locked: bool) {
        self.locked = locked;
    }

    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }

    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }
}

/// Video project
#[derive(Debug, Clone)]
pub struct VideoProject {
    pub id: String,
    pub name: String,
    pub clips: BTreeMap<String, MediaClip>,
    pub tracks: BTreeMap<String, TimelineTrack>,
    pub resolution: VideoResolution,
    pub frame_rate: FrameRate,
    pub output_codec: VideoCodec,
}

impl VideoProject {
    pub fn new(
        id: String,
        name: String,
        resolution: VideoResolution,
        frame_rate: FrameRate,
    ) -> Self {
        Self {
            id,
            name,
            clips: BTreeMap::new(),
            tracks: BTreeMap::new(),
            resolution,
            frame_rate,
            output_codec: VideoCodec::H264,
        }
    }

    pub fn add_clip(&mut self, clip: MediaClip) {
        self.clips.insert(clip.id.clone(), clip);
    }

    pub fn remove_clip(&mut self, id: &str) -> bool {
        self.clips.remove(id).is_some()
    }

    pub fn get_clip(&self, id: &str) -> Option<&MediaClip> {
        self.clips.get(id)
    }

    pub fn get_clip_mut(&mut self, id: &str) -> Option<&mut MediaClip> {
        self.clips.get_mut(id)
    }

    pub fn add_track(&mut self, track: TimelineTrack) {
        self.tracks.insert(track.id.clone(), track);
    }

    pub fn remove_track(&mut self, id: &str) -> bool {
        self.tracks.remove(id).is_some()
    }

    pub fn get_track(&self, id: &str) -> Option<&TimelineTrack> {
        self.tracks.get(id)
    }

    pub fn get_track_mut(&mut self, id: &str) -> Option<&mut TimelineTrack> {
        self.tracks.get_mut(id)
    }

    pub fn set_output_codec(&mut self, codec: VideoCodec) {
        self.output_codec = codec;
    }

    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }
}

/// SigmaFilm - Film and video editing system manager
#[derive(Debug)]
pub struct SigmaFilm {
    projects: BTreeMap<String, VideoProject>,
    current_project: Option<String>,
}

impl SigmaFilm {
    pub fn new() -> Self {
        Self {
            projects: BTreeMap::new(),
            current_project: None,
        }
    }

    pub fn add_project(&mut self, project: VideoProject) {
        self.projects.insert(project.id.clone(), project);
    }

    pub fn remove_project(&mut self, id: &str) -> bool {
        if self.current_project.as_ref() == Some(&id.to_string()) {
            self.current_project = None;
        }
        self.projects.remove(id).is_some()
    }

    pub fn get_project(&self, id: &str) -> Option<&VideoProject> {
        self.projects.get(id)
    }

    pub fn get_project_mut(&mut self, id: &str) -> Option<&mut VideoProject> {
        self.projects.get_mut(id)
    }

    pub fn set_current_project(&mut self, id: &str) -> bool {
        if self.projects.contains_key(id) {
            self.current_project = Some(id.to_string());
            true
        } else {
            false
        }
    }

    pub fn get_current_project(&self) -> Option<&VideoProject> {
        self.current_project
            .as_ref()
            .and_then(|id| self.projects.get(id))
    }

    pub fn get_current_project_mut(&mut self) -> Option<&mut VideoProject> {
        self.current_project
            .as_ref()
            .and_then(|id| self.projects.get_mut(id.as_str()))
    }

    pub fn list_projects(&self) -> Vec<&VideoProject> {
        self.projects.values().collect()
    }

    pub fn project_count(&self) -> usize {
        self.projects.len()
    }
}

impl Default for SigmaFilm {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaFilm instance
static mut SIGMAFILM_INSTANCE: Option<SigmaFilm> = None;

/// Initialize the global SigmaFilm instance
pub fn init_sigmafilm() {
    unsafe {
        SIGMAFILM_INSTANCE = Some(SigmaFilm::new());
    }
}

/// Get the global SigmaFilm instance
pub fn get_sigmafilm() -> Option<&'static SigmaFilm> {
    unsafe { SIGMAFILM_INSTANCE.as_ref() }
}

/// Get the global SigmaFilm instance (mutable)
pub fn get_sigmafilm_mut() -> Option<&'static mut SigmaFilm> {
    unsafe { SIGMAFILM_INSTANCE.as_mut() }
}

/// Convenience function to add a project
pub fn add_project(project: VideoProject) -> bool {
    if let Some(film) = get_sigmafilm_mut() {
        film.add_project(project);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_clip_creation() {
        let clip = MediaClip::new("clip1".to_string(), "Video".to_string(), 60000);
        assert_eq!(clip.id, "clip1");
        assert_eq!(clip.duration, 60000);
        assert_eq!(clip.start_time, 0);
        assert_eq!(clip.end_time, 60000);
    }

    #[test]
    fn test_media_clip_modification() {
        let mut clip = MediaClip::new("clip1".to_string(), "Video".to_string(), 60000);
        clip.set_time_range(10000, 50000);
        clip.set_video_codec(VideoCodec::H264);
        clip.set_resolution(VideoResolution::FHD1080);
        assert_eq!(clip.start_time, 10000);
        assert_eq!(clip.end_time, 50000);
        assert_eq!(clip.get_trimmed_duration(), 40000);
    }

    #[test]
    fn test_timeline_track_creation() {
        let track = TimelineTrack::new(
            "track1".to_string(),
            "Video Track".to_string(),
            "video".to_string(),
        );
        assert_eq!(track.id, "track1");
        assert_eq!(track.track_type, "video");
        assert!(!track.locked);
        assert!(!track.muted);
    }

    #[test]
    fn test_timeline_track_management() {
        let mut track = TimelineTrack::new(
            "track1".to_string(),
            "Test".to_string(),
            "video".to_string(),
        );
        track.add_clip("clip1".to_string());
        track.add_clip("clip2".to_string());
        assert_eq!(track.clip_count(), 2);
        assert!(track.remove_clip(0));
        assert_eq!(track.clip_count(), 1);
    }

    #[test]
    fn test_video_project_creation() {
        let project = VideoProject::new(
            "project1".to_string(),
            "My Video".to_string(),
            VideoResolution::FHD1080,
            FrameRate::Fps30,
        );
        assert_eq!(project.id, "project1");
        assert_eq!(project.resolution, VideoResolution::FHD1080);
    }

    #[test]
    fn test_video_project_management() {
        let mut project = VideoProject::new(
            "project1".to_string(),
            "Test".to_string(),
            VideoResolution::FHD1080,
            FrameRate::Fps30,
        );
        let clip = MediaClip::new("clip1".to_string(), "Video".to_string(), 60000);
        project.add_clip(clip);
        assert_eq!(project.clip_count(), 1);
    }

    #[test]
    fn test_sigmafilm_creation() {
        let film = SigmaFilm::new();
        assert_eq!(film.project_count(), 0);
    }

    #[test]
    fn test_sigmafilm_project_management() {
        let mut film = SigmaFilm::new();
        let project = VideoProject::new(
            "project1".to_string(),
            "Test".to_string(),
            VideoResolution::FHD1080,
            FrameRate::Fps30,
        );
        film.add_project(project);
        assert_eq!(film.project_count(), 1);
    }
}
