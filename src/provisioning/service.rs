#![no_std]
#![no_main]

/// OOP-based Device Provisioning Service for SigmaOS
/// Implements device provisioning using OOP principles with traits and structs
/// No dependency on external provisioning frameworks
/// Based on Roadmap Item 15: Device provisioning service

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Device ID
pub type DeviceID = usize;

/// Device state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceState {
    Unprovisioned = 0,
    Provisioning = 1,
    Provisioned = 2,
    Active = 3,
    Deactivated = 4,
}

/// Device trait (OOP interface)
pub trait Device {
    /// Get device ID
    fn id(&self) -> DeviceID;
    /// Get device name
    fn name(&self) -> &[u8];
    /// Get device serial
    fn serial(&self) -> &[u8];
    /// Provision device
    fn provision(&mut self) -> Result<(), ProvisioningError>;
    /// Deactivate device
    fn deactivate(&mut self) -> Result<(), ProvisioningError>;
    /// Get device state
    fn state(&self) -> DeviceState;
    /// Get device info
    fn info(&self) -> DeviceInfo;
}

/// Provisioning error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProvisioningError {
    Success = 0,
    AlreadyProvisioned = 1,
    ProvisioningFailed = 2,
    PermissionDenied = 3,
    InvalidSerial = 4,
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

/// Simple device (OOP: Concrete device class)
#[repr(C)]
pub struct SimpleDevice {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub serial: [u8; 64],
    pub state: AtomicUsize, // DeviceState as usize
    pub capability: DeviceCapability,
    pub configuration: [u8; 512],
}

impl SimpleDevice {
    pub fn new(id: DeviceID, name: &[u8], serial: &[u8], capability: DeviceCapability) -> Self {
        let mut name_array = [0u8; 64];
        let mut serial_array = [0u8; 64];

        let name_len = name.len().min(63);
        let serial_len = serial.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(serial.as_ptr(), serial_array.as_mut_ptr(), serial_len);
        }

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
        unsafe {
            core::ptr::copy_nonoverlapping(config.as_ptr(), self.configuration.as_mut_ptr(), len);
        }
    }

    pub fn get_state(&self) -> DeviceState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
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

        // In a real implementation, this would apply configuration
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

/// Provisioning service trait (OOP interface)
pub trait ProvisioningService {
    /// Register device
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, ProvisioningError>;
    /// Unregister device
    fn unregister_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError>;
    /// Provision device
    fn provision_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError>;
    /// Deactivate device
    fn deactivate_device(&mut self, id: DeviceID) -> Result<(), ProvisioningError>;
    /// Get device
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
    /// List devices by state
    fn list_devices(&self, state: DeviceState) -> Vec<DeviceID>;
    /// Get service statistics
    fn stats(&self) -> ProvisioningStats;
}

/// Provisioning statistics
#[repr(C)]
pub struct ProvisioningStats {
    pub total_devices: usize,
    pub provisioned_devices: usize,
    pub active_devices: usize,
    pub deactivated_devices: usize,
}

impl ProvisioningStats {
    pub fn new() -> Self {
        ProvisioningStats {
            total_devices: 0,
            provisioned_devices: 0,
            active_devices: 0,
            deactivated_devices: 0,
        }
    }
}

/// Simple provisioning service (OOP: Concrete service class)
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

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

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
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

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

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
