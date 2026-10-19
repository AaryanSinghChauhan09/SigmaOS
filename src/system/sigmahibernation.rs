//! SigmaHibernation - System Hibernation Management
//!
//! Provides comprehensive hibernation (suspend-to-disk) management inspired by Linux
//! hibernation and BSD ACPI sleep states. Supports hibernation, suspend, and resume operations.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Power state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerState {
    Running,
    Suspend,
    Hibernation,
    Unknown,
}

/// Hibernation method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HibernationMethod {
    Platform,
    Shutdown,
    Reboot,
    Unknown,
}

/// Hibernation state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HibernationState {
    Idle,
    Saving,
    Saved,
    Resuming,
    Failed,
    Unknown,
}

/// Hibernation image
#[derive(Debug, Clone)]
pub struct HibernationImage {
    pub id: String,
    pub swap_device: String,
    pub size: u64,
    pub image_size: u64,
    pub compression: bool,
    pub checksum: Option<String>,
}

impl HibernationImage {
    pub fn new(id: String, swap_device: String, size: u64) -> Self {
        Self {
            id,
            swap_device,
            size,
            image_size: 0,
            compression: false,
            checksum: None,
        }
    }

    pub fn with_compression(mut self, compression: bool) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_checksum(mut self, checksum: String) -> Self {
        self.checksum = Some(checksum);
        self
    }

    pub fn set_image_size(&mut self, size: u64) {
        self.image_size = size;
    }
}

/// Hibernation manager
pub struct SigmaHibernation {
    images: BTreeMap<String, HibernationImage>,
    next_image_id: u64,
    current_state: PowerState,
    hibernation_state: HibernationState,
    hibernation_method: HibernationMethod,
    resume_device: Option<String>,
}

impl SigmaHibernation {
    pub fn new() -> Self {
        Self {
            images: BTreeMap::new(),
            next_image_id: 1,
            current_state: PowerState::Running,
            hibernation_state: HibernationState::Idle,
            hibernation_method: HibernationMethod::Platform,
            resume_device: None,
        }
    }

    pub fn create_image(&mut self, swap_device: String, size: u64) -> String {
        let id = format!("image-{}", self.next_image_id);
        self.next_image_id += 1;
        let image = HibernationImage::new(id.clone(), swap_device, size);
        self.images.insert(id.clone(), image);
        id
    }

    pub fn get_image(&self, id: &str) -> Option<&HibernationImage> {
        self.images.get(id)
    }

    pub fn get_image_mut(&mut self, id: &str) -> Option<&mut HibernationImage> {
        self.images.get_mut(id)
    }

    pub fn list_images(&self) -> Vec<&HibernationImage> {
        self.images.values().collect()
    }

    pub fn remove_image(&mut self, id: &str) -> bool {
        self.images.remove(id).is_some()
    }

    pub fn set_hibernation_method(&mut self, method: HibernationMethod) {
        self.hibernation_method = method;
    }

    pub fn get_hibernation_method(&self) -> HibernationMethod {
        self.hibernation_method
    }

    pub fn set_resume_device(&mut self, device: String) {
        self.resume_device = Some(device);
    }

    pub fn get_resume_device(&self) -> Option<&String> {
        self.resume_device.as_ref()
    }

    pub fn hibernate(&mut self) -> bool {
        if self.current_state == PowerState::Running {
            self.hibernation_state = HibernationState::Saving;
            self.current_state = PowerState::Hibernation;
            true
        } else {
            false
        }
    }

    pub fn suspend(&mut self) -> bool {
        if self.current_state == PowerState::Running {
            self.current_state = PowerState::Suspend;
            true
        } else {
            false
        }
    }

    pub fn resume(&mut self) -> bool {
        if self.current_state == PowerState::Hibernation {
            self.hibernation_state = HibernationState::Resuming;
            self.current_state = PowerState::Running;
            self.hibernation_state = HibernationState::Idle;
            true
        } else {
            false
        }
    }

    pub fn wake(&mut self) -> bool {
        if self.current_state == PowerState::Suspend {
            self.current_state = PowerState::Running;
            true
        } else {
            false
        }
    }

    pub fn mark_saved(&mut self) {
        self.hibernation_state = HibernationState::Saved;
    }

    pub fn mark_failed(&mut self) {
        self.hibernation_state = HibernationState::Failed;
        self.current_state = PowerState::Running;
    }

    pub fn get_current_state(&self) -> PowerState {
        self.current_state
    }

    pub fn get_hibernation_state(&self) -> HibernationState {
        self.hibernation_state
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }
}

impl Default for SigmaHibernation {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMAHIBERNATION_INSTANCE: Option<SigmaHibernation> = None;

pub fn init_sigmahibernation() {
    unsafe {
        SIGMAHIBERNATION_INSTANCE = Some(SigmaHibernation::new());
    }
}

pub fn get_sigmahibernation() -> &'static SigmaHibernation {
    unsafe {
        SIGMAHIBERNATION_INSTANCE
            .as_ref()
            .expect("SigmaHibernation not initialized")
    }
}

