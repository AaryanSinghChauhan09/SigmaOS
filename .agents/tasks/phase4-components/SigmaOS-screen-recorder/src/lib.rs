//! SigmaOS-screen-recorder — omareel-inspired desktop screen recording pipeline
//!
//! Provides a `CaptureRegion`, `RecorderConfig`, and a `Recorder` state
//! machine with `start`/`stop` transitions.  No external dependencies.
//!
//! Upstream inspiration: omareel (https://github.com/omacom/omareel),
//! wf-recorder (https://github.com/ammen99/wf-recorder)

#![allow(dead_code)]

use std::fmt;

// ---------------------------------------------------------------------------
// Module entry point (re-exported from scaffold)
// ---------------------------------------------------------------------------

/// Returns the crate version from Cargo.toml.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// ---------------------------------------------------------------------------
// Capture region
// ---------------------------------------------------------------------------

/// A rectangular capture region in screen coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CaptureRegion {
    pub fn full_display() -> Self {
        Self { x: 0, y: 0, width: 0, height: 0 }  // 0×0 = "full output"
    }

    pub fn is_full_display(&self) -> bool {
        self.width == 0 && self.height == 0
    }
}

impl fmt::Display for CaptureRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_full_display() {
            write!(f, "full display")
        } else {
            write!(f, "{}x{}+{}+{}", self.width, self.height, self.x, self.y)
        }
    }
}

// ---------------------------------------------------------------------------
// Recorder configuration
// ---------------------------------------------------------------------------

/// Container format for the recording output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    Mp4,
    Mkv,
    Webm,
}

impl fmt::Display for Container {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Container::Mp4  => write!(f, "mp4"),
            Container::Mkv  => write!(f, "mkv"),
            Container::Webm => write!(f, "webm"),
        }
    }
}

/// Configuration for a recording session.
#[derive(Debug, Clone)]
pub struct RecorderConfig {
    /// Screen region to capture.
    pub region: CaptureRegion,
    /// Output container format.
    pub container: Container,
    /// Frames per second.
    pub fps: u32,
    /// Output file path.
    pub output_path: String,
}

impl RecorderConfig {
    pub fn new(region: CaptureRegion, container: Container, fps: u32, output_path: impl Into<String>) -> Self {
        Self { region, container, fps, output_path: output_path.into() }
    }

    pub fn default_display(output_path: impl Into<String>) -> Self {
        Self::new(CaptureRegion::full_display(), Container::Mp4, 30, output_path)
    }
}

// ---------------------------------------------------------------------------
// Recorder state machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecorderState {
    Idle,
    Recording,
    Stopped,
}

#[derive(Debug)]
pub enum RecorderError {
    /// Tried to start when already recording.
    AlreadyRecording,
    /// Tried to stop when not recording.
    NotRecording,
}

impl fmt::Display for RecorderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecorderError::AlreadyRecording => write!(f, "recorder is already recording"),
            RecorderError::NotRecording     => write!(f, "recorder is not recording"),
        }
    }
}

impl std::error::Error for RecorderError {}

/// Screen recorder with start/stop state transitions.
///
/// In a real implementation, `start` would invoke `wf-recorder` or a
/// similar Wayland capture tool and hold the child process handle.
pub struct Recorder {
    pub config: RecorderConfig,
    state: RecorderState,
    frames_captured: u64,
}

impl Recorder {
    pub fn new(config: RecorderConfig) -> Self {
        Self { config, state: RecorderState::Idle, frames_captured: 0 }
    }

    pub fn state(&self) -> RecorderState {
        self.state
    }

    /// Begin recording.
    pub fn start(&mut self) -> Result<(), RecorderError> {
        if self.state == RecorderState::Recording {
            return Err(RecorderError::AlreadyRecording);
        }
        self.state = RecorderState::Recording;
        self.frames_captured = 0;
        Ok(())
    }

    /// Stop recording and return the number of frames captured.
    pub fn stop(&mut self) -> Result<u64, RecorderError> {
        if self.state != RecorderState::Recording {
            return Err(RecorderError::NotRecording);
        }
        self.state = RecorderState::Stopped;
        Ok(self.frames_captured)
    }

    /// Simulate capturing `n` frames (called by the render loop in a real impl).
    pub fn tick(&mut self, frames: u64) {
        if self.state == RecorderState::Recording {
            self.frames_captured += frames;
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }

    #[test]
    fn capture_region_full_display() {
        let r = CaptureRegion::full_display();
        assert!(r.is_full_display());
    }

    #[test]
    fn capture_region_custom() {
        let r = CaptureRegion { x: 0, y: 0, width: 1920, height: 1080 };
        assert!(!r.is_full_display());
    }

    #[test]
    fn recorder_state_machine() {
        let cfg = RecorderConfig::default_display("/tmp/test.mp4");
        let mut rec = Recorder::new(cfg);
        assert_eq!(rec.state(), RecorderState::Idle);

        rec.start().expect("start should succeed from Idle");
        assert_eq!(rec.state(), RecorderState::Recording);

        // Double-start should fail.
        assert!(rec.start().is_err());

        rec.tick(120);
        let frames = rec.stop().expect("stop should succeed while Recording");
        assert_eq!(frames, 120);
        assert_eq!(rec.state(), RecorderState::Stopped);

        // Double-stop should fail.
        assert!(rec.stop().is_err());
    }

    #[test]
    fn container_display() {
        assert_eq!(Container::Mp4.to_string(), "mp4");
        assert_eq!(Container::Webm.to_string(), "webm");
    }
}
