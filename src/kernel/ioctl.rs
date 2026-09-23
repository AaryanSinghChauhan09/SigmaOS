// Linux-inspired terminal I/O control (ioctl)
// Provides ioctl for device control

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// IOCTL request
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoctlRequest {
    pub cmd: u32,
    pub arg: u64,
}

impl IoctlRequest {
    pub fn new(cmd: u32, arg: u64) -> Self {
        Self { cmd, arg }
    }
}

/// IOCTL response
#[derive(Debug, Clone)]
pub struct IoctlResponse {
    pub result: i32,
    pub data: Vec<u8>,
}

impl IoctlResponse {
    pub fn new(result: i32, data: Vec<u8>) -> Self {
        Self { result, data }
    }

    pub fn success() -> Self {
        Self::new(0, vec![])
    }

    pub fn error(code: i32) -> Self {
        Self::new(code, vec![])
    }
}

/// Device with ioctl support
#[derive(Debug, Clone)]
pub struct IoctlDevice {
    pub fd: i32,
    pub device_type: String,
}

impl IoctlDevice {
    pub fn new(fd: i32, device_type: String) -> Self {
        Self { fd, device_type }
    }

    /// Handle ioctl request
    pub fn ioctl(&self, request: IoctlRequest) -> IoctlResponse {
        // Simulate ioctl handling
        match request.cmd {
            0x5401 => IoctlResponse::success(), // TCGETS
            0x5402 => IoctlResponse::success(), // TCSETS
            0x5403 => IoctlResponse::success(), // TCSETSW
            0x5404 => IoctlResponse::success(), // TCSETSF
            _ => IoctlResponse::error(-25),     // ENOTTY
        }
    }
}

/// IOCTL manager for system-wide ioctl management
pub struct IoctlManager {
    pub devices: Arc<Mutex<HashMap<i32, IoctlDevice>>>,
    pub next_fd: Arc<Mutex<i32>>,
}

impl IoctlManager {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(Mutex::new(HashMap::new())),
            next_fd: Arc::new(Mutex::new(3)),
        }
    }

    /// Open device
    pub fn open_device(&self, device_type: String) -> i32 {
        let mut next_fd = self.next_fd.lock().unwrap();
        let fd = *next_fd;
        *next_fd += 1;
        drop(next_fd);

        let device = IoctlDevice::new(fd, device_type);
        let mut devices = self.devices.lock().unwrap();
        devices.insert(fd, device);

        fd
    }

    /// Close device
    pub fn close_device(&self, fd: i32) -> Result<(), String> {
        let mut devices = self.devices.lock().unwrap();
        match devices.remove(&fd) {
            Some(_) => Ok(()),
            None => Err(format!("Device fd {} not found", fd)),
        }
    }

    /// Perform ioctl
    pub fn ioctl(&self, fd: i32, request: IoctlRequest) -> Result<IoctlResponse, String> {
        let devices = self.devices.lock().unwrap();
        match devices.get(&fd) {
            Some(device) => Ok(device.ioctl(request)),
            None => Err(format!("Device fd {} not found", fd)),
        }
    }

    /// Get device
    pub fn get_device(&self, fd: i32) -> Option<IoctlDevice> {
        let devices = self.devices.lock().unwrap();
        devices.get(&fd).cloned()
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        let devices = self.devices.lock().unwrap();
        devices.len()
    }
}

impl Default for IoctlManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioctl_request() {
        let request = IoctlRequest::new(0x5401, 0);
        assert_eq!(request.cmd, 0x5401);
        assert_eq!(request.arg, 0);
    }

    #[test]
    fn test_ioctl_response() {
        let response = IoctlResponse::success();
        assert_eq!(response.result, 0);
        assert!(response.data.is_empty());
    }

    #[test]
    fn test_ioctl_response_error() {
        let response = IoctlResponse::error(-25);
        assert_eq!(response.result, -25);
    }

    #[test]
    fn test_ioctl_device() {
        let device = IoctlDevice::new(3, "tty".to_string());
        assert_eq!(device.fd, 3);
        assert_eq!(device.device_type, "tty");
    }

    #[test]
    fn test_ioctl_device_ioctl() {
        let device = IoctlDevice::new(3, "tty".to_string());
        let request = IoctlRequest::new(0x5401, 0);
        let response = device.ioctl(request);

        assert_eq!(response.result, 0);
    }

    #[test]
    fn test_ioctl_device_ioctl_unknown() {
        let device = IoctlDevice::new(3, "tty".to_string());
        let request = IoctlRequest::new(0x9999, 0);
        let response = device.ioctl(request);

        assert_eq!(response.result, -25);
    }

    #[test]
    fn test_ioctl_manager() {
        let manager = IoctlManager::new();

        let fd = manager.open_device("tty".to_string());
        assert_eq!(fd, 3);
        assert_eq!(manager.device_count(), 1);
    }

    #[test]
    fn test_ioctl_manager_ioctl() {
        let manager = IoctlManager::new();

        let fd = manager.open_device("tty".to_string());
        let request = IoctlRequest::new(0x5401, 0);
        let response = manager.ioctl(fd, request).unwrap();

        assert_eq!(response.result, 0);
    }

    #[test]
    fn test_ioctl_manager_close() {
        let manager = IoctlManager::new();

        let fd = manager.open_device("tty".to_string());
        manager.close_device(fd).unwrap();

        assert_eq!(manager.device_count(), 0);
    }

    #[test]
    fn test_ioctl_manager_invalid() {
        let manager = IoctlManager::new();
        assert!(manager.get_device(999).is_none());
        assert!(manager.ioctl(999, IoctlRequest::new(0x5401, 0)).is_err());
    }
}
