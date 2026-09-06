#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Desktop Compositor Module
// Implements window management and composition
// Inspired by Wayland and Windows DWM

use std::string::String;
use std::vec::Vec;

/// Window position and size
#[derive(Debug, Clone, Copy)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl WindowGeometry {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

/// Window state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Tiled,
}

/// Window layer (z-order)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowLayer {
    Background = 0,
    Bottom = 1,
    Normal = 2,
    Top = 3,
    Overlay = 4,
    Dock = 5,
    Menu = 6,
    Notification = 7,
}

/// Window representation
#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub geometry: WindowGeometry,
    pub state: WindowState,
    pub layer: WindowLayer,
    pub is_focused: bool,
    pub is_decorated: bool,
}

impl Window {
    pub fn new(id: u32, title: String, geometry: WindowGeometry) -> Self {
        Self {
            id,
            title,
            geometry,
            state: WindowState::Normal,
            layer: WindowLayer::Normal,
            is_focused: false,
            is_decorated: true,
        }
    }

    /// Set window geometry
    pub fn set_geometry(&mut self, geometry: WindowGeometry) {
        self.geometry = geometry;
    }

    /// Set window state
    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    /// Set window layer
    pub fn set_layer(&mut self, layer: WindowLayer) {
        self.layer = layer;
    }

    /// Focus window
    pub fn focus(&mut self) {
        self.is_focused = true;
    }

    /// Unfocus window
    pub fn unfocus(&mut self) {
        self.is_focused = false;
    }
}

/// Compositor surface
#[derive(Debug, Clone)]
pub struct Surface {
    pub window_id: u32,
    pub buffer_id: u32,
    pub is_damage: bool,
}

/// Compositor state
pub struct Compositor {
    pub windows: Vec<Window>,
    pub surfaces: Vec<Surface>,
    pub focused_window: Option<u32>,
    pub next_window_id: u32,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            surfaces: Vec::new(),
            focused_window: None,
            next_window_id: 1,
        }
    }

    /// Create a new window
    pub fn create_window(&mut self, title: String, geometry: WindowGeometry) -> u32 {
        let id = self.next_window_id;
        self.next_window_id += 1;

        let window = Window::new(id, title, geometry);
        self.windows.push(window);

        id
    }

    /// Destroy a window
    pub fn destroy_window(&mut self, window_id: u32) -> Result<(), String> {
        if let Some(pos) = self.windows.iter().position(|w| w.id == window_id) {
            self.windows.remove(pos);
            self.surfaces.retain(|s| s.window_id != window_id);

            if self.focused_window == Some(window_id) {
                self.focused_window = None;
            }

            Ok(())
        } else {
            Err(format!("Window {} not found", window_id))
        }
    }

    /// Get window by ID
    pub fn get_window(&self, window_id: u32) -> Option<&Window> {
        self.windows.iter().find(|w| w.id == window_id)
    }

    /// Get mutable window by ID
    pub fn get_window_mut(&mut self, window_id: u32) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == window_id)
    }

    /// Focus a window
    pub fn focus_window(&mut self, window_id: u32) -> Result<(), String> {
        if self.get_window(window_id).is_some() {
            // Unfocus current window
            if let Some(current_id) = self.focused_window {
                if let Some(window) = self.get_window_mut(current_id) {
                    window.unfocus();
                }
            }

            // Focus new window
            if let Some(window) = self.get_window_mut(window_id) {
                window.focus();
            }

            self.focused_window = Some(window_id);
            Ok(())
        } else {
            Err(format!("Window {} not found", window_id))
        }
    }

    /// Get focused window
    pub fn get_focused_window(&self) -> Option<&Window> {
        self.focused_window.and_then(|id| self.get_window(id))
    }

    /// Sort windows by layer (for rendering)
    pub fn sort_windows_by_layer(&mut self) {
        self.windows.sort_by(|a, b| {
            // Focused window should be on top within its layer
            if a.is_focused && !b.is_focused {
                return core::cmp::Ordering::Greater;
            }
            if !a.is_focused && b.is_focused {
                return core::cmp::Ordering::Less;
            }

            // Sort by layer
            a.layer.cmp(&b.layer)
        });
    }

    /// Add surface for window
    pub fn add_surface(&mut self, window_id: u32, buffer_id: u32) -> Result<(), String> {
        if self.get_window(window_id).is_some() {
            self.surfaces.push(Surface {
                window_id,
                buffer_id,
                is_damage: true,
            });
            Ok(())
        } else {
            Err(format!("Window {} not found", window_id))
        }
    }

    /// Mark surface as damaged (needs repaint)
    pub fn mark_surface_damage(&mut self, window_id: u32) {
        for surface in &mut self.surfaces {
            if surface.window_id == window_id {
                surface.is_damage = true;
            }
        }
    }

    /// Clear damage from all surfaces
    pub fn clear_damage(&mut self) {
        for surface in &mut self.surfaces {
            surface.is_damage = false;
        }
    }
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let mut compositor = Compositor::new();
        let geometry = WindowGeometry::new(100, 100, 800, 600);
        let id = compositor.create_window("Test Window".to_string(), geometry);

        assert_eq!(id, 1);
        assert!(compositor.get_window(id).is_some());
    }

    #[test]
    fn test_window_focus() {
        let mut compositor = Compositor::new();
        let geometry = WindowGeometry::new(100, 100, 800, 600);
        let id = compositor.create_window("Test Window".to_string(), geometry);

        compositor.focus_window(id).unwrap();
        assert_eq!(compositor.focused_window, Some(id));
        assert!(compositor.get_window(id).unwrap().is_focused);
    }

    #[test]
    fn test_window_destruction() {
        let mut compositor = Compositor::new();
        let geometry = WindowGeometry::new(100, 100, 800, 600);
        let id = compositor.create_window("Test Window".to_string(), geometry);

        compositor.destroy_window(id).unwrap();
        assert!(compositor.get_window(id).is_none());
    }

    #[test]
    fn test_window_layer_sorting() {
        let mut compositor = Compositor::new();
        let geometry = WindowGeometry::new(100, 100, 800, 600);

        let id1 = compositor.create_window("Normal".to_string(), geometry);
        let id2 = compositor.create_window("Overlay".to_string(), geometry);

        compositor.get_window_mut(id2).unwrap().set_layer(WindowLayer::Overlay);
        compositor.sort_windows_by_layer();

        // Overlay should come after Normal
        let positions: Vec<u32> = compositor.windows.iter().map(|w| w.id).collect();
        assert_eq!(positions, vec![id1, id2]);
    }
}
