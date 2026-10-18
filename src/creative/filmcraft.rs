// SPDX-License-Identifier: MIT
// SigmaOS - Filmcraft (Video Editing Framework)
// Inspired by ArtCraft FilmCraft, Blender, Kdenlive, DaVinci Resolve
// Provides professional video editing, color grading, and sound design capabilities

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
    DNxHD,
    CineForm,
}

/// Audio codec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCodec {
    AAC,
    MP3,
    FLAC,
    PCM,
    Opus,
    Vorbis,
}

/// Video resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    SD480,
    HD720,
    HD1080,
    UHD2160,
    UHD4320,
    Custom(u32, u32),
}

/// Frame rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    FPS24,
    FPS25,
    FPS30,
    FPS48,
    FPS50,
    FPS60,
    FPS120,
    FPS240,
    Custom(u32),
}

/// Clip on a track
#[derive(Debug, Clone)]
pub struct Clip {
    pub id: u32,
    pub name: String,
    pub start_time: u32, // position on timeline in frames
    pub duration: u32,   // duration in frames
    pub in_point: u32,   // in point in frames
    pub out_point: u32,  // out point in frames
    pub speed: f32,      // playback speed (1.0 = normal)
    pub reversed: bool,  // whether clip is reversed
}

/// Track type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackType {
    Video,
    Audio,
    Subtitle,
    Effects,
}

/// Video track
#[derive(Debug, Clone)]
pub struct VideoTrack {
    pub id: u32,
    pub name: String,
    pub track_type: TrackType,
    pub clips: Vec<Clip>,
    pub locked: bool,
    pub muted: bool,
    pub solo: bool,
}

/// Transition type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Cut,
    Dissolve,
    Fade,
    Wipe,
    Slide,
    Zoom,
    Iris,
}

/// Transition between clips
#[derive(Debug, Clone)]
pub struct Transition {
    pub id: u32,
    pub transition_type: TransitionType,
    pub duration: u32, // in frames
    pub from_clip_id: u32,
    pub to_clip_id: u32,
}

/// Color wheel values
#[derive(Debug, Clone, Copy)]
pub struct ColorWheel {
    pub hue: f32,
    pub saturation: f32,
    pub brightness: f32,
}

/// Color grading state
#[derive(Debug, Clone)]
pub struct ColorGrading {
    pub lift: ColorWheel,
    pub gamma: ColorWheel,
    pub gain: ColorWheel,
    pub offset: ColorWheel,
    pub saturation: f32,
    pub contrast: f32,
}

/// Audio channel
#[derive(Debug, Clone)]
pub struct AudioChannel {
    pub id: u32,
    pub name: String,
    pub volume: f32,
    pub pan: f32, // -1.0 (left) to 1.0 (right)
    pub muted: bool,
    pub solo: bool,
}

/// Audio mixer
#[derive(Debug, Clone)]
pub struct AudioMixer {
    pub channels: Vec<AudioChannel>,
    pub master_volume: f32,
}

/// Export settings
#[derive(Debug, Clone)]
pub struct ExportSettings {
    pub video_codec: VideoCodec,
    pub audio_codec: AudioCodec,
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub bitrate: u32,      // in kbps
    pub container: String, // e.g., "mp4", "mov", "mkv"
}

/// Timeline
#[derive(Debug)]
pub struct Timeline {
    pub tracks: Vec<VideoTrack>,
    pub transitions: Vec<Transition>,
    pub current_time: u32,
    pub duration: u32,
    pub zoom: f32,
    pub snap_enabled: bool,
}

/// Video effect
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    FadeIn,
    FadeOut,
    Speed,
    Reverse,
    ColorGrading,
    Blur,
    Sharpen,
    ChromaKey,
}

/// Video editor
pub struct VideoEditor {
    pub project: Option<String>,
    pub timeline: Timeline,
    pub color_grading: ColorGrading,
    pub audio_mixer: AudioMixer,
    pub export_settings: ExportSettings,
    pub active: bool,
    pub hardware_acceleration: bool,
    pub proxy_enabled: bool,
}

