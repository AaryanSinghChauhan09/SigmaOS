// SPDX-License-Identifier: MIT
// SigmaOS - Media Player
// Inspired by VLC Media Player and MPV
// Provides media playback functionality

#![no_std]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Buffering,
    Error,
}

/// Repeat mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepeatMode {
    None,
    All,
    One,
}

/// Shuffle mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShuffleMode {
    Off,
    On,
}

/// Audio track
#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: u32,
    pub language: String,
    pub codec: String,
    pub channels: u32,
    pub sample_rate: u32,
}

/// Subtitle track
#[derive(Debug, Clone)]
pub struct SubtitleTrack {
    pub id: u32,
    pub language: String,
    pub format: String,
}

/// Video track
#[derive(Debug, Clone)]
pub struct VideoTrack {
    pub id: u32,
    pub resolution: String,
    pub frame_rate: f32,
    pub codec: String,
    pub bitrate: u32,
}

/// Media item
#[derive(Debug, Clone)]
pub struct MediaItem {
    pub id: u32,
    pub path: String,
    pub title: String,
    pub duration: f32,
    pub position: f32,
    pub audio_tracks: Vec<AudioTrack>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub video_tracks: Vec<VideoTrack>,
}

/// Playlist
#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: u32,
    pub name: String,
    pub items: Vec<MediaItem>,
    pub current_index: usize,
}

/// Equalizer band
#[derive(Debug, Clone)]
pub struct EqualizerBand {
    pub frequency: f32,
    pub gain: f32,
}

/// Media player
pub struct MediaPlayer {
    pub current_item: Option<MediaItem>,
    pub playlists: Vec<Playlist>,
    pub playback_state: PlaybackState,
    pub volume: f32,
    pub mute: bool,
    pub speed: f32,
    pub repeat_mode: RepeatMode,
    pub shuffle_mode: ShuffleMode,
    pub equalizer: Vec<EqualizerBand>,
    pub audio_track_id: Option<u32>,
    pub subtitle_track_id: Option<u32>,
    pub video_track_id: Option<u32>,
    pub play_queue: VecDeque<u32>,
    pub played_history: Vec<u32>,
}

impl MediaPlayer {
    pub fn new() -> Self {
        let mut equalizer = Vec::new();
        let frequencies = [
            60.0, 170.0, 310.0, 600.0, 1000.0, 3000.0, 6000.0, 12000.0, 14000.0, 16000.0,
        ];
        for freq in frequencies {
            equalizer.push(EqualizerBand {
                frequency: freq,
                gain: 0.0,
            });
        }

        Self {
            current_item: None,
            playlists: Vec::new(),
            playback_state: PlaybackState::Stopped,
            volume: 1.0,
            mute: false,
            speed: 1.0,
            repeat_mode: RepeatMode::None,
            shuffle_mode: ShuffleMode::Off,
            equalizer,
            audio_track_id: None,
            subtitle_track_id: None,
            video_track_id: None,
            play_queue: VecDeque::new(),
            played_history: Vec::new(),
        }
    }

