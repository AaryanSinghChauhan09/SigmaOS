// SPDX-License-Identifier: MIT
// SigmaOS - Sigma Display Manager
// Provides display output management following COMPOSITION OVER INHERITANCE,
// SEPARATION OF CONCERNS, SINGLE RESPONSIBILITY, and MODULAR DESIGN principles.

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

// ---------------------------------------------------------------------------
// Display primitives (re-exported from display manager 2 for compatibility)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

impl DisplayMode {
    pub const fn new(width: u32, height: u32, refresh_rate: u32) -> Self {
        Self {
            width,
            height,
            refresh_rate,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayState {
    Connected,
    Disconnected,
    Suspended,
}

#[derive(Debug, Clone)]
pub struct DisplayOutput {
    pub name: String,
    pub state: DisplayState,
    pub current_mode: Option<DisplayMode>,
}

impl DisplayOutput {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: DisplayState::Disconnected,
            current_mode: None,
        }
    }
}

// ---------------------------------------------------------------------------
// SigmaDisplayManager — sovereign display output manager
// Follows LEAST PRIVILEGE: only exposes what callers need.
// ---------------------------------------------------------------------------

pub struct SigmaDisplayManager {
    outputs: Vec<DisplayOutput>,
    initialized: AtomicBool,
}

impl SigmaDisplayManager {
    pub const fn new() -> Self {
        Self {
            outputs: Vec::new(),
            initialized: AtomicBool::new(false),
        }
    }

    pub fn init(&self) {
        self.initialized.store(true, Ordering::SeqCst);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }

    pub fn add_output(&mut self, output: DisplayOutput) {
        self.outputs.push(output);
    }

    pub fn outputs(&self) -> &[DisplayOutput] {
        &self.outputs
    }

    pub fn output_count(&self) -> usize {
        self.outputs.len()
    }
}

// ---------------------------------------------------------------------------
// Module-level singleton accessor functions (INVERSION OF CONTROL pattern)
// ---------------------------------------------------------------------------

use core::cell::UnsafeCell;

struct DisplayManagerCell(UnsafeCell<Option<SigmaDisplayManager>>);
unsafe impl Sync for DisplayManagerCell {}

static DISPLAY_MANAGER: DisplayManagerCell = DisplayManagerCell(UnsafeCell::new(None));

/// Initialize the global display manager singleton.
/// # Safety
/// Must be called only once during system bootstrap (single-threaded).
pub fn init_sigmadisplaymanager() {
    unsafe {
        let cell = &mut *DISPLAY_MANAGER.0.get();
        if cell.is_none() {
            let mut mgr = SigmaDisplayManager::new();
            mgr.init();
            *cell = Some(mgr);
        }
    }
}

/// Obtain a shared reference to the display manager.
pub fn get_sigmadisplaymanager() -> Option<&'static SigmaDisplayManager> {
    unsafe { (*DISPLAY_MANAGER.0.get()).as_ref() }
}

/// Obtain a mutable reference to the display manager.
pub fn get_sigmadisplaymanager_mut() -> Option<&'static mut SigmaDisplayManager> {
    unsafe { (*DISPLAY_MANAGER.0.get()).as_mut() }
}

/// Add a display output to the global manager.
pub fn add_output(output: DisplayOutput) {
    if let Some(mgr) = get_sigmadisplaymanager_mut() {
        mgr.add_output(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_display_manager_init() {
        let mgr = SigmaDisplayManager::new();
        assert!(!mgr.is_initialized());
    }

    #[test]
    fn test_add_output() {
        let mut mgr = SigmaDisplayManager::new();
        mgr.init();
        mgr.add_output(DisplayOutput::new("HDMI-1".to_string()));
        assert_eq!(mgr.output_count(), 1);
    }

    #[test]
    fn test_display_mode() {
        let mode = DisplayMode::new(1920, 1080, 60);
        assert_eq!(mode.width, 1920);
        assert_eq!(mode.height, 1080);
        assert_eq!(mode.refresh_rate, 60);
    }
}
