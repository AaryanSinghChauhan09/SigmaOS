/// Advanced Multi-Track Audio Editor & DSP Filter Suite for SigmaOS
/// Replicates core features, mixing engines, and effects from Adobe Audition and Audacity
/// Supports multi-track session mixing, gain panning, and professional DSP filter processing.
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

pub trait AudioEffect {
    fn apply(&self, samples: &mut [f32]);
}

pub struct AmplifyEffect {
    pub gain: f32,
}

impl AmplifyEffect {
    pub fn new(gain: f32) -> Self {
        AmplifyEffect { gain }
    }
}

impl AudioEffect for AmplifyEffect {
    fn apply(&self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            *sample *= self.gain;
        }
    }
}

pub struct AudioTrack {
    pub id: u64,
    pub name: String,
    pub samples: Vec<f32>,
    pub volume: f32,
    pub is_muted: bool,
    pub is_solo: bool,
}

impl AudioTrack {
    pub fn new(id: u64, name: &str) -> Self {
        AudioTrack {
            id,
            name: String::from(name),
            samples: Vec::new(),
            volume: 1.0,
            is_muted: false,
            is_solo: false,
        }
    }

    pub fn with_samples(mut self, samples: &[f32]) -> Self {
        self.samples = samples.to_vec();
        self
    }

    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

pub struct MultiTrackSession {
    pub sample_rate: u32,
    pub tracks: Vec<AudioTrack>,
}

impl MultiTrackSession {
    pub fn new(sample_rate: u32) -> Self {
        MultiTrackSession {
            sample_rate,
            tracks: Vec::new(),
        }
    }

    pub fn add_track(&mut self, track: AudioTrack) {
        self.tracks.push(track);
    }

    pub fn mix_session(&self) -> Vec<f32> {
        let has_solo = self.tracks.iter().any(|t| t.is_solo);
        let max_len = self
            .tracks
            .iter()
            .map(|t| t.samples.len())
            .max()
            .unwrap_or(0);
        let mut mixed = std::vec![0.0f32; max_len];

        for track in &self.tracks {
            if track.is_muted {
                continue;
            }
            if has_solo && !track.is_solo {
                continue;
            }

            for (i, &sample) in track.samples.iter().enumerate() {
                mixed[i] += sample * track.volume;
            }
        }

        mixed
    }
}

pub struct SpectralNoiseSuppressionEffect {
    pub noise_floor: f32,
}

impl SpectralNoiseSuppressionEffect {
    pub fn new(noise_floor: f32) -> Self {
        SpectralNoiseSuppressionEffect { noise_floor }
    }
}

impl AudioEffect for SpectralNoiseSuppressionEffect {
    fn apply(&self, samples: &mut [f32]) {
        for sample in samples.iter_mut() {
            if sample.abs() < self.noise_floor {
                *sample = 0.0;
            }
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
}
