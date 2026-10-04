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

// Audio systems: validated integer PCM WAV import/export plus audio timing and metadata models.
// FLAC, MP3, and Vorbis signatures are recognized but their decoders are not implemented.

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
            self.current_bitrate_kbps =
                (self.current_bitrate_kbps * 4 / 5).max(self.min_bitrate_kbps);
        } else if buffer_fullness < 0.40 && cpu_usage < 0.60 {
            // High resource capacity: boost bitrate to maximize audio fidelity
            self.current_bitrate_kbps =
                (self.current_bitrate_kbps * 5 / 4).min(self.max_bitrate_kbps);
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
        Self {
            last_samples: [0; 64],
        }
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
                if b1 >= b'a' && b1 <= b'z' {
                    b1 -= 32;
                } // convert to uppercase

                let mut b2 = target_key.as_bytes()[i];
                if b2 >= b'a' && b2 <= b'z' {
                    b2 -= 32;
                }

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
        if data.len() >= 4
            && data[0] == 0x66
            && data[1] == 0x4C
            && data[2] == 0x61
            && data[3] == 0x43
        {
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
        if data.len() >= 4
            && data[0] == 0x52
            && data[1] == 0x49
            && data[2] == 0x46
            && data[3] == 0x46
        {
            if data.len() >= 12 && &data[8..12] == b"WAVE" {
                return AudioFormat::Wav;
            }
        }

        // OGG signature: OggS
        if data.len() >= 4
            && data[0] == 0x4F
            && data[1] == 0x67
            && data[2] == 0x67
            && data[3] == 0x53
        {
            return AudioFormat::OggVorbis;
        }

        AudioFormat::Unknown
    }

    /// Decode audio from raw data
    pub fn decode(&self, data: &[u8]) -> Result<DecodedAudio, &'static str> {
        let format = Self::detect_format(data);

        match format {
            AudioFormat::Wav => self.decode_wav(data),
            AudioFormat::Flac | AudioFormat::Mp3 | AudioFormat::OggVorbis => {
                Err("Codec is recognized but decoding is not implemented")
            }
            _ => Err("Unsupported audio format"),
        }
    }

    /// Decode integer PCM WAV (8/16/24/32-bit, one to eight channels).
    /// Floating-point, compressed, extensible, RF64, and malformed WAV files are rejected.
    fn decode_wav(&self, data: &[u8]) -> Result<DecodedAudio, &'static str> {
        if data.len() < 12 || &data[..4] != b"RIFF" || &data[8..12] != b"WAVE" {
            return Err("Invalid RIFF/WAVE header");
        }
        let riff_size = u32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;
        let riff_end = riff_size
            .checked_add(8)
            .filter(|end| *end >= 12 && *end <= data.len())
            .ok_or("RIFF size exceeds file bounds")?;

        let mut offset = 12usize;
        let mut format_chunk: Option<(u16, u16, u32, u32, u16, u16)> = None;
        let mut data_chunk: Option<(usize, usize)> = None;
        while offset < riff_end {
            let header_end = offset.checked_add(8).ok_or("WAV chunk offset overflow")?;
            if header_end > riff_end {
                return Err("Truncated WAV chunk header");
            }
            let chunk_id = &data[offset..offset + 4];
            let chunk_size =
                u32::from_le_bytes(data[offset + 4..header_end].try_into().unwrap()) as usize;
            let chunk_start = header_end;
            let chunk_end = chunk_start
                .checked_add(chunk_size)
                .filter(|end| *end <= riff_end)
                .ok_or("WAV chunk exceeds RIFF bounds")?;

            if chunk_id == b"fmt " {
                if format_chunk.is_some() || chunk_size < 16 {
                    return Err("Invalid or duplicate WAV format chunk");
                }
                format_chunk = Some((
                    u16::from_le_bytes(data[chunk_start..chunk_start + 2].try_into().unwrap()),
                    u16::from_le_bytes(data[chunk_start + 2..chunk_start + 4].try_into().unwrap()),
                    u32::from_le_bytes(data[chunk_start + 4..chunk_start + 8].try_into().unwrap()),
                    u32::from_le_bytes(data[chunk_start + 8..chunk_start + 12].try_into().unwrap()),
                    u16::from_le_bytes(
                        data[chunk_start + 12..chunk_start + 14].try_into().unwrap(),
                    ),
                    u16::from_le_bytes(
                        data[chunk_start + 14..chunk_start + 16].try_into().unwrap(),
                    ),
                ));
            } else if chunk_id == b"data" {
                if data_chunk.replace((chunk_start, chunk_end)).is_some() {
                    return Err("Multiple WAV data chunks are unsupported");
                }
            }

            let padded_end = chunk_end
                .checked_add(chunk_size & 1)
                .filter(|end| *end <= riff_end)
                .ok_or("Missing WAV chunk padding byte")?;
            offset = padded_end;
        }

        let (encoding, channels, sample_rate, byte_rate, block_align, bits) =
            format_chunk.ok_or("Missing WAV format chunk")?;
        if encoding != 1 {
            return Err("Only integer PCM WAV is supported");
        }
        if channels == 0 || channels > 8 || sample_rate == 0 {
            return Err("Unsupported WAV channel count or sample rate");
        }
        if !matches!(bits, 8 | 16 | 24 | 32) {
            return Err("Unsupported PCM bit depth");
        }
        let expected_align = channels
            .checked_mul(bits / 8)
            .ok_or("WAV block alignment overflow")?;
        let expected_byte_rate = sample_rate
            .checked_mul(block_align as u32)
            .ok_or("WAV byte rate overflow")?;
        if block_align != expected_align || byte_rate != expected_byte_rate {
            return Err("Inconsistent WAV byte rate or block alignment");
        }

        let (sample_start, sample_end) = data_chunk.ok_or("Missing WAV data chunk")?;
        let pcm = &data[sample_start..sample_end];
        if pcm.len() % block_align as usize != 0 {
            return Err("WAV data ends in a partial sample frame");
        }
        let sample_count = pcm.len() / (bits as usize / 8);
        let mut samples = Vec::new();
        samples
            .try_reserve_exact(sample_count)
            .map_err(|_| "WAV sample allocation failed")?;
        for bytes in pcm.chunks_exact(bits as usize / 8) {
            let sample = match bits {
                8 => ((bytes[0] as i32 - 128) << 8) as i16,
                16 => i16::from_le_bytes([bytes[0], bytes[1]]),
                24 => {
                    let value =
                        (bytes[0] as i32) | ((bytes[1] as i32) << 8) | ((bytes[2] as i32) << 16);
                    ((value << 8) >> 16) as i16
                }
                32 => (i32::from_le_bytes(bytes.try_into().unwrap()) >> 16) as i16,
                _ => unreachable!(),
            };
            samples.push(sample);
        }

        let channel_info = match channels {
            1 => AudioChannels::Mono,
            2 => AudioChannels::Stereo,
            6 => AudioChannels::Surround5_1,
            8 => AudioChannels::Surround7_1,
            count => AudioChannels::Custom(count as u8),
        };
        let rate_info = match sample_rate {
            8000 => AudioSampleRate::Hz8000,
            11025 => AudioSampleRate::Hz11025,
            16000 => AudioSampleRate::Hz16000,
            22050 => AudioSampleRate::Hz22050,
            44100 => AudioSampleRate::Hz44100,
            48000 => AudioSampleRate::Hz48000,
            96000 => AudioSampleRate::Hz96000,
            rate => AudioSampleRate::Custom(rate),
        };
        let frames = pcm.len() / block_align as usize;
        Ok(DecodedAudio {
            metadata: AudioMetadata {
                format: AudioFormat::Wav,
                sample_rate: rate_info,
                channels: channel_info,
                bits_per_sample: 16,
                duration_seconds: frames as f32 / sample_rate as f32,
                bitrate: byte_rate.saturating_mul(8) / 1000,
            },
            samples,
        })
    }

    /// Encode interleaved signed 16-bit PCM samples as a canonical RIFF/WAVE file.
    pub fn encode_wav_pcm16(
        sample_rate: u32,
        channels: u16,
        samples: &[i16],
    ) -> Result<Vec<u8>, &'static str> {
        if sample_rate == 0 || channels == 0 || channels > 8 {
            return Err("Unsupported WAV channel count or sample rate");
        }
        if samples.len() % channels as usize != 0 {
            return Err("PCM sample count is not a whole number of frames");
        }
        let data_size = samples
            .len()
            .checked_mul(2)
            .filter(|size| *size <= u32::MAX as usize - 36)
            .ok_or("WAV output exceeds RIFF size limit")?;
        let byte_rate = sample_rate
            .checked_mul(channels as u32 * 2)
            .ok_or("WAV byte rate overflow")?;
        let block_align = channels * 2;
        let output_size = data_size
            .checked_add(44)
            .ok_or("WAV output size overflow")?;
        let mut output = Vec::new();
        output
            .try_reserve_exact(output_size)
            .map_err(|_| "WAV output allocation failed")?;
        output.extend_from_slice(b"RIFF");
        output.extend_from_slice(&((36 + data_size) as u32).to_le_bytes());
        output.extend_from_slice(b"WAVEfmt ");
        output.extend_from_slice(&16u32.to_le_bytes());
        output.extend_from_slice(&1u16.to_le_bytes());
        output.extend_from_slice(&channels.to_le_bytes());
        output.extend_from_slice(&sample_rate.to_le_bytes());
        output.extend_from_slice(&byte_rate.to_le_bytes());
        output.extend_from_slice(&block_align.to_le_bytes());
        output.extend_from_slice(&16u16.to_le_bytes());
        output.extend_from_slice(b"data");
        output.extend_from_slice(&(data_size as u32).to_le_bytes());
        for sample in samples {
            output.extend_from_slice(&sample.to_le_bytes());
        }
        Ok(output)
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
    fn short_riff_and_ogg_prefixes_are_safe_unknown_formats() {
        assert_eq!(AudioCodec::detect_format(b"RIF"), AudioFormat::Unknown);
        assert_eq!(AudioCodec::detect_format(b"Ogg"), AudioFormat::Unknown);
    }

    #[test]
    fn pcm_wav_round_trip_preserves_samples_and_metadata() {
        let codec = AudioCodec::new();
        let expected = [-32768, -1, 0, 1, 32767];
        let wav = AudioCodec::encode_wav_pcm16(44100, 1, &expected).unwrap();
        let audio = codec.decode(&wav).unwrap();
        assert_eq!(audio.metadata.format, AudioFormat::Wav);
        assert_eq!(audio.metadata.sample_rate, AudioSampleRate::Hz44100);
        assert_eq!(audio.metadata.channels, AudioChannels::Mono);
        assert_eq!(audio.metadata.bits_per_sample, 16);
        assert_eq!(audio.samples, expected);
        assert!((audio.metadata.duration_seconds - 5.0 / 44100.0).abs() < 1e-7);
    }

    #[test]
    fn wav_decoder_converts_supported_integer_depths_to_signed_16_bit() {
        fn fixture(bits: u16, pcm: &[u8]) -> Vec<u8> {
            let bytes_per_sample = bits / 8;
            let byte_rate = 8000u32 * bytes_per_sample as u32;
            let data_size = pcm.len() as u32;
            let mut wav = Vec::new();
            wav.extend_from_slice(b"RIFF");
            wav.extend_from_slice(&(36 + data_size + (data_size & 1)).to_le_bytes());
            wav.extend_from_slice(b"WAVEfmt ");
            wav.extend_from_slice(&16u32.to_le_bytes());
            wav.extend_from_slice(&1u16.to_le_bytes());
            wav.extend_from_slice(&1u16.to_le_bytes());
            wav.extend_from_slice(&8000u32.to_le_bytes());
            wav.extend_from_slice(&byte_rate.to_le_bytes());
            wav.extend_from_slice(&bytes_per_sample.to_le_bytes());
            wav.extend_from_slice(&bits.to_le_bytes());
            wav.extend_from_slice(b"data");
            wav.extend_from_slice(&data_size.to_le_bytes());
            wav.extend_from_slice(pcm);
            if data_size & 1 == 1 {
                wav.push(0);
            }
            wav
        }

        let codec = AudioCodec::new();
        let eight_bit = codec.decode(&fixture(8, &[0, 128, 255])).unwrap();
        assert_eq!(eight_bit.samples, [-32768, 0, 32512]);

        let twenty_four_bit = codec
            .decode(&fixture(24, &[0, 0, 128, 255, 255, 127]))
            .unwrap();
        assert_eq!(twenty_four_bit.samples, [-32768, 32767]);

        let thirty_two_bit = codec
            .decode(&fixture(32, &[0, 0, 0, 128, 255, 255, 255, 127]))
            .unwrap();
        assert_eq!(thirty_two_bit.samples, [-32768, 32767]);
    }

    #[test]
    fn test_audio_resample() {
        let codec = AudioCodec::new();
        let wav = AudioCodec::encode_wav_pcm16(44100, 2, &[1, -1, 2, -2]).unwrap();
        let audio = codec.decode(&wav).unwrap();
        let resampled = AudioCodec::resample(&audio, AudioSampleRate::Hz48000);

        assert_eq!(resampled.metadata.sample_rate, AudioSampleRate::Hz48000);
    }

    #[test]
    fn wav_decoder_rejects_truncated_and_non_pcm_files() {
        let codec = AudioCodec::new();
        let wav = AudioCodec::encode_wav_pcm16(48000, 2, &[1, 2]).unwrap();
        assert!(codec.decode(&wav[..wav.len() - 1]).is_err());

        let mut compressed = wav;
        compressed[20] = 3;
        assert!(matches!(
            codec.decode(&compressed),
            Err("Only integer PCM WAV is supported")
        ));
    }

    #[test]
    fn wav_encoder_rejects_incomplete_channel_frames() {
        assert_eq!(
            AudioCodec::encode_wav_pcm16(44100, 2, &[100]),
            Err("PCM sample count is not a whole number of frames")
        );
        assert_eq!(
            AudioCodec::encode_wav_pcm16(0, 1, &[100]),
            Err("Unsupported WAV channel count or sample rate")
        );
    }

    #[test]
    fn recognized_compressed_formats_do_not_return_synthetic_silence() {
        let codec = AudioCodec::new();
        assert!(matches!(
            codec.decode(b"fLaC"),
            Err("Codec is recognized but decoding is not implemented")
        ));
        assert!(matches!(
            codec.decode(b"ID3"),
            Err("Codec is recognized but decoding is not implemented")
        ));
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

        assert_eq!(
            VorbisCommentParser::parse_tag(title_tag, "TITLE").unwrap(),
            "Song Name"
        );
        assert_eq!(
            VorbisCommentParser::parse_tag(artist_tag, "artist").unwrap(),
            "Sovereign Musician"
        );
        assert!(VorbisCommentParser::parse_tag(title_tag, "ALBUM").is_none());
    }
}
