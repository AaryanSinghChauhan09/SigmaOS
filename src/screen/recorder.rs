//! SigmaOS Screen Recorder — Inspired by OBS Studio, GNOME Screen Recorder, SimpleScreenRecorder.
//! Principles: Component Isolation, CQS, Encapsulation, Least Privilege, Safe Concurrency.

/// Screen recording output format.
#[derive(Debug, Clone, PartialEq)]
pub enum RecordFormat {
    Mp4,
    Mkv,
    Webm,
    Gif,
}

/// Screen region to capture.
#[derive(Debug, Clone)]
pub struct CaptureRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl CaptureRegion {
    pub fn full_screen(width: u32, height: u32) -> Self {
        Self { x: 0, y: 0, width, height }
    }

    pub fn area_pixels(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

/// Recording configuration.
#[derive(Debug, Clone)]
pub struct RecordConfig {
    pub format: RecordFormat,
    pub fps: u32,
    pub region: CaptureRegion,
    pub include_audio: bool,
    pub include_cursor: bool,
    pub output_path: String,
}

impl Default for RecordConfig {
    fn default() -> Self {
        Self {
            format: RecordFormat::Mp4,
            fps: 30,
            region: CaptureRegion::full_screen(1920, 1080),
            include_audio: true,
            include_cursor: true,
            output_path: "/home/user/Videos/recording.mp4".into(),
        }
    }
}

/// Screen recorder state machine (Fail-Fast, Safe Concurrency).
#[derive(Debug, PartialEq)]
pub enum RecorderState {
    Idle,
    Recording,
    Paused,
    Saving,
}

pub struct ScreenRecorder {
    config: RecordConfig,
    state: RecorderState,
    frames_captured: u64,
}

impl ScreenRecorder {
    pub fn new(config: RecordConfig) -> Self {
        Self { config, state: RecorderState::Idle, frames_captured: 0 }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.state != RecorderState::Idle {
            return Err("recorder is not idle");
        }
        self.state = RecorderState::Recording;
        self.frames_captured = 0;
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), &'static str> {
        if self.state != RecorderState::Recording {
            return Err("not recording");
        }
        self.state = RecorderState::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), &'static str> {
        if self.state != RecorderState::Paused {
            return Err("not paused");
        }
        self.state = RecorderState::Recording;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<String, &'static str> {
        if self.state == RecorderState::Idle {
            return Err("not recording");
        }
        self.state = RecorderState::Saving;
        let path = self.config.output_path.clone();
        self.state = RecorderState::Idle;
        Ok(path)
    }

    pub fn state(&self) -> &RecorderState { &self.state }
    pub fn frames_captured(&self) -> u64 { self.frames_captured }
    pub fn config(&self) -> &RecordConfig { &self.config }

    /// Estimate output file size in MB (query — no side effects).
    pub fn estimate_size_mb(&self, duration_secs: u32) -> u32 {
        let bitrate_mbps = match self.config.format {
            RecordFormat::Mp4  => 8,
            RecordFormat::Mkv  => 10,
            RecordFormat::Webm => 6,
            RecordFormat::Gif  => 20,
        };
        (bitrate_mbps * duration_secs) / 8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recorder() -> ScreenRecorder {
        ScreenRecorder::new(RecordConfig::default())
    }

    #[test]
    fn test_start_stop_cycle() {
        let mut r = recorder();
        assert_eq!(*r.state(), RecorderState::Idle);
        assert!(r.start().is_ok());
        assert_eq!(*r.state(), RecorderState::Recording);
        let path = r.stop().unwrap();
        assert!(path.ends_with(".mp4"));
        assert_eq!(*r.state(), RecorderState::Idle);
    }

    #[test]
    fn test_pause_resume() {
        let mut r = recorder();
        r.start().unwrap();
        assert!(r.pause().is_ok());
        assert_eq!(*r.state(), RecorderState::Paused);
        assert!(r.resume().is_ok());
        assert_eq!(*r.state(), RecorderState::Recording);
    }

    #[test]
    fn test_double_start_fails() {
        let mut r = recorder();
        r.start().unwrap();
        assert!(r.start().is_err());
    }

    #[test]
    fn test_capture_region_area() {
        let region = CaptureRegion::full_screen(1920, 1080);
        assert_eq!(region.area_pixels(), 1920 * 1080);
    }

    #[test]
    fn test_size_estimate() {
        let r = recorder();
        let sz = r.estimate_size_mb(60);
        assert!(sz > 0);
    }
}
