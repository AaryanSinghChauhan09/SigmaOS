// SPDX-License-Identifier: MIT
// SigmaOS - Mediacraft (Media Player Framework)
// Inspired by VLC Media Player, MPV, Kodi, Plex
// Provides professional media playback, streaming, and library management capabilities

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Media type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
    Image,
    Playlist,
    Stream,
}

/// Audio codec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCodec {
    AAC,
    MP3,
    FLAC,
    Opus,
    Vorbis,
    AC3,
    DTS,
    PCM,
}

/// Video codec
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodec {
    H264,
    H265,
    VP9,
    AV1,
    VP8,
    Theora,
    MPEG2,
    MPEG4,
}

/// Container format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerFormat {
    MP4,
    MKV,
    WebM,
    AVI,
    MOV,
    FLV,
    OGG,
    TS,
    MP3,
    WAV,
    FLAC,
}

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
    pub codec: AudioCodec,
    pub channels: u32,
    pub sample_rate: u32,
    pub enabled: bool,
}

/// Subtitle track
#[derive(Debug, Clone)]
pub struct SubtitleTrack {
    pub id: u32,
    pub language: String,
    pub format: String,
    pub enabled: bool,
}

/// Video track
#[derive(Debug, Clone)]
pub struct VideoTrack {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f32,
    pub codec: VideoCodec,
    pub bitrate: u32,
}

/// Media item
#[derive(Debug, Clone)]
pub struct MediaItem {
    pub id: u32,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: u32, // seconds
    pub file_path: String,
    pub media_type: MediaType,
    pub container: ContainerFormat,
    pub video_tracks: Vec<VideoTrack>,
    pub audio_tracks: Vec<AudioTrack>,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub position: u32, // current position in seconds
}

/// Playlist
#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: u32,
    pub name: String,
    pub items: Vec<u32>, // media item IDs
    pub current_index: usize,
    pub shuffle_mode: ShuffleMode,
    pub repeat_mode: RepeatMode,
}

/// Equalizer band
#[derive(Debug, Clone, Copy)]
pub struct EqualizerBand {
    pub frequency: f32,
    pub gain: f32, // dB
}

/// Equalizer preset
#[derive(Debug, Clone)]
pub struct EqualizerPreset {
    pub name: String,
    pub bands: Vec<EqualizerBand>,
}

/// Audio settings
#[derive(Debug, Clone)]
pub struct AudioSettings {
    pub volume: f32,
    pub muted: bool,
    pub equalizer_enabled: bool,
    pub equalizer_bands: Vec<EqualizerBand>,
    pub current_preset: Option<String>,
}

/// Video settings
#[derive(Debug, Clone)]
pub struct VideoSettings {
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub hue: f32,
    pub gamma: f32,
    pub sharpness: f32,
    pub deinterlace: bool,
    pub hardware_acceleration: bool,
}

/// Stream protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamProtocol {
    HTTP,
    HTTPS,
    RTMP,
    RTSP,
    HLS,
    DASH,
    UDP,
    RTP,
}

/// Stream source
#[derive(Debug, Clone)]
pub struct StreamSource {
    pub url: String,
    pub protocol: StreamProtocol,
    pub buffered: bool,
}

/// Media library
#[derive(Debug, Clone)]
pub struct MediaLibrary {
    pub id: u32,
    pub name: String,
    pub items: BTreeMap<u32, MediaItem>,
    pub playlists: BTreeMap<u32, Playlist>,
}

/// Media player
pub struct MediaPlayer {
    pub current_item: Option<u32>,     // media item ID
    pub current_playlist: Option<u32>, // playlist ID
    pub library: BTreeMap<u32, MediaLibrary>,
    pub playback_state: PlaybackState,
    pub position: u32, // current position in seconds
    pub duration: u32, // total duration in seconds
    pub volume: f32,
    pub speed: f32, // playback speed (1.0 = normal)
    pub audio_settings: AudioSettings,
    pub video_settings: VideoSettings,
    pub active: AtomicBool,
    pub next_item_id: AtomicU32,
    pub next_playlist_id: AtomicU32,
    pub next_library_id: AtomicU32,
}

