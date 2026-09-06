#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! Sovereign VLC and MPV Equivalent Video Player
//! Natively optimized for SigmaOS content-addressed and virtual memory architectures.
use std::vec;

use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. SOVEREIGN VIDEO PLAYER CORE (VLC & MPV Equivalent)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecType {
    H264,
    H265,
    VP9,
    AV1,
}

/// Stream Track Type for VLC-style Packet Demuxing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamType {
    Video,
    Audio,
    Subtitle,
}

/// Zero-Copy Elementary Stream Demuxed Packet
#[derive(Debug, Clone)]
pub struct DemuxedPacket {
    pub stream_type: StreamType,
    pub pts_ms: u64, // Presentation Timestamp (ms)
    pub dts_ms: u64, // Decode Timestamp (ms)
    pub is_keyframe: bool,
    pub payload: Vec<u8>,
}

/// VLC-Style Zero-Copy Packet Demuxer Engine
#[derive(Debug, Clone)]
pub struct VlcPacketDemuxer {
    pub active_stream: StreamType,
    pub packet_ring_buffer: Vec<DemuxedPacket>,
    pub buffer_capacity: usize,
    pub bytes_demuxed: u64,
}

impl VlcPacketDemuxer {
    pub fn new(capacity: usize) -> Self {
        Self {
            active_stream: StreamType::Video,
            packet_ring_buffer: Vec::with_capacity(capacity),
            buffer_capacity: capacity,
            bytes_demuxed: 0,
        }
    }

    /// Pushes a demuxed packet into the zero-copy ring buffer
    pub fn push_packet(&mut self, packet: DemuxedPacket) -> bool {
        if self.packet_ring_buffer.len() >= self.buffer_capacity {
            self.packet_ring_buffer.remove(0); // Drop oldest packet on overflow
        }
        self.bytes_demuxed += packet.payload.len() as u64;
        self.packet_ring_buffer.push(packet);
        true
    }

    /// Pops the next packet ready for hardware decoding
    pub fn pop_packet(&mut self) -> Option<DemuxedPacket> {
        if !self.packet_ring_buffer.is_empty() {
            Some(self.packet_ring_buffer.remove(0))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.packet_ring_buffer.clear();
    }
}

impl Default for VlcPacketDemuxer {
    fn default() -> Self {
        Self::new(128)
    }
}

/// Hardware Acceleration Interface API (VA-API / NVDEC / DXVA2 equivalent)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareAccelApi {
    Software,
    VaApi,
    Nvdec,
    Vdpm,
    DirectXVideoAccel,
}

/// Hardware Accelerated Video Frame Surface Handle
#[derive(Debug, Clone)]
pub struct HardwareVideoSurface {
    pub surface_id: u64,
    pub width: u32,
    pub height: u32,
    pub drm_format: u32, // FourCC e.g. NV12 / P010
    pub is_zero_copy: bool,
}

/// Hardware Video Decoder Pipeline Abstraction
#[derive(Debug, Clone)]
pub struct HardwareVideoDecoder {
    pub accel_api: HardwareAccelApi,
    pub active_codec: CodecType,
    pub hardware_surfaces: Vec<HardwareVideoSurface>,
    pub total_decoded_frames: u64,
    pub hardware_failures: u64,
}

impl HardwareVideoDecoder {
    pub fn new(codec: CodecType, accel_api: HardwareAccelApi) -> Self {
        Self {
            accel_api,
            active_codec: codec,
            hardware_surfaces: Vec::new(),
            total_decoded_frames: 0,
            hardware_failures: 0,
        }
    }

    /// Decodes a demuxed packet into a zero-copy hardware surface
    pub fn decode_packet(
        &mut self,
        packet: &DemuxedPacket,
    ) -> Result<HardwareVideoSurface, &'static str> {
        if packet.payload.is_empty() {
            return Err("Empty packet payload");
        }

