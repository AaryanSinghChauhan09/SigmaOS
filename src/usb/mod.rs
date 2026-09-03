#![allow(dead_code)]
// SigmaOS Usb Module
// USB device subsystem
// Zero-dependency implementation - no external libraries required


extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::boxed::Box;
use core::fmt;

/// Error type for the Usb module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UsbError {
    /// Operation not supported
    NotSupported,
    /// Invalid parameter
    InvalidParam,
    /// Resource not found
    NotFound,
    /// Permission denied
    PermissionDenied,
    /// Out of memory
    OutOfMemory,
    /// I/O error
    IoError,
    /// Unknown error
    Unknown,
}

impl fmt::Display for UsbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "Usb: operation not supported"),
            Self::InvalidParam => write!(f, "Usb: invalid parameter"),
            Self::NotFound => write!(f, "Usb: resource not found"),
            Self::PermissionDenied => write!(f, "Usb: permission denied"),
            Self::OutOfMemory => write!(f, "Usb: out of memory"),
            Self::IoError => write!(f, "Usb: I/O error"),
            Self::Unknown => write!(f, "Usb: unknown error"),
        }
    }
}

/// Result type alias for Usb operations
pub type UsbResult<T> = Result<T, UsbError>;

/// UsbDriver - primary abstraction for this module
#[derive(Debug, Clone)]
pub struct UsbDriver {
    pub id: u64,
    pub name: String,
    pub enabled: bool,
}

impl UsbDriver {
    /// Create a new UsbDriver with the given name
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.into(),
            enabled: false,
        }
    }
    
    /// Enable this resource
    pub fn enable(&mut self) -> UsbResult<()> {
        self.enabled = true;
        Ok(())
    }
    
    /// Disable this resource
    pub fn disable(&mut self) -> UsbResult<()> {
        self.enabled = false;
        Ok(())
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Manager for Usb resources
#[derive(Debug)]
pub struct UsbDevice {
    resources: Vec<UsbDriver>,
    initialized: bool,
}

impl UsbDevice {
    /// Create a new UsbDevice
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            initialized: false,
        }
    }
    
    /// Initialize the Usb subsystem
    pub fn init(&mut self) -> UsbResult<()> {
        self.initialized = true;
        Ok(())
    }
    
    /// Add a resource
    pub fn add(&mut self, resource: UsbDriver) -> UsbResult<u64> {
        if !self.initialized {
            return Err(UsbError::NotSupported);
        }
        let id = self.resources.len() as u64;
        self.resources.push(resource);
        Ok(id)
    }
    
    /// Get resource by ID
    pub fn get(&self, id: u64) -> Option<&UsbDriver> {
        self.resources.get(id as usize)
    }
    
    /// Get mutable resource by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut UsbDriver> {
        self.resources.get_mut(id as usize)
    }
    
    /// List all resources
    pub fn list(&self) -> &[UsbDriver] {
        &self.resources
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Shutdown the subsystem
    pub fn shutdown(&mut self) -> UsbResult<()> {
        self.initialized = false;
        self.resources.clear();
        Ok(())
    }
}

impl Default for UsbDevice {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_usb_manager_init() {
        let mut manager = UsbDevice::new();
        assert!(!manager.is_initialized());
        assert!(manager.init().is_ok());
        assert!(manager.is_initialized());
    }
    
    #[test]
    fn test_usb_resource_add() {
        let mut manager = UsbDevice::new();
        manager.init().unwrap();
        let resource = UsbDriver::new("test");
        let id = manager.add(resource).unwrap();
        assert_eq!(id, 0);
        assert!(manager.get(0).is_some());
    }
}