    /// Open media file
    pub fn open(&mut self, path: String) -> Result<(), &'static str> {
        let item = MediaItem {
            id: 0,
            path: path.clone(),
            title: path.split('/').last().unwrap_or("Unknown").to_string(),
            duration: 0.0,
            position: 0.0,
            audio_tracks: Vec::new(),
            subtitle_tracks: Vec::new(),
            video_tracks: Vec::new(),
        };
        self.current_item = Some(item);
        self.playback_state = PlaybackState::Playing;
        Ok(())
    }

    /// Play
    pub fn play(&mut self) {
        if self.current_item.is_some() {
            self.playback_state = PlaybackState::Playing;
        }
    }

    /// Pause
    pub fn pause(&mut self) {
        self.playback_state = PlaybackState::Paused;
    }

    /// Stop
    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        if let Some(ref mut item) = self.current_item {
            item.position = 0.0;
        }
    }

    /// Seek
    pub fn seek(&mut self, position: f32) {
        if let Some(ref mut item) = self.current_item {
            item.position = position.clamp(0.0, item.duration);
        }
    }

    /// Set volume
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    /// Toggle mute
    pub fn toggle_mute(&mut self) {
        self.mute = !self.mute;
    }

    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.25, 4.0);
    }

    /// Next track
    pub fn next(&mut self) {
        if let Some(ref mut item) = self.current_item {
            // Simplified - would advance to next item in playlist
            item.position = 0.0;
        }
    }

    /// Previous track
    pub fn previous(&mut self) {
        if let Some(ref mut item) = self.current_item {
            // Simplified - would go to previous item in playlist
            item.position = 0.0;
        }
    }

    /// Set repeat mode
    pub fn set_repeat_mode(&mut self, mode: RepeatMode) {
        self.repeat_mode = mode;
    }

    /// Set shuffle mode
    pub fn set_shuffle_mode(&mut self, mode: ShuffleMode) {
        self.shuffle_mode = mode;
    }

    /// Set equalizer band
    pub fn set_equalizer_band(&mut self, index: usize, gain: f32) {
        if index < self.equalizer.len() {
            self.equalizer[index].gain = gain.clamp(-20.0, 20.0);
        }
    }

    /// Set audio track
    pub fn set_audio_track(&mut self, id: u32) {
        self.audio_track_id = Some(id);
    }

    /// Set subtitle track
    pub fn set_subtitle_track(&mut self, id: u32) {
        self.subtitle_track_id = Some(id);
    }

    /// Set video track
    pub fn set_video_track(&mut self, id: u32) {
        self.video_track_id = Some(id);
    }

    /// Get current item
    pub fn get_current_item(&self) -> Option<&MediaItem> {
        self.current_item.as_ref()
    }

    /// Create playlist
    pub fn create_playlist(&mut self, name: String) -> u32 {
        let id = self.playlists.len() as u32;
        let playlist = Playlist {
            id,
            name,
            items: Vec::new(),
            current_index: 0,
        };
        self.playlists.push(playlist);
        id
    }

    /// Add to playlist
    pub fn add_to_playlist(
        &mut self,
        playlist_id: u32,
        item: MediaItem,
    ) -> Result<(), &'static str> {
        let playlist = self
            .playlists
            .get_mut(playlist_id as usize)
            .ok_or("Playlist not found")?;
        playlist.items.push(item);
        Ok(())
    }

    /// Get current position
    pub fn get_position(&self) -> f32 {
        self.current_item
            .as_ref()
            .map(|i| i.position)
            .unwrap_or(0.0)
    }

    /// Get duration
    pub fn get_duration(&self) -> f32 {
        self.current_item
            .as_ref()
            .map(|i| i.duration)
            .unwrap_or(0.0)
    }
}

/// Global media player
static mut MEDIA_PLAYER: Option<MediaPlayer> = None;

/// Initialize media player
pub fn init_media_player() {
    unsafe {
        MEDIA_PLAYER = Some(MediaPlayer::new());
    }
}

/// Get media player
pub fn get_media_player() -> &'static mut MediaPlayer {
    unsafe { MEDIA_PLAYER.as_mut().expect("Media player not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_player_creation() {
        let player = MediaPlayer::new();
        assert_eq!(player.playback_state, PlaybackState::Stopped);
    }

    #[test]
    fn test_open_media() {
        let mut player = MediaPlayer::new();
        player.open("/path/to/media.mp4".to_string()).unwrap();
        assert!(player.get_current_item().is_some());
    }

    #[test]
    fn test_play_pause() {
        let mut player = MediaPlayer::new();
        player.open("/path/to/media.mp4".to_string()).unwrap();
        player.play();
        assert_eq!(player.playback_state, PlaybackState::Playing);
        player.pause();
        assert_eq!(player.playback_state, PlaybackState::Paused);
    }

    #[test]
    fn test_volume() {
        let mut player = MediaPlayer::new();
        player.set_volume(0.5);
        assert_eq!(player.volume, 0.5);
    }

    #[test]
    fn test_mute() {
        let mut player = MediaPlayer::new();
        player.toggle_mute();
        assert!(player.mute);
    }

    #[test]
    fn test_speed() {
        let mut player = MediaPlayer::new();
        player.set_speed(2.0);
        assert_eq!(player.speed, 2.0);
    }

    #[test]
    fn test_create_playlist() {
        let mut player = MediaPlayer::new();
        let id = player.create_playlist("My Playlist".to_string());
        assert_eq!(player.playlists.len(), 1);
    }
}