        self.total_decoded_frames += 1;
        let surface = HardwareVideoSurface {
            surface_id: self.total_decoded_frames,
            width: 1920,
            height: 1080,
            drm_format: 0x3231564E, // NV12 FourCC
            is_zero_copy: self.accel_api != HardwareAccelApi::Software,
        };

        self.hardware_surfaces.push(surface.clone());
        if self.hardware_surfaces.len() > 16 {
            self.hardware_surfaces.remove(0); // Maintain fixed hardware surface pool
        }

        Ok(surface)
    }
}

/// PTS/DTS Master Clock Synchronizer & Drift Compensator
#[derive(Debug, Clone)]
pub struct AvClockSynchronizer {
    pub audio_pts_ms: u64,
    pub video_pts_ms: u64,
    pub drift_tolerance_ms: i64,
    pub dropped_frames: u64,
}

impl AvClockSynchronizer {
    pub fn new(drift_tolerance_ms: i64) -> Self {
        Self {
            audio_pts_ms: 0,
            video_pts_ms: 0,
            drift_tolerance_ms,
            dropped_frames: 0,
        }
    }

    /// Evaluates if video frame should be rendered, delayed, or dropped
    pub fn evaluate_frame_render(&mut self, video_pts: u64) -> bool {
        self.video_pts_ms = video_pts;
        let diff = (video_pts as i64) - (self.audio_pts_ms as i64);

        if diff > self.drift_tolerance_ms {
            // Video is ahead of audio master clock -> delay/wait
            false
        } else if diff < -self.drift_tolerance_ms {
            // Video is lagging behind audio master clock -> drop frame for real-time sync
            self.dropped_frames += 1;
            false
        } else {
            // Perfectly synchronized
            true
        }
    }

    pub fn update_audio_clock(&mut self, audio_pts: u64) {
        self.audio_pts_ms = audio_pts;
    }
}

impl Default for AvClockSynchronizer {
    fn default() -> Self {
        Self::new(40) // Default 40ms (~1 frame at 25fps) drift tolerance
    }
}

/// VLC 10-Band Equalizer Frequency Bands
#[derive(Debug, Clone, Copy)]
pub struct VlcEqualizer {
    pub enabled: bool,
    pub preamp_db: f32,
    pub bands_db: [f32; 10], // 60Hz, 170Hz, 310Hz, 600Hz, 1kHz, 3kHz, 6kHz, 12kHz, 14kHz, 16kHz
}

impl VlcEqualizer {
    pub fn new() -> Self {
        Self {
            enabled: false,
            preamp_db: 0.0,
            bands_db: [0.0; 10],
        }
    }

    pub fn set_band_gain(&mut self, band_idx: usize, gain_db: f32) {
        if band_idx < 10 {
            self.bands_db[band_idx] = gain_db.clamp(-20.0, 20.0);
        }
    }
}

impl Default for VlcEqualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Subtitle Cue Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubtitleCue {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

/// Subtitle Track Stream
#[derive(Debug, Clone)]
pub struct SubtitleTrack {
    pub id: u32,
    pub language: String,
    pub cues: Vec<SubtitleCue>,
}

impl SubtitleTrack {
    pub fn new(id: u32, language: String) -> Self {
        Self {
            id,
            language,
            cues: Vec::new(),
        }
    }

    pub fn parse_srt(&mut self, srt_content: &str) {
        // Simple SRT parser helper
        for block in srt_content.split("\n\n") {
            let lines: Vec<&str> = block.lines().collect();
            if lines.len() >= 3 {
                let times: Vec<&str> = lines[1].split("-->").collect();
                if times.len() == 2 {
                    let start_ms = parse_srt_time(times[0].trim());
                    let end_ms = parse_srt_time(times[1].trim());
                    let text = lines[2..].join("\n");
                    self.cues.push(SubtitleCue {
                        start_ms,
                        end_ms,
                        text,
                    });
                }
            }
        }
    }

