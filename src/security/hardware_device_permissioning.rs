// Sovereign Hardware & Device Permissioning Engine for SigmaOS (`src/security/hardware_device_permissioning.rs`)
// Inspired by libusb, udev, Flatpak device portals, and Android permission models.
// Provides native portal-based mediation for USB, Camera, Microphone, Mounts, and Serial devices.

use std::collections::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Hardware Device Class Category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceCategory {
    UsbDevice,
    Camera,
    Microphone,
    BlockMount,
    SerialPort,
    GpuDevice,
}

/// Device Access Permission State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevicePermissionState {
    Prompt,
    Granted,
    Denied,
    OneTimeGrant,
}

/// Hardware Device Access Request
#[derive(Debug, Clone)]
pub struct DeviceAccessRequest {
    pub request_id: u64,
    pub process_id: u32,
    pub app_id: String,
    pub device_category: DeviceCategory,
    pub sysfs_device_path: String,
    pub rationale: String,
    pub permission_state: DevicePermissionState,
}

/// Per-Process USB & Device Forwarding Mapping (libusb + udev + device-mapper)
#[derive(Debug, Clone)]
pub struct DeviceForwardingMap {
    pub process_id: u32,
    pub host_dev_path: String,
    pub mapped_container_path: String,
    pub is_active: bool,
}

/// Sovereign Hardware & Device Permissioning Engine
pub struct SovereignHardwareDevicePermissioningEngine {
    pub access_requests: HashMap<u64, DeviceAccessRequest>,
    pub active_forwarding_maps: Vec<DeviceForwardingMap>,
    pub next_request_id: u64,
}

impl SovereignHardwareDevicePermissioningEngine {
    pub fn new() -> Self {
        Self {
            access_requests: HashMap::new(),
            active_forwarding_maps: Vec::new(),
            next_request_id: 1000,
        }
    }

    /// Request device access via portal pattern (Flatpak/Android model)
    pub fn request_device_access(
        &mut self,
        pid: u32,
        app_id: &str,
        category: DeviceCategory,
        dev_path: &str,
        rationale: &str,
    ) -> u64 {
        let req_id = self.next_request_id;
        self.next_request_id += 1;

        let request = DeviceAccessRequest {
            request_id: req_id,
            process_id: pid,
            app_id: app_id.to_string(),
            device_category: category,
            sysfs_device_path: dev_path.to_string(),
            rationale: rationale.to_string(),
            permission_state: DevicePermissionState::Prompt,
        };

        self.access_requests.insert(req_id, request);
        req_id
    }

    /// User portal decision (Grant/Deny/OneTime)
    pub fn respond_portal_request(
        &mut self,
        request_id: u64,
        decision: DevicePermissionState,
    ) -> Result<String, &'static str> {
        let req = self
            .access_requests
            .get_mut(&request_id)
            .ok_or("Device request ID not found")?;

        req.permission_state = decision;

        if decision == DevicePermissionState::Granted || decision == DevicePermissionState::OneTimeGrant {
            // Setup libusb / udev device forwarding map
            let mapped_path = format!("/dev/sandbox/{}/{}", req.process_id, req.device_category as u8);
            self.active_forwarding_maps.push(DeviceForwardingMap {
                process_id: req.process_id,
                host_dev_path: req.sysfs_device_path.clone(),
                mapped_container_path: mapped_path.clone(),
                is_active: true,
            });
            Ok(mapped_path)
        } else {
            Err("Device access denied by portal policy")
        }
    }

    /// Check if a process is authorized to access a target device path
    pub fn is_device_access_granted(&self, pid: u32, host_path: &str) -> bool {
        self.active_forwarding_maps
            .iter()
            .any(|m| m.process_id == pid && m.host_dev_path == host_path && m.is_active)
    }

    /// Revoke device forwarding on process exit or user portal revocation
    pub fn revoke_process_forwarding(&mut self, pid: u32) {
        for map in self.active_forwarding_maps.iter_mut() {
            if map.process_id == pid {
                map.is_active = false;
            }
        }
    }
}

impl Default for SovereignHardwareDevicePermissioningEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_permission_portal_lifecycle() {
        let mut engine = SovereignHardwareDevicePermissioningEngine::new();

        let req_id = engine.request_device_access(
            101,
            "org.sigmaos.camera_app",
            DeviceCategory::Camera,
            "/dev/video0",
            "Video recording for call",
        );

        assert_eq!(engine.access_requests.len(), 1);
        assert!(!engine.is_device_access_granted(101, "/dev/video0"));

        let mapped_path = engine
            .respond_portal_request(req_id, DevicePermissionState::Granted)
            .unwrap();

        assert!(mapped_path.contains("/dev/sandbox/101/"));
        assert!(engine.is_device_access_granted(101, "/dev/video0"));

        engine.revoke_process_forwarding(101);
        assert!(!engine.is_device_access_granted(101, "/dev/video0"));
    }
}
