pub mod router;

pub use router::{AudioRouter, AudioStream, RingBuffer};

/// SigmaAudio: Native Audio Subsystem
/// Displaces PipeWire and PulseAudio to provide a low-latency native audio router.
pub struct SigmaAudio {
    pub router: AudioRouter,
}

impl Default for SigmaAudio {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaAudio {
    pub fn new() -> Self {
        Self {
            router: AudioRouter::new(),
        }
    }
}
