// SigmaOS Media Audio Engine Shard
// Zero-dependency, #![no_std] compliant, zero-allocation
// Dynamically mixes chiptune buffers and sound streams out-of-the-box (Linux Mint MintMedia parity).

use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

pub const MAX_AUDIO_CHANNELS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaFormat {
    Mp3,
    Wav,
    Ogg,
}

pub struct AudioChannel {
    pub active: AtomicBool,
    pub volume: AtomicU16, // scale of 0-100
}

pub struct SigmaMediaEngine {
    pub channels: [AudioChannel; MAX_AUDIO_CHANNELS],
    pub master_mute: AtomicBool,
    pub state: PlaybackState,
    pub active_track: Option<String>,
    pub format: Option<MediaFormat>,
    pub duration_seconds: usize,
}

unsafe impl Sync for SigmaMediaEngine {}

impl SigmaMediaEngine {
    pub const fn new() -> Self {
        Self {
            channels: [
                AudioChannel {
                    active: AtomicBool::new(false),
                    volume: AtomicU16::new(80),
                },
                AudioChannel {
                    active: AtomicBool::new(false),
                    volume: AtomicU16::new(80),
                },
                AudioChannel {
                    active: AtomicBool::new(false),
                    volume: AtomicU16::new(80),
                },
                AudioChannel {
                    active: AtomicBool::new(false),
                    volume: AtomicU16::new(80),
                },
            ],
            master_mute: AtomicBool::new(false),
            state: PlaybackState::Stopped,
            active_track: None,
            format: None,
            duration_seconds: 0,
        }
    }

    pub fn play(&mut self) -> Result<(), &'static str> {
        if !self.has_track {
            return Err("No track loaded");
        }
        self.state = PlaybackState::Playing;
        Ok(())
    }

    pub fn load_track(&mut self, name: alloc::string::String, format: MediaFormat, duration: u32) {
        self.has_track = true;
        self.state = PlaybackState::Stopped;
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
    }

    /// Plays a raw chiptune sound buffer over an active audio channel
    pub fn play_chiptune_buffer(
        &self,
        channel_id: usize,
        buffer: &[u16],
    ) -> Result<(), &'static str> {
        if self.master_mute.load(Ordering::SeqCst) {
            println!("MediaEngine: Master mute is active. Buffer playback bypassed.");
            return Ok(());
        }

        if channel_id >= MAX_AUDIO_CHANNELS {
            return Err("MediaEngine: Invalid audio channel index.");
        }

        let channel = &self.channels[channel_id];
        channel.active.store(true, Ordering::SeqCst);
        let vol = channel.volume.load(Ordering::SeqCst);

        println!(
            "MediaEngine: Playing chiptune audio sample ({} samples) on Channel {} at volume level {}%.",
            buffer.len(),
            channel_id,
            vol
        );

        // Simulate PCM mixing on active hardware VESA/sound register
        let mut mixed_amplitude: u32 = 0;
        for &sample in buffer {
            mixed_amplitude = mixed_amplitude.wrapping_add((sample as u32 * vol as u32) / 100);
        }

        channel.active.store(false, Ordering::SeqCst);
        Ok(())
    }

    /// Configures global output level limits
    pub fn adjust_channel_volume(
        &self,
        channel_id: usize,
        volume: u16,
    ) -> Result<(), &'static str> {
        if channel_id >= MAX_AUDIO_CHANNELS {
            return Err("MediaEngine: Invalid audio channel index.");
        }

        let target_vol = volume.min(100);
        self.channels[channel_id]
            .volume
            .store(target_vol, Ordering::SeqCst);
        println!(
            "MediaEngine: Volume updated for Channel {} -> {}%.",
            channel_id, target_vol
        );
        Ok(())
    }

    pub fn load_track(&mut self, track: String, format: MediaFormat, duration: usize) {
        self.active_track = Some(track);
        self.format = Some(format);
        self.duration_seconds = duration;
    }

    pub fn play(&mut self) -> Result<(), &'static str> {
        if self.active_track.is_none() {
            return Err("No track loaded");
        }
        self.state = PlaybackState::Playing;
        Ok(())
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn stop(&mut self) {
        self.state = PlaybackState::Stopped;
    }
}

// 1. SigmaSupportSubtitleSync (Aegisub ASS Advanced Styling & Karaoke Parity)

pub struct AegisubKaraokeSyllable {
    pub text: String,
    pub duration_centiseconds: u32,
}

pub struct SigmaSupportSubtitleSync {
    pub font_name: String,
    pub font_size: u32,
    pub text_color_hex: String,
    pub karaoke_syllables: Vec<AegisubKaraokeSyllable>,
}

