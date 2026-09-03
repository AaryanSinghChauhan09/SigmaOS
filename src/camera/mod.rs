#![allow(dead_code)]
// SigmaOS Camera Module
// Camera device driver
// Zero-dependency implementation - no external libraries required


extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::boxed::Box;
use core::fmt;

/// Error type for the Camera module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CameraError {
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

impl fmt::Display for CameraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "Camera: operation not supported"),
            Self::InvalidParam => write!(f, "Camera: invalid parameter"),
            Self::NotFound => write!(f, "Camera: resource not found"),
            Self::PermissionDenied => write!(f, "Camera: permission denied"),
            Self::OutOfMemory => write!(f, "Camera: out of memory"),
            Self::IoError => write!(f, "Camera: I/O error"),
            Self::Unknown => write!(f, "Camera: unknown error"),
        }
    }
}

/// Result type alias for Camera operations
pub type CameraResult<T> = Result<T, CameraError>;

/// CameraDriver - primary abstraction for this module
#[derive(Debug, Clone)]
pub struct CameraDriver {
    pub id: u64,
    pub name: String,
    pub enabled: bool,
}

impl CameraDriver {
    /// Create a new CameraDriver with the given name
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.into(),
            enabled: false,
        }
    }
    
    /// Enable this resource
    pub fn enable(&mut self) -> CameraResult<()> {
        self.enabled = true;
        Ok(())
    }
    
    /// Disable this resource
    pub fn disable(&mut self) -> CameraResult<()> {
        self.enabled = false;
        Ok(())
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Manager for Camera resources
#[derive(Debug)]
pub struct CameraFrame {
    resources: Vec<CameraDriver>,
    initialized: bool,
}

impl CameraFrame {
    /// Create a new CameraFrame
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            initialized: false,
        }
    }
    
    /// Initialize the Camera subsystem
    pub fn init(&mut self) -> CameraResult<()> {
        self.initialized = true;
        Ok(())
    }
    
    /// Add a resource
    pub fn add(&mut self, resource: CameraDriver) -> CameraResult<u64> {
        if !self.initialized {
            return Err(CameraError::NotSupported);
        }
        let id = self.resources.len() as u64;
        self.resources.push(resource);
        Ok(id)
    }
    
    /// Get resource by ID
    pub fn get(&self, id: u64) -> Option<&CameraDriver> {
        self.resources.get(id as usize)
    }
    
    /// Get mutable resource by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut CameraDriver> {
        self.resources.get_mut(id as usize)
    }
    
    /// List all resources
    pub fn list(&self) -> &[CameraDriver] {
        &self.resources
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Shutdown the subsystem
    pub fn shutdown(&mut self) -> CameraResult<()> {
        self.initialized = false;
        self.resources.clear();
        Ok(())
    }
}

impl Default for CameraFrame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_camera_manager_init() {
        let mut manager = CameraFrame::new();
        assert!(!manager.is_initialized());
        assert!(manager.init().is_ok());
        assert!(manager.is_initialized());
    }
    
    #[test]
    fn test_camera_resource_add() {
        let mut manager = CameraFrame::new();
        manager.init().unwrap();
        let resource = CameraDriver::new("test");
        let id = manager.add(resource).unwrap();
        assert_eq!(id, 0);
        assert!(manager.get(0).is_some());
    }
}
