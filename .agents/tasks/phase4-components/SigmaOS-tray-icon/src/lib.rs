//! SigmaOS-tray-icon: System tray icon library.
//!
//! Provides a shared library for applications to display system tray icons
//! and menus. Inspired by rhythmbox-tray-icon and libappindicator.

pub mod tray;
pub mod menu;

/// Initialise the tray icon subsystem.
///
/// Call once at application startup before creating any tray icons.
pub fn init() {
    // TODO: connect to the desktop's appindicator or StatusNotifierItem service
}
