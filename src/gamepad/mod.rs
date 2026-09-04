#![allow(dead_code)]
// SigmaOS Gamepad Module
// Gamepad/controller input subsystem
// Zero-dependency implementation - no external libraries required


use std::vec::Vec;
use std::string::{String, ToString};
use std::boxed::Box;
use core::fmt;

/// Error type for the Gamepad module
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamepadError {
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

impl fmt::Display for GamepadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotSupported => write!(f, "Gamepad: operation not supported"),
            Self::InvalidParam => write!(f, "Gamepad: invalid parameter"),
            Self::NotFound => write!(f, "Gamepad: resource not found"),
            Self::PermissionDenied => write!(f, "Gamepad: permission denied"),
            Self::OutOfMemory => write!(f, "Gamepad: out of memory"),
            Self::IoError => write!(f, "Gamepad: I/O error"),
            Self::Unknown => write!(f, "Gamepad: unknown error"),
        }
    }
}

/// Result type alias for Gamepad operations
pub type GamepadResult<T> = Result<T, GamepadError>;

/// GamepadEvent - primary abstraction for this module
#[derive(Debug, Clone)]
pub struct GamepadEvent {
    pub id: u64,
    pub name: String,
    pub enabled: bool,
}

impl GamepadEvent {
    /// Create a new GamepadEvent with the given name
    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            name: name.into(),
            enabled: false,
        }
    }
    
    /// Enable this resource
    pub fn enable(&mut self) -> GamepadResult<()> {
        self.enabled = true;
        Ok(())
    }
    
    /// Disable this resource
    pub fn disable(&mut self) -> GamepadResult<()> {
        self.enabled = false;
        Ok(())
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Manager for Gamepad resources
#[derive(Debug)]
pub struct GamepadDriver {
    resources: Vec<GamepadEvent>,
    initialized: bool,
}

impl GamepadDriver {
    /// Create a new GamepadDriver
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            initialized: false,
        }
    }
    
    /// Initialize the Gamepad subsystem
    pub fn init(&mut self) -> GamepadResult<()> {
        self.initialized = true;
        Ok(())
    }
    
    /// Add a resource
    pub fn add(&mut self, resource: GamepadEvent) -> GamepadResult<u64> {
        if !self.initialized {
            return Err(GamepadError::NotSupported);
        }
        let id = self.resources.len() as u64;
        self.resources.push(resource);
        Ok(id)
    }
    
    /// Get resource by ID
    pub fn get(&self, id: u64) -> Option<&GamepadEvent> {
        self.resources.get(id as usize)
    }
    
    /// Get mutable resource by ID
    pub fn get_mut(&mut self, id: u64) -> Option<&mut GamepadEvent> {
        self.resources.get_mut(id as usize)
    }
    
    /// List all resources
    pub fn list(&self) -> &[GamepadEvent] {
        &self.resources
    }
    
    /// Check if initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    /// Shutdown the subsystem
    pub fn shutdown(&mut self) -> GamepadResult<()> {
        self.initialized = false;
        self.resources.clear();
        Ok(())
    }
}

impl Default for GamepadDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gamepad_manager_init() {
        let mut manager = GamepadDriver::new();
        assert!(!manager.is_initialized());
        assert!(manager.init().is_ok());
        assert!(manager.is_initialized());
    }
    
    #[test]
    fn test_gamepad_resource_add() {
        let mut manager = GamepadDriver::new();
        manager.init().unwrap();
        let resource = GamepadEvent::new("test");
        let id = manager.add(resource).unwrap();
        assert_eq!(id, 0);
        assert!(manager.get(0).is_some());
    }
}
