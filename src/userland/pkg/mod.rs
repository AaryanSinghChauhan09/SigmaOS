#![allow(dead_code)]
// SigmaOS UserPkg Module
// Userland package utilities
// Zero-dependency implementation - no external libraries required


extern crate alloc;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::boxed::Box;
use core::fmt;

/// Error type for the UserPkg module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageError {
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

impl fmt::Display for PackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "UserPkg: operation not supported"),
            Self::InvalidParam => write!(f, "UserPkg: invalid parameter"),
            Self::NotFound => write!(f, "UserPkg: resource not found"),
            Self::PermissionDenied => write!(f, "UserPkg: permission denied"),
            Self::OutOfMemory => write!(f, "UserPkg: out of memory"),
            Self::IoError => write!(f, "UserPkg: I/O error"),
            Self::Unknown => write!(f, "UserPkg: unknown error"),
        }
    }
}

/// Result type alias for UserPkg operations
pub type UserPkgResult<T> = Result<T, PackageError>;

/// UserPackage - primary abstraction for this module
#[derive(Debug, Clone)]
pub struct UserPackage {
    pub id: u64,
    pub name: String,
    pub enabled: bool,
}

impl UserPackage {
    /// Create a new UserPackage with the given name
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.into(),
            enabled: false,
        }
    }
    
    /// Enable this resource
    pub fn enable(&mut self) -> UserPkgResult<()> {
        self.enabled = true;
        Ok(())
    }
    
    /// Disable this resource
    pub fn disable(&mut self) -> UserPkgResult<()> {
        self.enabled = false;
        Ok(())
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Manager for UserPkg resources
#[derive(Debug)]
pub struct PackageInstaller {
    resources: Vec<UserPackage>,
    initialized: bool,
}

impl PackageInstaller {
    /// Create a new PackageInstaller
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            initialized: false,
        }
    }
    
    /// Initialize the UserPkg subsystem
    pub fn init(&mut self) -> UserPkgResult<()> {
        self.initialized = true;
        Ok(())
    }
    
    /// Add a resource
    pub fn add(&mut self, resource: UserPackage) -> UserPkgResult<u64> {
        if !self.initialized {
            return Err(PackageError::NotSupported);
        }
        let id = self.resources.len() as u64;
        self.resources.push(resource);
        Ok(id)
    }
    
    /// Get resource by ID
    pub fn get(&self, id: u64) -> Option<&UserPackage> {
        self.resources.get(id as usize)
    }
    
    /// Get mutable resource by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut UserPackage> {
        self.resources.get_mut(id as usize)
    }
    
    /// List all resources
    pub fn list(&self) -> &[UserPackage] {
        &self.resources
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Shutdown the subsystem
    pub fn shutdown(&mut self) -> UserPkgResult<()> {
        self.initialized = false;
        self.resources.clear();
        Ok(())
    }
}

impl Default for PackageInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_userpkg_manager_init() {
        let mut manager = PackageInstaller::new();
        assert!(!manager.is_initialized());
        assert!(manager.init().is_ok());
        assert!(manager.is_initialized());
    }
    
    #[test]
    fn test_userpkg_resource_add() {
        let mut manager = PackageInstaller::new();
        manager.init().unwrap();
        let resource = UserPackage::new("test");
        let id = manager.add(resource).unwrap();
        assert_eq!(id, 0);
        assert!(manager.get(0).is_some());
    }
}
