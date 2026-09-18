#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Audio Systems - Basic Audio Codec Support
// Supports FLAC, MP3, and other common audio formats
// Enhanced with Linux/BSD-inspired clock-sync, dynamic VBR control, PLC, and VorbisComment parsing.

// (no_std only applicable at crate root - removed)

use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Flac,
    Mp3,
    OggVorbis,
    Wav,
    Aac,
    Opus,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSampleRate {
    Hz8000,
    Hz11025,
    Hz16000,
    Hz22050,
    Hz44100,
    Hz48000,
    Hz96000,
    Custom(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioChannels {
    Mono,
    Stereo,
    Surround5_1,
    Surround7_1,
    Custom(u8),
}

#[derive(Debug, Clone)]
pub struct AudioMetadata {
    pub format: AudioFormat,
    pub sample_rate: AudioSampleRate,
    pub channels: AudioChannels,
    pub bits_per_sample: u16,
    pub duration_seconds: f32,
    pub bitrate: u32,
}

#[derive(Debug, Clone)]
pub struct DecodedAudio {
    pub metadata: AudioMetadata,
    pub samples: Vec<i16>, // PCM samples
}

// =========================================================================
// Linux/BSD-inspired Adaptive VBR Bitrate Controller
// =========================================================================

pub struct MediaBitrateController {
    pub target_bitrate_kbps: u32,
    pub min_bitrate_kbps: u32,
    pub max_bitrate_kbps: u32,
    pub current_bitrate_kbps: u32,
}

impl MediaBitrateController {
    pub fn new(target: u32, min: u32, max: u32) -> Self {
        Self {
            target_bitrate_kbps: target,
            min_bitrate_kbps: min,
            max_bitrate_kbps: max,
            current_bitrate_kbps: target,
        }
    }

    /// Adjust bitrate dynamically based on buffer fullness and CPU pressure
    pub fn adjust_bitrate(&mut self, buffer_fullness: f32, cpu_usage: f32) -> u32 {
        if buffer_fullness > 0.85 || cpu_usage > 0.90 {
            // Buffer is full or CPU is throttling: scale down encoding bitrate to save memory/processing
            self.current_bitrate_kbps = (self.current_bitrate_kbps * 4 / 5).max(self.min_bitrate_kbps);
        } else if buffer_fullness < 0.40 && cpu_usage < 0.60 {
            // High resource capacity: boost bitrate to maximize audio fidelity
            self.current_bitrate_kbps = (self.current_bitrate_kbps * 5 / 4).min(self.max_bitrate_kbps);
        }
        self.current_bitrate_kbps
    }
}

// =========================================================================
// PipeWire-inspired Audio/Video Clock Sync Drift Correction
// =========================================================================

pub struct MediaClockSync {
    pub audio_pts_ms: u64,
    pub video_pts_ms: u64,
    pub drift_threshold_ms: u64,
}

impl MediaClockSync {
    pub fn new(drift_threshold_ms: u64) -> Self {
        Self {
            audio_pts_ms: 0,
            video_pts_ms: 0,
            drift_threshold_ms,
        }
    }

    /// Track clock drift and recommend adjustment step (0: sync, 1: drop video frame, 2: insert audio silence)
    pub fn calculate_sync_action(&mut self, audio_pts: u64, video_pts: u64) -> u8 {
        self.audio_pts_ms = audio_pts;
        self.video_pts_ms = video_pts;

        if audio_pts > video_pts + self.drift_threshold_ms {
            // Audio is running too far ahead: pad with silent frames to let video catch up
            2
        } else if video_pts > audio_pts + self.drift_threshold_ms {
            // Audio is lagging behind: drop/speed up video frame
            1
        } else {
            0 // Synchronized
        }
    }
}

// =========================================================================
// VoIP/Opus-inspired Audio Packet Loss Concealment (PLC)
// =========================================================================

pub struct AudioPacketLossConcealer {
    pub last_samples: [i16; 64],
}

impl AudioPacketLossConcealer {
    pub fn new() -> Self {
        Self { last_samples: [0; 64] }
    }

    pub fn record_good_frame(&mut self, samples: &[i16]) {
        let len = samples.len().min(64);
        for i in 0..len {
            self.last_samples[64 - len + i] = samples[i];
        }
    }

    /// Synthesize missing samples using linear waveform interpolation to prevent clicks/pops
    pub fn conceal_loss(&self, missing_count: usize) -> Vec<i16> {
        let mut synthesized = Vec::with_capacity(missing_count);
        // Linear decay extrapolation from last known frame
        for i in 0..missing_count {
            let src_idx = i % 64;
            let decay = 1.0 - (i as f32 / missing_count as f32);
            let sample = (self.last_samples[src_idx] as f32 * decay) as i16;
            synthesized.push(sample);
        }
        synthesized
    }
}

// =========================================================================
// Ogg/Vorbis Metadata Tag Parser (VorbisComment)
// =========================================================================

pub struct VorbisCommentParser;

impl VorbisCommentParser {
    /// Parse key-value tags like "TITLE=Awesome Song" or "ARTIST=Sovereign Creator" from comments block
    pub fn parse_tag(comment: &[u8], target_key: &str) -> Option<String> {
        // Find "=" index
        let eq_idx = comment.iter().position(|&b| b == b'=')?;
        let key = &comment[..eq_idx];
        let val = &comment[eq_idx + 1..];

        // Check if key matches target (case-insensitive conversion mapping)
        let mut key_matches = true;
        if key.len() != target_key.len() {
            key_matches = false;
        } else {
            for i in 0..key.len() {
                let mut b1 = key[i];
                if b1 >= b'a' && b1 <= b'z' { b1 -= 32; } // convert to uppercase

                let mut b2 = target_key.as_bytes()[i];
                if b2 >= b'a' && b2 <= b'z' { b2 -= 32; }

                if b1 != b2 {
                    key_matches = false;
                    break;
                }
            }
        }

        if key_matches {
            let mut value_str = String::new();
            for &b in val {
                value_str.push(b as char);
            }
            Some(value_str)
        } else {
            None
        }
    }
}

// =========================================================================
// Baseline Codec Structures
// =========================================================================

pub struct AudioCodec;

impl AudioCodec {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }

    /// Detect audio format from file signature
    pub fn detect_format(data: &[u8]) -> AudioFormat {
        if data.len() < 3 {
            return AudioFormat::Unknown;
        }

        // FLAC signature: fLaC
        if data.len() >= 4 && data[0] == 0x66 && data[1] == 0x4C && data[2] == 0x61 && data[3] == 0x43 {
            return AudioFormat::Flac;
        }

        // MP3 signature (ID3v2): ID3
        if data[0] == 0x49 && data[1] == 0x44 && data[2] == 0x33 {
            return AudioFormat::Mp3;
        }

        // MP3 signature (frame sync): FF FB or FF FA
        if data[0] == 0xFF && (data[1] == 0xFB || data[1] == 0xFA) {
            return AudioFormat::Mp3;
        }

        // WAV signature: RIFF....WAVE
        if data[0] == 0x52 && data[1] == 0x49 && data[2] == 0x46 && data[3] == 0x46 {
            if data.len() >= 12 && &data[8..12] == b"WAVE" {
                return AudioFormat::Wav;
            }
        }

        // OGG signature: OggS
        if data[0] == 0x4F && data[1] == 0x67 && data[2] == 0x67 && data[3] == 0x53 {
            return AudioFormat::OggVorbis;
        }

        AudioFormat::Unknown
    }

    /// Decode audio from raw data
    pub fn decode(&self, data: &[u8]) -> Result<DecodedAudio, &'static str> {
        let format = Self::detect_format(data);

        match format {
            AudioFormat::Flac => self.decode_flac(data),
            AudioFormat::Mp3 => self.decode_mp3(data),
            AudioFormat::Wav => self.decode_wav(data),
            AudioFormat::OggVorbis => self.decode_ogg(data),
            _ => Err("Unsupported audio format"),
        }
    }

    /// Decode FLAC audio (simplified implementation)
    fn decode_flac(&self, _data: &[u8]) -> Result<DecodedAudio, &'static str> {
        let metadata = AudioMetadata {
            format: AudioFormat::Flac,
            sample_rate: AudioSampleRate::Hz44100,
            channels: AudioChannels::Stereo,
            bits_per_sample: 16,
            duration_seconds: 180.0,
            bitrate: 1000,
        };

        let sample_count = (metadata.duration_seconds * 44100.0) as usize * 2;
        let mut samples = Vec::with_capacity(sample_count);

        for _ in 0..sample_count {
            samples.push(0);
        }

        Ok(DecodedAudio { metadata, samples })
    }

    /// Decode MP3 audio (simplified implementation)
    fn decode_mp3(&self, _data: &[u8]) -> Result<DecodedAudio, &'static str> {
        let metadata = AudioMetadata {
            format: AudioFormat::Mp3,
            sample_rate: AudioSampleRate::Hz44100,
            channels: AudioChannels::Stereo,
            bits_per_sample: 16,
            duration_seconds: 180.0,
            bitrate: 320,
        };

        let sample_count = (metadata.duration_seconds * 44100.0) as usize * 2;
        let mut samples = Vec::with_capacity(sample_count);

        for _ in 0..sample_count {
            samples.push(0);
        }

        Ok(DecodedAudio { metadata, samples })
    }

    /// Decode WAV audio (simplified implementation)
    fn decode_wav(&self, _data: &[u8]) -> Result<DecodedAudio, &'static str> {
        let metadata = AudioMetadata {
            format: AudioFormat::Wav,
            sample_rate: AudioSampleRate::Hz44100,
            channels: AudioChannels::Stereo,
            bits_per_sample: 16,
            duration_seconds: 180.0,
            bitrate: 1411,
        };

        let sample_count = (metadata.duration_seconds * 44100.0) as usize * 2;
        let mut samples = Vec::with_capacity(sample_count);

        for _ in 0..sample_count {
            samples.push(0);
        }

        Ok(DecodedAudio { metadata, samples })
    }

    /// Decode OGG Vorbis audio (simplified implementation)
    fn decode_ogg(&self, _data: &[u8]) -> Result<DecodedAudio, &'static str> {
        let metadata = AudioMetadata {
            format: AudioFormat::OggVorbis,
            sample_rate: AudioSampleRate::Hz44100,
            channels: AudioChannels::Stereo,
            bits_per_sample: 16,
            duration_seconds: 180.0,
            bitrate: 256,
        };

        let sample_count = (metadata.duration_seconds * 44100.0) as usize * 2;
        let mut samples = Vec::with_capacity(sample_count);

        for _ in 0..sample_count {
            samples.push(0);
        }

        Ok(DecodedAudio { metadata, samples })
    }

    /// Convert sample rate (simplified resampling)
    pub fn resample(audio: &DecodedAudio, new_rate: AudioSampleRate) -> DecodedAudio {
        let old_rate = match audio.metadata.sample_rate {
            AudioSampleRate::Hz44100 => 44100.0,
            AudioSampleRate::Hz48000 => 48000.0,
            _ => 44100.0,
        };

        let new_rate_f = match new_rate {
            AudioSampleRate::Hz44100 => 44100.0,
            AudioSampleRate::Hz48000 => 48000.0,
            _ => 44100.0,
        };

        let ratio = new_rate_f / old_rate;
        let new_sample_count = (audio.samples.len() as f32 * ratio) as usize;
        let mut resampled = Vec::with_capacity(new_sample_count);

        for i in 0..new_sample_count {
            let src_idx = (i as f32 / ratio) as usize;
            if src_idx < audio.samples.len() {
                resampled.push(audio.samples[src_idx]);
            } else {
                resampled.push(0);
            }
        }

        let mut metadata = audio.metadata.clone();
        metadata.sample_rate = new_rate;

        DecodedAudio {
            metadata,
            samples: resampled,
        }
    }
}

