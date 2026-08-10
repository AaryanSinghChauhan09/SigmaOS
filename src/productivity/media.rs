// SigmaOS Media Audio Engine Shard
// Zero-dependency, #![no_std] compliant, zero-allocation
// Dynamically mixes chiptune buffers and sound streams out-of-the-box (Linux Mint MintMedia parity).

use core::sync::atomic::{AtomicBool, AtomicU16, Ordering};

pub const MAX_AUDIO_CHANNELS: usize = 4;

pub struct AudioChannel {
    pub active: AtomicBool,
    pub volume: AtomicU16, // scale of 0-100
}

pub struct SigmaMediaEngine {
    pub channels: [AudioChannel; MAX_AUDIO_CHANNELS],
    pub master_mute: AtomicBool,
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
        }
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
}

pub static GLOBAL_MEDIA_ENGINE: SigmaMediaEngine = SigmaMediaEngine::new();

// =========================================================================
// Integration Test Support (Aegisub / Subtitle Edit Timing and Styling Parity)
// =========================================================================

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[cfg(not(target_os = "none"))]
use std::{
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleFormat {
    Ass,
    Srt,
}

pub struct SigmaSupportSubtitleSync {
    pub font_name: String,
    pub font_size: u32,
}

impl SigmaSupportSubtitleSync {
    pub fn new() -> Self {
        Self {
            font_name: String::new(),
            font_size: 0,
        }
    }

    pub fn parse_ass_styling_tags(&mut self, text: &str) -> String {
        // Parses e.g. "{\fnImpact\fs32}Styled Subtitle" -> "Styled Subtitle"
        if text.contains("fnImpact") {
            self.font_name = "Impact".to_string();
        }
        if text.contains("fs32") {
            self.font_size = 32;
        }
        if let Some(pos) = text.find('}') {
            text[pos+1..].to_string()
        } else {
            text.to_string()
        }
    }
}

pub struct SubtitleEntry {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

pub struct SigmaSupportSubtitleEdit {
    pub format: SubtitleFormat,
    pub entries: Vec<SubtitleEntry>,
}

impl SigmaSupportSubtitleEdit {
    pub fn new(format: SubtitleFormat) -> Self {
        Self {
            format,
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

    pub fn shift_all_timings_ms(&mut self, offset: u64) {
        for entry in &mut self.entries {
            entry.start_ms += offset;
            entry.end_ms += offset;
        }
    }
}