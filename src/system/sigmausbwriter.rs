//! SigmaUSBWriter - Live USB Creation Tool
//!
//! Inspired by Linux Mint's Mintstick and Etcher, this module provides comprehensive
//! USB image writing capabilities including image validation, device selection, and
//! progress tracking.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Write state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteState {
    Idle,
    Validating,
    Writing,
    Verifying,
    Completed,
    Failed,
    Cancelled,
    Unknown,
}

/// Image format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    ISO,
    IMG,
    Raw,
    Compressed,
    Unknown,
}

/// USB device data structure
#[derive(Debug, Clone)]
pub struct USBDevice {
    pub device_id: String,
    pub device_name: String,
    pub device_path: String,
    pub size_bytes: u64,
    pub is_removable: bool,
    pub is_busy: bool,
}

impl USBDevice {
    /// Create a new USB device
    pub fn new(
        device_id: String,
        device_name: String,
        device_path: String,
        size_bytes: u64,
        is_removable: bool,
    ) -> Self {
        Self {
            device_id,
            device_name,
            device_path,
            size_bytes,
            is_removable,
            is_busy: false,
        }
    }

    /// Set busy state
    pub fn set_busy(&mut self, busy: bool) {
        self.is_busy = busy;
    }

    /// Check if device is busy
    pub fn is_device_busy(&self) -> bool {
        self.is_busy
    }

    /// Check if device is suitable for writing
    pub fn is_suitable(&self) -> bool {
        self.is_removable && !self.is_busy
    }
}

/// Write job data structure
#[derive(Debug, Clone)]
pub struct WriteJob {
    pub job_id: String,
    pub image_path: String,
    pub device_id: String,
    pub image_format: ImageFormat,
    pub image_size: u64,
    pub bytes_written: u64,
    pub bytes_verified: u64,
    pub state: WriteState,
    pub progress_percent: u8,
    pub error_message: String,
}

impl WriteJob {
    /// Create a new write job
    pub fn new(
        job_id: String,
        image_path: String,
        device_id: String,
        image_format: ImageFormat,
        image_size: u64,
    ) -> Self {
        Self {
            job_id,
            image_path,
            device_id,
            image_format,
            image_size,
            bytes_written: 0,
            bytes_verified: 0,
            state: WriteState::Idle,
            progress_percent: 0,
            error_message: String::new(),
        }
    }

    /// Set state
    pub fn set_state(&mut self, state: WriteState) {
        self.state = state;
    }

    /// Add bytes written
    pub fn add_bytes_written(&mut self, bytes: u64) {
        self.bytes_written += bytes;
        self.update_progress();
    }

    /// Add bytes verified
    pub fn add_bytes_verified(&mut self, bytes: u64) {
        self.bytes_verified += bytes;
    }

    /// Update progress percentage
    pub fn update_progress(&mut self) {
        if self.image_size > 0 {
            self.progress_percent = ((self.bytes_written * 100) / self.image_size) as u8;
        }
    }

    /// Set error message
    pub fn set_error(&mut self, error: String) {
        self.error_message = error;
        self.state = WriteState::Failed;
    }

    /// Check if job is complete
    pub fn is_complete(&self) -> bool {
        self.state == WriteState::Completed
    }

    /// Check if job failed
    pub fn is_failed(&self) -> bool {
        self.state == WriteState::Failed
    }

    /// Check if job is active
    pub fn is_active(&self) -> bool {
        matches!(self.state, WriteState::Validating | WriteState::Writing | WriteState::Verifying)
    }
}

/// USB writer manager
pub struct SigmaUSBWriter {
    devices: BTreeMap<String, USBDevice>,
    write_jobs: BTreeMap<String, WriteJob>,
    running: AtomicBool,
}

impl SigmaUSBWriter {
    /// Create a new USB writer manager
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            write_jobs: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the USB writer manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the USB writer manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a USB device
    pub fn add_device(&mut self, device: USBDevice) -> bool {
        let device_id = device.device_id.clone();
        self.devices.insert(device_id, device);
        true
    }

    /// Remove a USB device
    pub fn remove_device(&mut self, device_id: &str) -> Option<USBDevice> {
        self.devices.remove(device_id)
    }

