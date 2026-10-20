// SPDX-License-Identifier: MIT
// SigmaOS - Capture Manager
// Capture management inspired by Omarchy capture-* commands and Linux capture tools

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Capture type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureType {
    Screenshot,
    ScreenRecording,
    WindowRecording,
    RegionRecording,
}

/// Capture
#[derive(Debug, Clone)]
pub struct Capture {
    pub id: String,
    pub capture_type: CaptureType,
    pub file_path: String,
    pub timestamp: u64,
}

/// Capture manager
pub struct SigmaCaptureManager {
    captures: Vec<Capture>,
}

impl SigmaCaptureManager {
    pub fn new() -> Self {
        Self {
            captures: Vec::new(),
        }
    }

    pub fn add_capture(&mut self, capture: Capture) {
        self.captures.push(capture);
    }

    pub fn get_captures(&self) -> &[Capture] {
        &self.captures
    }

    pub fn get_capture(&self, id: &str) -> Option<&Capture> {
        self.captures.iter().find(|c| c.id == id)
    }

    pub fn get_captures_by_type(&self, capture_type: CaptureType) -> Vec<&Capture> {
        self.captures.iter().filter(|c| c.capture_type == capture_type).collect()
    }

    pub fn delete_capture(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.captures.iter().position(|c| c.id == id) {
            self.captures.remove(idx);
            Ok(())
        } else {
            Err(format!("Capture {} not found", id))
        }
    }
}

impl Default for SigmaCaptureManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_manager_creation() {
        let manager = SigmaCaptureManager::new();
        assert_eq!(manager.get_captures().len(), 0);
    }

    #[test]
    fn test_add_capture() {
        let mut manager = SigmaCaptureManager::new();
        let capture = Capture {
            id: "screenshot-001".to_string(),
            capture_type: CaptureType::Screenshot,
            file_path: "/screenshots/001.png".to_string(),
            timestamp: 0,
        };
        manager.add_capture(capture);
        assert_eq!(manager.get_captures().len(), 1);
    }

    #[test]
    fn test_captures_by_type() {
        let mut manager = SigmaCaptureManager::new();
        let capture1 = Capture {
            id: "screenshot-001".to_string(),
            capture_type: CaptureType::Screenshot,
            file_path: "/screenshots/001.png".to_string(),
            timestamp: 0,
        };
        let capture2 = Capture {
            id: "recording-001".to_string(),
            capture_type: CaptureType::ScreenRecording,
            file_path: "/recordings/001.mp4".to_string(),
            timestamp: 0,
        };
        manager.add_capture(capture1);
        manager.add_capture(capture2);
        assert_eq!(manager.get_captures_by_type(CaptureType::Screenshot).len(), 1);
    }

    #[test]
    fn test_delete_capture() {
        let mut manager = SigmaCaptureManager::new();
        let capture = Capture {
            id: "screenshot-001".to_string(),
            capture_type: CaptureType::Screenshot,
            file_path: "/screenshots/001.png".to_string(),
            timestamp: 0,
        };
        manager.add_capture(capture);
        manager.delete_capture("screenshot-001".to_string()).unwrap();
        assert_eq!(manager.get_captures().len(), 0);
    }
}
