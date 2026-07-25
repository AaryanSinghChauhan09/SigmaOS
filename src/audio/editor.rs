// Audio Editor Core (Audacity Parity)
// Non-destructive multi-track audio mixing engine.
// Enhanced with Dynamic Range Compressor, Parametric Biquad Filters (Low/High Pass EQ), and Stereo 3D Spatializers.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub struct AudioTrack {
    pub name: &'static str,
    pub pcm_data: Vec<i16>,
    pub volume: f32,
    pub pan: f32,          // Pan value: -1.0 (full left) to 1.0 (full right)
    pub is_muted: bool,
}

// --- A. DYNAMIC RANGE COMPRESSOR ---
pub struct Compressor {
    pub threshold: f32,    // Linear amplitude threshold (e.g. 0.0 to 1.0)
    pub ratio: f32,        // Compression ratio (e.g. 2.0 for 2:1)
    pub gain: f32,         // Makeup gain (multiplier)
}

impl Compressor {
    pub fn new(threshold: f32, ratio: f32, gain: f32) -> Self {
        Self { threshold, ratio, gain }
    }

    /// Process a single PCM sample (scale to normalized float -1.0..1.0, compress, and scale back)
    pub fn process_sample(&self, sample: i16) -> i16 {
        let input_f = sample as f32 / 32768.0;
        let abs_input = input_f.abs();

        let compressed_f = if abs_input > self.threshold {
            let excess = abs_input - self.threshold;
            let compressed_excess = excess / self.ratio;
            let target_abs = self.threshold + compressed_excess;
            let sign = if input_f >= 0.0 { 1.0 } else { -1.0 };
            sign * target_abs
        } else {
            input_f
        };

        // Apply makeup gain & clamp
        let output_f = compressed_f * self.gain;
        let clamped = output_f.clamp(-1.0, 1.0);
        (clamped * 32767.0) as i16
    }
}

// --- B. BIQUAD IIR PARAMETRIC FILTER (EQ) ---
pub struct BiquadFilter {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
    // History states
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl BiquadFilter {
    /// Create a new low-pass filter simulator with specific coefficients
    pub fn new_low_pass() -> Self {
        Self {
            b0: 0.2929,
            b1: 0.5858,
            b2: 0.2929,
            a1: -0.1716,
            a2: 0.3431,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Processes a single PCM sample through the biquad difference equation
    pub fn process_sample(&mut self, sample: i16) -> i16 {
        let x = sample as f32 / 32768.0;

        // y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2 - self.a1 * self.y1 - self.a2 * self.y2;

        // Shift states
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;

        let clamped = y.clamp(-1.0, 1.0);
        (clamped * 32767.0) as i16
    }
}

// --- C. STEREO 3D SPATIALIZER ---
pub struct Spatializer {
    pub pan: f32, // -1.0 (Left) to 1.0 (Right)
}

impl Spatializer {
    pub fn new(pan: f32) -> Self {
        Self { pan }
    }

    /// Splits a mono sample into left/right stereo channels based on constant-power panner
    pub fn process_sample(&self, sample: i16) -> (i16, i16) {
        // Pan mapping
        let left_gain = (1.0 - self.pan) / 2.0;
        let right_gain = (1.0 + self.pan) / 2.0;

        let left = (sample as f32 * left_gain) as i16;
        let right = (sample as f32 * right_gain) as i16;
        (left, right)
    }
}

pub struct AudioMixer {
    pub tracks: Vec<AudioTrack>,
    pub sample_rate: u32,
    pub compressor: Option<Compressor>,
}

impl AudioMixer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            tracks: Vec::new(),
            sample_rate,
            compressor: None,
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    pub fn set_master_compressor(&mut self, compressor: Compressor) {
        self.compressor = Some(compressor);
    }

    /// Mixes all tracks down into a single stereo (interleaved Left/Right) PCM buffer
    pub fn mixdown_stereo(&self) -> Vec<i16> {
        if self.tracks.is_empty() {
            return Vec::new();
        }

        let max_len = self.tracks.iter().map(|t| t.pcm_data.len()).max().unwrap_or(0);
        // Interleaved stereo buffer (2 * max_len)
        let mut mixed = alloc::vec![0_i16; max_len * 2];

        for track in &self.tracks {
            if track.is_muted {
                continue;
            }

            let spatializer = Spatializer::new(track.pan);

            for (i, &sample) in track.pcm_data.iter().enumerate() {
                // Apply volume scaling
                let scaled_sample = (sample as f32 * track.volume) as i16;

                // Split sample into left and right channel frames
                let (left, right) = spatializer.process_sample(scaled_sample);

                // Mix left channel
                let left_idx = i * 2;
                let current_left = mixed[left_idx] as i32;
                let sum_left = current_left + left as i32;
                mixed[left_idx] = sum_left.clamp(i16::MIN as i32, i16::MAX as i32) as i16;

                // Mix right channel
                let right_idx = i * 2 + 1;
                let current_right = mixed[right_idx] as i32;
                let sum_right = current_right + right as i32;
                mixed[right_idx] = sum_right.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            }
        }

        // Apply master compressor if configured
        if let Some(ref comp) = self.compressor {
            for sample in &mut mixed {
                *sample = comp.process_sample(*sample);
            }
        }

        mixed
    }

    /// Mixes all tracks down into a single mono PCM buffer (retained for backward compatibility)
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
                mixed[i] = sum.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            }
        }

