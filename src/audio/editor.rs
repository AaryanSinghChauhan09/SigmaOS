//! Advanced Multi-Track Audio Editor & DSP Filter Suite for SigmaOS
//! Replicates core features, mixing engines, and effects from Adobe Audition and Audacity
//! Supports multi-track session mixing, gain panning, automation envelopes, and professional DSP filter processing.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::f32::consts::PI;

/// Math helper approximations for no_std environment
pub mod audio_math {
    use core::f32::consts::PI;

    /// Fast sine approximation for no_std
    pub fn sin(x: f32) -> f32 {
        let mut angle = x % (2.0 * PI);
        if angle > PI {
            angle -= 2.0 * PI;
        } else if angle < -PI {
            angle += 2.0 * PI;
        }

        let b = 4.0 / PI;
        let c = -4.0 / (PI * PI);
        let y = b * angle + c * angle * angle.abs();
        let p = 0.225;
        p * (y * y.abs() - y) + y
    }

    /// Fast cosine approximation
    pub fn cos(x: f32) -> f32 {
        sin(x + PI / 2.0)
    }

    /// Fast hyperbolic tangent approximation for overdrive / wave shaping distortion
    pub fn tanh(x: f32) -> f32 {
        if x < -3.0 {
            -1.0
        } else if x > 3.0 {
            1.0
        } else {
            let x2 = x * x;
            x * (27.0 + x2) / (27.0 + 9.0 * x2)
        }
    }

    /// Linear gain factor multiplier approximation from decibels (dB)
    pub fn db_to_linear(db: f32) -> f32 {
        let exponent = db / 20.0;
        let ln10_x = 2.302_585 * exponent;
        if ln10_x.abs() < 1e-4 {
            1.0 + ln10_x
        } else {
            let z = ln10_x;
            let z2 = z * z;
            let z3 = z2 * z;
            1.0 + z + z2 / 2.0 + z3 / 6.0
        }
    }
}

/// Keyframe point for automation envelopes (volume / panning keyframes)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvelopePoint {
    pub sample_index: usize,
    pub value: f32, // Value at sample index (e.g. 0.0 to 2.0 for volume, -1.0 to 1.0 for pan)
}

impl EnvelopePoint {
    pub fn new(sample_index: usize, value: f32) -> Self {
        Self {
            sample_index,
            value,
        }
    }
}

/// Automation envelope curve for Audition/Audacity-style keyframe parameter modulation
#[derive(Debug, Clone, Default)]
pub struct AutomationEnvelope {
    pub points: Vec<EnvelopePoint>,
}

impl AutomationEnvelope {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point(&mut self, sample_index: usize, value: f32) {
        self.points.push(EnvelopePoint::new(sample_index, value));
        self.points.sort_by(|a, b| a.sample_index.cmp(&b.sample_index));
    }

    /// Evaluates interpolated envelope parameter at a specific sample index
    pub fn evaluate_at(&self, sample_index: usize, default_value: f32) -> f32 {
        if self.points.is_empty() {
            return default_value;
        }

        if sample_index <= self.points[0].sample_index {
            return self.points[0].value;
        }

        let last_point = &self.points[self.points.len() - 1];
        if sample_index >= last_point.sample_index {
            return last_point.value;
        }

        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];
            if sample_index >= p1.sample_index && sample_index <= p2.sample_index {
                let range = (p2.sample_index - p1.sample_index) as f32;
                if range == 0.0 {
                    return p1.value;
                }
                let alpha = (sample_index - p1.sample_index) as f32 / range;
                return p1.value + alpha * (p2.value - p1.value);
            }
        }

        default_value
    }
}

/// Audio Track representing a single audio channel or stereo PCM buffer
#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub id: usize,
    pub name: String,
    pub samples: Vec<f32>,       // PCM Float data normalized between -1.0 and 1.0 (Left channel / Mono)
    pub samples_right: Vec<f32>, // Right channel PCM Float data for stereo tracks
    pub volume: f32,             // Master Gain multiplier (0.0 to 2.0+)
    pub pan: f32,                // Stereo panning (-1.0 for full left, 1.0 for full right)
    pub is_muted: bool,
    pub is_solo: bool,
    pub volume_envelope: AutomationEnvelope,
    pub pan_envelope: AutomationEnvelope,
}

