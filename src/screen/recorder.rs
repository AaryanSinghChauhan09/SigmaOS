//! SigmaOS Screen Recording System
//!
//! Inspired by: Omarchy screen recording (omarchy-capture-screenrecording, omancy-capture-screenrecording-with-webcam)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, SECURE_DEFAULTS, DIRECT_DEVICE_CONTROL, LOW_LEVEL_CONTROL,
//! MEMORY_SAFETY, ZERO_COPY, EFFICIENT_SCHEDULING, SAFE_CONCURRENCY.

use std::collections::HashMap;

/// Recording status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordingStatus {
    Idle,
    Recording,
    Paused,
    Stopping,
    Error(String),
}

/// Recording format
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordingFormat {
    Mp4,
    Webm,
    Avi,
    Mov,
}

/// Screen recording configuration
#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub format: RecordingFormat,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
    pub audio_enabled: bool,
    pub webcam_enabled: bool,
    pub webcam_position: WebcamPosition,
    pub webcam_size: u32,
    pub bitrate: u32,
    pub show_cursor: bool,
    pub show_clicks: bool,
}

/// Webcam position on screen
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebcamPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

/// Screen recording session
#[derive(Debug, Clone)]
pub struct RecordingSession {
    pub id: u64,
    pub file_path: String,
    pub config: RecordingConfig,
    pub status: RecordingStatus,
    pub started_at: Option<u64>,
    pub stopped_at: Option<u64>,
    pub duration_seconds: u64,
    pub frame_count: u64,
    pub file_size_bytes: u64,
}

/// Screen recorder
pub struct ScreenRecorder {
    pub sessions: Vec<RecordingSession>,
    pub current_session: Option<u64>,
    pub next_id: u64,
    pub max_history: usize,
}

impl ScreenRecorder {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
            current_session: None,
            next_id: 1,
            max_history: 50,
        }
    }

    /// Start a new recording session
    pub fn start(
        &mut self,
        file_path: String,
        config: RecordingConfig,
    ) -> Result<u64, String> {
        if file_path.is_empty() {
            return Err("File path cannot be empty".to_string());
        }

        if self.current_session.is_some() {
            return Err("A recording is already in progress".to_string());
        }

        if config.fps == 0 || config.fps > 120 {
            return Err("FPS must be between 1 and 120".to_string());
        }

        if config.width == 0 || config.height == 0 {
            return Err("Width and height must be greater than 0".to_string());
        }

        let id = self.next_id;
        self.next_id += 1;
        let now = now_secs();

        let session = RecordingSession {
            id,
            file_path,
            config,
            status: RecordingStatus::Recording,
            started_at: Some(now),
            stopped_at: None,
            duration_seconds: 0,
            frame_count: 0,
            file_size_bytes: 0,
        };

        self.sessions.push(session);
        self.current_session = Some(id);

        Ok(id)
    }

    /// Pause the current recording
    pub fn pause(&mut self) -> Result<(), String> {
        let id = self.current_session.ok_or_else(|| "No recording in progress".to_string())?;
        let session = self.sessions.iter_mut().find(|s| s.id == id).ok_or_else(|| format!("Session {} not found", id))?;

        if session.status != RecordingStatus::Recording {
            return Err(format!("Session {} is not recording", id));
        }

        session.status = RecordingStatus::Paused;
        Ok(())
    }

    /// Resume the current recording
    pub fn resume(&mut self) -> Result<(), String> {
        let id = self.current_session.ok_or_else(|| "No recording in progress".to_string())?;
        let session = self.sessions.iter_mut().find(|s| s.id == id).ok_or_else(|| format!("Session {} not found", id))?;

        if session.status != RecordingStatus::Paused {
            return Err(format!("Session {} is not paused", id));
        }

        session.status = RecordingStatus::Recording;
        Ok(())
    }

    /// Stop the current recording
    pub fn stop(&mut self) -> Result<(), String> {
        let id = self.current_session.take().ok_or_else(|| "No recording in progress".to_string())?;
        let session = self.sessions.iter_mut().find(|s| s.id == id).ok_or_else(|| format!("Session {} not found", id))?;

        if session.status != RecordingStatus::Recording && session.status != RecordingStatus::Paused {
            return Err(format!("Session {} is not in a stoppable state", id));
        }

        let now = now_secs();
        session.status = RecordingStatus::Stopping;
        session.stopped_at = Some(now);

        if let Some(started) = session.started_at {
            session.duration_seconds = now.saturating_sub(started);
        }

        Ok(id)
    }

    /// Get the current session
    pub fn current(&self) -> Option<&RecordingSession> {
        self.current_session.and_then(|id| self.sessions.iter().find(|s| s.id == id))
    }

    /// List all sessions
    pub fn list(&self) -> Vec<&RecordingSession> {
        self.sessions.iter().rev().collect()
    }

    /// Get a session by ID
    pub fn get(&self, id: u64) -> Option<&RecordingSession> {
        self.sessions.iter().find(|s| s.id == id)
    }

    /// Delete a session
    pub fn delete(&mut self, id: u64) -> Result<(), String> {
        let pos = self
            .sessions
            .iter()
            .position(|s| s.id == id)
            .ok_or_else(|| format!("Session {} not found", id))?;
        self.sessions.remove(pos);
        if self.current_session == Some(id) {
            self.current_session = None;
        }
        Ok(())
    }

    /// Get recording stats
    pub fn stats(&self) -> RecordingStats {
        let total_sessions = self.sessions.len();
        let total_frames: u64 = self.sessions.iter().map(|s| s.frame_count).sum();
        let total_duration: u64 = self.sessions.iter().map(|s| s.duration_seconds).sum();
        let total_size: u64 = self.sessions.iter().map(|s| s.file_size_bytes).sum();

        RecordingStats {
            total_sessions,
            total_frames,
            total_duration_seconds: total_duration,
            total_size_bytes: total_size,
        }
    }

    fn trim_history(&mut self) {
        while self.sessions.len() > self.max_history {
            self.sessions.pop_front();
        }
    }
}

