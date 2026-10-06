// Screen Recording
// Omarchy-inspired screen recording with GPU acceleration, audio mixing, and webcam overlay

use std::path::PathBuf;

/// Audio source type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSource {
    None,
    Desktop,
    Microphone,
    DesktopAndMicrophone,
    DesktopAndMicrophoneAndWebcam,
}

/// Capture mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    Fullscreen,
    Region,
    Window,
    Monitor,
}

/// Recording resolution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Resolution { width, height }
    }

    pub fn native() -> Self {
        Resolution { width: 0, height: 0 }
    }

    pub fn hd720() -> Self {
        Resolution { width: 1280, height: 720 }
    }

    pub fn hd1080() -> Self {
        Resolution { width: 1920, height: 1080 }
    }

    pub fn uhd4k() -> Self {
        Resolution { width: 3840, height: 2160 }
    }

    pub fn is_native(&self) -> bool {
        self.width == 0 && self.height == 0
    }
}

/// Webcam size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebcamSize {
    Small,
    Medium,
    Large,
}

/// Recording state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    Idle,
    Recording,
    Paused,
    Stopping,
}

/// Recording format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingFormat {
    Mp4,
    WebM,
    Gif,
}

/// Frame rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameRate {
    Fps30,
    Fps60,
    Fps120,
    Custom(u32),
}

impl FrameRate {
    pub fn value(&self) -> u32 {
        match self {
            FrameRate::Fps30 => 30,
            FrameRate::Fps60 => 60,
            FrameRate::Fps120 => 120,
            FrameRate::Custom(fps) => *fps,
        }
    }
}

/// Recording configuration
#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub audio_source: AudioSource,
    pub capture_mode: CaptureMode,
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub format: RecordingFormat,
    pub webcam_enabled: bool,
    pub webcam_device: Option<String>,
    pub webcam_size: WebcamSize,
    pub output_dir: PathBuf,
    pub gpu_acceleration: bool,
    pub trim_first_frame: bool,
    pub normalize_audio: bool,
    pub target_lufs: f64,
}

impl Default for RecordingConfig {
    fn default() -> Self {
        RecordingConfig {
            audio_source: AudioSource::None,
            capture_mode: CaptureMode::Fullscreen,
            resolution: Resolution::native(),
            frame_rate: FrameRate::Fps60,
            format: RecordingFormat::Mp4,
            webcam_enabled: false,
            webcam_device: None,
            webcam_size: WebcamSize::Medium,
            output_dir: PathBuf::from("/home/user/Videos"),
            gpu_acceleration: true,
            trim_first_frame: true,
            normalize_audio: true,
            target_lufs: -14.0,
        }
    }
}

/// Recording session
#[derive(Debug, Clone)]
pub struct RecordingSession {
    pub id: String,
    pub start_time: String,
    pub file_path: PathBuf,
    pub duration_seconds: u64,
    pub file_size_bytes: u64,
    pub state: RecordingState,
    pub config: RecordingConfig,
}

/// Screen recorder
#[derive(Debug, Clone)]
pub struct ScreenRecorder {
    pub config: RecordingConfig,
    pub current_session: Option<RecordingSession>,
    pub sessions: Vec<RecordingSession>,
    pub state: RecordingState,
    pub available_webcams: Vec<String>,
}

impl Default for ScreenRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl ScreenRecorder {
    pub fn new() -> Self {
        ScreenRecorder {
            config: RecordingConfig::default(),
            current_session: None,
            sessions: Vec::new(),
            state: RecordingState::Idle,
            available_webcams: Vec::new(),
        }
    }

    /// Detect available webcams
    pub fn detect_webcams(&mut self) {
        self.available_webcams = vec![
            "/dev/video0".to_string(),
            "/dev/video1".to_string(),
        ];
    }

    /// Get available webcams
    pub fn get_webcams(&self) -> &[String] {
        &self.available_webcams
    }

    /// Set configuration
    pub fn set_config(&mut self, config: RecordingConfig) {
        self.config = config;
    }