impl VideoEditor {
    pub fn new() -> Self {
        Self {
            project: None,
            timeline: Timeline {
                tracks: Vec::new(),
                transitions: Vec::new(),
                current_time: 0,
                duration: 0,
                zoom: 1.0,
                snap_enabled: true,
            },
            color_grading: ColorGrading {
                lift: ColorWheel {
                    hue: 0.0,
                    saturation: 0.0,
                    brightness: 0.0,
                },
                gamma: ColorWheel {
                    hue: 0.0,
                    saturation: 0.0,
                    brightness: 0.0,
                },
                gain: ColorWheel {
                    hue: 0.0,
                    saturation: 0.0,
                    brightness: 0.0,
                },
                offset: ColorWheel {
                    hue: 0.0,
                    saturation: 0.0,
                    brightness: 0.0,
                },
                saturation: 1.0,
                contrast: 1.0,
            },
            audio_mixer: AudioMixer {
                channels: Vec::new(),
                master_volume: 1.0,
            },
            export_settings: ExportSettings {
                video_codec: VideoCodec::H264,
                audio_codec: AudioCodec::AAC,
                resolution: Resolution::HD1080,
                frame_rate: FrameRate::FPS30,
                bitrate: 5000,
                container: "mp4".to_string(),
            },
            active: false,
            hardware_acceleration: true,
            proxy_enabled: false,
        }
    }

    /// Initialize video editor
    pub fn init(&mut self) {
        self.active = true;
    }

    /// Create project
    pub fn create_project(&mut self, name: String) {
        self.project = Some(name);
    }

    /// Add track
    pub fn add_track(&mut self, name: String, track_type: TrackType) -> u32 {
        let id = self.timeline.tracks.len() as u32;
        let track = VideoTrack {
            id,
            name,
            track_type,
            clips: Vec::new(),
            locked: false,
            muted: false,
            solo: false,
        };
        self.timeline.tracks.push(track);
        id
    }