impl AudioTrack {
    pub fn new(id: usize, name: &str) -> Self {
        AudioTrack {
            id,
            name: name.to_string(),
            samples: Vec::new(),
            samples_right: Vec::new(),
            volume: 1.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            volume_envelope: AutomationEnvelope::new(),
            pan_envelope: AutomationEnvelope::new(),
        }
    }

    pub fn with_samples(mut self, samples: &[f32]) -> Self {
        self.samples = samples.to_vec();
        self
    }

    pub fn with_stereo_samples(mut self, left: &[f32], right: &[f32]) -> Self {
        self.samples = left.to_vec();
        self.samples_right = right.to_vec();
        self
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn with_pan(mut self, pan: f32) -> Self {
        self.pan = pan.clamp(-1.0, 1.0);
        self
    }

    pub fn is_stereo(&self) -> bool {
        !self.samples_right.is_empty()
    }
}

/// Stereo Mixdown Output Frame
#[derive(Debug, Clone)]
pub struct StereoMixdown {
    pub left: Vec<f32>,
    pub right: Vec<f32>,
}

/// The Audition-style Multi-Track Mixing Engine
pub struct MultiTrackSession {
    pub tracks: Vec<AudioTrack>,
    pub sample_rate: u32,
    pub master_volume: f32,
}

impl MultiTrackSession {
    pub fn new(sample_rate: u32) -> Self {
        MultiTrackSession {
            tracks: Vec::new(),
            sample_rate,
            master_volume: 1.0,
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    /// Mixes all enabled audio tracks down to a single master mono output channel
    pub fn mix_session(&self) -> Vec<f32> {
        let stereo = self.mix_session_stereo();
        let mut mono = Vec::with_capacity(stereo.left.len());
        for i in 0..stereo.left.len() {
            mono.push(((stereo.left[i] + stereo.right[i]) * 0.5).clamp(-1.0, 1.0));
        }
        mono
    }

    /// Mixes all enabled audio tracks down to a full stereo master output (Left/Right channels)
    pub fn mix_session_stereo(&self) -> StereoMixdown {
        let mut max_len = 0;
        let mut has_solo_active = false;

        for track in &self.tracks {
            if track.is_solo && !track.is_muted {
                has_solo_active = true;
            }
            if track.samples.len() > max_len {
                max_len = track.samples.len();
            }
            if track.samples_right.len() > max_len {
                max_len = track.samples_right.len();
            }
        }

        let mut master_left = Vec::with_capacity(max_len);
        let mut master_right = Vec::with_capacity(max_len);
        master_left.resize(max_len, 0.0);
        master_right.resize(max_len, 0.0);

        for track in &self.tracks {
            let is_active = if has_solo_active {
                track.is_solo && !track.is_muted
            } else {
                !track.is_muted
            };

            if is_active {
                let len = track.samples.len().max(track.samples_right.len());
                for sample_idx in 0..len {
                    let vol_env = track.volume_envelope.evaluate_at(sample_idx, 1.0);
                    let pan_env = track.pan_envelope.evaluate_at(sample_idx, track.pan);

                    let effective_vol = track.volume * vol_env * self.master_volume;
                    let effective_pan = pan_env.clamp(-1.0, 1.0);

                    let pan_angle = (effective_pan + 1.0) * (PI / 4.0);
                    let left_gain = effective_vol * audio_math::cos(pan_angle);
                    let right_gain = effective_vol * audio_math::sin(pan_angle);

                    let left_sample = if sample_idx < track.samples.len() {
                        track.samples[sample_idx]
                    } else {
                        0.0
                    };

                    let right_sample = if track.is_stereo() && sample_idx < track.samples_right.len() {
                        track.samples_right[sample_idx]
                    } else {
                        left_sample
                    };

                    master_left[sample_idx] += left_sample * left_gain;
                    master_right[sample_idx] += right_sample * right_gain;
                }
            }
        }

        for i in 0..max_len {
            master_left[i] = master_left[i].clamp(-1.0, 1.0);
            master_right[i] = master_right[i].clamp(-1.0, 1.0);
        }

        StereoMixdown {
            left: master_left,
            right: master_right,
        }
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
    pub db: f32,
}

impl AmplifyEffect {
    pub fn new(db: f32) -> Self {
        AmplifyEffect { db }
    }
}

impl AudioEffect for AmplifyEffect {
    fn apply(&self, samples: &mut [f32]) {
        let multiplier = audio_math::db_to_linear(self.db);
        for sample in samples.iter_mut() {
            *sample = (*sample * multiplier).clamp(-1.0, 1.0);
        }
    }
}

/// Delay / Echo DSP Effect
pub struct EchoEffect {
    pub delay_samples: usize,
    pub decay: f32,
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

        for i in self.delay_samples..samples.len() {
            let delayed_idx = i - self.delay_samples;
            let echoed_signal = samples[delayed_idx] * self.decay;
            samples[i] = (samples[i] + echoed_signal).clamp(-1.0, 1.0);
        }
    }
}

/// Schroeder Reverb Effect
pub struct ReverbEffect {
    pub room_size: f32,
    pub damp: f32,
    pub wet: f32,
}

impl ReverbEffect {
    pub fn new(room_size: f32, damp: f32, wet: f32) -> Self {
        Self {
            room_size: room_size.clamp(0.1, 0.98),
            damp: damp.clamp(0.0, 1.0),
            wet: wet.clamp(0.0, 1.0),
        }
    }
}

impl AudioEffect for ReverbEffect {
    fn apply(&self, samples: &mut [f32]) {
        if samples.len() < 100 {
            return;
        }

        let comb_delays = [1116, 1188, 1277, 1356];
        let feedback = self.room_size;
        let dry = 1.0 - self.wet;

        let mut reverb_accum = Vec::with_capacity(samples.len());
        reverb_accum.resize(samples.len(), 0.0);

        for &delay in &comb_delays {
            if delay >= samples.len() {
                continue;
            }
            let mut comb_buf = samples.to_vec();
            for i in delay..comb_buf.len() {
                let prev = comb_buf[i - delay];
                comb_buf[i] = comb_buf[i] + prev * feedback * (1.0 - self.damp);
            }
            for i in 0..samples.len() {
                reverb_accum[i] += comb_buf[i];
            }
        }

        for i in 0..samples.len() {
            let wet_sample = (reverb_accum[i] / 4.0) * self.wet;
            let dry_sample = samples[i] * dry;
            samples[i] = (dry_sample + wet_sample).clamp(-1.0, 1.0);
        }
    }
}

/// Flanger & Chorus Effect
pub struct FlangerEffect {
    pub depth_samples: usize,
    pub rate_hz: f32,
    pub sample_rate: u32,
}

impl FlangerEffect {
    pub fn new(depth_samples: usize, rate_hz: f32, sample_rate: u32) -> Self {
        Self {
            depth_samples: depth_samples.max(1),
            rate_hz: rate_hz.max(0.1),
            sample_rate,
        }
    }
}

impl AudioEffect for FlangerEffect {
    fn apply(&self, samples: &mut [f32]) {
        if samples.len() <= self.depth_samples * 2 {
            return;
        }

        let original = samples.to_vec();
        for i in 0..samples.len() {
            let time = (i as f32) / (self.sample_rate as f32);
            let lfo = (audio_math::sin(2.0 * PI * self.rate_hz * time) + 1.0) * 0.5;
            let current_delay = lfo * (self.depth_samples as f32);

            let delay_idx = current_delay as usize;
            if i >= delay_idx {
                let modulated_sample = original[i - delay_idx];
                samples[i] = (original[i] * 0.7 + modulated_sample * 0.5).clamp(-1.0, 1.0);
            }
        }
    }
}

/// Dynamic Low-Pass Infinite Impulse Response (IIR) Filter
pub struct LowPassFilter {
    pub cutoff_factor: f32,
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
            let smoothed_y = current_x * self.cutoff_factor + prev_y * (1.0 - self.cutoff_factor);
            samples[i] = smoothed_y;
            prev_y = smoothed_y;
        }
    }
}

/// High-Pass Infinite Impulse Response (IIR) Filter
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

/// Overdrive / Wave-Shaping Distortion Effect
pub struct DistortionEffect {
    pub drive: f32,
}

impl DistortionEffect {
    pub fn new(drive: f32) -> Self {
        Self {
            drive: drive.max(1.0),
        }
    }
}

impl AudioEffect for DistortionEffect {
    fn apply(&self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            let driven = *sample * self.drive;
            *sample = audio_math::tanh(driven);
        }
    }
}

