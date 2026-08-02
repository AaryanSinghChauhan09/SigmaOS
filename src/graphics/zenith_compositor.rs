#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Zenith Desktop Compositor
// Wayland-inspired compositor with OOP design

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

pub const SCREEN_WIDTH: u32 = 1920;
pub const SCREEN_HEIGHT: u32 = 1080;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowState {
    Active,
    Inactive,
    Minimized,
}

#[derive(Debug, Clone, Copy)]
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct WindowNode {
    pub id: u32,
    pub title: String,
    pub geometry: Geometry,
    pub state: WindowState,
}

impl WindowNode {
    pub fn new(id: u32, title: String, x: i32, y: i32, w: u32, h: u32) -> Self {
        Self {
            id,
            title,
            geometry: Geometry {
                x,
                y,
                width: w,
                height: h,
            },
            state: WindowState::Inactive,
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.geometry.width = w;
        self.geometry.height = h;
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.geometry.x = x;
        self.geometry.y = y;
    }

    pub fn set_state(&mut self, state: WindowState) {
        self.state = state;
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.geometry.x
            && x < self.geometry.x + self.geometry.width as i32
            && y >= self.geometry.y
            && y < self.geometry.y + self.geometry.height as i32
    }
}

pub struct ZenithCompositor {
    pub windows: Vec<Option<WindowNode>>,
    pub active_window_id: Option<u32>,
}

impl ZenithCompositor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window_id: None,
        }
    }

    pub fn register_window(&mut self, window: WindowNode) -> Result<(), &'static str> {
        if self.windows.len() >= 32 {
            return Err("Maximum window limit reached");
        }
        self.windows.push(Some(window));
        Ok(())
    }

    pub fn activate_window(&mut self, window_id: u32) -> Result<(), &'static str> {
        let window = self.get_window_mut(window_id).ok_or("Window not found")?;
        window.set_state(WindowState::Active);

        // Deactivate other windows
        for win_opt in self.windows.iter_mut() {
            if let Some(win) = win_opt {
                if win.id != window_id {
                    win.set_state(WindowState::Inactive);
                }
            }
        }

        self.active_window_id = Some(window_id);
        Ok(())
    }

    pub fn minimize_window(&mut self, window_id: u32) -> Result<(), &'static str> {
        let window = self.get_window_mut(window_id).ok_or("Window not found")?;
        window.set_state(WindowState::Minimized);

        if self.active_window_id == Some(window_id) {
            self.active_window_id = None;
        }
        Ok(())
    }

    pub fn get_window(&self, window_id: u32) -> Option<&WindowNode> {
        for win_opt in self.windows.iter() {
            if let Some(win) = win_opt {
                if win.id == window_id {
                    return Some(win);
                }
            }
        }
        None
    }

    pub fn get_window_mut(&mut self, window_id: u32) -> Option<&mut WindowNode> {
        for win_opt in self.windows.iter_mut() {
            if let Some(win) = win_opt {
                if win.id == window_id {
                    return Some(win);
                }
            }
        }
        None
    }

    pub fn get_active_window(&self) -> Option<&WindowNode> {
        if let Some(id) = self.active_window_id {
            self.get_window(id)
        } else {
            None
        }
    }

    pub fn get_window_at_point(&self, x: i32, y: i32) -> Option<&WindowNode> {
        // Check windows in reverse order (top to bottom)
        for win_opt in self.windows.iter().rev() {
            if let Some(win) = win_opt {
                if win.state != WindowState::Minimized && win.contains_point(x, y) {
                    return Some(win);
                }
            }
        }
        None
    }

    pub fn window_count(&self) -> usize {
        self.windows.iter().filter(|w| w.is_some()).count()
    }

    pub fn remove_window(&mut self, window_id: u32) -> Result<(), &'static str> {
        for win_opt in self.windows.iter_mut() {
            if let Some(win) = win_opt {
                if win.id == window_id {
                    *win_opt = None;
                    if self.active_window_id == Some(window_id) {
                        self.active_window_id = None;
                    }
                    return Ok(());
                }
            }
        }
        Err("Window not found")
    }
}