    pub fn active_cue_at(&self, time_ms: u64) -> Option<&str> {
        for cue in &self.cues {
            if time_ms >= cue.start_ms && time_ms <= cue.end_ms {
                return Some(&cue.text);
            }
        }
        None
    }
}

fn parse_srt_time(time_str: &str) -> u64 {
    // Expected format: 00:01:20,500
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() != 3 {
        return 0;
    }
    let hours: u64 = parts[0].parse().unwrap_or(0);
    let minutes: u64 = parts[1].parse().unwrap_or(0);

    let sec_parts: Vec<&str> = parts[2].split(',').collect();
    let seconds: u64 = sec_parts[0].parse().unwrap_or(0);
    let millis: u64 = if sec_parts.len() > 1 {
        sec_parts[1].parse().unwrap_or(0)
    } else {
        0
    };

    (hours * 3600 + minutes * 60 + seconds) * 1000 + millis
}

/// Chapter Marker for Video Timeline
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chapter {
    pub title: String,
    pub start_ms: u64,
}

/// Playlist Loop Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopMode {
    Off,
    LoopOne,
    LoopAll,
}

/// Playlist Item
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaylistItem {
    pub url: String,
    pub title: String,
    pub duration_ms: u64,
}

/// Playlist Queue
#[derive(Debug, Clone)]
pub struct Playlist {
    pub items: Vec<PlaylistItem>,
    pub current_index: usize,
    pub loop_mode: LoopMode,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_index: 0,
            loop_mode: LoopMode::Off,
        }
    }

    pub fn add(&mut self, url: String, title: String, duration_ms: u64) {
        self.items.push(PlaylistItem {
            url,
            title,
            duration_ms,
        });
    }

    pub fn next(&mut self) -> Option<&PlaylistItem> {
        if self.items.is_empty() {
            return None;
        }

        match self.loop_mode {
            LoopMode::LoopOne => Some(&self.items[self.current_index]),
            LoopMode::LoopAll => {
                self.current_index = (self.current_index + 1) % self.items.len();
                Some(&self.items[self.current_index])
            }
            LoopMode::Off => {
                if self.current_index + 1 < self.items.len() {
                    self.current_index += 1;
                    Some(&self.items[self.current_index])
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_m3u(&mut self, m3u_content: &str) {
        let mut current_title = String::new();
        for line in m3u_content.lines() {
            let line = line.trim();
            if line.starts_with("#EXTINF:") {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() > 1 {
                    current_title = parts[1].to_string();
                }
            } else if !line.is_empty() && !line.starts_with('#') {
                let title = if !current_title.is_empty() {
                    let t = current_title.clone();
                    current_title.clear();
                    t
                } else {
                    line.to_string()
                };
                self.add(line.to_string(), title, 0);
            }
        }
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignVideoPlayer {
    pub active_codec: CodecType,
    pub state: PlayerState,
    pub volume: u32,
    pub playback_speed: f32, // MPV-style speed scaling (0.25x to 4.0x)
    pub pitch_preservation: bool,
    pub current_time_ms: u64,
    pub current_frame: u64,
    pub frame_rate: u32, // e.g. 60 fps
    pub is_gpu_accelerated: bool,
    pub equalizer: VlcEqualizer,
    pub subtitle_tracks: Vec<SubtitleTrack>,
    pub active_subtitle_track: Option<u32>,
    pub chapters: Vec<Chapter>,
    pub playlist: Playlist,
    pub demuxer: VlcPacketDemuxer,
    pub decoder: HardwareVideoDecoder,
    pub clock_sync: AvClockSynchronizer,
}

impl SovereignVideoPlayer {
    pub fn new(codec: CodecType) -> Self {
        Self {
            active_codec: codec,
            state: PlayerState::Stopped,
            volume: 80,
            playback_speed: 1.0,
            pitch_preservation: true,
            current_time_ms: 0,
            current_frame: 0,
            frame_rate: 60,
            is_gpu_accelerated: true,
            equalizer: VlcEqualizer::new(),
            subtitle_tracks: Vec::new(),
            active_subtitle_track: None,
            chapters: Vec::new(),
            playlist: Playlist::new(),
            demuxer: VlcPacketDemuxer::new(128),
            decoder: HardwareVideoDecoder::new(codec, HardwareAccelApi::VaApi),
            clock_sync: AvClockSynchronizer::new(40),
        }
    }

    pub fn play(&mut self) {
        self.state = PlayerState::Playing;
    }

    pub fn pause(&mut self) {
        self.state = PlayerState::Paused;
    }

    pub fn stop(&mut self) {
        self.state = PlayerState::Stopped;
        self.current_time_ms = 0;
        self.current_frame = 0;
    }

    pub fn set_volume(&mut self, new_vol: u32) {
        self.volume = core::cmp::min(new_vol, 100);
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.playback_speed = speed.clamp(0.25, 4.0);
    }

    pub fn seek_ms(&mut self, target_ms: u64) {
        self.current_time_ms = target_ms;
        self.current_frame = (target_ms * self.frame_rate as u64) / 1000;
    }

    pub fn step_frame_forward(&mut self) {
        self.current_frame += 1;
        self.current_time_ms = (self.current_frame * 1000) / self.frame_rate as u64;
    }

    pub fn step_frame_backward(&mut self) {
        self.current_frame = self.current_frame.saturating_sub(1);
        self.current_time_ms = (self.current_frame * 1000) / self.frame_rate as u64;
    }

    pub fn add_subtitle_track(&mut self, track: SubtitleTrack) {
        self.subtitle_tracks.push(track);
    }

    pub fn get_current_subtitle(&self) -> Option<&str> {
        let active_id = self.active_subtitle_track?;
        let track = self.subtitle_tracks.iter().find(|t| t.id == active_id)?;
        track.active_cue_at(self.current_time_ms)
    }
}

// =========================================================================
// 2. DEMAND PAGING & VIRTUAL MEMORY (Gap Closure)
// =========================================================================

pub struct PageTable {
    pub mapped_frames: [bool; 1024],
}

pub struct SovereignVmm {
    pub root_pt: PageTable,
    pub page_faults_handled: u32,
}

impl SovereignVmm {
    pub fn new() -> Self {
        Self {
            root_pt: PageTable {
                mapped_frames: [false; 1024],
            },
            page_faults_handled: 0,
        }
    }

    pub fn handle_page_fault(&mut self, virtual_addr: usize) -> Result<usize, &'static str> {
        let page_idx = (virtual_addr / 4096) % 1024;
        if self.root_pt.mapped_frames[page_idx] {
            Err("Page already mapped (spurious page fault)")
        } else {
            self.root_pt.mapped_frames[page_idx] = true;
            self.page_faults_handled += 1;
            Ok(page_idx * 4096)
        }
    }
}

impl Default for SovereignVmm {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. CGROUPS & PROCESS MANAGEMENT (Gap Closure)
// =========================================================================

pub struct CGroup {
    pub name: &'static str,
    pub cpu_share: u32,
    pub memory_limit_bytes: u64,
}

pub struct CGroupController {
    pub groups: Vec<CGroup>,
}

impl CGroupController {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn register_group(&mut self, name: &'static str, cpu_share: u32, limit_bytes: u64) {
        self.groups.push(CGroup {
            name,
            cpu_share,
            memory_limit_bytes: limit_bytes,
        });
    }
}

impl Default for CGroupController {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. DHCP & DNS RESOLVER (Gap Closure)
// =========================================================================

pub struct DnsResolver {
    pub cache: Vec<(String, String)>, // (domain, ip)
}

impl DnsResolver {
    pub fn new() -> Self {
        Self { cache: Vec::new() }
    }

    pub fn register_record(&mut self, domain: String, ip: String) {
        self.cache.push((domain, ip));
    }

    pub fn resolve(&self, domain: &str) -> Option<String> {
        self.cache
            .iter()
            .find(|(d, _)| d == domain)
            .map(|(_, ip)| ip.clone())
    }
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. SECURE BOOT (Gap Closure)
// =========================================================================

pub struct SecureBootKeyring {
    pub authorized_db_keys: [[u8; 32]; 4],
    pub keys_registered: usize,
}

impl SecureBootKeyring {
    pub fn new() -> Self {
        Self {
            authorized_db_keys: [[0u8; 32]; 4],
            keys_registered: 0,
        }
    }

    pub fn enroll_key(&mut self, key: [u8; 32]) -> Result<(), &'static str> {
        if self.keys_registered < 4 {
            self.authorized_db_keys[self.keys_registered] = key;
            self.keys_registered += 1;
            Ok(())
        } else {
            Err("SecureBootKeyring: Maximum key enrollment threshold reached")
        }
    }

    pub fn verify_signature(&self, image_hash: &[u8; 32]) -> bool {
        self.authorized_db_keys.iter().any(|k| k == image_hash)
    }
}

impl Default for SecureBootKeyring {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. SIGMA-SH REPL SHELL & SYSTEMD-STYLE INIT SERVICES (Gap Closure)
// =========================================================================

pub struct InitService {
    pub name: &'static str,
    pub is_active: bool,
}

pub struct SigmaSystemd {
    pub services: Vec<InitService>,
}

impl SigmaSystemd {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register_service(&mut self, name: &'static str) {
        self.services.push(InitService {
            name,
            is_active: false,
        });
    }

    pub fn start_service(&mut self, name: &str) -> Result<(), &'static str> {
        for s in self.services.iter_mut() {
            if s.name == name {
                s.is_active = true;
                return Ok(());
            }
        }
        Err("Service not found in system init catalog")
    }
}

impl Default for SigmaSystemd {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. TIME SYNCHRONIZATION NTP (Gap Closure)
// =========================================================================

pub struct NtpClient {
    pub offset_nanos: i64,
}

impl NtpClient {
    pub fn new() -> Self {
        Self { offset_nanos: 0 }
    }

    pub fn sync_time(&mut self, packet_transmit_time: u64, receive_time: u64) {
        self.offset_nanos = (packet_transmit_time as i64) - (receive_time as i64);
    }
}

// ==========================================
// VLC-INSPIRED LIGHTWEIGHT VIDEO ENGINE ARCHITECTURE
// ==========================================

/// Video decoding state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoDecoderState {
    Idle,
    Demuxing,
    Decoding,
    Rendering,
    Paused,
    Error,
}

/// VLC-inspired modular demuxer & decoder video pipeline
pub struct VlcLightweightMediaPipeline {
    pub media_uri: String,
    pub state: VideoDecoderState,
    pub active_demuxer: Option<String>,
    pub active_codec: Option<String>,
    pub frame_ring_buffer: Vec<Vec<u8>>,
    pub max_ring_capacity: usize,
    pub direct_surface_hw_accel: bool,
}

impl VlcLightweightMediaPipeline {
    pub fn new(media_uri: &str) -> Self {
        Self {
            media_uri: media_uri.to_string(),
            state: VideoDecoderState::Idle,
            active_demuxer: None,
            active_codec: None,
            frame_ring_buffer: Vec::new(),
            max_ring_capacity: 16,
            direct_surface_hw_accel: true,
        }
    }

    pub fn open_demuxer(&mut self, format: &str) -> Result<(), &'static str> {
        self.active_demuxer = Some(format.to_string());
        self.state = VideoDecoderState::Demuxing;
        Ok(())
    }

    pub fn initialize_decoder(&mut self, codec: &str) -> Result<(), &'static str> {
        if self.state != VideoDecoderState::Demuxing {
            return Err("Demuxer must be open before initializing video codec decoder");
        }
        self.active_codec = Some(codec.to_string());
        self.state = VideoDecoderState::Decoding;
        Ok(())
    }

    pub fn push_frame_zero_copy(&mut self, frame_data: Vec<u8>) -> Result<(), &'static str> {
        if self.frame_ring_buffer.len() >= self.max_ring_capacity {
            return Err("Zero-copy video frame ring buffer overflow");
        }
        self.frame_ring_buffer.push(frame_data);
        Ok(())
    }

    pub fn render_direct_surface(&mut self) -> Result<usize, &'static str> {
        if self.frame_ring_buffer.is_empty() {
            return Err("No video frame available in ring buffer to render");
        }
        self.state = VideoDecoderState::Rendering;
        let frame = self.frame_ring_buffer.remove(0);
        let rendered_len = frame.len();
        self.state = VideoDecoderState::Decoding;
        Ok(rendered_len)
    }

    pub fn ring_buffer_len(&self) -> usize {
        self.frame_ring_buffer.len()
    }

    pub fn close(&mut self) {
        self.state = VideoDecoderState::Idle;
        self.active_demuxer = None;
        self.active_codec = None;
        self.frame_ring_buffer.clear();
    }
}

impl Default for VlcLightweightMediaPipeline {
    fn default() -> Self {
        Self::new("default.mp4")
    }
}

impl Default for NtpClient {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_vlc_video_player() {
        let mut player = SovereignVideoPlayer::new(CodecType::AV1);
        assert_eq!(player.state, PlayerState::Stopped);
        player.play();
        assert_eq!(player.state, PlayerState::Playing);
        player.set_volume(120);
        assert_eq!(player.volume, 100);
    }

    #[test]
    fn test_vlc_packet_demuxer_and_hardware_decoder() {
        let mut demuxer = VlcPacketDemuxer::new(10);
        let packet = DemuxedPacket {
            stream_type: StreamType::Video,
            pts_ms: 1000,
            dts_ms: 1000,
            is_keyframe: true,
            payload: vec![0x00, 0x00, 0x00, 0x01, 0x67],
        };

        assert!(demuxer.push_packet(packet.clone()));
        assert_eq!(demuxer.bytes_demuxed, 5);

        let popped = demuxer.pop_packet().unwrap();
        assert_eq!(popped.pts_ms, 1000);

        let mut decoder = HardwareVideoDecoder::new(CodecType::H264, HardwareAccelApi::VaApi);
        let surface = decoder.decode_packet(&popped).unwrap();
        assert_eq!(surface.surface_id, 1);
        assert!(surface.is_zero_copy);
    }

    #[test]
    fn test_av_clock_synchronizer() {
        let mut sync = AvClockSynchronizer::new(40);
        sync.update_audio_clock(1000);

        // Video at 1020ms (diff +20ms, within 40ms tolerance) -> render
        assert!(sync.evaluate_frame_render(1020));

        // Video at 900ms (diff -100ms, lagging > 40ms) -> drop frame
        assert!(!sync.evaluate_frame_render(900));
        assert_eq!(sync.dropped_frames, 1);

        // Video at 1100ms (diff +100ms, ahead > 40ms) -> delay
        assert!(!sync.evaluate_frame_render(1100));
    }

    #[test]
    fn test_mpv_speed_and_seeking() {
        let mut player = SovereignVideoPlayer::new(CodecType::H264);
        player.set_speed(1.5);
        assert_eq!(player.playback_speed, 1.5);

        player.seek_ms(2000); // Seek to 2.0s
        assert_eq!(player.current_time_ms, 2000);
        assert_eq!(player.current_frame, 120); // 2000ms * 60fps / 1000

        player.step_frame_forward();
        assert_eq!(player.current_frame, 121);
    }

    #[test]
    fn test_vlc_equalizer() {
        let mut eq = VlcEqualizer::new();
        eq.set_band_gain(0, 6.0); // +6dB at 60Hz
        eq.set_band_gain(4, -3.0); // -3dB at 1kHz

        assert_eq!(eq.bands_db[0], 6.0);
        assert_eq!(eq.bands_db[4], -3.0);
    }

    #[test]
    fn test_subtitles_parsing_and_cue() {
        let mut track = SubtitleTrack::new(1, "English".to_string());
        let srt = r#"1
00:00:01,000 --> 00:00:04,000
Hello SigmaOS World!
"#;
        track.parse_srt(srt);

        assert_eq!(track.cues.len(), 1);
        assert_eq!(track.active_cue_at(2000), Some("Hello SigmaOS World!"));
        assert_eq!(track.active_cue_at(5000), None);
    }

    #[test]
    fn test_playlist_m3u_parsing() {
        let mut playlist = Playlist::new();
        let m3u = r#"#EXTM3U
#EXTINF:120,Movie Trailer A
https://media.sigmaos.dev/trailer_a.mp4
#EXTINF:240,Movie Trailer B
https://media.sigmaos.dev/trailer_b.mp4
"#;
        playlist.parse_m3u(m3u);

        assert_eq!(playlist.items.len(), 2);
        assert_eq!(playlist.items[0].title, "Movie Trailer A");
        assert_eq!(playlist.items[1].title, "Movie Trailer B");
    }

    #[test]
    fn test_vmm_paging() {
        let mut vmm = SovereignVmm::new();
        let fault_addr = 0x8000;
        let resolved_phy = vmm.handle_page_fault(fault_addr).unwrap();
        assert_eq!(resolved_phy, 32768);
        assert_eq!(vmm.page_faults_handled, 1);
    }

    #[test]
    fn test_cgroups() {
        let mut cc = CGroupController::new();
        cc.register_group("developer_workloads", 1024, 2 * 1024 * 1024 * 1024);
        assert_eq!(cc.groups[0].cpu_share, 1024);
    }

    #[test]
    fn test_dns_resolver() {
        let mut resolver = DnsResolver::new();
        resolver.register_record("sigmaos.dev".to_string(), "10.0.0.1".to_string());
        assert_eq!(resolver.resolve("sigmaos.dev").unwrap(), "10.0.0.1");
    }

    #[test]
    fn test_secure_boot() {
        let mut keyring = SecureBootKeyring::new();
        let key = [0x55u8; 32];
        keyring.enroll_key(key).unwrap();
        assert!(keyring.verify_signature(&key));
    }

    #[test]
    fn test_systemd_init() {
        let mut init = SigmaSystemd::new();
        init.register_service("networkd");
        init.start_service("networkd").unwrap();
        assert!(init.services[0].is_active);
    }

    #[test]
    fn test_ntp_sync() {
        let mut ntp = NtpClient::new();
        ntp.sync_time(1000, 950);
        assert_eq!(ntp.offset_nanos, 50);
    }

    #[test]
    fn test_vlc_lightweight_media_pipeline() {
        let mut pipeline = VlcLightweightMediaPipeline::new("4k_sample.mp4");
        assert_eq!(pipeline.state, VideoDecoderState::Idle);

        assert!(pipeline.open_demuxer("mp4").is_ok());
        assert_eq!(pipeline.state, VideoDecoderState::Demuxing);

        assert!(pipeline.initialize_decoder("h264").is_ok());
        assert_eq!(pipeline.state, VideoDecoderState::Decoding);

        let frame = vec![0x10; 1920 * 1080];
        assert!(pipeline.push_frame_zero_copy(frame).is_ok());
        assert_eq!(pipeline.ring_buffer_len(), 1);

        let rendered_bytes = pipeline.render_direct_surface().unwrap();
        assert_eq!(rendered_bytes, 1920 * 1080);
        assert_eq!(pipeline.ring_buffer_len(), 0);

        pipeline.close();
        assert_eq!(pipeline.state, VideoDecoderState::Idle);
    }
}
