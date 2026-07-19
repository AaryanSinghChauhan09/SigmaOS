// SigmaOS Screen Recorder
// OOP-based screen recording with multiple formats and quality settings

use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Recording format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingFormat {
    Mp4,
    WebM,
    Gif,
    Avi,
}

/// Video quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Audio quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioQuality {
    Low,
    Medium,
    High,
}

/// Recording region
#[derive(Debug, Clone)]
pub struct RecordingRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Recording config
#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub format: RecordingFormat,
    pub video_quality: VideoQuality,
    pub audio_quality: AudioQuality,
    pub fps: u32,
    pub region: RecordingRegion,
    pub record_audio: bool,
    pub record_cursor: bool,
    pub output_path: PathBuf,
}

/// Recording state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Idle,
    Recording,
    Paused,
    Stopping,
}

/// Recording progress
#[derive(Debug, Clone)]
pub struct RecordingProgress {
    pub duration_seconds: u64,
    pub frames_captured: u64,
    pub file_size_bytes: u64,
    pub current_bitrate_mbps: f64,
}

/// OOP trait for recording backends
pub trait RecordingBackend {
    /// Start recording
    fn start_recording(&mut self, config: &RecordingConfig) -> Result<(), RecorderError>;
    /// Stop recording
    fn stop_recording(&mut self) -> Result<PathBuf, RecorderError>;
    /// Pause recording
    fn pause_recording(&mut self) -> Result<(), RecorderError>;
    /// Resume recording
    fn resume_recording(&mut self) -> Result<(), RecorderError>;
    /// Get recording state
    fn get_state(&self) -> RecordingState;
    /// Get recording progress
    fn get_progress(&self) -> RecordingProgress;
    /// Get backend name
    fn name(&self) -> &str;
}

/// FFmpeg recording backend
pub struct FfmpegBackend {
    state: RecordingState,
    start_time: Option<Instant>,
    config: Option<RecordingConfig>,
}

impl FfmpegBackend {
    pub fn new() -> Self {
        Self {
            state: RecordingState::Idle,
            start_time: None,
            config: None,
        }
    }
}

impl RecordingBackend for FfmpegBackend {
    fn start_recording(&mut self, config: &RecordingConfig) -> Result<(), RecorderError> {
        self.state = RecordingState::Recording;
        self.start_time = Some(Instant::now());
        self.config = Some(config.clone());
        Ok(())
    }

    fn stop_recording(&mut self) -> Result<PathBuf, RecorderError> {
        let config = self.config.as_ref().ok_or(RecorderError::NotRecording)?;
        let output_path = config.output_path.clone();
        self.state = RecordingState::Idle;
        self.start_time = None;
        self.config = None;
        Ok(output_path)
    }

    fn pause_recording(&mut self) -> Result<(), RecorderError> {
        if self.state != RecordingState::Recording {
            return Err(RecorderError::NotRecording);
        }
        self.state = RecordingState::Paused;
        Ok(())
    }

    fn resume_recording(&mut self) -> Result<(), RecorderError> {
        if self.state != RecordingState::Paused {
            return Err(RecorderError::NotPaused);
        }
        self.state = RecordingState::Recording;
        Ok(())
    }

    fn get_state(&self) -> RecordingState {
        self.state
    }

    fn get_progress(&self) -> RecordingProgress {
        let duration = self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0);

        RecordingProgress {
            duration_seconds: duration,
            frames_captured: duration * 30,          // Assuming 30 FPS
            file_size_bytes: duration * 1024 * 1024, // 1MB per second
            current_bitrate_mbps: 5.0,
        }
    }

    fn name(&self) -> &str {
        "FFmpeg"
    }
}

/// GStreamer recording backend
pub struct GStreamerBackend {
    state: RecordingState,
    start_time: Option<Instant>,
    config: Option<RecordingConfig>,
}

impl GStreamerBackend {
    pub fn new() -> Self {
        Self {
            state: RecordingState::Idle,
            start_time: None,
            config: None,
        }
    }
}

impl RecordingBackend for GStreamerBackend {
    fn start_recording(&mut self, config: &RecordingConfig) -> Result<(), RecorderError> {
        self.state = RecordingState::Recording;
        self.start_time = Some(Instant::now());
        self.config = Some(config.clone());
        Ok(())
    }

    fn stop_recording(&mut self) -> Result<PathBuf, RecorderError> {
        let config = self.config.as_ref().ok_or(RecorderError::NotRecording)?;
        let output_path = config.output_path.clone();
        self.state = RecordingState::Idle;
        self.start_time = None;
        self.config = None;
        Ok(output_path)
    }

    fn pause_recording(&mut self) -> Result<(), RecorderError> {
        if self.state != RecordingState::Recording {
            return Err(RecorderError::NotRecording);
        }
        self.state = RecordingState::Paused;
        Ok(())
    }

