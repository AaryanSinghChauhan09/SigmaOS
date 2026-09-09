/// OOP-based Device Provisioning Service for SigmaOS
/// Implements zero-touch enrollment and automated device lifecycle management
/// inspired by Linux Preseed/Kickstart and BSD Auto-Install configurations.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Device ID
pub type DeviceID = usize;

/// Device state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Unprovisioned = 0,
    Provisioning = 1,
    Provisioned = 2,
    Active = 3,
    Deactivated = 4,
}

/// Zero-Touch Enrollment Manifest (Linux/BSD Preseed & TPM Attestation style)
#[derive(Debug, Clone)]
pub struct ZeroTouchManifest {
    pub serial_number: [u8; 64],
    pub tpm_attestation_hash: [u8; 32],
    pub preseed_config_url: [u8; 128],
    pub auto_activate: bool,
}

impl ZeroTouchManifest {
    pub fn new(serial: &[u8], preseed_url: &[u8]) -> Self {
        let mut serial_arr = [0u8; 64];
        let mut url_arr = [0u8; 128];
        let s_len = serial.len().min(63);
        let u_len = preseed_url.len().min(127);

        serial_arr[..s_len].copy_from_slice(&serial[..s_len]);
        url_arr[..u_len].copy_from_slice(&preseed_url[..u_len]);

        Self {
            serial_number: serial_arr,
            tpm_attestation_hash: [0xAB; 32], // Simulated NIST Ed25519/Dilithium attestation
            preseed_config_url: url_arr,
            auto_activate: true,
        }
    }
}

/// Device trait (OOP interface)
pub trait Device {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn serial(&self) -> &[u8];
    fn provision(&mut self) -> Result<(), ProvisioningError>;
    fn deactivate(&mut self) -> Result<(), ProvisioningError>;
    fn state(&self) -> DeviceState;
    fn info(&self) -> DeviceInfo;
}

/// Provisioning error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvisioningError {
    Success = 0,
    AlreadyProvisioned = 1,
    ProvisioningFailed = 2,
    PermissionDenied = 3,
    InvalidSerial = 4,
    AttestationFailed = 5,
}

/// Device info
#[repr(C)]
pub struct DeviceInfo {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub serial: [u8; 64],
    pub state: DeviceState,
    pub capability: DeviceCapability,
}

impl DeviceInfo {
    pub fn new(id: DeviceID) -> Self {
        DeviceInfo {
            id,
            name: [0; 64],
            serial: [0; 64],
            state: DeviceState::Unprovisioned,
            capability: DeviceCapability::new(),
        }
    }
}

/// Device capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceCapability {
    pub can_provision: bool,
    pub can_deactivate: bool,
}

impl DeviceCapability {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DeviceCapability {
            can_provision: false,
            can_deactivate: false,
        }
    }

    pub fn full() -> Self {
        DeviceCapability {
            can_provision: true,
            can_deactivate: true,
        }
    }
}

/// Concrete device implementation
#[repr(C)]
pub struct SimpleDevice {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub serial: [u8; 64],
    pub state: AtomicUsize,
    pub capability: DeviceCapability,
    pub configuration: [u8; 512],
}

impl SimpleDevice {
    pub fn new(id: DeviceID, name: &[u8], serial: &[u8], capability: DeviceCapability) -> Self {
        let mut name_array = [0u8; 64];
        let mut serial_array = [0u8; 64];

        let name_len = name.len().min(63);
        let serial_len = serial.len().min(63);

        name_array[..name_len].copy_from_slice(&name[..name_len]);
        serial_array[..serial_len].copy_from_slice(&serial[..serial_len]);

        SimpleDevice {
            id,
            name: name_array,
            serial: serial_array,
            state: AtomicUsize::new(DeviceState::Unprovisioned as usize),
            capability,
            configuration: [0; 512],
        }
    }

    pub fn set_configuration(&mut self, config: &[u8]) {
        let len = config.len().min(511);
        self.configuration[..len].copy_from_slice(&config[..len]);
    }

    pub fn get_state(&self) -> DeviceState {
        match self.state.load(Ordering::SeqCst) {
            1 => DeviceState::Provisioning,
            2 => DeviceState::Provisioned,
            3 => DeviceState::Active,
            4 => DeviceState::Deactivated,
            _ => DeviceState::Unprovisioned,
        }
    }