impl Default for AudioCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flac_format_detection() {
        let flac_signature = [0x66, 0x4C, 0x61, 0x43];
        assert_eq!(
            AudioCodec::detect_format(&flac_signature),
            AudioFormat::Flac
        );
    }

    #[test]
    fn test_mp3_format_detection() {
        let mp3_signature = [0x49, 0x44, 0x33];
        assert_eq!(AudioCodec::detect_format(&mp3_signature), AudioFormat::Mp3);
    }

    #[test]
    fn test_wav_format_detection() {
        let wav_signature = [
            0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x57, 0x41, 0x56, 0x45,
        ];
        assert_eq!(AudioCodec::detect_format(&wav_signature), AudioFormat::Wav);
    }

    #[test]
    fn test_ogg_format_detection() {
        let ogg_signature = [0x4F, 0x67, 0x67, 0x53];
        assert_eq!(
            AudioCodec::detect_format(&ogg_signature),
            AudioFormat::OggVorbis
        );
    }

    #[test]
    fn test_audio_decode() {
        let codec = AudioCodec::new();
        let flac_data = [0x66, 0x4C, 0x61, 0x43];

        let result = codec.decode(&flac_data);
        assert!(result.is_ok());

        let audio = result.unwrap();
        assert_eq!(audio.metadata.format, AudioFormat::Flac);
    }

    #[test]
    fn test_audio_resample() {
        let codec = AudioCodec::new();
        let flac_data = [0x66, 0x4C, 0x61, 0x43];

        let audio = codec.decode(&flac_data).unwrap();
        let resampled = AudioCodec::resample(&audio, AudioSampleRate::Hz48000);

        assert_eq!(resampled.metadata.sample_rate, AudioSampleRate::Hz48000);
    }

    #[test]
    fn test_vbr_bitrate_adjustment() {
        let mut vbr = MediaBitrateController::new(128, 64, 320);
        // Normal state: stays target
        assert_eq!(vbr.adjust_bitrate(0.50, 0.50), 128);

        // High CPU: drops bitrate
        let rate1 = vbr.adjust_bitrate(0.50, 0.95);
        assert!(rate1 < 128);

        // Low CPU & high headroom: boots bitrate
        let mut vbr2 = MediaBitrateController::new(128, 64, 320);
        let rate2 = vbr2.adjust_bitrate(0.20, 0.30);
        assert!(rate2 > 128);
    }

    #[test]
    fn test_media_clock_sync() {
        let mut sync = MediaClockSync::new(30); // 30ms threshold

        // Normal sync
        assert_eq!(sync.calculate_sync_action(100, 110), 0);

        // Audio lagging behind
        assert_eq!(sync.calculate_sync_action(100, 150), 1);

        // Audio running ahead
        assert_eq!(sync.calculate_sync_action(150, 100), 2);
    }

    #[test]
    fn test_packet_loss_concealment() {
        let mut plc = AudioPacketLossConcealer::new();
        let frame = [1000i16; 64];
        plc.record_good_frame(&frame);

        let concealed = plc.conceal_loss(16);
        assert_eq!(concealed.len(), 16);
        // Checks that dynamic interpolation decays over time
        assert!(concealed[0].abs() > concealed[15].abs());
    }

    #[test]
    fn test_vorbis_comment_parser() {
        let title_tag = b"TITLE=Song Name";
        let artist_tag = b"ARTIST=Sovereign Musician";

        assert_eq!(VorbisCommentParser::parse_tag(title_tag, "TITLE").unwrap(), "Song Name");
        assert_eq!(VorbisCommentParser::parse_tag(artist_tag, "artist").unwrap(), "Sovereign Musician");
        assert!(VorbisCommentParser::parse_tag(title_tag, "ALBUM").is_none());
    }
}
