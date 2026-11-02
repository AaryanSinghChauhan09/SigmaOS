//! SigmaOS Term
//!
//! Terminal emulator with multiple backend support for SigmaOS. Supports Foot,
//! Ghostty, Alacritty, and Kitty as interchangeable backends.
//!
//! # Modules
//!
//! - [`backend`]: Backend abstraction and implementations for Foot, Ghostty,
//!   Alacritty, and Kitty.
//! - [`config`]: Unified configuration model that translates to each backend's
//!   native format.

pub mod backend;
pub mod config;

/// Initialize the terminal emulator subsystem.
///
/// Call once at startup to detect available backends, load configuration,
/// and select the active backend based on user preferences.
pub fn init() {
    // TODO: detect available backends, load config, select active backend
}
