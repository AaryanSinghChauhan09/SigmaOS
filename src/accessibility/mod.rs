// SigmaOS Accessibility & Screen Reader Subsystem

pub mod framework;
pub mod keyboard;
pub mod magnifier;
pub mod screenreader;

pub use framework::*;
pub use keyboard::*;
pub use magnifier::*;
pub use screenreader::*;

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
