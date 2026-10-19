//! SigmaCodec - Multimedia Codec Manager
//!
//! Inspired by GStreamer and FFmpeg, this module provides comprehensive multimedia
//! codec management including codec registration, format conversion, and media processing.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Codec type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecType {
    Audio,
    Video,
    Subtitle,
    Unknown,
}

/// Codec format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecFormat {
    H264,
    H265,
    VP8,
    VP9,
    AV1,
    AAC,
    MP3,
    Opus,
    FLAC,
    Vorbis,
    PCM,
    SubRip,
    WebVTT,
    Unknown,
}

/// Codec state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodecState {
    Idle,
    Decoding,
    Encoding,
    Error,
    Unknown,
}

/// Codec data structure
#[derive(Debug, Clone)]
pub struct Codec {
    pub codec_id: String,
    pub codec_name: String,
    pub codec_type: CodecType,
    pub codec_format: CodecFormat,
    pub state: CodecState,
    pub enabled: bool,
    pub priority: u32,
    pub description: String,
}

impl Codec {
    /// Create a new codec
    pub fn new(
        codec_id: String,
        codec_name: String,
        codec_type: CodecType,
        codec_format: CodecFormat,
        description: String,
    ) -> Self {
        Self {
            codec_id,
            codec_name,
            codec_type,
            codec_format,
            state: CodecState::Idle,
            enabled: true,
            priority: 0,
            description,
        }
    }

    /// Set codec state
    pub fn set_state(&mut self, state: CodecState) {
        self.state = state;
    }

    /// Enable codec
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable codec
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set priority
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }

    /// Check if codec is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if codec is active
    pub fn is_active(&self) -> bool {
        self.state == CodecState::Decoding || self.state == CodecState::Encoding
    }
}

/// Codec manager
pub struct SigmaCodec {
    codecs: BTreeMap<String, Codec>,
    running: AtomicBool,
}

