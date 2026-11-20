//! SigmaOS ASD control daemon.
//!
//! Coordinates hardware-level audio and display adjustments across input
//! devices and system events. Exposes a D-Bus API.

#![warn(missing_docs)]

/// Audio control operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioOp {
    /// Set the master volume (0-100).
    SetVolume(u8),
    /// Mute or unmute output.
    SetMute(bool),
    /// Switch the active output device.
    SwitchOutput(String),
}

/// Display control operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayOp {
    /// Set the backlight brightness (0-100).
    SetBrightness(u8),
    /// Change the active display mode.
    SetMode(String),
}

/// Trait implemented by hardware controllers the daemon drives.
pub trait HardwareController {
    /// Apply an audio operation.
    fn apply_audio(&mut self, op: AudioOp) -> Result<(), ControlError>;
    /// Apply a display operation.
    fn apply_display(&mut self, op: DisplayOp) -> Result<(), ControlError>;
}

/// Errors returned by the control daemon.
#[derive(Debug)]
pub enum ControlError {
    /// The requested value was out of range.
    OutOfRange,
    /// The hardware controller did not respond.
    HardwareUnavailable,
}

impl std::fmt::Display for ControlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlError::OutOfRange => write!(f, "value out of range"),
            ControlError::HardwareUnavailable => write!(f, "hardware unavailable"),
        }
    }
}

impl std::error::Error for ControlError {}
