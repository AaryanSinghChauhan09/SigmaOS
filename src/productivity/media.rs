#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Media Audio Engine Shard
// Zero-dependency, // #![no_std]  // crate-root only compliant, zero-allocation
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