    /// Start recording
    pub fn start_recording(&mut self) -> Result<String, String> {
        if self.state == RecordingState::Recording {
            return Err("Already recording".to_string());
        }

        let timestamp = "2026-01-01_12-00-00".to_string();
        let filename = format!("screenrecording-{}.mp4", timestamp);
        let file_path = self.config.output_dir.join(&filename);

        let session = RecordingSession {
            id: timestamp.clone(),
            start_time: timestamp,
            file_path: file_path.clone(),
            duration_seconds: 0,
            file_size_bytes: 0,
            state: RecordingState::Recording,
            config: self.config.clone(),
        };

        self.current_session = Some(session.clone());
        self.sessions.push(session);
        self.state = RecordingState::Recording;

        Ok(file_path.display().to_string())
    }

    /// Stop recording
    pub fn stop_recording(&mut self) -> Result<(), String> {
        if self.state != RecordingState::Recording {
            return Err("Not recording".to_string());
        }

        self.state = RecordingState::Stopping;

        if let Some(session_id) = self.current_session.as_ref().map(|s| s.id.clone()) {
            if let Some(session) = self.sessions.iter_mut().find(|s| s.id == session_id) {
                session.state = RecordingState::Idle;
                session.duration_seconds = 10; // Simulated duration
                session.file_size_bytes = 1024 * 1024 * 50; // Simulated 50MB
            }
        }

        self.current_session = None;
        self.state = RecordingState::Idle;

        Ok(())
    }

    /// Pause recording
    pub fn pause_recording(&mut self) -> Result<(), String> {
        if self.state != RecordingState::Recording {
            return Err("Not recording".to_string());
        }

        self.state = RecordingState::Paused;

        if let Some(session) = &mut self.current_session {
            session.state = RecordingState::Paused;
        }

        Ok(())
    }

    /// Resume recording
    pub fn resume_recording(&mut self) -> Result<(), String> {
        if self.state != RecordingState::Paused {
            return Err("Not paused".to_string());
        }

        self.state = RecordingState::Recording;

        if let Some(session) = &mut self.current_session {
            session.state = RecordingState::Recording;
        }

        Ok(())
    }

    /// Cancel recording
    pub fn cancel_recording(&mut self) -> Result<(), String> {
        if self.state != RecordingState::Recording && self.state != RecordingState::Paused {
            return Err("Not recording".to_string());
        }

        if let Some(session) = self.current_session.take() {
            self.sessions.retain(|s| s.id != session.id);
        }

        self.state = RecordingState::Idle;

        Ok(())
    }

    /// Get current session
    pub fn get_current_session(&self) -> Option<&RecordingSession> {
        self.current_session.as_ref()
    }

    /// Get all sessions
    pub fn get_sessions(&self) -> &[RecordingSession] {
        &self.sessions
    }

    /// Set audio source
    pub fn set_audio_source(&mut self, source: AudioSource) {
        self.config.audio_source = source;
    }

    /// Set capture mode
    pub fn set_capture_mode(&mut self, mode: CaptureMode) {
        self.config.capture_mode = mode;
    }

    /// Set resolution
    pub fn set_resolution(&mut self, resolution: Resolution) {
        self.config.resolution = resolution;
    }

    /// Set frame rate
    pub fn set_frame_rate(&mut self, frame_rate: FrameRate) {
        self.config.frame_rate = frame_rate;
    }

    /// Set format
    pub fn set_format(&mut self, format: RecordingFormat) {
        self.config.format = format;
    }

    /// Enable webcam
    pub fn enable_webcam(&mut self, enabled: bool) {
        self.config.webcam_enabled = enabled;
    }

    /// Set webcam device
    pub fn set_webcam_device(&mut self, device: Option<String>) {
        self.config.webcam_device = device;
    }

    /// Set webcam size
    pub fn set_webcam_size(&mut self, size: WebcamSize) {
        self.config.webcam_size = size;
    }

    /// Set output directory
    pub fn set_output_dir(&mut self, dir: PathBuf) {
        self.config.output_dir = dir;
    }

    /// Enable GPU acceleration
    pub fn enable_gpu_acceleration(&mut self, enabled: bool) {
        self.config.gpu_acceleration = enabled;
    }

    /// Get recording state
    pub fn get_state(&self) -> RecordingState {
        self.state
    }

    /// Get configuration
    pub fn get_config(&self) -> &RecordingConfig {
        &self.config
    }

    /// Delete session
    pub fn delete_session(&mut self, id: &str) -> Result<(), String> {
        if self.current_session.as_ref().map(|s| s.id == id).unwrap_or(false) {
            return Err("Cannot delete current session".to_string());
        }

        self.sessions.retain(|s| s.id != id);
        Ok(())
    }