/// Noise Gate filter
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

/// 3-Band Parametric Graphic Equalizer
pub struct GraphicEqualizer {
    pub bass_gain: f32,
    pub mid_gain: f32,
    pub treble_gain: f32,
}

impl GraphicEqualizer {
    pub fn new(bass: f32, mid: f32, treble: f32) -> Self {
        GraphicEqualizer {
            bass_gain: bass,
            mid_gain: mid,
            treble_gain: treble,
        }
    }
}

impl AudioEffect for GraphicEqualizer {
    fn apply(&self, samples: &mut [f32]) {
        let bass_factor = audio_math::db_to_linear(self.bass_gain);
        let mid_factor = audio_math::db_to_linear(self.mid_gain);
        let treble_factor = audio_math::db_to_linear(self.treble_gain);

        if samples.len() < 3 {
            return;
        }

        for i in 1..samples.len() - 1 {
            let prev = samples[i - 1];
            let curr = samples[i];
            let next = samples[i + 1];

            let high_pass = (curr - (prev + next) / 2.0).clamp(-1.0, 1.0);
            let low_pass = ((prev + curr + next) / 3.0).clamp(-1.0, 1.0);
            let band_pass = (curr - low_pass - high_pass).clamp(-1.0, 1.0);

            let modified = low_pass * bass_factor + band_pass * mid_factor + high_pass * treble_factor;
            samples[i] = modified.clamp(-1.0, 1.0);
        }
    }
}