impl MediaPlayer {
    pub fn new() -> Self {
        Self {
            current_item: None,
            current_playlist: None,
            library: BTreeMap::new(),
            playback_state: PlaybackState::Stopped,
            position: 0,
            duration: 0,
            volume: 1.0,
            speed: 1.0,
            audio_settings: AudioSettings {
                volume: 1.0,
                muted: false,
                equalizer_enabled: false,
                equalizer_bands: vec![
                    EqualizerBand {
                        frequency: 60.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 170.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 310.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 600.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 1000.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 3000.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 6000.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 12000.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 14000.0,
                        gain: 0.0,
                    },
                    EqualizerBand {
                        frequency: 16000.0,
                        gain: 0.0,
                    },
                ],
                current_preset: None,
            },
            video_settings: VideoSettings {
                brightness: 0.0,
                contrast: 0.0,
                saturation: 0.0,
                hue: 0.0,
                gamma: 0.0,
                sharpness: 0.0,
                deinterlace: false,
                hardware_acceleration: true,
            },
            active: AtomicBool::new(false),
            next_item_id: AtomicU32::new(1),
            next_playlist_id: AtomicU32::new(1),
            next_library_id: AtomicU32::new(1),
        }
    }

    /// Initialize media player
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Create library
    pub fn create_library(&mut self, name: String) -> u32 {
        let id = self.next_library_id.fetch_add(1, Ordering::Relaxed);
        let library = MediaLibrary {
            id,
            name,
            items: BTreeMap::new(),
            playlists: BTreeMap::new(),
        };
        self.library.insert(id, library);
        id
    }