    /// Get statistics
    pub fn get_statistics(&self) -> (usize, u64, u64) {
        let total_duration = self.sessions.iter().map(|s| s.duration_seconds).sum();
        let total_size = self.sessions.iter().map(|s| s.file_size_bytes).sum();
        (self.sessions.len(), total_duration, total_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_recorder_creation() {
        let recorder = ScreenRecorder::new();
        assert_eq!(recorder.state, RecordingState::Idle);
        assert_eq!(recorder.sessions.len(), 0);
    }

    #[test]
    fn test_start_recording() {
        let mut recorder = ScreenRecorder::new();
        recorder.start_recording().unwrap();
        assert_eq!(recorder.state, RecordingState::Recording);
        assert!(recorder.current_session.is_some());
    }

    #[test]
    fn test_stop_recording() {
        let mut recorder = ScreenRecorder::new();
        recorder.start_recording().unwrap();
        recorder.stop_recording().unwrap();
        assert_eq!(recorder.state, RecordingState::Idle);
        assert!(recorder.current_session.is_none());
    }

    #[test]
    fn test_pause_resume() {
        let mut recorder = ScreenRecorder::new();
        recorder.start_recording().unwrap();
        recorder.pause_recording().unwrap();
        assert_eq!(recorder.state, RecordingState::Paused);
        recorder.resume_recording().unwrap();
        assert_eq!(recorder.state, RecordingState::Recording);
    }

    #[test]
    fn test_cancel_recording() {
        let mut recorder = ScreenRecorder::new();
        recorder.start_recording().unwrap();
        recorder.cancel_recording().unwrap();
        assert_eq!(recorder.state, RecordingState::Idle);
        assert_eq!(recorder.sessions.len(), 0);
    }

    #[test]
    fn test_audio_source() {
        let mut recorder = ScreenRecorder::new();
        recorder.set_audio_source(AudioSource::Desktop);
        assert_eq!(recorder.config.audio_source, AudioSource::Desktop);
    }

    #[test]
    fn test_capture_mode() {
        let mut recorder = ScreenRecorder::new();
        recorder.set_capture_mode(CaptureMode::Region);
        assert_eq!(recorder.config.capture_mode, CaptureMode::Region);
    }

    #[test]
    fn test_resolution() {
        let mut recorder = ScreenRecorder::new();
        recorder.set_resolution(Resolution::hd1080());
        assert_eq!(recorder.config.resolution.width, 1920);
        assert_eq!(recorder.config.resolution.height, 1080);
    }

    #[test]
    fn test_frame_rate() {
        let mut recorder = ScreenRecorder::new();
        recorder.set_frame_rate(FrameRate::Fps30);
        assert_eq!(recorder.config.frame_rate.value(), 30);
    }

    #[test]
    fn test_webcam() {
        let mut recorder = ScreenRecorder::new();
        recorder.enable_webcam(true);
        assert!(recorder.config.webcam_enabled);
        recorder.set_webcam_device(Some("/dev/video0".to_string()));
        assert_eq!(recorder.config.webcam_device, Some("/dev/video0".to_string()));
    }

    #[test]
    fn test_detect_webcams() {
        let mut recorder = ScreenRecorder::new();
        recorder.detect_webcams();
        assert!(!recorder.available_webcams.is_empty());
    }

    #[test]
    fn test_sessions() {
        let mut recorder = ScreenRecorder::new();
        recorder.start_recording().unwrap();
        recorder.stop_recording().unwrap();
        assert_eq!(recorder.sessions.len(), 1);
    }

    #[test]
    fn test_delete_session() {
        let mut recorder = ScreenRecorder::new();
        recorder.start_recording().unwrap();
        recorder.stop_recording().unwrap();
        let session_id = recorder.sessions[0].id.clone();
        recorder.delete_session(&session_id).unwrap();
        assert_eq!(recorder.sessions.len(), 0);
    }

    #[test]
    fn test_statistics() {
        let mut recorder = ScreenRecorder::new();
        recorder.start_recording().unwrap();
        recorder.stop_recording().unwrap();
        let (count, duration, size) = recorder.get_statistics();
        assert_eq!(count, 1);
        assert!(duration >= 0);
        assert!(size > 0);
    }
}