pub fn get_sigmahibernation_mut() -> &'static mut SigmaHibernation {
    unsafe {
        SIGMAHIBERNATION_INSTANCE
            .as_mut()
            .expect("SigmaHibernation not initialized")
    }
}

pub fn create_image(swap_device: String, size: u64) -> String {
    let manager = get_sigmahibernation_mut();
    manager.create_image(swap_device, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hibernation_image_creation() {
        let image = HibernationImage::new(
            "image-1".to_string(),
            "/dev/sda1".to_string(),
            4 * 1024 * 1024 * 1024,
        );
        assert_eq!(image.id, "image-1");
        assert_eq!(image.swap_device, "/dev/sda1");
        assert_eq!(image.size, 4 * 1024 * 1024 * 1024);
        assert!(!image.compression);
    }

    #[test]
    fn test_hibernation_image_builder() {
        let image = HibernationImage::new(
            "image-1".to_string(),
            "/dev/sda1".to_string(),
            4 * 1024 * 1024 * 1024,
        )
        .with_compression(true)
        .with_checksum("abc123".to_string());

        assert!(image.compression);
        assert_eq!(image.checksum, Some("abc123".to_string()));
    }

    #[test]
    fn test_hibernation_manager() {
        let manager = SigmaHibernation::new();
        assert_eq!(manager.get_current_state(), PowerState::Running);
        assert_eq!(manager.get_hibernation_state(), HibernationState::Idle);
    }

    #[test]
    fn test_create_image() {
        let mut manager = SigmaHibernation::new();
        let id = manager.create_image("/dev/sda1".to_string(), 4 * 1024 * 1024 * 1024);
        assert_eq!(id, "image-1");
        assert_eq!(manager.image_count(), 1);
    }

    #[test]
    fn test_hibernate_resume() {
        let mut manager = SigmaHibernation::new();
        assert_eq!(manager.get_current_state(), PowerState::Running);

        assert!(manager.hibernate());
        assert_eq!(manager.get_current_state(), PowerState::Hibernation);
        assert_eq!(manager.get_hibernation_state(), HibernationState::Saving);

        manager.mark_saved();
        assert_eq!(manager.get_hibernation_state(), HibernationState::Saved);

        assert!(manager.resume());
        assert_eq!(manager.get_current_state(), PowerState::Running);
        assert_eq!(manager.get_hibernation_state(), HibernationState::Idle);
    }

    #[test]
    fn test_suspend_wake() {
        let mut manager = SigmaHibernation::new();
        assert_eq!(manager.get_current_state(), PowerState::Running);

        assert!(manager.suspend());
        assert_eq!(manager.get_current_state(), PowerState::Suspend);

        assert!(manager.wake());
        assert_eq!(manager.get_current_state(), PowerState::Running);
    }

    #[test]
    fn test_hibernation_method() {
        let mut manager = SigmaHibernation::new();
        manager.set_hibernation_method(HibernationMethod::Shutdown);
        assert_eq!(
            manager.get_hibernation_method(),
            HibernationMethod::Shutdown
        );
    }

    #[test]
    fn test_resume_device() {
        let mut manager = SigmaHibernation::new();
        manager.set_resume_device("/dev/sda1".to_string());
        assert_eq!(manager.get_resume_device(), Some(&"/dev/sda1".to_string()));
    }

    #[test]
    fn test_hibernation_failure() {
        let mut manager = SigmaHibernation::new();
        manager.hibernate();
        manager.mark_failed();
        assert_eq!(manager.get_hibernation_state(), HibernationState::Failed);
        assert_eq!(manager.get_current_state(), PowerState::Running);
    }

    #[test]
    fn test_hibernate_from_non_running() {
        let mut manager = SigmaHibernation::new();
        manager.suspend();
        assert!(!manager.hibernate());
    }

    #[test]
    fn test_resume_from_non_hibernation() {
        let mut manager = SigmaHibernation::new();
        assert!(!manager.resume());
    }

    #[test]
    fn test_global_instance() {
        init_sigmahibernation();
        let manager = get_sigmahibernation();
        assert_eq!(manager.image_count(), 0);

        let manager_mut = get_sigmahibernation_mut();
        manager_mut.create_image("/dev/sda1".to_string(), 4 * 1024 * 1024 * 1024);
        assert_eq!(manager.image_count(), 1);
    }

    #[test]
    fn test_create_image_convenience() {
        init_sigmahibernation();
        let id = create_image("/dev/sda1".to_string(), 4 * 1024 * 1024 * 1024);
        assert_eq!(id, "image-1");

        let manager = get_sigmahibernation();
        assert!(manager.get_image(&id).is_some());
    }
}