impl Default for ScreenRecorder {
    fn default() -> Self {
        Self::new()
    }
}

/// Recording statistics
#[derive(Debug, Clone)]
pub struct RecordingStats {
    pub total_sessions: usize,
    pub total_frames: u64,
    pub total_duration_seconds: u64,
    pub total_size_bytes: u64,
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_recording() {
        let mut recorder = ScreenRecorder::new();
        let config = RecordingConfig {
            format: RecordingFormat::Mp4,
            fps: 30,
            width: 1920,
            height: 1080,
            audio_enabled: true,
            webcam_enabled: false,
            webcam_position: WebcamPosition::BottomRight,
            webcam_size: 160,
            bitrate: 5000,
            show_cursor: true,
            show_clicks: false,
        };
        let id = recorder.start("/tmp/recording.mp4".to_string(), config).unwrap();
        assert_eq!(id, 1);
        assert!(recorder.current().is_some());
        assert_eq!(recorder.current().unwrap().status, RecordingStatus::Recording);
    }

    #[test]
    fn test_start_duplicate_fails() {
        let mut recorder = ScreenRecorder::new();
        let config = RecordingConfig {
            format: RecordingFormat::Mp4,
            fps: 30,
            width: 1920,
            height: 1080,
            audio_enabled: true,
            webcam_enabled: false,
            webcam_position: WebcamPosition::BottomRight,
            webcam_size: 160,
            bitrate: 5000,
            show_cursor: true,
            show_clicks: false,
        };
        recorder.start("/tmp/rec1.mp4".to_string(), config.clone()).unwrap();
        let result = recorder.start("/tmp/rec2.mp4".to_string(), config);
        assert!(result.is_err());
    }

    #[test]
    fn test_pause_resume() {
        let mut recorder = ScreenRecorder::new();
        let config = RecordingConfig {
            format: RecordingFormat::Mp4,
            fps: 30,
            width: 1920,
            height: 1080,
            audio_enabled: true,
            webcam_enabled: false,
            webcam_position: WebcamPosition::BottomRight,
            webcam_size: 160,
            bitrate: 5000,
            show_cursor: true,
            show_clicks: false,
        };
        let id = recorder.start("/tmp/rec.mp4".to_string(), config).unwrap();
        recorder.pause().unwrap();
        assert_eq!(recorder.current().unwrap().status, RecordingStatus::Paused);
        recorder.resume().unwrap();
        assert_eq!(recorder.current().unwrap().status, RecordingStatus::Recording);
    }

    #[test]
    fn test_stop_recording() {
        let mut recorder = ScreenRecorder::new();
        let config = RecordingConfig {
            format: RecordingFormat::Mp4,
            fps: 30,
            width: 1920,
            height: 1080,
            audio_enabled: true,
            webcam_enabled: false,
            webcam_position: WebcamPosition::BottomRight,
            webcam_size: 160,
            bitrate: 5000,
            show_cursor: true,
            show_clicks: false,
        };
        recorder.start("/tmp/rec.mp4".to_string(), config).unwrap();
        let stopped_id = recorder.stop().unwrap();
        assert_eq!(stopped_id, 1);
        assert!(recorder.current().is_none());
        assert_eq!(recorder.sessions[0].status, RecordingStatus::Stopping);
    }

    #[test]
    fn test_stop_without_recording_fails() {
        let mut recorder = ScreenRecorder::new();
        let result = recorder.stop();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_fps() {
        let mut recorder = ScreenRecorder::new();
        let mut config = RecordingConfig {
            format: RecordingFormat::Mp4,
            fps: 0,
            width: 1920,
            height: 1080,
            audio_enabled: true,
            webcam_enabled: false,
            webcam_position: WebcamPosition::BottomRight,
            webcam_size: 160,
            bitrate: 5000,
            show_cursor: true,
            show_clicks: false,
        };
        let result = recorder.start("/tmp/rec.mp4".to_string(), config);
        assert!(result.is_err());
    }
}