/// Audacity / Adobe Audition-style Spectral Noise Suppression DSP Filter
pub struct SpectralNoiseSuppressionEffect {
    pub noise_threshold: f32,
    pub reduction_db: f32,
}

impl SpectralNoiseSuppressionEffect {
    pub fn new(noise_threshold: f32, reduction_db: f32) -> Self {
        Self {
            noise_threshold: noise_threshold.clamp(0.001, 1.0),
            reduction_db: reduction_db.max(0.0),
        }
    }

    fn attenuation_factor(&self) -> f32 {
        audio_math::db_to_linear(-self.reduction_db)
    }
}

impl AudioEffect for SpectralNoiseSuppressionEffect {
    fn apply(&self, samples: &mut [f32]) {
        let att_factor = self.attenuation_factor();
        for sample in samples.iter_mut() {
            let magnitude = sample.abs();
            if magnitude <= self.noise_threshold {
                *sample *= att_factor;
            } else {
                let ratio = ((magnitude - self.noise_threshold) / self.noise_threshold).clamp(0.0, 1.0);
                let smooth_att = att_factor + (1.0 - att_factor) * ratio;
                *sample *= smooth_att;
            }
        }
    }
}

/// Dynamic Range Compressor
pub struct DynamicRangeCompressor {
    pub threshold_db: f32,
    pub ratio: f32,
}

impl DynamicRangeCompressor {
    pub fn new(threshold: f32, ratio: f32) -> Self {
        DynamicRangeCompressor {
            threshold_db: threshold,
            ratio: ratio.max(1.0),
        }
    }

    fn linear_threshold(&self) -> f32 {
        audio_math::db_to_linear(self.threshold_db)
    }
}

impl AudioEffect for DynamicRangeCompressor {
    fn apply(&self, samples: &mut [f32]) {
        let limit = self.linear_threshold();
        for sample in samples.iter_mut() {
            let abs_val = sample.abs();
            if abs_val > limit {
                let excess = abs_val - limit;
                let compressed = limit + excess / self.ratio;
                let sign = if *sample >= 0.0 { 1.0 } else { -1.0 };
                *sample = sign * compressed;
            }
        }
    }
}

/// Pitch Shifter / Resampler Effect
pub struct PitchShifter {
    pub pitch_factor: f32,
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

            let interpolated = s1 * (1.0 - frac) + s2 * frac;
            resampled.push(interpolated);

            float_idx += self.pitch_factor;
        }

        let copy_len = resampled.len().min(samples.len());
        samples[..copy_len].copy_from_slice(&resampled.as_slice()[..copy_len]);

        for i in copy_len..samples.len() {
            samples[i] = 0.0;
        }
    }
}

/// Signal Generator for Tones & Noise synthesis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveformType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    WhiteNoise,
}

pub struct SignalGenerator;

