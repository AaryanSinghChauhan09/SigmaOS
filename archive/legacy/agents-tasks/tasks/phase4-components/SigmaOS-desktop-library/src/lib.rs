//! SigmaOS shared desktop library.
//!
//! Provides data types, GSettings schemas, and helper functions shared
//! between the desktop environment, configurator, and applets.

#![warn(missing_docs)]

/// A desktop background type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundType {
    /// Solid color background.
    Color,
    /// Image background.
    Image,
    /// Slideshow background.
    Slideshow,
}

/// A panel position on the screen edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelEdge {
    /// Top edge.
    Top,
    /// Bottom edge.
    Bottom,
    /// Left edge.
    Left,
    /// Right edge.
    Right,
}

/// A monitor layout descriptor.
#[derive(Debug, Clone)]
pub struct Monitor {
    /// X11 or Wayland connector name.
    pub name: String,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Whether this is the primary monitor.
    pub primary: bool,
}

/// Helper returning the primary monitor from a list.
pub fn primary_monitor(monitors: &[Monitor]) -> Option<&Monitor> {
    monitors.iter().find(|m| m.primary)
}