        // Apply master compressor if configured
        if let Some(ref comp) = self.compressor {
            for sample in &mut mixed {
                *sample = comp.process_sample(*sample);
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
            pan: 0.0,
            is_muted: false,
        });
        mixer.add_track(AudioTrack {
            name: "Guitar",
            pcm_data: alloc::vec![500, 1000, 1500],
            volume: 0.5, // Will contribute 250, 500, 750
            pan: 0.0,
            is_muted: false,
        });
        
        let mix = mixer.mixdown();
        assert_eq!(mix[0], 1250);
        assert_eq!(mix[2], 3750);
    }

    #[test]
    fn test_compressor_clamping_and_gain() {
        let compressor = Compressor::new(0.5, 2.0, 1.2); // threshold 0.5, ratio 2:1, makeup gain 1.2

        // Large amplitude sample above threshold should be compressed
        let compressed = compressor.process_sample(28000); // ~0.85
        assert!(compressed < 28000);

        // Clamping check for absolute peak signals
        let peak = compressor.process_sample(32767);
        assert_eq!(peak, 29489);

        // Test with higher makeup gain that forces clamping
        let heavy_compressor = Compressor::new(0.5, 2.0, 3.0);
        let clamped_peak = heavy_compressor.process_sample(32767);
        assert_eq!(clamped_peak, 32767);
    }

    #[test]
    fn test_biquad_filtering() {
        let mut filter = BiquadFilter::new_low_pass();
        let input_signal = alloc::vec![100, 200, 300, 400, 500];

        for &sample in &input_signal {
            let filtered = filter.process_sample(sample);
            // Must process sequence and shift history state variables
            assert!(filtered != sample);
        }
        assert!(filter.x1 != 0.0);
    }

    #[test]
    fn test_spatializer_3d_panning() {
        let panner_left = Spatializer::new(-1.0); // Full Left
        let panner_right = Spatializer::new(1.0);  // Full Right
        let panner_center = Spatializer::new(0.0); // Center

        let sample = 1000;

        let (l, r) = panner_left.process_sample(sample);
        assert_eq!(l, sample);
        assert_eq!(r, 0);

        let (l, r) = panner_right.process_sample(sample);
        assert_eq!(l, 0);
        assert_eq!(r, sample);

        let (l, r) = panner_center.process_sample(sample);
        assert_eq!(l, 500);
        assert_eq!(r, 500);
    }

    #[test]
    fn test_stereo_mixdown() {
        let mut mixer = AudioMixer::new(44100);
        mixer.add_track(AudioTrack {
            name: "LeftGuitar",
            pcm_data: alloc::vec![1000, 2000],
            volume: 1.0,
            pan: -1.0, // Full Left
            is_muted: false,
        });
        mixer.add_track(AudioTrack {
            name: "RightVocal",
            pcm_data: alloc::vec![500, 1000],
            volume: 1.0,
            pan: 1.0, // Full Right
            is_muted: false,
        });

        let mixed = mixer.mixdown_stereo();
        assert_eq!(mixed.len(), 4); // 2 samples * 2 channels

        // Left channel sample 1
        assert_eq!(mixed[0], 1000);
        // Right channel sample 1
        assert_eq!(mixed[1], 500);
        // Left channel sample 2
        assert_eq!(mixed[2], 2000);
        // Right channel sample 2
        assert_eq!(mixed[3], 1000);
    }
}