impl SignalGenerator {
    pub fn generate_waveform(
        waveform: WaveformType,
        frequency_hz: f32,
        duration_samples: usize,
        amplitude: f32,
        sample_rate: u32,
    ) -> Vec<f32> {
        let mut samples = Vec::with_capacity(duration_samples);
        let amp = amplitude.clamp(0.0, 1.0);
        let mut lcg_state: u32 = 12345;

        for i in 0..duration_samples {
            let t = (i as f32) / (sample_rate as f32);
            let phase = 2.0 * PI * frequency_hz * t;

            let sample_val = match waveform {
                WaveformType::Sine => audio_math::sin(phase),
                WaveformType::Square => {
                    if audio_math::sin(phase) >= 0.0 {
                        1.0
                    } else {
                        -1.0
                    }
                }
                WaveformType::Sawtooth => {
                    let norm_phase = (t * frequency_hz) % 1.0;
                    2.0 * norm_phase - 1.0
                }
                WaveformType::Triangle => {
                    let norm_phase = (t * frequency_hz) % 1.0;
                    if norm_phase < 0.5 {
                        4.0 * norm_phase - 1.0
                    } else {
                        3.0 - 4.0 * norm_phase
                    }
                }
                WaveformType::WhiteNoise => {
                    lcg_state = lcg_state.wrapping_mul(1664525).wrapping_add(1013904223);
                    let norm_rand = (lcg_state as f32) / (u32::MAX as f32);
                    2.0 * norm_rand - 1.0
                }
            };

            samples.push(sample_val * amp);
        }

        samples
    }
}

/// Audacity / Adobe Audition Center Channel Extractor & Vocal Remover
pub struct CenterChannelExtractor;

impl CenterChannelExtractor {
    pub fn process_stereo(left: &mut [f32], right: &mut [f32], remove_center: bool) {
        let len = left.len().min(right.len());
        for i in 0..len {
            let l = left[i];
            let r = right[i];

            if remove_center {
                let side = (l - r) * 0.707;
                left[i] = side;
                right[i] = -side;
            } else {
                let mid = (l + r) * 0.707;
                left[i] = mid;
                right[i] = mid;
            }
        }
    }
}

/// Professional Wave Editor Operations
pub struct AudioEditor;

impl AudioEditor {
    pub fn normalize(track: &mut AudioTrack) {
        if track.samples.is_empty() {
            return;
        }

        let mut peak: f32 = 0.0;
        for &s in &track.samples {
            let val = s.abs();
            if val > peak {
                peak = val;
            }
        }

        if peak > 0.0 && peak < 1.0 {
            let scale_factor = 1.0 / peak;
            for sample in track.samples.iter_mut() {
                *sample *= scale_factor;
            }
            for sample in track.samples_right.iter_mut() {
                *sample *= scale_factor;
            }
        }
    }

