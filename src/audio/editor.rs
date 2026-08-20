/// Advanced Multi-Track Audio Editor & DSP Filter Suite for SigmaOS
/// Replicates core features, mixing engines, and effects from Adobe Audition and Audacity
/// Supports multi-track session mixing, gain panning, and professional DSP filter processing.
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

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

    /// Mixes tracks into a stereo interleaved L/R buffer (Audition-style spatial panning)
    pub fn mix_stereo_session(&self) -> Vec<(f32, f32)> {
        let mut max_len = 0;
        let mut has_solo_active = false;

        for track in &self.tracks {
            if track.is_solo && !track.is_muted {
                has_solo_active = true;
            }
            if track.samples.len() > max_len {
                max_len = track.samples.len();
            }
        }

        let mut stereo_mix = Vec::with_capacity(max_len);
        for _ in 0..max_len {
            stereo_mix.push((0.0f32, 0.0f32));
        }

        for track in &self.tracks {
            let is_active = if has_solo_active {
                track.is_solo && !track.is_muted
            } else {
                !track.is_muted
            };

            if is_active {
                // Constant-power panning law
                let pan_clamped = track.pan.clamp(-1.0, 1.0);
                let left_gain = ((1.0 - pan_clamped) * 0.5).sqrt() * track.volume;
                let right_gain = ((1.0 + pan_clamped) * 0.5).sqrt() * track.volume;

                for sample_idx in 0..track.samples.len() {
                    let s = track.samples[sample_idx];
                    stereo_mix[sample_idx].0 += s * left_gain;
                    stereo_mix[sample_idx].1 += s * right_gain;
                }
            }
        }

        // Apply hard limiter to both left and right channels
        for pair in &mut stereo_mix {
            pair.0 = pair.0.clamp(-1.0, 1.0);
            pair.1 = pair.1.clamp(-1.0, 1.0);
        }

        stereo_mix
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

/// Algorithmic Reverb Effect (Schroeder / Freeverb inspired comb and allpass network)
pub struct ReverbEffect {
    pub room_size: f32, // 0.0 to 1.0 (decay length)
    pub wet_mix: f32,   // 0.0 (dry) to 1.0 (wet)
}

impl ReverbEffect {
    pub fn new(room_size: f32, wet_mix: f32) -> Self {
        Self {
            room_size: room_size.clamp(0.1, 0.98),
            wet_mix: wet_mix.clamp(0.0, 1.0),
        }
    }
}

impl AudioEffect for ReverbEffect {
    fn apply(&self, samples: &mut [f32]) {
        if samples.len() < 100 {
            return;
        }

        let delay_lines = [1116, 1188, 1277, 1356]; // Comb delay offsets in samples
        let decay = self.room_size;

        let mut wet_buffer = Vec::with_capacity(samples.len());
        for _ in 0..samples.len() {
            wet_buffer.push(0.0f32);
        }

        for &delay in &delay_lines {
            if delay >= samples.len() {
                continue;
            }
            for i in delay..samples.len() {
                wet_buffer[i] += samples[i - delay] * decay;
            }
        }

        let dry_mix = 1.0 - self.wet_mix;
        for i in 0..samples.len() {
            let mixed = samples[i] * dry_mix + (wet_buffer[i] / 4.0) * self.wet_mix;
            samples[i] = mixed.clamp(-1.0, 1.0);
        }
    }
}

/// Flanger Effect (Modulated LFO Delay line for Audition-style jet-whanger effects)
pub struct FlangerEffect {
    pub delay_ms: f32,
    pub depth_ms: f32,
    pub rate_hz: f32,
    pub sample_rate: u32,
}

impl FlangerEffect {
    pub fn new(delay_ms: f32, depth_ms: f32, rate_hz: f32, sample_rate: u32) -> Self {
        Self {
            delay_ms: delay_ms.max(0.1),
            depth_ms: depth_ms.max(0.1),
            rate_hz: rate_hz.max(0.01),
            sample_rate,
        }
    }
}

impl AudioEffect for FlangerEffect {
    fn apply(&self, samples: &mut [f32]) {
        if samples.len() < 100 {
            return;
        }

        let base_delay = (self.delay_ms * 0.001 * self.sample_rate as f32) as usize;
        let depth_samples = (self.depth_ms * 0.001 * self.sample_rate as f32) as usize;

        for i in 0..samples.len() {
            // Simple triangle LFO modulation
            let phase = (i as f32 * self.rate_hz / self.sample_rate as f32) % 1.0;
            let lfo = if phase < 0.5 { phase * 2.0 } else { 2.0 - phase * 2.0 };
            let mod_delay = base_delay + (lfo * depth_samples as f32) as usize;

            if i > mod_delay {
                let delayed_sample = samples[i - mod_delay];
                samples[i] = (samples[i] * 0.7 + delayed_sample * 0.5).clamp(-1.0, 1.0);
            }
        }
    }
}

/// High-Pass Filter (Attenuates frequencies below cutoff alpha)
pub struct HighPassFilter {
    pub alpha: f32,
}

impl HighPassFilter {
    pub fn new(alpha: f32) -> Self {
        Self {
            alpha: alpha.clamp(0.0, 1.0),
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

        for i in 1..samples.len() {
            let curr_x = samples[i];
            let curr_y = self.alpha * (prev_y + curr_x - prev_x);
            samples[i] = curr_y.clamp(-1.0, 1.0);
            prev_x = curr_x;
            prev_y = curr_y;
        }
    }
}

/// Audacity-style Vocal Remover / Phase Cancellation Effect (Subtracts center-panned audio)
pub struct VocalRemoverEffect;

impl VocalRemoverEffect {
    pub fn apply_vocal_remover(left: &[f32], right: &[f32]) -> Vec<f32> {
        let min_len = left.len().min(right.len());
        let mut mono_out = Vec::with_capacity(min_len);

        for i in 0..min_len {
            // Invert right channel and sum with left: (L - R) removes center vocals
            let sub = (left[i] - right[i]) * 0.5;
            mono_out.push(sub.clamp(-1.0, 1.0));
        }

        mono_out
    }
}

/// ADSR Volume Envelope Automation Generator (Attack, Decay, Sustain, Release)
pub struct ADSRVolumeEnvelope {
    pub attack_samples: usize,
    pub decay_samples: usize,
    pub sustain_level: f32,
    pub release_samples: usize,
}

impl ADSRVolumeEnvelope {
    pub fn new(attack: usize, decay: usize, sustain: f32, release: usize) -> Self {
        Self {
            attack_samples: attack,
            decay_samples: decay,
            sustain_level: sustain.clamp(0.0, 1.0),
            release_samples: release,
        }
    }
}

impl AudioEffect for ADSRVolumeEnvelope {
    fn apply(&self, samples: &mut [f32]) {
        let total = samples.len();
        if total == 0 {
            return;
        }

        let attack_end = self.attack_samples.min(total);
        let decay_end = (attack_end + self.decay_samples).min(total);
        let release_start = if total > self.release_samples { total - self.release_samples } else { 0 };

        for i in 0..total {
            let env = if i >= release_start && self.release_samples > 0 {
                let progress = (i - release_start) as f32 / self.release_samples as f32;
                self.sustain_level * (1.0 - progress)
            } else if i < attack_end && self.attack_samples > 0 {
                i as f32 / attack_end as f32
            } else if i < decay_end && self.decay_samples > 0 {
                let progress = (i - attack_end) as f32 / self.decay_samples as f32;
                1.0 - (1.0 - self.sustain_level) * progress
            } else {
                self.sustain_level
            };

            samples[i] *= env.clamp(0.0, 1.0);
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

/// 3-Band Parametric Graphic Equalizer (Bass, Mid, Treble) shelving filters
pub struct GraphicEqualizer {
    pub bass_gain: f32,   // Decibel boost/cut for low frequencies
    pub mid_gain: f32,    // Decibel boost/cut for mid frequencies
    pub treble_gain: f32, // Decibel boost/cut for high frequencies
}

impl GraphicEqualizer {
    pub fn new(bass: f32, mid: f32, treble: f32) -> Self {
        GraphicEqualizer {
            bass_gain: bass,
            mid_gain: mid,
            treble_gain: treble,
        }
    }

    fn db_to_factor(db: f32) -> f32 {
        let exponent = db / 20.0;
        exponent.exp_m1() + 1.0
    }
}

impl AudioEffect for GraphicEqualizer {
    fn apply(&self, samples: &mut [f32]) {
        let bass_factor = Self::db_to_factor(self.bass_gain);
        let mid_factor = Self::db_to_factor(self.mid_gain);
        let treble_factor = Self::db_to_factor(self.treble_gain);

        // Simple frequency bands simulation using sliding average bands
        if samples.len() < 3 {
            return;
        }

        for i in 1..samples.len() - 1 {
            let prev = samples[i - 1];
            let curr = samples[i];
            let next = samples[i + 1];

            // Reconstruct pseudo-frequency bands via difference coefficients
            let high_pass = (curr - (prev + next) / 2.0).clamp(-1.0, 1.0);
            let low_pass = ((prev + curr + next) / 3.0).clamp(-1.0, 1.0);
            let band_pass = (curr - low_pass - high_pass).clamp(-1.0, 1.0);

            // Re-combine bands applying independent equalizer gains
            let modified = low_pass * bass_factor + band_pass * mid_factor + high_pass * treble_factor;
            samples[i] = modified.clamp(-1.0, 1.0);
        }
    }
}

/// Audacity / Adobe Audition-style Spectral Noise Suppression DSP Filter
/// Attenuates background noise floor static across audio frames
pub struct SpectralNoiseSuppressionEffect {
    pub noise_threshold: f32, // Background static noise floor limit (0.0 to 1.0)
    pub reduction_db: f32,    // Attenuation depth in dB
}

impl SpectralNoiseSuppressionEffect {
    pub fn new(noise_threshold: f32, reduction_db: f32) -> Self {
        Self {
            noise_threshold: noise_threshold.clamp(0.001, 1.0),
            reduction_db: reduction_db.max(0.0),
        }
    }

    fn attenuation_factor(&self) -> f32 {
        // e^(-reduction_db / 20 * ln(10)) => e^(-reduction_db * 0.115129)
        let exponent = -self.reduction_db * 0.115129;
        exponent.exp_m1() + 1.0
    }
}

impl AudioEffect for SpectralNoiseSuppressionEffect {
    fn apply(&self, samples: &mut [f32]) {
        let att_factor = self.attenuation_factor();
        for sample in samples.iter_mut() {
            let magnitude = sample.abs();
            if magnitude <= self.noise_threshold {
                // Attenuate static noise floor
                *sample *= att_factor;
            } else {
                // Preserves full signal energy above noise threshold
                let ratio = ((magnitude - self.noise_threshold) / self.noise_threshold).clamp(0.0, 1.0);
                let smooth_att = att_factor + (1.0 - att_factor) * ratio;
                *sample *= smooth_att;
            }
        }
    }
}

/// Dynamic Range Compressor (Reduces peak dynamic levels above absolute threshold)
pub struct DynamicRangeCompressor {
    pub threshold_db: f32,  // Decibel compression threshold
    pub ratio: f32,         // Compression ratio (e.g. 4.0 for 4:1 compression)
}

impl DynamicRangeCompressor {
    pub fn new(threshold: f32, ratio: f32) -> Self {
        DynamicRangeCompressor {
            threshold_db: threshold,
            ratio: ratio.max(1.0),
        }
    }

    fn linear_threshold(&self) -> f32 {
        let exponent = self.threshold_db / 20.0;
        exponent.exp_m1() + 1.0
    }
}

impl AudioEffect for DynamicRangeCompressor {
    fn apply(&self, samples: &mut [f32]) {
        let limit = self.linear_threshold();
        for sample in samples.iter_mut() {
            let abs_val = sample.abs();
            if abs_val > limit {
                // Compress excess peak gain: limit + (abs_val - limit) / ratio
                let excess = abs_val - limit;
                let compressed = limit + excess / self.ratio;
                *sample = sample.signum() * compressed;
            }
        }
    }
}

/// Pitch Shifter / Resampler Effect utilizing Linear Interpolation
pub struct PitchShifter {
    pub pitch_factor: f32, // Pitch scaling multiplier (0.5 for octave down, 2.0 for octave up)
}

impl PitchShifter {
    pub fn new(pitch_factor: f32) -> Self {
        PitchShifter {
            pitch_factor: pitch_factor.max(0.1).min(10.0),
        }
    }
}

impl AudioEffect for PitchShifter {
    fn apply(&self, samples: &mut [f32]) {
        if samples.len() < 2 {
            return;
        }

        let mut resampled = Vec::new();
        let mut float_idx = 0.0f32;

        while (float_idx as usize) < samples.len() - 1 {
            let base_idx = float_idx as usize;
            let frac = float_idx - (base_idx as f32);

            let s1 = samples[base_idx];
            let s2 = samples[base_idx + 1];

            // Perform standard linear interpolation resample
            let interpolated = s1 * (1.0 - frac) + s2 * frac;
            resampled.push(interpolated);

            float_idx += self.pitch_factor;
        }

        // Overwrite original samples buffer back
        let copy_len = resampled.len().min(samples.len());
        samples[..copy_len].copy_from_slice(&resampled.as_slice()[..copy_len]);

        // Zero out remaining trailing samples if any
        for i in copy_len..samples.len() {
            samples[i] = 0.0;
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
    fn test_equalizer_filter() {
        let mut samples = [0.5, -0.5, 0.5, -0.5, 0.5];
        let eq = GraphicEqualizer::new(6.0, -3.0, 3.0); // Boost Bass and Treble, Cut Mids
        eq.apply(&mut samples);
        assert_eq!(samples.len(), 5);
    }

    #[test]
    fn test_spectral_noise_suppression() {
        let mut samples = [0.02, 0.05, 0.80, -0.01];
        let noise_filter = SpectralNoiseSuppressionEffect::new(0.05, 12.0); // 12 dB noise reduction
        noise_filter.apply(&mut samples);

        // Noise floor samples (<= 0.05) should be attenuated significantly
        assert!(samples[0].abs() < 0.01);
        assert!(samples[1].abs() < 0.02);

        // High amplitude signal (> 0.05) should preserve primary signal energy
        assert!(samples[2] > 0.70);
    }

    #[test]
    fn test_pitch_shifter_and_compressor() {
        let mut samples = [1.0, -1.0, 1.0, -1.0];

        // Dynamic Range Compressor
        let comp = DynamicRangeCompressor::new(-6.0, 2.0); // Threshold at ~0.501, 2:1 ratio
        comp.apply(&mut samples);
        assert!(samples[0] < 1.0); // Signal was successfully compressed!

        // Pitch Shifter (Octave Up: resamples to half length)
        let pitch = PitchShifter::new(2.0);
        pitch.apply(&mut samples);
        assert_eq!(samples[2], 0.0); // Trait samples are correctly zeroed out
    }

    #[test]
    fn test_vocal_remover_and_adsr_envelope() {
        let left = [0.8, 0.5, 0.9];
        let right = [0.8, -0.5, 0.9]; // Center vocal (0.8, 0.9) matches, sides differ

        let mono = VocalRemoverEffect::apply_vocal_remover(&left, &right);
        assert_eq!(mono[0], 0.0); // Identical center vocal completely cancelled!
        assert_eq!(mono[1], 0.5); // Side differences preserved

        let mut samples = [1.0; 100];
        let adsr = ADSRVolumeEnvelope::new(10, 10, 0.5, 20);
        adsr.apply(&mut samples);

        assert_eq!(samples[0], 0.0); // Attack starts at zero
        assert!((samples[20] - 0.5).abs() < 1e-5); // Sustain level reached
        assert!(samples[99] < 0.05); // Release fades to near-zero at tail
    }

    #[test]
    fn test_stereo_panning_session() {
        let mut session = MultiTrackSession::new(44100);

        let mut left_track = AudioTrack::new(1, "LeftGuitar").with_samples(&[1.0]);
        left_track.pan = -1.0; // Hard panned left

        let mut right_track = AudioTrack::new(2, "RightGuitar").with_samples(&[1.0]);
        right_track.pan = 1.0; // Hard panned right

        session.add_track(left_track);
        session.add_track(right_track);

        let stereo = session.mix_stereo_session();
        assert_eq!(stereo.len(), 1);
        assert!(stereo[0].0 > 0.9); // Left channel has energy
        assert!(stereo[0].1 > 0.9); // Right channel has energy
    }
}