    /// Add clip to track
    pub fn add_clip(
        &mut self,
        track_id: u32,
        name: String,
        start_time: u32,
        duration: u32,
    ) -> Result<u32, &'static str> {
        let track = self
            .timeline
            .tracks
            .get_mut(track_id as usize)
            .ok_or("Track not found")?;
        let clip_id = track.clips.len() as u32;
        let clip = Clip {
            id: clip_id,
            name,
            start_time,
            duration,
            in_point: 0,
            out_point: duration,
            speed: 1.0,
            reversed: false,
        };
        track.clips.push(clip);
        Ok(clip_id)
    }

    /// Add transition
    pub fn add_transition(
        &mut self,
        transition_type: TransitionType,
        duration: u32,
        from_clip_id: u32,
        to_clip_id: u32,
    ) -> u32 {
        let id = self.timeline.transitions.len() as u32;
        let transition = Transition {
            id,
            transition_type,
            duration,
            from_clip_id,
            to_clip_id,
        };
        self.timeline.transitions.push(transition);
        id
    }

    /// Lock track
    pub fn lock_track(&mut self, track_id: u32, locked: bool) -> Result<(), &'static str> {
        let track = self
            .timeline
            .tracks
            .get_mut(track_id as usize)
            .ok_or("Track not found")?;
        track.locked = locked;
        Ok(())
    }

    /// Mute track
    pub fn mute_track(&mut self, track_id: u32, muted: bool) -> Result<(), &'static str> {
        let track = self
            .timeline
            .tracks
            .get_mut(track_id as usize)
            .ok_or("Track not found")?;
        track.muted = muted;
        Ok(())
    }

    /// Set clip speed
    pub fn set_clip_speed(
        &mut self,
        track_id: u32,
        clip_id: u32,
        speed: f32,
    ) -> Result<(), &'static str> {
        let track = self
            .timeline
            .tracks
            .get_mut(track_id as usize)
            .ok_or("Track not found")?;
        let clip = track
            .clips
            .get_mut(clip_id as usize)
            .ok_or("Clip not found")?;
        clip.speed = speed;
        Ok(())
    }

    /// Reverse clip
    pub fn reverse_clip(
        &mut self,
        track_id: u32,
        clip_id: u32,
        reversed: bool,
    ) -> Result<(), &'static str> {
        let track = self
            .timeline
            .tracks
            .get_mut(track_id as usize)
            .ok_or("Track not found")?;
        let clip = track
            .clips
            .get_mut(clip_id as usize)
            .ok_or("Clip not found")?;
        clip.reversed = reversed;
        Ok(())
    }

    /// Set color grading
    pub fn set_color_grading(&mut self, lift: ColorWheel, gamma: ColorWheel, gain: ColorWheel) {
        self.color_grading.lift = lift;
        self.color_grading.gamma = gamma;
        self.color_grading.gain = gain;
    }

    /// Add audio channel
    pub fn add_audio_channel(&mut self, name: String) -> u32 {
        let id = self.audio_mixer.channels.len() as u32;
        let channel = AudioChannel {
            id,
            name,
            volume: 1.0,
            pan: 0.0,
            muted: false,
            solo: false,
        };
        self.audio_mixer.channels.push(channel);
        id
    }

    /// Set channel volume
    pub fn set_channel_volume(&mut self, channel_id: u32, volume: f32) -> Result<(), &'static str> {
        let channel = self
            .audio_mixer
            .channels
            .get_mut(channel_id as usize)
            .ok_or("Channel not found")?;
        channel.volume = volume.clamp(0.0, 1.0);
        Ok(())
    }

    /// Set export settings
    pub fn set_export_settings(
        &mut self,
        video_codec: VideoCodec,
        audio_codec: AudioCodec,
        resolution: Resolution,
        frame_rate: FrameRate,
    ) {
        self.export_settings.video_codec = video_codec;
        self.export_settings.audio_codec = audio_codec;
        self.export_settings.resolution = resolution;
        self.export_settings.frame_rate = frame_rate;
    }

    /// Set current time
    pub fn set_current_time(&mut self, time: u32) {
        self.timeline.current_time = time;
    }

    /// Get current time
    pub fn get_current_time(&self) -> u32 {
        self.timeline.current_time
    }

    /// Set timeline zoom
    pub fn set_zoom(&mut self, zoom: f32) {
        self.timeline.zoom = zoom.clamp(0.1, 10.0);
    }

    /// Toggle snapping
    pub fn toggle_snapping(&mut self) {
        self.timeline.snap_enabled = !self.timeline.snap_enabled;
    }

    /// Toggle proxy media rendering
    pub fn toggle_proxy(&mut self, enabled: bool) -> bool {
        self.proxy_enabled = enabled;
        self.proxy_enabled
    }

    /// Set active multicam camera angle on a clip
    pub fn set_multicam_angle(
        &mut self,
        track_id: u32,
        clip_id: u32,
        angle: u32,
    ) -> Result<(), &'static str> {
        let track = self
            .timeline
            .tracks
            .get_mut(track_id as usize)
            .ok_or("Track not found")?;
        let clip = track
            .clips
            .get_mut(clip_id as usize)
            .ok_or("Clip not found")?;
        clip.in_point = angle * 10; // Angle offset representation
        Ok(())
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Apply effect
    pub fn apply_effect(&mut self, track_id: u32, effect: Effect) -> Result<(), &'static str> {
        self.timeline
            .tracks
            .get(track_id as usize)
            .ok_or("Track not found")?;
        Ok(())
    }
}

impl Default for VideoEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Global filmcraft
static mut FILMCRAFT: Option<VideoEditor> = None;

/// Initialize filmcraft
pub fn init_filmcraft() {
    unsafe {
        FILMCRAFT = Some(VideoEditor::new());
    }
}