impl Default for ZenithCompositor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation() {
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);
        assert_eq!(window.id, 1);
        assert_eq!(window.geometry.width, 800);
        assert_eq!(window.state, WindowState::Inactive);
    }

    #[test]
    fn test_window_resize() {
        let mut window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);
        window.resize(1024, 768);
        assert_eq!(window.geometry.width, 1024);
        assert_eq!(window.geometry.height, 768);
    }

    #[test]
    fn test_window_move() {
        let mut window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);
        window.move_to(100, 200);
        assert_eq!(window.geometry.x, 100);
        assert_eq!(window.geometry.y, 200);
    }

    #[test]
    fn test_window_contains_point() {
        let window = WindowNode::new(1, "Test".to_string(), 100, 100, 800, 600);
        assert!(window.contains_point(150, 150));
        assert!(!window.contains_point(50, 50));
        assert!(!window.contains_point(1000, 1000));
    }

    #[test]
    fn test_compositor_register() {
        let mut compositor = ZenithCompositor::new();
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);

        compositor.register_window(window).unwrap();
        assert_eq!(compositor.window_count(), 1);
    }

    #[test]
    fn test_compositor_activate() {
        let mut compositor = ZenithCompositor::new();
        let window1 = WindowNode::new(1, "Test1".to_string(), 0, 0, 800, 600);
        let window2 = WindowNode::new(2, "Test2".to_string(), 100, 100, 800, 600);

        compositor.register_window(window1).unwrap();
        compositor.register_window(window2).unwrap();

        compositor.activate_window(1).unwrap();
        assert_eq!(compositor.active_window_id, Some(1));
        assert_eq!(compositor.get_window(1).unwrap().state, WindowState::Active);
        assert_eq!(
            compositor.get_window(2).unwrap().state,
            WindowState::Inactive
        );
    }

    #[test]
    fn test_compositor_minimize() {
        let mut compositor = ZenithCompositor::new();
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);

        compositor.register_window(window).unwrap();
        compositor.activate_window(1).unwrap();
        compositor.minimize_window(1).unwrap();

        assert_eq!(
            compositor.get_window(1).unwrap().state,
            WindowState::Minimized
        );
        assert_eq!(compositor.active_window_id, None);
    }

    #[test]
    fn test_compositor_remove() {
        let mut compositor = ZenithCompositor::new();
        let window = WindowNode::new(1, "Test".to_string(), 0, 0, 800, 600);

        compositor.register_window(window).unwrap();
        compositor.activate_window(1).unwrap();
        compositor.remove_window(1).unwrap();

        assert_eq!(compositor.window_count(), 0);
        assert_eq!(compositor.active_window_id, None);
    }

    #[test]
    fn test_get_window_at_point() {
        let mut compositor = ZenithCompositor::new();
        let window1 = WindowNode::new(1, "Test1".to_string(), 0, 0, 800, 600);
        let window2 = WindowNode::new(2, "Test2".to_string(), 400, 300, 800, 600);

        compositor.register_window(window1).unwrap();
        compositor.register_window(window2).unwrap();

        // Window2 is on top (added later)
        let hit = compositor.get_window_at_point(500, 400);
        assert_eq!(hit.unwrap().id, 2);
    }

    #[test]
    fn test_window_limit() {
        let mut compositor = ZenithCompositor::new();

        for i in 0..32 {
            let window = WindowNode::new(i, format!("Window {}", i), 0, 0, 800, 600);
            compositor.register_window(window).unwrap();
        }

        let window = WindowNode::new(32, "Extra".to_string(), 0, 0, 800, 600);
        assert!(compositor.register_window(window).is_err());
    }
}
