#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// Audio Editor Core (Audacity Parity)
/// Non-destructive multi-track audio mixing engine.

pub struct AudioTrack {
    pub name: &'static str,
    pub pcm_data: Vec<i16>,
    pub volume: f32,
    pub is_muted: bool,
}

pub struct AudioMixer {
    pub tracks: Vec<AudioTrack>,
    pub sample_rate: u32,
}

impl AudioMixer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            tracks: Vec::new(),
            sample_rate,
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    /// Mixes all tracks down into a single PCM buffer
    pub fn mixdown(&self) -> Vec<i16> {
        if self.tracks.is_empty() {
            return Vec::new();
        }
        
        let max_len = self.tracks.iter().map(|t| t.pcm_data.len()).max().unwrap_or(0);
        let mut mixed = alloc::vec![0_i16; max_len];

        for track in &self.tracks {
            if track.is_muted {
                continue;
            }
            for (i, &sample) in track.pcm_data.iter().enumerate() {
                // Apply volume scaling and add to master mix with clamping to prevent overflow
                let scaled = (sample as f32 * track.volume) as i32;
                let current = mixed[i] as i32;
                let sum = current + scaled;
                mixed[i] = if sum > i16::MAX as i32 {
                    i16::MAX
                } else if sum < i16::MIN as i32 {
                    i16::MIN
                } else {
                    sum as i16
                };
            }
        }
        mixed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_mixdown() {
        let mut mixer = AudioMixer::new(44100);
        mixer.add_track(AudioTrack {
            name: "Vocals",
            pcm_data: alloc::vec![1000, 2000, 3000],
            volume: 1.0,
            is_muted: false,
        });
        mixer.add_track(AudioTrack {
            name: "Guitar",
            pcm_data: alloc::vec![500, 1000, 1500],
            volume: 0.5, // Will contribute 250, 500, 750
            is_muted: false,
        });
        
        let mix = mixer.mixdown();
        assert_eq!(mix[0], 1250);
        assert_eq!(mix[2], 3750);
    }
}
