//! Audio capture device following SigmaOS principles
//!
//! Implements low-latency hardware audio capture following WDM-style driver architecture.

use core::fmt;

/// Audio capture device following SigmaOS principles
#[derive(Debug, Clone)]
pub struct AudioCaptureDevice {
    /// Device name
    pub device_name: String,
    /// Number of channels
    pub channels: u8,
    /// Sample rate in Hz
    pub sample_rate_hz: u32,
    /// Bit depth
    pub bit_depth: u8,
    /// Noise cancellation active
    pub noise_cancellation_active: bool,
    /// Input gain percent
    pub input_gain_percent: u8,
    /// Muted state
    pub muted: bool,
}

/// Audio format specification
#[derive(Debug, Clone, PartialEq)]
pub enum AudioFormat {
    /// 8-bit PCM
    EightBitPCM,
    /// 16-bit PCM
    SixteenBitPCM,
    /// 24-bit PCM
    TwentyFourBitPCM,
    /// 32-bit PCM
    ThirtyTwoBitPCM,
}

/// Audio source type
#[derive(Debug, Clone, PartialEq)]
pub enum AudioSource {
    /// Internal microphone
    Internal,
    /// External microphone
    External,
    /// Line input
    LineIn,
    /// Digital input
    Digital,
}

/// Audio endpoint direction
#[derive(Debug, Clone, PartialEq)]
pub enum AudioEndpoint {
    /// Capture (input)
    Capture,
    /// Render (output)
    Render,
}

impl fmt::Display for AudioCaptureDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AudioCaptureDevice[device={}, channels={}, rate={}Hz, bit_depth={}, noise_cancellation={}, gain={}%, muted={}]",
               self.device_name, self.channels, self.sample_rate_hz, self.bit_depth, self.noise_cancellation_active, self.input_gain_percent, self.muted)
}
