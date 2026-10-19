//! Microphone manager following SigmaOS principles
//!
//! Implements audio capture with noise cancellation, gain control,
//! and hardware permission gates following the WDM-style driver architecture.

use core::fmt;
use crate::capability::CapabilityToken;

/// Microphone capture device configuration
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

impl fmt::Display for AudioCaptureDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AudioCaptureDevice[device={}, channels={}, rate={}Hz, bit_depth={}, noise_cancellation={}, gain={}%, muted={}]",
               self.device_name, self.channels, self.sample_rate_hz, self.bit_depth, self.noise_cancellation_active, self.input_gain_percent, self.muted)
}
}

/// Microphone manager following WDM-style driver pattern
pub struct MicrophoneManager {
    /// Active device
    device: Option<AudioCaptureDevice>,
    /// Noise cancellation state
    noise_cancellation: bool,
    /// Input gain
    gain: u8,
    /// Mute state
    muted: bool,
    /// Hardware permission gate
    hardware_permission: bool,
}

impl MicrophoneManager {
    /// Create new microphone manager
    pub fn new() -> Self {
        MicrophoneManager {
            device: None,
            noise_cancellation: false,
            gain: 50,
            muted: false,
            hardware_permission: false,
        }
    }
    
    /// Set active device
    pub fn set_device(&mut self, device: AudioCaptureDevice) {
        self.device = Some(device);
    }
    
    /// Get active device (if any)
    pub fn device(&self) -> Option<&AudioCaptureDevice> {
        self.device.as_ref()
    }
    
    /// Set noise cancellation
    pub fn set_noise_cancellation(&mut self, enabled: bool) {
        self.noise_cancellation = enabled;
    }
    
    /// Check if noise cancellation is enabled
    pub fn is_noise_cancellation_enabled(&self) -> bool {
        self.noise_cancellation
    }
    
    /// Set input gain
    pub fn set_gain(&mut self, percent: u8) {
        self.gain = percent.clamp(0, 100);
    }
    
    /// Get input gain
    pub fn gain(&self) -> u8 {
        self.gain
    }
    
    /// Mute microphone
    pub fn set_muted(&mut self, muted: bool) {
        self.muted = muted;
    }
    
    /// Check if muted
    pub fn is_muted(&self) -> bool {
        self.muted
    }
    
    /// Request hardware permission
    pub fn request_hardware_permission(&mut self, granted: bool) {
        self.hardware_permission = granted;
    }
    
    /// Check hardware permission status
    pub fn has_hardware_permission(&self) -> bool {
        self.hardware_permission
    }
}

impl fmt::Display for MicrophoneManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MicrophoneManager[device={:?}, noise_cancellation={}, gain={}%, muted={}, hardware_permission={}]",
               self.device.as_ref().map(|d| &d.device_name).unwrap_or("none"),
               self.noise_cancellation,
               self.gain,
               self.muted,
               self.hardware_permission)
}
