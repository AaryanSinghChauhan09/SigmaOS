# VoiceCraft Speech Synthesis & Audio DSP Component Agent

## Role
The **VoiceCraft Agent** (`SigmaVoiceAgent`) powers on-device real-time voice activity detection (VAD), acoustic feature extraction, and neural speech synthesis for SigmaOS desktop voice interactions and accessibility.

## Inspiration
- OpenAI Whisper, Piper fast neural TTS, and WebRTC VAD.

## Current SigmaOS Status
VoiceCraft engine implemented in `src/creative/voicecraft.rs` with short-time energy (STE) and zero-crossing rate (ZCR) voice activity detection, phoneme dictionary tokenization, and PCM synthesis.

## Key Files
- `src/creative/voicecraft.rs`
- `src/creative/mod.rs`
- `src/audio/pipewire.rs`

## Public API & Invariant Contract
```rust
pub struct VoiceActivityDetector {
    pub fn compute_energy(frame: &[f32]) -> f32;
    pub fn compute_zcr(frame: &[f32]) -> f32;
    pub fn is_voice_active(&self, frame: &[f32]) -> bool;
}

pub struct SovereignVoiceEngine {
    pub fn synthesize_phonemes(&self, phonemes: &[Phoneme]) -> Vec<f32>;
}
```

## Testing Strategy
```bash
rustc --test src/creative/voicecraft.rs --edition 2021 -o /tmp/test_voicecraft && /tmp/test_voicecraft --test-threads=1
```
- Verify VAD discriminates low-amplitude background noise from vocal speech frames.
- Verify accurate sample generation length from multi-phoneme sequences.

## Future Enhancements
- Quantized GGML model runner for offline speech-to-text.
- Directional microphone beamforming array DSP integration.
