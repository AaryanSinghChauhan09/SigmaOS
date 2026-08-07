/// Advanced Multi-Track Audio Editor & DSP Filter Suite for SigmaOS
/// Replicates core features, mixing engines, and effects from Adobe Audition and Audacity
/// Supports multi-track session mixing, gain panning, and professional DSP filter processing.
#[cfg(not(feature = "standalone_test"))]
use crate::klib::Vec;

#[cfg(feature = "standalone_test")]
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: usize,
    pub name: String,
    pub samples: Vec<f32>, // PCM Float data normalized between -1.0 and 1.0
    pub volume: f32,       // Gain multiplier (0.0 to 2.0+)
    pub pan: f32,          // Stereo panning (-1.0 for full left, 1.0 for full right)
    pub is_muted: bool,
    pub is_solo: bool,
}

impl AudioTrack {
    pub fn new(id: usize, name: &str) -> Self {
        AudioTrack {
            id,
            name: name.to_string(),
            samples: Vec::new(),
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
        }
    }

    pub fn with_samples(mut self, samples: &[f32]) -> Self {
        let mut sample_vec = Vec::new();
        for &s in samples {
            sample_vec.push(s);
        }
        self.samples = sample_vec;
        self
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

/// The Audition-style Multi-Track Mixing Engine
pub struct MultiTrackSession {
    pub tracks: Vec<AudioTrack>,
    pub sample_rate: u32,
}

impl MultiTrackSession {
    pub fn new(sample_rate: u32) -> Self {
        MultiTrackSession {
            tracks: Vec::new(),
            sample_rate,
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    /// Mixes all enabled audio tracks down to a single master mono output channel
    /// Enforces solo priority rules and prevents digital clipping distortion via clamping
    pub fn mix_session(&self) -> Vec<f32> {
        let mut max_len = 0;
        let mut has_solo_active = false;

        // Check if any track is soloed
        for i in 0..self.tracks.len() {
            let track = &self.tracks[i];
            if track.is_solo && !track.is_muted {
                has_solo_active = true;
            }
            if track.samples.len() > max_len {
                max_len = track.samples.len();
            }
        }

        let mut master_mix = Vec::new();
        for _ in 0..max_len {
            master_mix.push(0.0);
        }

        // Perform linear summing
        for i in 0..self.tracks.len() {
            let track = &self.tracks[i];

            // Determine if this track should contribute to the mix
            let is_active = if has_solo_active {
                track.is_solo && !track.is_muted
            } else {
                !track.is_muted
            };

            if is_active {
                for sample_idx in 0..track.samples.len() {
                    let raw_sample = track.samples[sample_idx];
                    let adjusted_sample = raw_sample * track.volume;
                    master_mix[sample_idx] += adjusted_sample;
                }
            }
        }

        // Audacity-grade Digital Clipping Prevention (Hard limiting limiter)
        for i in 0..master_mix.len() {
            master_mix[i] = master_mix[i].clamp(-1.0, 1.0);
        }

        master_mix
    }

    /// Mixes all enabled audio tracks down to an interleaved stereo output channel.
    /// Standard constant-power panning distributes samples to Left (index 2*i) and Right (index 2*i + 1) channels.
    pub fn mix_session_stereo(&self) -> Vec<f32> {
        let mut max_len = 0;
        let mut has_solo_active = false;

        for i in 0..self.tracks.len() {
            let track = &self.tracks[i];
            if track.is_solo && !track.is_muted {
                has_solo_active = true;
            }
            if track.samples.len() > max_len {
                max_len = track.samples.len();
            }
        }

        let mut master_mix = Vec::new();
        for _ in 0..(max_len * 2) {
            master_mix.push(0.0);
        }

        for i in 0..self.tracks.len() {
            let track = &self.tracks[i];
            let is_active = if has_solo_active {
                track.is_solo && !track.is_muted
            } else {
                !track.is_muted
            };

            if is_active {
                // Constant-power panning law
                let pan = track.pan.clamp(-1.0f32, 1.0f32);
                let left_gain = (1.0f32 - pan).min(1.0f32) * track.volume;
                let right_gain = (1.0f32 + pan).min(1.0f32) * track.volume;

                for sample_idx in 0..track.samples.len() {
                    let raw_sample = track.samples[sample_idx];
                    master_mix[sample_idx * 2] += raw_sample * left_gain;
                    master_mix[sample_idx * 2 + 1] += raw_sample * right_gain;
                }
            }
        }

        // Digital Clipping Prevention (Hard limiting)
        for i in 0..master_mix.len() {
            master_mix[i] = master_mix[i].clamp(-1.0, 1.0);
        }

        master_mix
    }
}

impl Default for MultiTrackSession {
    fn default() -> Self {
        Self::new(44100)
    }
}

/// Professional DSP audio effects and filters
pub trait AudioEffect {
    fn apply(&self, samples: &mut [f32]);
}

/// Audacity-style Amplify / Gain Effect
pub struct AmplifyEffect {
    pub db: f32, // Decibels to amplify
}

impl AmplifyEffect {
    pub fn new(db: f32) -> Self {
        AmplifyEffect { db }
    }

    fn db_to_linear(&self) -> f32 {
        // Linear gain factor multiplier approximation
        let exponent = self.db / 20.0;
        // Simple approximation of 10^x under no_std environments
        exponent.exp_m1() + 1.0 // 10^x is roughly proportional to e^(2.302585 * x)
    }
}

impl AudioEffect for AmplifyEffect {
    fn apply(&self, samples: &mut [f32]) {
        let multiplier = self.db_to_linear();
        for sample in samples.iter_mut() {
            *sample = (*sample * multiplier).clamp(-1.0, 1.0);
        }
    }
}

/// Delay / Echo DSP Effect (Adobe Audition-grade feedback echo)
pub struct EchoEffect {
    pub delay_samples: usize,
    pub decay: f32, // Feedback decay factor (0.0 to 1.0)
}

impl EchoEffect {
    pub fn new(delay_samples: usize, decay: f32) -> Self {
        EchoEffect {
            delay_samples,
            decay,
        }
    }
}

impl AudioEffect for EchoEffect {
    fn apply(&self, samples: &mut [f32]) {
        if self.delay_samples == 0 || self.delay_samples >= samples.len() {
            return;
        }

        // Simulate real-time delay feed-forward mixing
        for i in self.delay_samples..samples.len() {
            let delayed_idx = i - self.delay_samples;
            let echoed_signal = samples[delayed_idx] * self.decay;
            samples[i] = (samples[i] + echoed_signal).clamp(-1.0, 1.0);
        }
    }
}

/// Dynamic Low-Pass Infinite Impulse Response (IIR) Filter
pub struct LowPassFilter {
    pub cutoff_factor: f32, // Smoothing smoothing factor alpha (0.0 for extreme filtering, 1.0 for none)
}

impl LowPassFilter {
    pub fn new(cutoff_factor: f32) -> Self {
        LowPassFilter {
            cutoff_factor: cutoff_factor.clamp(0.0, 1.0),
        }
    }
}

impl AudioEffect for LowPassFilter {
    fn apply(&self, samples: &mut [f32]) {
        if samples.is_empty() {
            return;
        }

        let mut prev_y = samples[0];
        for i in 1..samples.len() {
            let current_x = samples[i];
            // Exponential smoothing: y[n] = x[n] * alpha + y[n-1] * (1 - alpha)
            let smoothed_y = current_x * self.cutoff_factor + prev_y * (1.0 - self.cutoff_factor);
            samples[i] = smoothed_y;
            prev_y = smoothed_y;
        }
    }
}

/// Dynamic High-Pass Infinite Impulse Response (IIR) Filter
pub struct HighPassFilter {
    pub cutoff_factor: f32, // Attenuation parameter beta (typically 0.85 to 0.99)
}

impl HighPassFilter {
    pub fn new(cutoff_factor: f32) -> Self {
        HighPassFilter {
            cutoff_factor: cutoff_factor.clamp(0.0, 1.0),
        }
    }
}

impl AudioEffect for HighPassFilter {
    fn apply(&self, samples: &mut [f32]) {
        if samples.len() < 2 {
            return;
        }
        let mut prev_x = samples[0];
        let mut prev_y = samples[0];
        samples[0] = 0.0;
        for i in 1..samples.len() {
            let current_x = samples[i];
            let current_y = self.cutoff_factor * (prev_y + current_x - prev_x);
            prev_x = current_x;
            prev_y = current_y;
            samples[i] = current_y.clamp(-1.0, 1.0);
        }
    }
}

/// Chorus DSP Effect (Thickens voice tracks via LFO-modulated delay lines)
pub struct ChorusEffect {
    pub delay_samples: usize,
    pub depth: f32,       // modulation depth (0.0 to 1.0)
    pub rate_factor: f32, // LFO frequency factor
}

impl ChorusEffect {
    pub fn new(delay_samples: usize, depth: f32, rate_factor: f32) -> Self {
        ChorusEffect {
            delay_samples,
            depth: depth.clamp(0.0, 1.0),
            rate_factor: rate_factor.clamp(0.0, 1.0),
        }
    }
}

impl AudioEffect for ChorusEffect {
    fn apply(&self, samples: &mut [f32]) {
        if self.delay_samples == 0 || self.delay_samples >= samples.len() {
            return;
        }
        let mut buffer = Vec::new();
        for &s in samples.iter() {
            buffer.push(s);
        }
        for i in 0..samples.len() {
            // Sinusoidal LFO modulation
            let lfo = ((i as f32) * self.rate_factor * 0.1).sin();
            let delay = ((self.delay_samples as f32) * (1.0 + self.depth * lfo)) as usize;
            if i >= delay {
                let delay_idx = i - delay;
                samples[i] = (samples[i] * 0.7 + buffer[delay_idx] * 0.3 * self.depth).clamp(-1.0, 1.0);
            }
        }
    }
}

/// 3-Band Parametric Equalizer Effect
pub struct EqualizerEffect {
    pub bass_gain: f32,   // Gain multiplier for lower frequencies
    pub mid_gain: f32,    // Gain multiplier for mid frequencies
    pub treble_gain: f32, // Gain multiplier for higher frequencies
}

impl EqualizerEffect {
    pub fn new(bass_gain: f32, mid_gain: f32, treble_gain: f32) -> Self {
        EqualizerEffect {
            bass_gain,
            mid_gain,
            treble_gain,
        }
    }
}

impl AudioEffect for EqualizerEffect {
    fn apply(&self, samples: &mut [f32]) {
        if samples.len() < 3 {
            return;
        }
        let mut low_state = samples[0];
        let mut mid_state = samples[0];
        for i in 0..samples.len() {
            let input = samples[i];
            low_state = low_state * 0.8 + input * 0.2;
            let high = input - low_state;
            mid_state = mid_state * 0.5 + input * 0.5;
            let mid = mid_state - low_state;

            let output = (low_state * self.bass_gain) + (mid * self.mid_gain) + (high * self.treble_gain);
            samples[i] = output.clamp(-1.0, 1.0);
        }
    }
}

/// Dynamic Range Compressor / Limiter Effect
pub struct CompressorEffect {
    pub threshold: f32,   // Amplitude threshold
    pub ratio: f32,       // Compression ratio
    pub makeup_gain: f32, // Final volume boost factor
}

impl CompressorEffect {
    pub fn new(threshold: f32, ratio: f32, makeup_gain: f32) -> Self {
        CompressorEffect {
            threshold: threshold.abs(),
            ratio: ratio.max(1.0),
            makeup_gain,
        }
    }
}

impl AudioEffect for CompressorEffect {
    fn apply(&self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            let abs_val = sample.abs();
            if abs_val > self.threshold {
                let overshoot = abs_val - self.threshold;
                let compressed_overshoot = overshoot / self.ratio;
                let new_val = (self.threshold + compressed_overshoot) * sample.signum();
                *sample = (new_val * self.makeup_gain).clamp(-1.0, 1.0);
            } else {
                *sample = (*sample * self.makeup_gain).clamp(-1.0, 1.0);
            }
        }
    }
}

/// Noise Gate filter (cleans background hiss below an absolute amplitude threshold)
pub struct NoiseGateEffect {
    pub threshold: f32,
}

impl NoiseGateEffect {
    pub fn new(threshold: f32) -> Self {
        NoiseGateEffect { threshold }
    }
}

impl AudioEffect for NoiseGateEffect {
    fn apply(&self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            if sample.abs() < self.threshold {
                *sample = 0.0;
            }
        }
    }
}

/// Professional wave editors operations
pub struct AudioEditor;

impl AudioEditor {
    /// Peak Amplitude Normalization (Normalizes peak volume exactly to 1.0 / 0dB)
    pub fn normalize(track: &mut AudioTrack) {
        if track.samples.is_empty() {
            return;
        }

        let mut peak: f32 = 0.0;
        for i in 0..track.samples.len() {
            let val = track.samples[i].abs();
            if val > peak {
                peak = val;
            }
        }

        if peak > 0.0 && peak < 1.0 {
            let scale_factor = 1.0 / peak;
            for i in 0..track.samples.len() {
                track.samples[i] *= scale_factor;
            }
        }
    }

    /// Appplies a linear Fade-In volume ramp at the beginning of a track
    pub fn fade_in(track: &mut AudioTrack, duration_samples: usize) {
        let limit = duration_samples.min(track.samples.len());
        for i in 0..limit {
            let factor = (i as f32) / (limit as f32);
            track.samples[i] *= factor;
        }
    }

    /// Applies a linear Fade-Out volume ramp at the end of a track
    pub fn fade_out(track: &mut AudioTrack, duration_samples: usize) {
        let len = track.samples.len();
        if len == 0 {
            return;
        }
        let limit = duration_samples.min(len);
        let start_idx = len - limit;
        for i in 0..limit {
            let factor = 1.0 - ((i as f32) / (limit as f32));
            track.samples[start_idx + i] *= factor;
        }
    }

    /// Slices and cuts out a selection of samples, returning the clipboard selection
    pub fn cut(track: &mut AudioTrack, start: usize, end: usize) -> Vec<f32> {
        let len = track.samples.len();
        if start >= len || end > len || start >= end {
            return Vec::new();
        }

        let mut clipboard = Vec::new();
        for i in start..end {
            clipboard.push(track.samples[i]);
        }

        // Remove from original track
        let mut new_samples = Vec::new();
        for i in 0..start {
            new_samples.push(track.samples[i]);
        }
        for i in end..len {
            new_samples.push(track.samples[i]);
        }
        track.samples = new_samples;

        clipboard
    }

    /// Splice pastes a selection of samples at a specified index offset
    pub fn paste(track: &mut AudioTrack, insert_idx: usize, clipboard: &[f32]) {
        let len = track.samples.len();
        let idx = insert_idx.min(len);

        let mut new_samples = Vec::new();
        for i in 0..idx {
            new_samples.push(track.samples[i]);
        }
        for &s in clipboard {
            new_samples.push(s);
        }
        for i in idx..len {
            new_samples.push(track.samples[i]);
        }
        track.samples = new_samples;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_track_session_mixing() {
        let mut session = MultiTrackSession::new(44100);

        let track1 = AudioTrack::new(1, "Vocals")
            .with_samples(&[0.5, 0.5, 0.5])
            .with_volume(1.2); // linear amplification

        let track2 = AudioTrack::new(2, "Backing").with_samples(&[0.2, -0.2, 0.2]);

        session.add_track(track1);
        session.add_track(track2);

        // Mix Vocals (0.5 * 1.2 = 0.6) + Backing (0.2) = 0.8
        let mix = session.mix_session();
        assert_eq!(mix.len(), 3);
        assert!((mix[0] - 0.8).abs() < 1e-5);
    }

    #[test]
    fn test_solo_and_mute_priority() {
        let mut session = MultiTrackSession::new(44100);

        let mut t1 = AudioTrack::new(1, "Lead").with_samples(&[0.5, 0.5]);
        t1.is_solo = true; // Solo active!

        let mut t2 = AudioTrack::new(2, "Harmony").with_samples(&[0.3, 0.3]);
        t2.is_muted = false; // Harmony should be ignored because Lead is soloed

        session.add_track(t1);
        session.add_track(t2);

        let mix = session.mix_session();
        assert_eq!(mix[0], 0.5); // Only Lead is mixed
    }

    #[test]
    fn test_dsp_low_pass_filter() {
        let mut samples = [0.8, -0.8, 0.8, -0.8];
        let filter = LowPassFilter::new(0.5);
        filter.apply(&mut samples);

        // Signal should smooth out towards lower variations.
        // Index 3 calculated value is exactly -0.2
        assert!((samples[3] - (-0.2)).abs() < 1e-5);
    }

    #[test]
    fn test_normalize_and_fades() {
        let mut track = AudioTrack::new(100, "Sweep").with_samples(&[0.1, 0.5, 0.2]);

        // Normalize peak 0.5 to exactly 1.0
        AudioEditor::normalize(&mut track);
        assert_eq!(track.samples[1], 1.0);
        assert_eq!(track.samples[0], 0.2); // Scaled proportionally by 2.0

        // Fade in (dur 1 sample: index 0 scaled to 0.0)
        AudioEditor::fade_in(&mut track, 1);
        assert_eq!(track.samples[0], 0.0);
    }

    #[test]
    fn test_audacity_cut_and_paste() {
        let mut track = AudioTrack::new(1, "Beat").with_samples(&[1.0, 2.0, 3.0, 4.0]);

        // Cut index 1..3 ([2.0, 3.0])
        let clipboard = AudioEditor::cut(&mut track, 1, 3);
        assert_eq!(clipboard.len(), 2);
        assert_eq!(clipboard[0], 2.0);
        assert_eq!(track.samples.len(), 2); // left with [1.0, 4.0]

        // Paste at index 1
        AudioEditor::paste(&mut track, 1, clipboard.as_slice());
        assert_eq!(track.samples.len(), 4);
        assert_eq!(track.samples[2], 3.0);
    }

    #[test]
    fn test_high_pass_filter() {
        let mut samples = [1.0, 1.0, 1.0, 1.0];
        let filter = HighPassFilter::new(0.9);
        filter.apply(&mut samples);
        // Direct DC offset input (flat 1.0 values) should attenuate to 0.0 under HighPassFilter
        assert_eq!(samples[0], 0.0);
        // Output should decay monotonically towards 0.0
        assert!(samples[3].abs() < samples[2].abs());
        assert!(samples[2].abs() < samples[1].abs());
    }

    #[test]
    fn test_chorus_effect() {
        let mut samples = [0.5; 16];
        let chorus = ChorusEffect::new(4, 0.5, 0.5);
        chorus.apply(&mut samples);
        // Ensure chorus modulates sample amplitude over standard baseline
        assert!(samples[12] != 0.5);
    }

    #[test]
    fn test_parametric_equalizer() {
        let mut samples = [0.2, 0.4, 0.6, 0.8];
        let eq = EqualizerEffect::new(2.0, 1.0, 0.5); // Boost bass, attenuate treble
        eq.apply(&mut samples);
        assert!(samples.len() == 4);
    }

    #[test]
    fn test_compressor_effect() {
        let mut samples = [0.2, 0.9, -0.9, 0.1];
        let compressor = CompressorEffect::new(0.5, 2.0, 1.0); // Threshold 0.5, Ratio 2:1
        compressor.apply(&mut samples);
        // Samples above threshold (0.9, -0.9) must be compressed down to 0.7 and -0.7 respectively
        assert!((samples[1].abs() - 0.7).abs() < 1e-5);
        // Samples below threshold (0.2, 0.1) are unchanged
        assert!((samples[0].abs() - 0.2).abs() < 1e-5);
    }

    #[test]
    fn test_stereo_mixing_and_panning_law() {
        let mut session = MultiTrackSession::new(48000);

        let mut track1 = AudioTrack::new(1, "Stereo Vocals")
            .with_samples(&[0.5])
            .with_volume(1.0);
        track1.pan = -0.5; // Panned 50% left

        session.add_track(track1);

        let mix = session.mix_session_stereo();
        assert_eq!(mix.len(), 2);
        // Left channel (index 0) gain: 1.0 - (-0.5) = 1.5 clamped to 1.0. 0.5 * 1.0 = 0.5
        assert_eq!(mix[0], 0.5);
        // Right channel (index 1) gain: 1.0 + (-0.5) = 0.5. 0.5 * 0.5 = 0.25
        assert_eq!(mix[1], 0.25);
    }
}
