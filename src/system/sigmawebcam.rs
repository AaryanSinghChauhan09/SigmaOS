//! SigmaWebcam - Webcam Management System
//!
//! Provides comprehensive webcam management inspired by Linux V4L2 (Video4Linux2).
//! Supports webcam discovery, video capture, and camera control.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Webcam state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebcamState {
    Idle,
    Streaming,
    Error,
    Unknown,
;

/// Resolution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
;

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
;

/// Frame format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    MJPEG,
    YUYV,
    RGB24,
    NV12,
    Unknown,
;

/// Webcam
#[derive(Debug, Clone)]
pub struct Webcam {
    pub id: String,
    pub name: String,
    pub state: WebcamState,
    pub device: String,
    pub resolutions: Vec<Resolution>,
    pub formats: Vec<FrameFormat>,
;

impl Webcam {
    pub fn new(id: String, name: String, device: String) -> Self {
        Self {
            id,
            name,
            state: WebcamState::Idle,
            device,
            resolutions: Vec::new(),
            formats: Vec::new(),
        }
    }

    pub fn with_resolution(mut self, resolution: Resolution) -> Self {
        self.resolutions.push(resolution);
        self
    }

    pub fn with_format(mut self, format: FrameFormat) -> Self {
        self.formats.push(format);
        self
    }

    pub fn start_stream(&mut self) {
        self.state = WebcamState::Streaming;
    }

    pub fn stop_stream(&mut self) {
        self.state = WebcamState::Idle;
    }
;

/// Webcam manager
pub struct SigmaWebcam {
    webcams: BTreeMap<String, Webcam>,
    next_webcam_id: u64,
;

impl SigmaWebcam {
    pub fn new() -> Self {
        Self {
            webcams: BTreeMap::new(),
            next_webcam_id: 1,
        }
    }

    pub fn add_webcam(&mut self, name: String, device: String) -> String {
        let id = format!("webcam-{}", self.next_webcam_id);
        self.next_webcam_id += 1;
        let webcam = Webcam::new(id.clone(), name, device);
        self.webcams.insert(id.clone(), webcam);
        id
    }

    pub fn get_webcam(&self, id: &str) -> Option<&Webcam> {
        self.webcams.get(id)
    }

    pub fn get_webcam_mut(&mut self, id: &str) -> Option<&mut Webcam> {
        self.webcams.get_mut(id)
    }

    pub fn list_webcams(&self) -> Vec<&Webcam> {
        self.webcams.values().collect()
    }

    pub fn remove_webcam(&mut self, id: &str) -> bool {
        self.webcams.remove(id).is_some()
    }

    pub fn webcam_count(&self) -> usize {
        self.webcams.len()
    }
;

impl Default for SigmaWebcam {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAWEBCAM_INSTANCE: Option<SigmaWebcam> = None;

pub fn init_sigmawebcam() {
    unsafe {
        SIGMAWEBCAM_INSTANCE = Some(SigmaWebcam::new());
    }
;

pub fn get_sigmawebcam() -> &'static SigmaWebcam {
    unsafe {
        SIGMAWEBCAM_INSTANCE
            .as_ref()
            .expect("SigmaWebcam not initialized")
    }
;

pub fn get_sigmawebcam_mut() -> &'static mut SigmaWebcam {
    unsafe {
        SIGMAWEBCAM_INSTANCE
            .as_mut()
            .expect("SigmaWebcam not initialized")
    }
;

pub fn add_webcam(name: String, device: String) -> String {
    let manager = get_sigmawebcam_mut();
    manager.add_webcam(name, device)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webcam_creation() {
        let webcam = Webcam::new(
            "webcam-1".to_string(),
            "Logitech C920".to_string(),
            "/dev/video0".to_string(),
        );
        assert_eq!(webcam.id, "webcam-1");
        assert_eq!(webcam.name, "Logitech C920");
        assert_eq!(webcam.state, WebcamState::Idle);
    }

    #[test]
    fn test_webcam_state() {
        let mut webcam = Webcam::new(
            "webcam-1".to_string(),
            "Logitech".to_string(),
            "/dev/video0".to_string(),
        );
        assert_eq!(webcam.state, WebcamState::Idle);

        webcam.start_stream();
        assert_eq!(webcam.state, WebcamState::Streaming);

        webcam.stop_stream();
        assert_eq!(webcam.state, WebcamState::Idle);
    }

    #[test]
    fn test_resolution() {
        let res = Resolution::new(1920, 1080);
        assert_eq!(res.width, 1920);
        assert_eq!(res.height, 1080);
    }

    #[test]
    fn test_webcam_with_resolution() {
        let webcam = Webcam::new(
            "webcam-1".to_string(),
            "Logitech".to_string(),
            "/dev/video0".to_string(),
        )
        .with_resolution(Resolution::new(1920, 1080))
        .with_resolution(Resolution::new(1280, 720));
        assert_eq!(webcam.resolutions.len(), 2);
    }

    #[test]
    fn test_webcam_with_format() {
        let webcam = Webcam::new(
            "webcam-1".to_string(),
            "Logitech".to_string(),
            "/dev/video0".to_string(),
        )
        .with_format(FrameFormat::MJPEG)
        .with_format(FrameFormat::YUYV);
        assert_eq!(webcam.formats.len(), 2);
    }

    #[test]
    fn test_webcam_manager() {
        let manager = SigmaWebcam::new();
        assert_eq!(manager.webcam_count(), 0);
    }

    #[test]
    fn test_add_webcam() {
        let mut manager = SigmaWebcam::new();
        let id = manager.add_webcam("Logitech C920".to_string(), "/dev/video0".to_string());
        assert_eq!(id, "webcam-1");
        assert_eq!(manager.webcam_count(), 1);
    }

    #[test]
    fn test_list_webcams() {
        let mut manager = SigmaWebcam::new();
        manager.add_webcam("Logitech".to_string(), "/dev/video0".to_string());
        manager.add_webcam("Microsoft".to_string(), "/dev/video1".to_string());

        let webcams = manager.list_webcams();
        assert_eq!(webcams.len(), 2);
    }

    #[test]
    fn test_remove_webcam() {
        let mut manager = SigmaWebcam::new();
        let id = manager.add_webcam("Logitech".to_string(), "/dev/video0".to_string());
        assert_eq!(manager.webcam_count(), 1);

        manager.remove_webcam(&id);
        assert_eq!(manager.webcam_count(), 0);
    }

    #[test]
    fn test_global_instance() {
        init_sigmawebcam();
        let manager = get_sigmawebcam();
        assert_eq!(manager.webcam_count(), 0);

        let manager_mut = get_sigmawebcam_mut();
        manager_mut.add_webcam("Logitech".to_string(), "/dev/video0".to_string());
        assert_eq!(manager.webcam_count(), 1);
    }

    #[test]
    fn test_add_webcam_convenience() {
        init_sigmawebcam();
        let id = add_webcam("Logitech".to_string(), "/dev/video0".to_string());
        assert_eq!(id, "webcam-1");

        let manager = get_sigmawebcam();
        assert!(manager.get_webcam(&id).is_some());
    }
;