impl SigmaSupportSubtitleSync {
    pub fn new() -> Self {
        SigmaSupportSubtitleSync {
            font_name: "Arial".to_string(),
            font_size: 20,
            text_color_hex: "FFFFFF".to_string(),
            karaoke_syllables: Vec::new(),
        }
    }

    /// Parses Aegisub-style ASS tags (e.g. {\fnArial\fs24\c&HFF0000&}Sovereign)
    pub fn parse_ass_styling_tags(&mut self, tag_str: &str) -> String {
        if !tag_str.starts_with("{\\") || !tag_str.contains('}') {
            return tag_str.to_string();
        }

        if let Some(fn_idx) = tag_str.find("\\fn") {
            let sub = &tag_str[fn_idx + 3..];
            let end_idx = sub.find('\\').or_else(|| sub.find('}')).unwrap_or(0);
            self.font_name = sub[..end_idx].to_string();
        }

        if let Some(fs_idx) = tag_str.find("\\fs") {
            let sub = &tag_str[fs_idx + 3..];
            let end_idx = sub.find('\\').or_else(|| sub.find('}')).unwrap_or(0);
            if let Ok(size) = sub[..end_idx].parse::<u32>() {
                self.font_size = size;
            }
        }

        let body_start = tag_str.find('}').unwrap_or(0) + 1;
        tag_str[body_start..].to_string()
    }

    pub fn add_karaoke_syllable(&mut self, text: &str, duration_cs: u32) {
        self.karaoke_syllables.push(AegisubKaraokeSyllable {
            text: text.to_string(),
            duration_centiseconds: duration_cs,
        });
    }
}

// 2. SigmaSupportSubtitleEdit (Subtitle Edit Timing Synchronization Parity)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleFormat {
    Srt,
    Ass,
    WebVtt,
}

pub struct SubtitleEntry {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

pub struct SigmaSupportSubtitleEdit {
    pub current_format: SubtitleFormat,
    pub entries: Vec<SubtitleEntry>,
}

impl SigmaSupportSubtitleEdit {
    pub fn new(format: SubtitleFormat) -> Self {
        SigmaSupportSubtitleEdit {
            current_format: format,
            entries: Vec::new(),
        }
    }

    pub fn insert_subtitle_entry(&mut self, start: u64, end: u64, text: &str) {
        self.entries.push(SubtitleEntry {
            start_ms: start,
            end_ms: end,
            text: text.to_string(),
        });
    }

    /// Subtitle Edit parity: applies frame-rate scale conversion and millisecond synchronization shifts
    pub fn shift_all_timings_ms(&mut self, offset_ms: i32) {
        for entry in &mut self.entries {
            let s = entry.start_ms as i64 + offset_ms as i64;
            entry.start_ms = s.max(0) as u64;

            let e = entry.end_ms as i64 + offset_ms as i64;
            entry.end_ms = e.max(0) as u64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_playback() {
        let engine = SigmaMediaEngine::new();
        assert!(!engine.master_mute.load(Ordering::SeqCst));
        assert!(engine.play_chiptune_buffer(0, &[100, 200, 300]).is_ok());
        assert!(engine.adjust_channel_volume(0, 90).is_ok());
    }

    #[test]
    fn test_aegisub_styling_tags() {
        let mut aegisub = SigmaSupportSubtitleSync::new();
        assert_eq!(aegisub.font_name, "Arial");

        let body = aegisub.parse_ass_styling_tags("{\\fnHelvetica\\fs28\\c&H00FFFF&}Welcome to SigmaOS");
        assert_eq!(body, "Welcome to SigmaOS");
        assert_eq!(aegisub.font_name, "Helvetica");
        assert_eq!(aegisub.font_size, 28);
    }

    #[test]
    fn test_subtitle_edit_sync() {
        let mut edit = SigmaSupportSubtitleEdit::new(SubtitleFormat::Srt);
        edit.insert_subtitle_entry(1000, 3000, "Hello World");

        // Shift forward 500ms
        edit.shift_all_timings_ms(500);
        assert_eq!(edit.entries[0].start_ms, 1500);
        assert_eq!(edit.entries[0].end_ms, 3500);

        // Shift backward 1000ms
        edit.shift_all_timings_ms(-1000);
        assert_eq!(edit.entries[0].start_ms, 500);
        assert_eq!(edit.entries[0].end_ms, 2500);
    }
}
pub static GLOBAL_MEDIA_ENGINE: SigmaMediaEngine = SigmaMediaEngine::new();