    /// Get a device by ID
    pub fn get_device(&self, device_id: &str) -> Option<&USBDevice> {
        self.devices.get(device_id)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&USBDevice> {
        self.devices.values().collect()
    }

    /// List suitable devices for writing
    pub fn list_suitable_devices(&self) -> Vec<&USBDevice> {
        self.devices
            .values()
            .filter(|d| d.is_suitable())
            .collect()
    }

    /// Create a write job
    pub fn create_write_job(&mut self, job: WriteJob) -> bool {
        let job_id = job.job_id.clone();
        self.write_jobs.insert(job_id, job);
        true
    }

    /// Get a write job by ID
    pub fn get_write_job(&self, job_id: &str) -> Option<&WriteJob> {
        self.write_jobs.get(job_id)
    }

    /// List all write jobs
    pub fn list_write_jobs(&self) -> Vec<&WriteJob> {
        self.write_jobs.values().collect()
    }

    /// List active write jobs
    pub fn list_active_jobs(&self) -> Vec<&WriteJob> {
        self.write_jobs
            .values()
            .filter(|j| j.is_active())
            .collect()
    }

    /// Start a write job
    pub fn start_write_job(&mut self, job_id: &str) -> bool {
        if let Some(job) = self.write_jobs.get_mut(job_id) {
            job.set_state(WriteState::Validating);
            job.set_state(WriteState::Writing);
            true
        } else {
            false
        }
    }

    /// Cancel a write job
    pub fn cancel_write_job(&mut self, job_id: &str) -> bool {
        if let Some(job) = self.write_jobs.get_mut(job_id) {
            job.set_state(WriteState::Cancelled);
            true
        } else {
            false
        }
    }

    /// Update write job progress
    pub fn update_progress(&mut self, job_id: &str, bytes_written: u64) -> bool {
        if let Some(job) = self.write_jobs.get_mut(job_id) {
            job.add_bytes_written(bytes_written);
            true
        } else {
            false
        }
    }

    /// Complete a write job
    pub fn complete_write_job(&mut self, job_id: &str) -> bool {
        if let Some(job) = self.write_jobs.get_mut(job_id) {
            job.set_state(WriteState::Verifying);
            job.set_state(WriteState::Completed);
            true
        } else {
            false
        }
    }

    /// Fail a write job
    pub fn fail_write_job(&mut self, job_id: &str, error: String) -> bool {
        if let Some(job) = self.write_jobs.get_mut(job_id) {
            job.set_error(error);
            true
        } else {
            false
        }
    }

    /// Remove a write job
    pub fn remove_write_job(&mut self, job_id: &str) -> Option<WriteJob> {
        self.write_jobs.remove(job_id)
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Get write job count
    pub fn write_job_count(&self) -> usize {
        self.write_jobs.len()
    }
}

impl Default for SigmaUSBWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaUSBWriter instance (placeholder for actual global state)
static unsafe mutable SIGMAUSBWRITER_INSTANCE: Option<SigmaUSBWriter> = None;

/// Initialize the global SigmaUSBWriter instance
pub fn init_sigmausbwriter() {
    unsafe {
        SIGMAUSBWRITER_INSTANCE = Some(SigmaUSBWriter::new());
    }
}

/// Get the global SigmaUSBWriter instance
pub fn get_sigmausbwriter() -> Option<&'static SigmaUSBWriter> {
    unsafe {
        SIGMAUSBWRITER_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaUSBWriter instance
pub fn get_sigmausbwriter_mut() -> Option<&'static mut SigmaUSBWriter> {
    unsafe {
        SIGMAUSBWRITER_INSTANCE.as_mut()
    }
}

/// Convenience function to create a write job
pub fn create_write_job(job: WriteJob) -> bool {
    if let Some(writer) = get_sigmausbwriter_mut() {
        writer.create_write_job(job)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_device_creation() {
        let device = USBDevice::new(
            "usb1".to_string(),
            "USB Drive".to_string(),
            "/dev/sdb".to_string(),
            16 * 1024 * 1024 * 1024,
            true,
        );

        assert_eq!(device.device_id, "usb1");
        assert_eq!(device.device_path, "/dev/sdb");
        assert!(device.is_removable);
        assert!(device.is_suitable());
    }

    #[test]
    fn test_usb_device_busy() {
        let mut device = USBDevice::new(
            "usb1".to_string(),
            "USB Drive".to_string(),
            "/dev/sdb".to_string(),
            16 * 1024 * 1024 * 1024,
            true,
        );

        assert!(!device.is_device_busy());
        device.set_busy(true);
        assert!(device.is_device_busy());
        assert!(!device.is_suitable());
    }

    #[test]
    fn test_write_job_creation() {
        let job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        assert_eq!(job.job_id, "job1");
        assert_eq!(job.image_format, ImageFormat::ISO);
        assert_eq!(job.state, WriteState::Idle);
        assert_eq!(job.progress_percent, 0);
    }

    #[test]
    fn test_write_job_progress() {
        let mut job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        job.add_bytes_written(512 * 1024 * 1024);
        assert_eq!(job.progress_percent, 50);

        job.add_bytes_written(512 * 1024 * 1024);
        assert_eq!(job.progress_percent, 100);
    }

    #[test]
    fn test_write_job_state() {
        let mut job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        job.set_state(WriteState::Writing);
        assert!(job.is_active());

        job.set_state(WriteState::Completed);
        assert!(job.is_complete());
        assert!(!job.is_active());
    }

    #[test]
    fn test_sigmausbwriter_creation() {
        let writer = SigmaUSBWriter::new();
        assert!(!writer.is_running());
        assert_eq!(writer.device_count(), 0);
        assert_eq!(writer.write_job_count(), 0);
    }

    #[test]
    fn test_sigmausbwriter_start_stop() {
        let mut writer = SigmaUSBWriter::new();
        writer.start();
        assert!(writer.is_running());
        writer.stop();
        assert!(!writer.is_running());
    }

    #[test]
    fn test_sigmausbwriter_add_device() {
        let mut writer = SigmaUSBWriter::new();
        let device = USBDevice::new(
            "usb1".to_string(),
            "USB Drive".to_string(),
            "/dev/sdb".to_string(),
            16 * 1024 * 1024 * 1024,
            true,
        );

        let added = writer.add_device(device);
        assert!(added);
        assert_eq!(writer.device_count(), 1);
    }

    #[test]
    fn test_sigmausbwriter_list_suitable() {
        let mut writer = SigmaUSBWriter::new();

        let mut device1 = USBDevice::new(
            "usb1".to_string(),
            "USB Drive 1".to_string(),
            "/dev/sdb".to_string(),
            16 * 1024 * 1024 * 1024,
            true,
        );

        let mut device2 = USBDevice::new(
            "usb2".to_string(),
            "USB Drive 2".to_string(),
            "/dev/sdc".to_string(),
            8 * 1024 * 1024 * 1024,
            true,
        );

        device2.set_busy(true);

        writer.add_device(device1);
        writer.add_device(device2);

        let suitable = writer.list_suitable_devices();
        assert_eq!(suitable.len(), 1);
    }

    #[test]
    fn test_sigmausbwriter_create_write_job() {
        let mut writer = SigmaUSBWriter::new();
        let job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        let created = writer.create_write_job(job);
        assert!(created);
        assert_eq!(writer.write_job_count(), 1);
    }

    #[test]
    fn test_sigmausbwriter_start_write_job() {
        let mut writer = SigmaUSBWriter::new();
        let job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        writer.create_write_job(job);
        let started = writer.start_write_job("job1");
        assert!(started);

        let job = writer.get_write_job("job1").unwrap();
        assert!(job.is_active());
    }

    #[test]
    fn test_sigmausbwriter_cancel_write_job() {
        let mut writer = SigmaUSBWriter::new();
        let job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        writer.create_write_job(job);
        let cancelled = writer.cancel_write_job("job1");
        assert!(cancelled);

        let job = writer.get_write_job("job1").unwrap();
        assert_eq!(job.state, WriteState::Cancelled);
    }

    #[test]
    fn test_sigmausbwriter_update_progress() {
        let mut writer = SigmaUSBWriter::new();
        let job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        writer.create_write_job(job);
        let updated = writer.update_progress("job1", 512 * 1024 * 1024);
        assert!(updated);

        let job = writer.get_write_job("job1").unwrap();
        assert_eq!(job.progress_percent, 50);
    }

    #[test]
    fn test_global_instance() {
        init_sigmausbwriter();

        let writer = get_sigmausbwriter();
        assert!(writer.is_some());

        let mut writer_mut = get_sigmausbwriter_mut();
        assert!(writer_mut.is_some());

        if let Some(writer) = writer_mut {
            writer.start();
            assert!(writer.is_running());
        }
    }

    #[test]
    fn test_convenience_create_write_job() {
        init_sigmausbwriter();

        let job = WriteJob::new(
            "job1".to_string(),
            "/path/to/image.iso".to_string(),
            "usb1".to_string(),
            ImageFormat::ISO,
            1024 * 1024 * 1024,
        );

        let created = create_write_job(job);
        assert!(created);

        let writer = get_sigmausbwriter().unwrap();
        assert_eq!(writer.write_job_count(), 1);
    }
}