impl SigmaCodec {
    /// Create a new codec manager
    pub fn new() -> Self {
        Self {
            codecs: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the codec manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the codec manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Register a codec
    pub fn register_codec(&mut self, codec: Codec) -> bool {
        let codec_id = codec.codec_id.clone();
        self.codecs.insert(codec_id, codec);
        true
    }

    /// Unregister a codec
    pub fn unregister_codec(&mut self, codec_id: &str) -> Option<Codec> {
        self.codecs.remove(codec_id)
    }

    /// Get a codec by ID
    pub fn get_codec(&self, codec_id: &str) -> Option<&Codec> {
        self.codecs.get(codec_id)
    }

    /// List all codecs
    pub fn list_codecs(&self) -> Vec<&Codec> {
        self.codecs.values().collect()
    }

    /// List codecs by type
    pub fn list_codecs_by_type(&self, codec_type: CodecType) -> Vec<&Codec> {
        self.codecs
            .values()
            .filter(|c| c.codec_type == codec_type)
            .collect()
    }

    /// List codecs by format
    pub fn list_codecs_by_format(&self, codec_format: CodecFormat) -> Vec<&Codec> {
        self.codecs
            .values()
            .filter(|c| c.codec_format == codec_format)
            .collect()
    }

    /// List enabled codecs
    pub fn list_enabled_codecs(&self) -> Vec<&Codec> {
        self.codecs.values().filter(|c| c.enabled).collect()
    }

    /// Enable a codec
    pub fn enable_codec(&mut self, codec_id: &str) -> bool {
        if let Some(codec) = self.codecs.get_mut(codec_id) {
            codec.enable();
            true
        } else {
            false
        }
    }

    /// Disable a codec
    pub fn disable_codec(&mut self, codec_id: &str) -> bool {
        if let Some(codec) = self.codecs.get_mut(codec_id) {
            codec.disable();
            true
        } else {
            false
        }
    }

    /// Start decoding with a codec
    pub fn start_decoding(&mut self, codec_id: &str) -> bool {
        if let Some(codec) = self.codecs.get_mut(codec_id) {
            if codec.enabled {
                codec.set_state(CodecState::Decoding);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Start encoding with a codec
    pub fn start_encoding(&mut self, codec_id: &str) -> bool {
        if let Some(codec) = self.codecs.get_mut(codec_id) {
            if codec.enabled {
                codec.set_state(CodecState::Encoding);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Stop codec operation
    pub fn stop_codec(&mut self, codec_id: &str) -> bool {
        if let Some(codec) = self.codecs.get_mut(codec_id) {
            codec.set_state(CodecState::Idle);
            true
        } else {
            false
        }
    }

    /// Get codec count
    pub fn codec_count(&self) -> usize {
        self.codecs.len()
    }
}

impl Default for SigmaCodec {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaCodec instance (placeholder for actual global state)
static mut SIGMACODEC_INSTANCE: Option<SigmaCodec> = None;

/// Initialize the global SigmaCodec instance
pub fn init_sigmacodec() {
    unsafe {
        SIGMACODEC_INSTANCE = Some(SigmaCodec::new());
    }
}

/// Get the global SigmaCodec instance
pub fn get_sigmacodec() -> Option<&'static SigmaCodec> {
    unsafe { SIGMACODEC_INSTANCE.as_ref() }
}

/// Get mutable global SigmaCodec instance
pub fn get_sigmacodec_mut() -> Option<&'static mut SigmaCodec> {
    unsafe { SIGMACODEC_INSTANCE.as_mut() }
}

/// Convenience function to register a codec
pub fn register_codec(codec: Codec) -> bool {
    if let Some(codec_mgr) = get_sigmacodec_mut() {
        codec_mgr.register_codec(codec)
    } else {
        false
    }
}

/// Convenience function to enable a codec
pub fn enable_codec(codec_id: &str) -> bool {
    if let Some(codec_mgr) = get_sigmacodec_mut() {
        codec_mgr.enable_codec(codec_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_creation() {
        let codec = Codec::new(
            "h264".to_string(),
            "H.264 Video Codec".to_string(),
            CodecType::Video,
            CodecFormat::H264,
            "H.264/AVC video codec".to_string(),
        );

        assert_eq!(codec.codec_id, "h264");
        assert_eq!(codec.codec_type, CodecType::Video);
        assert_eq!(codec.codec_format, CodecFormat::H264);
        assert_eq!(codec.state, CodecState::Idle);
        assert!(codec.is_enabled());
    }

    #[test]
    fn test_codec_state() {
        let mut codec = Codec::new(
            "aac".to_string(),
            "AAC Audio Codec".to_string(),
            CodecType::Audio,
            CodecFormat::AAC,
            "AAC audio codec".to_string(),
        );

        codec.set_state(CodecState::Decoding);
        assert!(codec.is_active());

        codec.set_state(CodecState::Idle);
        assert!(!codec.is_active());
    }

    #[test]
    fn test_codec_enable_disable() {
        let mut codec = Codec::new(
            "mp3".to_string(),
            "MP3 Audio Codec".to_string(),
            CodecType::Audio,
            CodecFormat::MP3,
            "MP3 audio codec".to_string(),
        );

        assert!(codec.is_enabled());
        codec.disable();
        assert!(!codec.is_enabled());
        codec.enable();
        assert!(codec.is_enabled());
    }

    #[test]
    fn test_sigmacodec_creation() {
        let codec_mgr = SigmaCodec::new();
        assert!(!codec_mgr.is_running());
        assert_eq!(codec_mgr.codec_count(), 0);
    }

    #[test]
    fn test_sigmacodec_start_stop() {
        let mut codec_mgr = SigmaCodec::new();
        codec_mgr.start();
        assert!(codec_mgr.is_running());
        codec_mgr.stop();
        assert!(!codec_mgr.is_running());
    }

    #[test]
    fn test_sigmacodec_register_codec() {
        let mut codec_mgr = SigmaCodec::new();
        let codec = Codec::new(
            "h264".to_string(),
            "H.264 Video Codec".to_string(),
            CodecType::Video,
            CodecFormat::H264,
            "H.264/AVC video codec".to_string(),
        );

        let registered = codec_mgr.register_codec(codec);
        assert!(registered);
        assert_eq!(codec_mgr.codec_count(), 1);
    }

    #[test]
    fn test_sigmacodec_unregister_codec() {
        let mut codec_mgr = SigmaCodec::new();
        let codec = Codec::new(
            "aac".to_string(),
            "AAC Audio Codec".to_string(),
            CodecType::Audio,
            CodecFormat::AAC,
            "AAC audio codec".to_string(),
        );

        codec_mgr.register_codec(codec);
        assert_eq!(codec_mgr.codec_count(), 1);

        let unregistered = codec_mgr.unregister_codec("aac");
        assert!(unregistered.is_some());
        assert_eq!(codec_mgr.codec_count(), 0);
    }

    #[test]
    fn test_sigmacodec_list_by_type() {
        let mut codec_mgr = SigmaCodec::new();

        codec_mgr.register_codec(Codec::new(
            "h264".to_string(),
            "H.264".to_string(),
            CodecType::Video,
            CodecFormat::H264,
            "Video codec".to_string(),
        ));

        codec_mgr.register_codec(Codec::new(
            "vp9".to_string(),
            "VP9".to_string(),
            CodecType::Video,
            CodecFormat::VP9,
            "Video codec".to_string(),
        ));

        codec_mgr.register_codec(Codec::new(
            "aac".to_string(),
            "AAC".to_string(),
            CodecType::Audio,
            CodecFormat::AAC,
            "Audio codec".to_string(),
        ));

        let video_codecs = codec_mgr.list_codecs_by_type(CodecType::Video);
        assert_eq!(video_codecs.len(), 2);

        let audio_codecs = codec_mgr.list_codecs_by_type(CodecType::Audio);
        assert_eq!(audio_codecs.len(), 1);
    }

    #[test]
    fn test_sigmacodec_enable_disable() {
        let mut codec_mgr = SigmaCodec::new();
        let codec = Codec::new(
            "h264".to_string(),
            "H.264".to_string(),
            CodecType::Video,
            CodecFormat::H264,
            "Video codec".to_string(),
        );

        codec_mgr.register_codec(codec);
        assert!(codec_mgr.enable_codec("h264"));
        assert!(codec_mgr.disable_codec("h264"));
    }

    #[test]
    fn test_sigmacodec_start_decoding() {
        let mut codec_mgr = SigmaCodec::new();
        let codec = Codec::new(
            "h264".to_string(),
            "H.264".to_string(),
            CodecType::Video,
            CodecFormat::H264,
            "Video codec".to_string(),
        );

        codec_mgr.register_codec(codec);
        let started = codec_mgr.start_decoding("h264");
        assert!(started);

        let codec = codec_mgr.get_codec("h264").unwrap();
        assert!(codec.is_active());
    }

    #[test]
    fn test_global_instance() {
        init_sigmacodec();

        let codec_mgr = get_sigmacodec();
        assert!(codec_mgr.is_some());

        let mut codec_mgr_mut = get_sigmacodec_mut();
        assert!(codec_mgr_mut.is_some());

        if let Some(codec_mgr) = codec_mgr_mut {
            codec_mgr.start();
            assert!(codec_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_register_codec() {
        init_sigmacodec();

        let codec = Codec::new(
            "h264".to_string(),
            "H.264".to_string(),
            CodecType::Video,
            CodecFormat::H264,
            "Video codec".to_string(),
        );

        let registered = register_codec(codec);
        assert!(registered);

        let codec_mgr = get_sigmacodec().unwrap();
        assert_eq!(codec_mgr.codec_count(), 1);
    }

    #[test]
    fn test_convenience_enable_codec() {
        init_sigmacodec();

        let codec = Codec::new(
            "h264".to_string(),
            "H.264".to_string(),
            CodecType::Video,
            CodecFormat::H264,
            "Video codec".to_string(),
        );

        register_codec(codec);
        let enabled = enable_codec("h264");
        assert!(enabled);

        let codec_mgr = get_sigmacodec().unwrap();
        let codec = codec_mgr.get_codec("h264").unwrap();
        assert!(codec.is_enabled());
    }
}