/// Get global filmcraft
pub fn get_filmcraft() -> Option<&'static mut VideoEditor> {
    unsafe { FILMCRAFT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filmcraft_init() {
        let mut editor = VideoEditor::new();
        editor.init();
        assert!(editor.is_active());
    }

    #[test]
    fn test_create_project() {
        let mut editor = VideoEditor::new();
        editor.init();
        editor.create_project("Test Project".to_string());
        assert!(editor.project.is_some());
    }

    #[test]
    fn test_add_track() {
        let mut editor = VideoEditor::new();
        editor.init();
        let track_id = editor.add_track("Video Track".to_string(), TrackType::Video);
        assert_eq!(editor.timeline.tracks.len(), 1);
        assert_eq!(track_id, 0);
    }

    #[test]
    fn test_add_clip() {
        let mut editor = VideoEditor::new();
        editor.init();
        let track_id = editor.add_track("Video Track".to_string(), TrackType::Video);
        let clip_id = editor
            .add_clip(track_id, "Clip 1".to_string(), 0, 300)
            .unwrap();
        assert_eq!(clip_id, 0);
        assert_eq!(editor.timeline.tracks[track_id as usize].clips.len(), 1);
    }

    #[test]
    fn test_clip_speed() {
        let mut editor = VideoEditor::new();
        editor.init();
        let track_id = editor.add_track("Video Track".to_string(), TrackType::Video);
        let clip_id = editor
            .add_clip(track_id, "Clip 1".to_string(), 0, 300)
            .unwrap();
        editor.set_clip_speed(track_id, clip_id, 2.0).unwrap();
        assert_eq!(
            editor.timeline.tracks[track_id as usize].clips[clip_id as usize].speed,
            2.0
        );
    }

    #[test]
    fn test_clip_reverse() {
        let mut editor = VideoEditor::new();
        editor.init();
        let track_id = editor.add_track("Video Track".to_string(), TrackType::Video);
        let clip_id = editor
            .add_clip(track_id, "Clip 1".to_string(), 0, 300)
            .unwrap();
        editor.reverse_clip(track_id, clip_id, true).unwrap();
        assert!(editor.timeline.tracks[track_id as usize].clips[clip_id as usize].reversed);
    }

    #[test]
    fn test_track_lock_mute() {
        let mut editor = VideoEditor::new();
        editor.init();
        let track_id = editor.add_track("Video Track".to_string(), TrackType::Video);
        editor.lock_track(track_id, true).unwrap();
        assert!(editor.timeline.tracks[track_id as usize].locked);
        editor.mute_track(track_id, true).unwrap();
        assert!(editor.timeline.tracks[track_id as usize].muted);
    }

    #[test]
    fn test_add_transition() {
        let mut editor = VideoEditor::new();
        editor.init();
        let transition_id = editor.add_transition(TransitionType::Dissolve, 30, 0, 1);
        assert_eq!(editor.timeline.transitions.len(), 1);
        assert_eq!(transition_id, 0);
    }

    #[test]
    fn test_color_grading() {
        let mut editor = VideoEditor::new();
        editor.init();
        let lift = ColorWheel {
            hue: 0.1,
            saturation: 0.2,
            brightness: 0.3,
        };
        let gamma = ColorWheel {
            hue: 0.0,
            saturation: 0.0,
            brightness: 0.0,
        };
        let gain = ColorWheel {
            hue: -0.1,
            saturation: 0.1,
            brightness: 0.1,
        };
        editor.set_color_grading(lift, gamma, gain);
        assert_eq!(editor.color_grading.lift.hue, 0.1);
    }

    #[test]
    fn test_audio_mixer() {
        let mut editor = VideoEditor::new();
        editor.init();
        let channel_id = editor.add_audio_channel("Audio 1".to_string());
        assert_eq!(editor.audio_mixer.channels.len(), 1);
        editor.set_channel_volume(channel_id, 0.8).unwrap();
        assert_eq!(editor.audio_mixer.channels[channel_id as usize].volume, 0.8);
    }

    #[test]
    fn test_export_settings() {
        let mut editor = VideoEditor::new();
        editor.init();
        editor.set_export_settings(
            VideoCodec::H265,
            AudioCodec::AAC,
            Resolution::UHD2160,
            FrameRate::FPS60,
        );
        assert_eq!(editor.export_settings.video_codec, VideoCodec::H265);
        assert_eq!(editor.export_settings.resolution, Resolution::UHD2160);
        assert_eq!(editor.export_settings.frame_rate, FrameRate::FPS60);
    }

    #[test]
    fn test_zoom_and_snapping() {
        let mut editor = VideoEditor::new();
        editor.init();
        editor.set_zoom(2.0);
        assert_eq!(editor.timeline.zoom, 2.0);
        editor.toggle_snapping();
        assert!(!editor.timeline.snap_enabled);
    }

    #[test]
    fn test_hardware_acceleration() {
        let editor = VideoEditor::new();
        assert!(editor.hardware_acceleration);
        assert!(!editor.proxy_enabled);
    }
}
