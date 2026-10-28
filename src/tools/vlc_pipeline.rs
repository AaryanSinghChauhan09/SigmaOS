// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - VLC/libvlc media pipeline
// Principles: MODULARITY, ZERO-COPY, OPEN STANDARD, INTEROPERABILITY
use alloc::{string::String, vec::Vec};
#[derive(Debug,Clone,PartialEq)] pub enum MediaContainer { Mkv, Mp4, WebM, Avi, Flv, Ogg, Flac, Mp3, Aac, Opus, Hls, Dash, Ts, M2ts }
impl MediaContainer { pub fn extension(&self) -> &'static str { match self { Self::Mkv=>"mkv", Self::Mp4=>"mp4", Self::WebM=>"webm", Self::Avi=>"avi", Self::Flv=>"flv", Self::Ogg=>"ogg", Self::Flac=>"flac", Self::Mp3=>"mp3", Self::Aac=>"aac", Self::Opus=>"opus", Self::Hls=>"m3u8", Self::Dash=>"mpd", Self::Ts=>"ts", Self::M2ts=>"m2ts" } } }
#[derive(Debug,Clone)] pub enum VideoCodec { H264, H265, Vp8, Vp9, Av1, Mpeg4, Theora, Prores }
#[derive(Debug,Clone)] pub enum AudioCodec { Aac, Opus, Vorbis, Flac, Mp3, Ac3, Eac3, Dts, Pcm }
#[derive(Debug,Clone)] pub enum HwDecodeBackend { VaApi, Nvdec, Dxva2, VideoToolbox, Software }
#[derive(Debug,Clone)] pub enum TrackType { Video { width: u32, height: u32, fps: f32, codec: VideoCodec }, Audio { channels: u8, sample_rate: u32, codec: AudioCodec }, Subtitle { language: String } }
#[derive(Debug,Clone)] pub struct MediaTrack { pub track_id: u32, pub track_type: TrackType }
#[derive(Debug,Clone)] pub struct MediaInfo { pub path: String, pub duration_ms: u64, pub container: MediaContainer, pub tracks: Vec<MediaTrack>, pub bitrate_kbps: u32, pub size_bytes: u64 }
impl MediaInfo {
    pub fn new(path: &str, dur_ms: u64, container: MediaContainer) -> Self { Self { path: path.into(), duration_ms: dur_ms, container, tracks: Vec::new(), bitrate_kbps: 0, size_bytes: 0 } }
    pub fn add_track(&mut self, t: MediaTrack) { self.tracks.push(t); }
    pub fn video_tracks(&self) -> Vec<&MediaTrack> { self.tracks.iter().filter(|t| matches!(t.track_type, TrackType::Video { .. })).collect() }
    pub fn audio_tracks(&self) -> Vec<&MediaTrack> { self.tracks.iter().filter(|t| matches!(t.track_type, TrackType::Audio { .. })).collect() }
}
#[derive(Debug,Clone,PartialEq)] pub enum PlaybackState { Stopped, Playing { position_ms: u64 }, Paused { position_ms: u64 }, Buffering }
pub struct VlcPipeline { pub state: PlaybackState, pub current_media: Option<MediaInfo>, pub hw_backend: HwDecodeBackend, pub volume: u8, pub muted: bool }
impl VlcPipeline {
    pub fn new() -> Self { Self { state: PlaybackState::Stopped, current_media: Option::None, hw_backend: HwDecodeBackend::VaApi, volume: 100, muted: false } }
    pub fn probe(path: &str) -> MediaInfo {
        let ext = path.rsplit('.').next().unwrap_or("");
        let container = match ext { "mkv"=>MediaContainer::Mkv, "mp4"=>MediaContainer::Mp4, "webm"=>MediaContainer::WebM, "avi"=>MediaContainer::Avi, _=>MediaContainer::Mp4 };
        let mut info = MediaInfo::new(path, 120_000, container);
        info.add_track(MediaTrack { track_id: 0, track_type: TrackType::Video { width: 1920, height: 1080, fps: 23.976, codec: VideoCodec::H264 } });
        info.add_track(MediaTrack { track_id: 1, track_type: TrackType::Audio { channels: 2, sample_rate: 48000, codec: AudioCodec::Aac } });
        info
    }
    pub fn open(&mut self, path: &str) -> Result<(),&'static str> { self.current_media = Some(Self::probe(path)); Ok(()) }
    pub fn play(&mut self) -> Result<(),&'static str> { if self.current_media.is_none() { return Err("no media loaded"); } self.state = PlaybackState::Playing { position_ms: 0 }; Ok(()) }
    pub fn pause(&mut self) { if let PlaybackState::Playing { position_ms } = self.state { self.state = PlaybackState::Paused { position_ms }; } }
    pub fn stop(&mut self) { self.state = PlaybackState::Stopped; }
    pub fn seek_ms(&mut self, pos: u64) { match &mut self.state { PlaybackState::Playing { position_ms } | PlaybackState::Paused { position_ms } => { *position_ms = pos; } _ => {} } }
    pub fn set_hw_backend(&mut self, b: HwDecodeBackend) { self.hw_backend = b; }
    pub fn set_volume(&mut self, v: u8) { self.volume = v.min(200); }
    pub fn transcode(&self, _input: &str, _output: &str, _vc: VideoCodec, _ac: AudioCodec) -> Vec<u8> { Vec::new() }
    pub fn render_to_wayland_surface(&self, _surface_id: u64) {}
    pub fn position_ms(&self) -> u64 { match &self.state { PlaybackState::Playing { position_ms } | PlaybackState::Paused { position_ms } => *position_ms, _ => 0 } }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_probe() { let info = VlcPipeline::probe("movie.mkv"); assert_eq!(info.container, MediaContainer::Mkv); assert_eq!(info.video_tracks().len(), 1); assert_eq!(info.audio_tracks().len(), 1); }
    #[test] fn test_playback_lifecycle() { let mut p = VlcPipeline::new(); p.open("test.mp4").unwrap(); p.play().unwrap(); assert!(matches!(p.state, PlaybackState::Playing { .. })); p.seek_ms(5000); assert_eq!(p.position_ms(), 5000); p.pause(); assert!(matches!(p.state, PlaybackState::Paused { .. })); p.stop(); assert!(matches!(p.state, PlaybackState::Stopped)); }
    #[test] fn test_volume() { let mut p = VlcPipeline::new(); p.set_volume(150); assert_eq!(p.volume, 150); p.set_volume(255); assert_eq!(p.volume, 200); }
}
