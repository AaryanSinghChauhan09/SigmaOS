//! SigmaOS Device Driver Abstraction Layer
//! Universal driver framework with OOP principles using Rust traits
//! Ensures old drivers remain functional as kernel evolves

#![no_std]

use crate::drivers::common_types::{SigmaU8, SigmaU16, SigmaU32, SigmaU64, SigmaI32, SigmaI64, SigmaBool, SigmaUsize};

/// Driver lifecycle status
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DriverStatus {
    Uninitialized = 0,
    Initializing = 1,
    Active = 2,
    Suspended = 3,
    Failed = 4,
    Deprecated = 5,
}

/// Driver capability flags
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DriverCapability {
    None = 0,
    Read = 1,
    Write = 2,
    Ioctl = 4,
    Mmap = 8,
    Dma = 16,
    Interrupt = 32,
    Hotplug = 64,
    PowerManagement = 128,
}

/// Driver information
#[repr(C)]
pub struct DriverInfo {
    pub name: [SigmaU8; 64],
    pub version: [SigmaU8; 32],
    pub vendor: [SigmaU8; 64],
    pub description: [SigmaU8; 256],
    pub capabilities: SigmaU64,
    pub status: DriverStatus,
}

/// Driver statistics
#[repr(C)]
pub struct DriverStats {
    pub read_count: SigmaU64,
    pub write_count: SigmaU64,
    pub error_count: SigmaU64,
    pub interrupt_count: SigmaU64,
    pub dma_transfers: SigmaU64,
    pub uptime_seconds: SigmaU64,
}

/// Universal Device Driver trait
/// All SigmaOS drivers must implement this trait for consistent interface
pub trait DeviceDriver {
    /// Initialize the driver
    fn init(&mut self) -> SigmaI32;
    
    /// Shutdown the driver
    fn shutdown(&mut self) -> SigmaI32;
    
    /// Read from device
    fn read(&mut self, buffer: *mut SigmaU8, size: SigmaUsize) -> SigmaI32;
    
    /// Write to device
    fn write(&mut self, buffer: *const SigmaU8, size: SigmaUsize) -> SigmaI32;
    
    /// IOCTL operation
    fn ioctl(&mut self, request: SigmaU32, arg: SigmaU64) -> SigmaI32;
    
    /// Get driver information
    fn get_info(&self) -> DriverInfo;
    
    /// Get driver statistics
    fn get_stats(&self) -> DriverStats;
    
    /// Reset the driver
    fn reset(&mut self) -> SigmaI32;
    
    /// Suspend the driver
    fn suspend(&mut self) -> SigmaI32;
    
    /// Resume the driver
    fn resume(&mut self) -> SigmaI32;
    
    /// Check if driver has specific capability
    fn has_capability(&self, cap: DriverCapability) -> SigmaBool;
    
    /// Get current status
    fn get_status(&self) -> DriverStatus;
}

/// Base driver implementation with common functionality
pub struct BaseDriver {
    pub info: DriverInfo,
    pub stats: DriverStats,
    pub status: DriverStatus,
}

impl BaseDriver {
    pub const fn new(name: &str, version: &str, vendor: &str) -> Self {
        // Convert strings to byte arrays at compile time
        let name_bytes = Self::str_to_bytes(name);
        let version_bytes = Self::str_to_bytes(version);
        let vendor_bytes = Self::str_to_bytes(vendor);
        
        Self {
            info: DriverInfo {
                name: name_bytes,
                version: version_bytes,
                vendor: vendor_bytes,
                description: [0; 256],
                capabilities: 0,
                status: DriverStatus::Uninitialized,
            },
            stats: DriverStats {
                read_count: 0,
                write_count: 0,
                error_count: 0,
                interrupt_count: 0,
                dma_transfers: 0,
                uptime_seconds: 0,
            },
            status: DriverStatus::Uninitialized,
        }
    }
    
    const fn str_to_bytes(s: &str) -> [SigmaU8; 64] {
        let mut bytes = [0u8; 64];
        let s_bytes = s.as_bytes();
        let mut i = 0;
        while i < s_bytes.len() && i < 63 {
            bytes[i] = s_bytes[i];
            i += 1;
        }
        bytes
    }
    
    pub fn set_status(&mut self, status: DriverStatus) {
        self.status = status;
        self.info.status = status;
    }
    
    pub fn add_capability(&mut self, cap: DriverCapability) {
        self.info.capabilities |= cap as SigmaU64;
    }
    
    pub fn increment_read(&mut self) {
        self.stats.read_count += 1;
    }
    
    pub fn increment_write(&mut self) {
        self.stats.write_count += 1;
    }
    
    pub fn increment_error(&mut self) {
        self.stats.error_count += 1;
    }
    
    pub fn increment_interrupt(&mut self) {
        self.stats.interrupt_count += 1;
    }
    
