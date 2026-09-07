// SigmaOS Accessibility & Screen Reader Subsystem

pub mod framework;
pub use framework::*;

pub struct AccessibilityEngine;

impl AccessibilityEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AccessibilityEngine {
    fn default() -> Self {
        Self::new()
    }
}
