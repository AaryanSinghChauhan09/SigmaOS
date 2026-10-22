//! VoiceCraft: Sovereign On-Device Speech Synthesis & Audio DSP Engine for SigmaOS
//!
//! Inspired by Whisper, Piper TTS, and modern zero-latency speech pipelines:
//! - Real-time Voice Activity Detection (VAD) via short-time energy & zero-crossing rate
//! - Phoneme sequence tokenization and acoustic duration model
//! - Linear pulse-code modulation (PCM) audio synthesis buffer
//! - Low-latency streaming integration for Wayland voice commands and accessibility

extern crate alloc;
use core::f32::consts::PI;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub const SAMPLE_RATE: usize = 16000; // 16kHz voice standard
pub const FRAME_SIZE: usize = 320; // 20ms frames at 16kHz

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhonemeType {
    Vowel,
    Consonant,
    Plosive,
    Fricative,
    Silence,
}

#[derive(Debug, Clone)]
pub struct Phoneme {
    pub symbol: String,
    pub phoneme_type: PhonemeType,
    pub duration_ms: u32,
    pub pitch_hz: f32,
}

pub struct VoiceActivityDetector {
    pub energy_threshold: f32,
    pub zcr_threshold: f32,
}

impl VoiceActivityDetector {
    pub fn new(energy_threshold: f32, zcr_threshold: f32) -> Self {
        Self {
            energy_threshold,
            zcr_threshold,
        }
    }

    /// Calculate Short-Time Energy (STE) of a frame
    pub fn compute_energy(frame: &[f32]) -> f32 {
        if frame.is_empty() {
            return 0.0;
        }
        let sum_sq: f32 = frame.iter().map(|&s| s * s).sum();
        sum_sq / frame.len() as f32
    }

    /// Calculate Zero-Crossing Rate (ZCR)
    pub fn compute_zcr(frame: &[f32]) -> f32 {
        if frame.len() < 2 {
            return 0.0;
        }
        let mut crossings = 0;
        for i in 1..frame.len() {
            if (frame[i] >= 0.0 && frame[i - 1] < 0.0) || (frame[i] < 0.0 && frame[i - 1] >= 0.0) {
                crossings += 1;
            }
        }
        crossings as f32 / (frame.len() - 1) as f32
    }

    /// Determine if the audio frame contains active voice
    pub fn is_voice_active(&self, frame: &[f32]) -> bool {
        let energy = Self::compute_energy(frame);
        let zcr = Self::compute_zcr(frame);
        energy >= self.energy_threshold && zcr >= self.zcr_threshold
    }
}

pub struct SovereignVoiceEngine {
    pub vad: VoiceActivityDetector,
    pub phoneme_dictionary: BTreeMap<String, Vec<Phoneme>>,
}

impl SovereignVoiceEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            vad: VoiceActivityDetector::new(0.01, 0.05),
            phoneme_dictionary: BTreeMap::new(),
        };
        engine.bootstrap_dictionary();
        engine
    }

    fn bootstrap_dictionary(&mut self) {
        let mut hello = Vec::new();
        hello.push(Phoneme {
            symbol: String::from("HH"),
            phoneme_type: PhonemeType::Fricative,
            duration_ms: 60,
            pitch_hz: 120.0,
        });
        hello.push(Phoneme {
            symbol: String::from("EH"),
            phoneme_type: PhonemeType::Vowel,
            duration_ms: 100,
            pitch_hz: 140.0,
        });
        hello.push(Phoneme {
            symbol: String::from("L"),
            phoneme_type: PhonemeType::Consonant,
            duration_ms: 80,
            pitch_hz: 130.0,
        });
        hello.push(Phoneme {
            symbol: String::from("OW"),
            phoneme_type: PhonemeType::Vowel,
            duration_ms: 120,
            pitch_hz: 110.0,
        });
        self.phoneme_dictionary.insert(String::from("hello"), hello);
    }

    /// Synthesize simple sine-tone modulated waveform from phoneme sequence
    pub fn synthesize_phonemes(&self, phonemes: &[Phoneme]) -> Vec<f32> {
        let mut pcm = Vec::new();
        let sample_rate = SAMPLE_RATE as f32;

        for p in phonemes {
            let num_samples = (p.duration_ms as f32 * sample_rate / 1000.0) as usize;
            for i in 0..num_samples {
                let t = i as f32 / sample_rate;
                // Harmonic synthesis based on phoneme pitch
                let sample = (2.0 * PI * p.pitch_hz * t).sin() * 0.5;
                pcm.push(sample);
            }
        }
        pcm
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_voicecraft_vad() {
        let vad = VoiceActivityDetector::new(0.01, 0.05);

        // Silent frame
        let silent = alloc::vec![0.001f32; 320];
        assert!(!vad.is_voice_active(&silent));

        // Active speech frame with alternating values
        let mut active = Vec::with_capacity(320);
        for i in 0..320 {
            active.push(if i % 2 == 0 { 0.2 } else { -0.2 });
        }
        assert!(vad.is_voice_active(&active));
    }

    #[test]
    fn test_voicecraft_phoneme_sequence() {
        let engine = SovereignVoiceEngine::new();
        let phonemes = engine.phoneme_dictionary.get("hello").unwrap();
        assert_eq!(phonemes.len(), 4);

        let pcm = engine.synthesize_phonemes(phonemes);
        assert!(!pcm.is_empty());
        // Expected total samples = (60+100+80+120)*16000/1000 = 360 * 16 = 5760
        assert_eq!(pcm.len(), 5760);
    }
}