    pub fn increment_dma(&mut self) {
        self.stats.dma_transfers += 1;
    }
}

impl DeviceDriver for BaseDriver {
    fn init(&mut self) -> SigmaI32 {
        self.set_status(DriverStatus::Initializing);
        // Subclasses should override this
        self.set_status(DriverStatus::Active);
        0
    }
    
    fn shutdown(&mut self) -> SigmaI32 {
        self.set_status(DriverStatus::Uninitialized);
        0
    }
    
    fn read(&mut self, _buffer: *mut SigmaU8, _size: SigmaUsize) -> SigmaI32 {
        self.increment_read();
        0
    }
    
    fn write(&mut self, _buffer: *const SigmaU8, _size: SigmaUsize) -> SigmaI32 {
        self.increment_write();
        0
    }
    
    fn ioctl(&mut self, _request: SigmaU32, _arg: SigmaU64) -> SigmaI32 {
        0
    }
    
    fn get_info(&self) -> DriverInfo {
        self.info
    }
    
    fn get_stats(&self) -> DriverStats {
        self.stats
    }
    
    fn reset(&mut self) -> SigmaI32 {
        self.stats = DriverStats {
            read_count: 0,
            write_count: 0,
            error_count: 0,
            interrupt_count: 0,
            dma_transfers: 0,
            uptime_seconds: 0,
        };
        0
    }
    
    fn suspend(&mut self) -> SigmaI32 {
        self.set_status(DriverStatus::Suspended);
        0
    }
    
    fn resume(&mut self) -> SigmaI32 {
        self.set_status(DriverStatus::Active);
        0
    }
    
    fn has_capability(&self, cap: DriverCapability) -> SigmaBool {
        (self.info.capabilities & (cap as SigmaU64)) != 0
    }
    
    fn get_status(&self) -> DriverStatus {
        self.status
    }
}

/// Driver registry for managing all loaded drivers
pub struct DriverRegistry {
    pub drivers: [*mut dyn DeviceDriver; 256],
    pub driver_count: SigmaU32,
}

impl DriverRegistry {
    pub const fn new() -> Self {
        Self {
            drivers: [core::ptr::null_mut(); 256],
            driver_count: 0,
        }
    }
    
    pub fn register(&mut self, driver: *mut dyn DeviceDriver) -> SigmaI32 {
        if self.driver_count >= 256 {
            return -1; // Registry full
        }
        
        self.drivers[self.driver_count as usize] = driver;
        self.driver_count += 1;
        0
    }
    
    pub fn unregister(&mut self, index: SigmaU32) -> SigmaI32 {
        if index >= self.driver_count {
            return -1;
        }
        
        self.drivers[index as usize] = core::ptr::null_mut();
        0
    }
    
    pub fn get_driver(&self, index: SigmaU32) -> Option<*mut dyn DeviceDriver> {
        if index < self.driver_count {
            Some(self.drivers[index as usize])
        } else {
            None
        }
    }
    
    pub fn get_driver_count(&self) -> SigmaU32 {
        self.driver_count
    }
    
    pub fn initialize_all(&mut self) -> SigmaI32 {
        let mut failures = 0;
        for i in 0..self.driver_count as usize {
            unsafe {
                if let Some(driver) = self.drivers[i].as_mut() {
                    if driver.init() != 0 {
                        failures += 1;
                    }
                }
            }
        }
        if failures > 0 {
            -1
        } else {
            0
        }
    }
    
    pub fn shutdown_all(&mut self) -> SigmaI32 {
        let mut failures = 0;
        for i in 0..self.driver_count as usize {
            unsafe {
                if let Some(driver) = self.drivers[i].as_mut() {
                    if driver.shutdown() != 0 {
                        failures += 1;
                    }
                }
            }
        }
        if failures > 0 {
            -1
        } else {
            0
        }
    }
}

/// Global driver registry
static mut DRIVER_REGISTRY: Option<DriverRegistry> = None;

/// Initialize global driver registry
#[no_mangle]
pub unsafe extern "C" fn driver_registry_init() -> SigmaI32 {
    DRIVER_REGISTRY = Some(DriverRegistry::new());
    0
}

/// Get global driver registry
#[no_mangle]
pub unsafe extern "C" fn driver_registry_get() -> *mut DriverRegistry {
    match &mut DRIVER_REGISTRY {
        Some(registry) => registry as *mut DriverRegistry,
        None => core::ptr::null_mut(),
    }
}

/// Helper function to check driver capability
pub fn check_capability(driver: &dyn DeviceDriver, cap: DriverCapability) -> SigmaBool {
    driver.has_capability(cap)
}

/// Helper function to get driver status
pub fn get_driver_status(driver: &dyn DeviceDriver) -> DriverStatus {
    driver.get_status()
}