    pub fn fade_in(track: &mut AudioTrack, duration_samples: usize) {
        let limit = duration_samples.min(track.samples.len());
        for i in 0..limit {
            let factor = (i as f32) / (limit as f32);
            track.samples[i] *= factor;
            if i < track.samples_right.len() {
                track.samples_right[i] *= factor;
            }
        }
    }

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
            if (start_idx + i) < track.samples_right.len() {
                track.samples_right[start_idx + i] *= factor;
            }
        }
    }

    pub fn reverse(track: &mut AudioTrack) {
        track.samples.reverse();
        track.samples_right.reverse();
    }

    pub fn invert_phase(track: &mut AudioTrack) {
        for sample in track.samples.iter_mut() {
            *sample = -*sample;
        }
        for sample in track.samples_right.iter_mut() {
            *sample = -*sample;
        }
    }

    pub fn silence(track: &mut AudioTrack, start: usize, end: usize) {
        let len = track.samples.len();
        let limit_end = end.min(len);
        if start < limit_end {
            for i in start..limit_end {
                track.samples[i] = 0.0;
                if i < track.samples_right.len() {
                    track.samples_right[i] = 0.0;
                }
            }
        }
    }

    pub fn crossfade(track_a: &mut AudioTrack, track_b: &AudioTrack, crossfade_samples: usize) {
        let fade_len = crossfade_samples
            .min(track_a.samples.len())
            .min(track_b.samples.len());
        if fade_len == 0 {
            return;
        }

        let start_idx = track_a.samples.len() - fade_len;
        for i in 0..fade_len {
            let alpha = (i as f32) / (fade_len as f32);
            track_a.samples[start_idx + i] =
                track_a.samples[start_idx + i] * (1.0 - alpha) + track_b.samples[i] * alpha;
        }
    }

    pub fn cut(track: &mut AudioTrack, start: usize, end: usize) -> Vec<f32> {
        let len = track.samples.len();
        if start >= len || end > len || start >= end {
            return Vec::new();
        }

        let mut clipboard = Vec::new();
        for i in start..end {
            clipboard.push(track.samples[i]);
        }

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
            .with_volume(1.0)
            .with_pan(0.0);

        let track2 = AudioTrack::new(2, "Backing")
            .with_samples(&[0.2, -0.2, 0.2])
            .with_volume(1.0)
            .with_pan(0.0);

        session.add_track(track1);
        session.add_track(track2);

        let mix = session.mix_session();
        assert_eq!(mix.len(), 3);
        assert!((mix[0] - 0.495).abs() < 0.05);
    }

    #[test]
    fn test_stereo_panning_mixdown() {
        let mut session = MultiTrackSession::new(44100);

        let track_left = AudioTrack::new(1, "GtrLeft")
            .with_samples(&[1.0, 1.0])
            .with_pan(-1.0);

        let track_right = AudioTrack::new(2, "GtrRight")
            .with_samples(&[1.0, 1.0])
            .with_pan(1.0);

        session.add_track(track_left);
        session.add_track(track_right);

        let stereo = session.mix_session_stereo();
        assert_eq!(stereo.left.len(), 2);
        assert!(stereo.left[0] > 0.9);
        assert!(stereo.right[0] > 0.9);
    }

    #[test]
    fn test_automation_envelope_interpolation() {
        let mut env = AutomationEnvelope::new();
        env.add_point(0, 0.0);
        env.add_point(100, 1.0);

        assert_eq!(env.evaluate_at(0, 0.5), 0.0);
        assert_eq!(env.evaluate_at(50, 0.5), 0.5);
        assert_eq!(env.evaluate_at(100, 0.5), 1.0);
    }

    #[test]
    fn test_reverb_and_flanger_effects() {
        let mut samples = [0.8, -0.8, 0.5, -0.5, 0.2, -0.2];

        let reverb = ReverbEffect::new(0.8, 0.2, 0.3);
        reverb.apply(&mut samples);
        assert_eq!(samples.len(), 6);

        let flanger = FlangerEffect::new(2, 1.0, 44100);
        flanger.apply(&mut samples);
        assert_eq!(samples.len(), 6);
    }

    #[test]
    fn test_highpass_and_distortion_effects() {
        let mut samples = [0.1, 0.8, -0.8, 0.5];

        let hp = HighPassFilter::new(0.8);
        hp.apply(&mut samples);

        let dist = DistortionEffect::new(5.0);
        dist.apply(&mut samples);
        assert!(samples[1].abs() <= 1.0);
    }

    #[test]
    fn test_signal_generator() {
        let sine_wave = SignalGenerator::generate_waveform(WaveformType::Sine, 440.0, 100, 0.8, 44100);
        assert_eq!(sine_wave.len(), 100);
        assert!(sine_wave[0].abs() < 0.1);

        let noise = SignalGenerator::generate_waveform(WaveformType::WhiteNoise, 0.0, 100, 0.5, 44100);
        assert_eq!(noise.len(), 100);
    }

    #[test]
    fn test_center_channel_vocal_remover() {
        let mut left = [0.8, 0.5, -0.3];
        let mut right = [0.8, -0.5, -0.3];

        CenterChannelExtractor::process_stereo(&mut left, &mut right, true);
        assert!(left[0].abs() < 1e-4);
    }

    #[test]
    fn test_audio_editor_reverse_and_silence() {
        let mut track = AudioTrack::new(10, "Lead").with_samples(&[1.0, 2.0, 3.0, 4.0]);

        AudioEditor::reverse(&mut track);
        assert_eq!(track.samples[0], 4.0);

        AudioEditor::silence(&mut track, 1, 3);
        assert_eq!(track.samples[1], 0.0);
        assert_eq!(track.samples[2], 0.0);
    }
}
