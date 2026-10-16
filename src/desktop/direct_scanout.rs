// SPDX-License-Identifier: MIT
// SigmaOS - Direct Scanout Bypass
// Inspired by Linux direct scanout for gaming (DRM/KMS)
// Provides direct scanout bypass for zero-latency fullscreen rendering

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use core::sync::atomic::{AtomicBool, Ordering};

/// Scanout mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanoutMode {
    Composited, // Normal composited rendering
    Direct,     // Direct scanout (compositor bypass)
    Overlay,    // Hardware overlay
}

/// Direct scanout config
#[derive(Debug)]
pub struct DirectScanoutConfig {
    pub enabled: AtomicBool,
    pub require_fullscreen: bool,
    pub require_exclusive: bool,
    pub allow_tearing: bool,
    pub allow_vrr: bool,
}

impl Default for DirectScanoutConfig {
    fn default() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            require_fullscreen: true,
            require_exclusive: true,
            allow_tearing: true,
            allow_vrr: true,
        }
    }
}

/// Direct scanout manager
pub struct DirectScanoutManager {
    pub config: DirectScanoutConfig,
    pub current_mode: ScanoutMode,
    pub active_window: Option<u32>, // Window ID using direct scanout
    pub surface_crtc: Option<u32>,  // CRTC for direct scanout
}

impl DirectScanoutManager {
    pub fn new() -> Self {
        Self {
            config: DirectScanoutConfig::default(),
            current_mode: ScanoutMode::Composited,
            active_window: None,
            surface_crtc: None,
        }
    }

    /// Initialize direct scanout manager
    pub fn init(&mut self) {
        // In real implementation, would initialize DRM/KMS
    }

    /// Enable direct scanout
    pub fn enable(&self) {
        self.config.enabled.store(true, Ordering::Relaxed);
    }

    /// Disable direct scanout
    pub fn disable(&self) {
        self.config.enabled.store(false, Ordering::Relaxed);
    }

    /// Is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled.load(Ordering::Relaxed)
    }

    /// Request direct scanout for window
    pub fn request_direct_scanout(
        &mut self,
        window_id: u32,
        fullscreen: bool,
        exclusive: bool,
    ) -> Result<(), &'static str> {
        if !self.is_enabled() {
            return Err("Direct scanout disabled");
        }

        if self.config.require_fullscreen && !fullscreen {
            return Err("Fullscreen required");
        }

        if self.config.require_exclusive && !exclusive {
            return Err("Exclusive access required");
        }

        // In real implementation, would:
        // 1. Check if window is suitable for direct scanout
        // 2. Allocate CRTC
        // 3. Configure plane for direct scanout
        // 4. Flip to direct scanout mode

        self.current_mode = ScanoutMode::Direct;
        self.active_window = Some(window_id);

        Ok(())
    }

    /// Release direct scanout
    pub fn release_direct_scanout(&mut self) -> Result<(), &'static str> {
        if self.current_mode != ScanoutMode::Direct {
            return Err("Not in direct scanout mode");
        }

        // In real implementation, would:
        // 1. Flip back to composited mode
        // 2. Release CRTC
        // 3. Restore compositor

        self.current_mode = ScanoutMode::Composited;
        self.active_window = None;
        self.surface_crtc = None;

        Ok(())
    }

    /// Get current mode
    pub fn get_current_mode(&self) -> ScanoutMode {
        self.current_mode
    }

    /// Is in direct scanout mode
    pub fn is_direct_scanout(&self) -> bool {
        self.current_mode == ScanoutMode::Direct
    }

    /// Get active window
    pub fn get_active_window(&self) -> Option<u32> {
        self.active_window
    }

    /// Set config
    pub fn set_config(&mut self, config: DirectScanoutConfig) {
        self.config = config;
    }
}

impl Default for DirectScanoutManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global direct scanout manager
static mut DIRECT_SCANOUT_MANAGER: Option<DirectScanoutManager> = None;

/// Initialize direct scanout manager
pub fn init_direct_scanout_manager() {
    unsafe {
        DIRECT_SCANOUT_MANAGER = Some(DirectScanoutManager::new());
    }
}

/// Get global direct scanout manager
pub fn get_direct_scanout_manager() -> Option<&'static mut DirectScanoutManager> {
    unsafe { DIRECT_SCANOUT_MANAGER.as_mut() }
}

/// Enable direct scanout for gaming
pub fn enable_gaming_direct_scanout() {
    if let Some(manager) = get_direct_scanout_manager() {
        manager.enable();
    }
}

/// Disable direct scanout
pub fn disable_direct_scanout() {
    if let Some(manager) = get_direct_scanout_manager() {
        manager.disable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_scanout_manager_init() {
        let manager = DirectScanoutManager::new();
        assert!(!manager.is_direct_scanout());
    }

    #[test]
    fn test_enable_disable() {
        let manager = DirectScanoutManager::new();
        manager.enable();
        assert!(manager.is_enabled());
        manager.disable();
        assert!(!manager.is_enabled());
    }

    #[test]
    fn test_request_direct_scanout() {
        let mut manager = DirectScanoutManager::new();
        manager.enable();
        manager.request_direct_scanout(1234, true, true).unwrap();
        assert!(manager.is_direct_scanout());
        assert_eq!(manager.get_active_window(), Some(1234));
    }

    #[test]
    fn test_request_fullscreen_required() {
        let mut manager = DirectScanoutManager::new();
        manager.enable();
        let result = manager.request_direct_scanout(1234, false, true);
        assert!(result.is_err()); // Not fullscreen
    }

    #[test]
    fn test_release_direct_scanout() {
        let mut manager = DirectScanoutManager::new();
        manager.enable();
        manager.request_direct_scanout(1234, true, true).unwrap();
        manager.release_direct_scanout().unwrap();
        assert!(!manager.is_direct_scanout());
    }

    #[test]
    fn test_enable_gaming_direct_scanout() {
        init_direct_scanout_manager();
        enable_gaming_direct_scanout();
        assert!(get_direct_scanout_manager().unwrap().is_enabled());
    }
}