    /// Add media item to library
    pub fn add_media_item(
        &mut self,
        library_id: u32,
        title: String,
        file_path: String,
        media_type: MediaType,
        container: ContainerFormat,
        duration: u32,
    ) -> Result<u32, &'static str> {
        let library = self
            .library
            .get_mut(&library_id)
            .ok_or("Library not found")?;
        let id = self.next_item_id.fetch_add(1, Ordering::Relaxed);
        let item = MediaItem {
            id,
            title,
            artist: None,
            album: None,
            duration,
            file_path,
            media_type,
            container,
            video_tracks: Vec::new(),
            audio_tracks: Vec::new(),
            subtitle_tracks: Vec::new(),
            position: 0,
        };
        library.items.insert(id, item);
        Ok(id)
    }

    /// Create playlist
    pub fn create_playlist(&mut self, library_id: u32, name: String) -> Result<u32, &'static str> {
        let library = self
            .library
            .get_mut(&library_id)
            .ok_or("Library not found")?;
        let id = self.next_playlist_id.fetch_add(1, Ordering::Relaxed);
        let playlist = Playlist {
            id,
            name,
            items: Vec::new(),
            current_index: 0,
            shuffle_mode: ShuffleMode::Off,
            repeat_mode: RepeatMode::None,
        };
        library.playlists.insert(id, playlist);
        Ok(id)
    }

    /// Add item to playlist
    pub fn add_to_playlist(
        &mut self,
        library_id: u32,
        playlist_id: u32,
        item_id: u32,
    ) -> Result<(), &'static str> {
        let library = self
            .library
            .get_mut(&library_id)
            .ok_or("Library not found")?;
        let playlist = library
            .playlists
            .get_mut(&playlist_id)
            .ok_or("Playlist not found")?;
        if !playlist.items.contains(&item_id) {
            playlist.items.push(item_id);
        }
        Ok(())
    }

    /// Play media item
    pub fn play(&mut self, library_id: u32, item_id: u32) -> Result<(), &'static str> {
        let library = self.library.get(&library_id).ok_or("Library not found")?;
        let item = library.items.get(&item_id).ok_or("Item not found")?;
        self.current_item = Some(item_id);
        self.duration = item.duration;
        self.position = 0;
        self.playback_state = PlaybackState::Playing;
        Ok(())
    }

    /// Play playlist
    pub fn play_playlist(&mut self, library_id: u32, playlist_id: u32) -> Result<(), &'static str> {
        let library = self.library.get(&library_id).ok_or("Library not found")?;
        let playlist = library
            .playlists
            .get(&playlist_id)
            .ok_or("Playlist not found")?;
        if playlist.items.is_empty() {
            return Err("Playlist is empty");
        }
        self.current_playlist = Some(playlist_id);
        let item_id = playlist.items[playlist.current_index];
        self.play(library_id, item_id)
    }

    /// Pause playback
    pub fn pause(&mut self) {
        if self.playback_state == PlaybackState::Playing {
            self.playback_state = PlaybackState::Paused;
        }
    }

    /// Resume playback
    pub fn resume(&mut self) {
        if self.playback_state == PlaybackState::Paused {
            self.playback_state = PlaybackState::Playing;
        }
    }

    /// Stop playback
    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.position = 0;
    }

    /// Seek to position
    pub fn seek(&mut self, position: u32) {
        self.position = position.min(self.duration);
    }

    /// Set volume
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        self.audio_settings.volume = volume;
    }

    /// Mute/unmute
    pub fn set_mute(&mut self, muted: bool) {
        self.audio_settings.muted = muted;
    }

    /// Set playback speed
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed.clamp(0.25, 4.0);
    }

    /// Next track
    pub fn next(&mut self) -> Result<(), &'static str> {
        if let Some(playlist_id) = self.current_playlist {
            if let Some(library_id) = self.get_current_library_id() {
                let library = self
                    .library
                    .get_mut(&library_id)
                    .ok_or("Library not found")?;
                let playlist = library
                    .playlists
                    .get_mut(&playlist_id)
                    .ok_or("Playlist not found")?;

                match playlist.repeat_mode {
                    RepeatMode::One => {
                        // Replay current
                    }
                    _ => {
                        playlist.current_index =
                            (playlist.current_index + 1) % playlist.items.len();
                        let item_id = playlist.items[playlist.current_index];
                        self.play(library_id, item_id)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Previous track
    pub fn previous(&mut self) -> Result<(), &'static str> {
        if let Some(playlist_id) = self.current_playlist {
            if let Some(library_id) = self.get_current_library_id() {
                let library = self
                    .library
                    .get_mut(&library_id)
                    .ok_or("Library not found")?;
                let playlist = library
                    .playlists
                    .get_mut(&playlist_id)
                    .ok_or("Playlist not found")?;

                if playlist.current_index > 0 {
                    playlist.current_index -= 1;
                } else {
                    playlist.current_index = playlist.items.len() - 1;
                }
                let item_id = playlist.items[playlist.current_index];
                self.play(library_id, item_id)?;
            }
        }
        Ok(())
    }

    /// Set repeat mode
    pub fn set_repeat_mode(&mut self, repeat_mode: RepeatMode) -> Result<(), &'static str> {
        if let Some(playlist_id) = self.current_playlist {
            if let Some(library_id) = self.get_current_library_id() {
                let library = self
                    .library
                    .get_mut(&library_id)
                    .ok_or("Library not found")?;
                let playlist = library
                    .playlists
                    .get_mut(&playlist_id)
                    .ok_or("Playlist not found")?;
                playlist.repeat_mode = repeat_mode;
            }
        }
        Ok(())
    }

    /// Toggle shuffle
    pub fn toggle_shuffle(&mut self) -> Result<(), &'static str> {
        if let Some(playlist_id) = self.current_playlist {
            if let Some(library_id) = self.get_current_library_id() {
                let library = self
                    .library
                    .get_mut(&library_id)
                    .ok_or("Library not found")?;
                let playlist = library
                    .playlists
                    .get_mut(&playlist_id)
                    .ok_or("Playlist not found")?;
                playlist.shuffle_mode = match playlist.shuffle_mode {
                    ShuffleMode::Off => ShuffleMode::On,
                    ShuffleMode::On => ShuffleMode::Off,
                };
            }
        }
        Ok(())
    }

    /// Set equalizer band
    pub fn set_equalizer_band(&mut self, band_index: usize, gain: f32) -> Result<(), &'static str> {
        if band_index < self.audio_settings.equalizer_bands.len() {
            self.audio_settings.equalizer_bands[band_index].gain = gain.clamp(-20.0, 20.0);
            Ok(())
        } else {
            Err("Invalid band index")
        }
    }

    /// Enable/disable equalizer
    pub fn set_equalizer_enabled(&mut self, enabled: bool) {
        self.audio_settings.equalizer_enabled = enabled;
    }

    /// Set video brightness
    pub fn set_brightness(&mut self, brightness: f32) {
        self.video_settings.brightness = brightness.clamp(-1.0, 1.0);
    }

    /// Set video contrast
    pub fn set_contrast(&mut self, contrast: f32) {
        self.video_settings.contrast = contrast.clamp(-1.0, 1.0);
    }

    /// Set video saturation
    pub fn set_saturation(&mut self, saturation: f32) {
        self.video_settings.saturation = saturation.clamp(0.0, 3.0);
    }

    /// Play stream
    pub fn play_stream(
        &mut self,
        url: String,
        protocol: StreamProtocol,
    ) -> Result<u32, &'static str> {
        let library_id = self.create_library("Streams".to_string());
        let item_id = self.add_media_item(
            library_id,
            url.clone(),
            url.clone(),
            MediaType::Stream,
            ContainerFormat::TS,
            0,
        )?;
        self.play(library_id, item_id)?;
        Ok(item_id)
    }

    /// Get current library ID
    fn get_current_library_id(&self) -> Option<u32> {
        for (id, library) in &self.library {
            if let Some(item_id) = self.current_item {
                if library.items.contains_key(&item_id) {
                    return Some(*id);
                }
            }
        }
        None
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for MediaPlayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Global mediacraft
static mut MEDIACRAFT: Option<MediaPlayer> = None;

/// Initialize mediacraft
pub fn init_mediacraft() {
    unsafe {
        MEDIACRAFT = Some(MediaPlayer::new());
    }
}

/// Get global mediacraft
pub fn get_mediacraft() -> Option<&'static mut MediaPlayer> {
    unsafe { MEDIACRAFT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mediacraft_init() {
        let mut player = MediaPlayer::new();
        player.init();
        assert!(player.is_active());
    }

    #[test]
    fn test_create_library() {
        let mut player = MediaPlayer::new();
        player.init();
        let library_id = player.create_library("Music".to_string());
        assert!(player.library.contains_key(&library_id));
    }

    #[test]
    fn test_add_media_item() {
        let mut player = MediaPlayer::new();
        player.init();
        let library_id = player.create_library("Music".to_string());
        let item_id = player
            .add_media_item(
                library_id,
                "Song.mp3".to_string(),
                "/music/song.mp3".to_string(),
                MediaType::Audio,
                ContainerFormat::MP3,
                180,
            )
            .unwrap();
        assert!(player.library[&library_id].items.contains_key(&item_id));
    }

    #[test]
    fn test_playlist() {
        let mut player = MediaPlayer::new();
        player.init();
        let library_id = player.create_library("Music".to_string());
        let item_id = player
            .add_media_item(
                library_id,
                "Song.mp3".to_string(),
                "/music/song.mp3".to_string(),
                MediaType::Audio,
                ContainerFormat::MP3,
                180,
            )
            .unwrap();
        let playlist_id = player
            .create_playlist(library_id, "Favorites".to_string())
            .unwrap();
        player
            .add_to_playlist(library_id, playlist_id, item_id)
            .unwrap();
        assert_eq!(
            player.library[&library_id].playlists[&playlist_id]
                .items
                .len(),
            1
        );
    }

    #[test]
    fn test_playback() {
        let mut player = MediaPlayer::new();
        player.init();
        let library_id = player.create_library("Music".to_string());
        let item_id = player
            .add_media_item(
                library_id,
                "Song.mp3".to_string(),
                "/music/song.mp3".to_string(),
                MediaType::Audio,
                ContainerFormat::MP3,
                180,
            )
            .unwrap();
        player.play(library_id, item_id).unwrap();
        assert_eq!(player.playback_state, PlaybackState::Playing);
        assert_eq!(player.current_item, Some(item_id));
    }

    #[test]
    fn test_pause_resume() {
        let mut player = MediaPlayer::new();
        player.init();
        let library_id = player.create_library("Music".to_string());
        let item_id = player
            .add_media_item(
                library_id,
                "Song.mp3".to_string(),
                "/music/song.mp3".to_string(),
                MediaType::Audio,
                ContainerFormat::MP3,
                180,
            )
            .unwrap();
        player.play(library_id, item_id).unwrap();
        player.pause();
        assert_eq!(player.playback_state, PlaybackState::Paused);
        player.resume();
        assert_eq!(player.playback_state, PlaybackState::Playing);
    }

    #[test]
    fn test_seek() {
        let mut player = MediaPlayer::new();
        player.init();
        let library_id = player.create_library("Music".to_string());
        let item_id = player
            .add_media_item(
                library_id,
                "Song.mp3".to_string(),
                "/music/song.mp3".to_string(),
                MediaType::Audio,
                ContainerFormat::MP3,
                180,
            )
            .unwrap();
        player.play(library_id, item_id).unwrap();
        player.seek(90);
        assert_eq!(player.position, 90);
    }

    #[test]
    fn test_volume() {
        let mut player = MediaPlayer::new();
        player.init();
        player.set_volume(0.5);
        assert_eq!(player.volume, 0.5);
        assert_eq!(player.audio_settings.volume, 0.5);
    }

    #[test]
    fn test_speed() {
        let mut player = MediaPlayer::new();
        player.init();
        player.set_speed(2.0);
        assert_eq!(player.speed, 2.0);
    }

    #[test]
    fn test_repeat_mode() {
        let mut player = MediaPlayer::new();
        player.init();
        let library_id = player.create_library("Music".to_string());
        let item_id = player
            .add_media_item(
                library_id,
                "Song.mp3".to_string(),
                "/music/song.mp3".to_string(),
                MediaType::Audio,
                ContainerFormat::MP3,
                180,
            )
            .unwrap();
        let playlist_id = player
            .create_playlist(library_id, "Favorites".to_string())
            .unwrap();
        player
            .add_to_playlist(library_id, playlist_id, item_id)
            .unwrap();
        player.play_playlist(library_id, playlist_id).unwrap();
        player.set_repeat_mode(RepeatMode::All).unwrap();
        assert_eq!(
            player.library[&library_id].playlists[&playlist_id].repeat_mode,
            RepeatMode::All
        );
    }

    #[test]
    fn test_equalizer() {
        let mut player = MediaPlayer::new();
        player.init();
        player.set_equalizer_band(0, 5.0).unwrap();
        assert_eq!(player.audio_settings.equalizer_bands[0].gain, 5.0);
        player.set_equalizer_enabled(true);
        assert!(player.audio_settings.equalizer_enabled);
    }

    #[test]
    fn test_video_settings() {
        let mut player = MediaPlayer::new();
        player.init();
        player.set_brightness(0.5);
        assert_eq!(player.video_settings.brightness, 0.5);
        player.set_contrast(0.3);
        assert_eq!(player.video_settings.contrast, 0.3);
        player.set_saturation(1.5);
        assert_eq!(player.video_settings.saturation, 1.5);
    }

    #[test]
    fn test_stream() {
        let mut player = MediaPlayer::new();
        player.init();
        let item_id = player
            .play_stream(
                "https://example.com/stream.m3u8".to_string(),
                StreamProtocol::HLS,
            )
            .unwrap();
        assert!(player.current_item == Some(item_id));
        assert_eq!(player.playback_state, PlaybackState::Playing);
    }
}