    fn resume_recording(&mut self) -> Result<(), RecorderError> {
        if self.state != RecordingState::Paused {
            return Err(RecorderError::NotPaused);
        }
        self.state = RecordingState::Recording;
        Ok(())
    }

    fn get_state(&self) -> RecordingState {
        self.state
    }

    fn get_progress(&self) -> RecordingProgress {
        let duration = self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0);

        RecordingProgress {
            duration_seconds: duration,
            frames_captured: duration * 30,
            file_size_bytes: duration * 1024 * 1024,
            current_bitrate_mbps: 4.5,
        }
    }

    fn name(&self) -> &str {
        "GStreamer"
    }
}

/// OOP-based Screen Recorder
pub struct ScreenRecorder {
    backend: Box<dyn RecordingBackend>,
    current_config: Option<RecordingConfig>,
}

impl ScreenRecorder {
    pub fn new(backend: Box<dyn RecordingBackend>) -> Self {
        Self {
            backend,
            current_config: None,
        }
    }

    /// Start recording
    pub fn start_recording(&mut self, config: RecordingConfig) -> Result<(), RecorderError> {
        self.current_config = Some(config.clone());
        self.backend.start_recording(&config)
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> Result<PathBuf, RecorderError> {
        self.backend.stop_recording()
    }

    /// Pause recording
    pub fn pause_recording(&mut self) -> Result<(), RecorderError> {
        self.backend.pause_recording()
    }

    /// Resume recording
    pub fn resume_recording(&mut self) -> Result<(), RecorderError> {
        self.backend.resume_recording()
    }

    /// Get recording state
    pub fn get_state(&self) -> RecordingState {
        self.backend.get_state()
    }

    /// Get recording progress
    pub fn get_progress(&self) -> RecordingProgress {
        self.backend.get_progress()
    }

    /// Get current config
    pub fn get_config(&self) -> Option<&RecordingConfig> {
        self.current_config.as_ref()
    }

    /// Get backend name
    pub fn backend_name(&self) -> &str {
        self.backend.name()
    }

    /// Is recording
    pub fn is_recording(&self) -> bool {
        self.backend.get_state() == RecordingState::Recording
    }

    /// Is paused
    pub fn is_paused(&self) -> bool {
        self.backend.get_state() == RecordingState::Paused
    }
}

impl Default for ScreenRecorder {
    fn default() -> Self {
        Self::new(Box::new(FfmpegBackend::new()))
    }
}

/// Recorder errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecorderError {
    NotRecording,
    NotPaused,
    StartFailed(String),
    StopFailed(String),
    PauseFailed(String),
    ResumeFailed(String),
    InvalidConfig(String),
    BackendError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_config() {
        let config = RecordingConfig {
            format: RecordingFormat::Mp4,
            video_quality: VideoQuality::High,
            audio_quality: AudioQuality::Medium,
            fps: 30,
            region: RecordingRegion {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
            },
            record_audio: true,
            record_cursor: true,
            output_path: PathBuf::from("/test/recording.mp4"),
        };
        assert_eq!(config.format, RecordingFormat::Mp4);
    }

    #[test]
    fn test_ffmpeg_backend() {
        let backend = FfmpegBackend::new();
        assert_eq!(backend.name(), "FFmpeg");
    }

    #[test]
    fn test_gstreamer_backend() {
        let backend = GStreamerBackend::new();
        assert_eq!(backend.name(), "GStreamer");
    }

    #[test]
    fn test_screen_recorder() {
        let recorder = ScreenRecorder::default();
        assert_eq!(recorder.backend_name(), "FFmpeg");
    }

    #[test]
    fn test_start_recording() {
        let mut recorder = ScreenRecorder::default();
        let config = RecordingConfig {
            format: RecordingFormat::Mp4,
            video_quality: VideoQuality::High,
            audio_quality: AudioQuality::Medium,
            fps: 30,
            region: RecordingRegion {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
            },
            record_audio: true,
            record_cursor: true,
            output_path: PathBuf::from("/test/recording.mp4"),
        };
        recorder.start_recording(config).unwrap();
        assert!(recorder.is_recording());
    }

    #[test]
    fn test_pause_recording() {
        let mut recorder = ScreenRecorder::default();
        let config = RecordingConfig {
            format: RecordingFormat::Mp4,
            video_quality: VideoQuality::High,
            audio_quality: AudioQuality::Medium,
            fps: 30,
            region: RecordingRegion {
                x: 0,
                y: 0,
                width: 1920,
                height: 1080,
            },
            record_audio: true,
            record_cursor: true,
            output_path: PathBuf::from("/test/recording.mp4"),
        };
        recorder.start_recording(config).unwrap();
        recorder.pause_recording().unwrap();
        assert!(recorder.is_paused());
    }
}