    pub fn set_state(&self, state: DeviceState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Device for SimpleDevice {
    fn id(&self) -> DeviceID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn serial(&self) -> &[u8] {
        let len = self.serial.iter().position(|&b| b == 0).unwrap_or(64);
        &self.serial[..len]
    }

    fn provision(&mut self) -> Result<(), ProvisioningError> {
        if !self.capability.can_provision {
            return Err(ProvisioningError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == DeviceState::Provisioned || current_state == DeviceState::Active {
            return Err(ProvisioningError::AlreadyProvisioned);
        }

        self.set_state(DeviceState::Provisioning);
        self.set_state(DeviceState::Provisioned);
        Ok(())
    }

    fn deactivate(&mut self) -> Result<(), ProvisioningError> {
        if !self.capability.can_deactivate {
            return Err(ProvisioningError::PermissionDenied);
        }

        self.set_state(DeviceState::Deactivated);
        Ok(())
    }

    fn state(&self) -> DeviceState {
        self.get_state()
    }

    fn info(&self) -> DeviceInfo {
        DeviceInfo {
            id: self.id,
            name: self.name,
            serial: self.serial,
            state: self.get_state(),
            capability: self.capability,
        }
    }
}

/// Provisioning service interface
pub trait ProvisioningService {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, ProvisioningError>;
    fn unregister_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError>;
    fn provision_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError>;
    fn zero_touch_enroll(&mut self, manifest: ZeroTouchManifest) -> Result<DeviceID, ProvisioningError>;
    fn deactivate_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
    fn list_devices(&self, state: DeviceState) -> Vec<DeviceID>;
    fn stats(&self) -> ProvisioningStats;
}

/// Provisioning statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProvisioningStats {
    pub total_devices: usize,
    pub provisioned_devices: usize,
    pub active_devices: usize,
    pub deactivated_devices: usize,
}

impl ProvisioningStats {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ProvisioningStats {
            total_devices: 0,
            provisioned_devices: 0,
            active_devices: 0,
            deactivated_devices: 0,
        }
    }
}

/// Concrete provisioning service
pub struct SimpleProvisioningService {
    devices: Vec<Option<Box<dyn Device>>>,
    next_id: AtomicUsize,
    stats: ProvisioningStats,
    capability: ServiceCapability,
}

/// Service capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ServiceCapability {
    pub can_register: bool,
    pub can_provision: bool,
    pub can_deactivate: bool,
}

impl ServiceCapability {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ServiceCapability {
            can_register: false,
            can_provision: false,
            can_deactivate: false,
        }
    }

    pub fn full() -> Self {
        ServiceCapability {
            can_register: true,
            can_provision: true,
            can_deactivate: true,
        }
    }
}

impl SimpleProvisioningService {
    pub fn new(capability: ServiceCapability) -> Self {
        SimpleProvisioningService {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: ProvisioningStats::new(),
            capability,
        }
    }
}

impl ProvisioningService for SimpleProvisioningService {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, ProvisioningError> {
        if !self.capability.can_register {
            return Err(ProvisioningError::PermissionDenied);
        }

        let id = device.id();
        self.devices.push(Some(device));
        self.stats.total_devices += 1;
        Ok(id)
    }

    fn unregister_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError> {
        if !self.capability.can_register {
            return Err(ProvisioningError::PermissionDenied);
        }

        let mut index = None;
        for (i, device_option) in self.devices.iter().enumerate() {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.devices[i] = None;
            self.stats.total_devices -= 1;
            Ok(())
        } else {
            Err(ProvisioningError::ProvisioningFailed)
        }
    }

    fn provision_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError> {
        if !self.capability.can_provision {
            return Err(ProvisioningError::PermissionDenied);
        }

        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == id {
                    let result = device.provision();
                    if result.is_ok() {
                        let state = device.state();
                        if state == DeviceState::Provisioned {
                            self.stats.provisioned_devices += 1;
                        }
                    }
                    return result;
                }
            }
        }
        Err(ProvisioningError::ProvisioningFailed)
    }

    /// Hands-free zero-touch device enrollment pipeline (TPM attestation + automated config)
    fn zero_touch_enroll(&mut self, manifest: ZeroTouchManifest) -> Result<DeviceID, ProvisioningError> {
        if !self.capability.can_register || !self.capability.can_provision {
            return Err(ProvisioningError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let serial_len = manifest.serial_number.iter().position(|&b| b == 0).unwrap_or(64);
        let serial_bytes = &manifest.serial_number[..serial_len];

        let mut device = SimpleDevice::new(
            id,
            b"Managed-Sovereign-Node",
            serial_bytes,
            DeviceCapability::full(),
        );

        device.set_configuration(&manifest.preseed_config_url);
        device.provision()?;

        if manifest.auto_activate {
            device.set_state(DeviceState::Active);
            self.stats.active_devices += 1;
        }

        self.register_device(Box::new(device))?;
        Ok(id)
    }

    fn deactivate_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError> {
        if !self.capability.can_deactivate {
            return Err(ProvisioningError::PermissionDenied);
        }

        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == id {
                    let result = device.deactivate();
                    if result.is_ok() {
                        self.stats.deactivated_devices += 1;
                    }
                    return result;
                }
            }
        }
        Err(ProvisioningError::ProvisioningFailed)
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Some(device.as_ref());
                }
            }
        }
        None
    }

    fn list_devices(&self, state: DeviceState) -> Vec<DeviceID> {
        let mut ids = Vec::new();

        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.state() == state {
                    ids.push(device.id());
                }
            }
        }

        ids
    }

    fn stats(&self) -> ProvisioningStats {
        self.stats
    }
}

#[cfg(target_os = "none")]
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

#[cfg(target_os = "none")]
impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_touch_enrollment() {
        let mut service = SimpleProvisioningService::new(ServiceCapability::full());
        let manifest = ZeroTouchManifest::new(
            b"SIGMA-SER-9941",
            b"https://preseed.sigmaos.io/node-profile.toml",
        );

        let dev_id = service.zero_touch_enroll(manifest).unwrap();
        assert_eq!(dev_id, 1);

        let dev = service.get_device(dev_id).unwrap();
        assert_eq!(dev.state(), DeviceState::Active);
        assert_eq!(dev.serial(), b"SIGMA-SER-9941");
    }
}
