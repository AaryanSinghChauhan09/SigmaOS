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
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Window Manager for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 686
/// Implements window creation, management, and composition

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WindowID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WindowState { Normal = 0, Minimized = 1, Maximized = 2, Fullscreen = 3, Hidden = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WindowError { Success = 0, NotFound = 1, InvalidState = 2 }

pub trait Window {
    fn id(&self) -> WindowID;
    fn title(&self) -> &[u8];
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn state(&self) -> WindowState;
    fn set_state(&mut self, state: WindowState);
    fn move_to(&mut self, x: i32, y: i32);
    fn resize(&mut self, width: u32, height: u32);
}

#[repr(C)]
pub struct SimpleWindow {
    pub id: WindowID,
    pub title: [u8; 128],
    pub x: AtomicUsize,
    pub y: AtomicUsize,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub state: AtomicUsize,
}

impl SimpleWindow {
    pub fn new(id: WindowID, title: &[u8], x: i32, y: i32, width: u32, height: u32) -> Self {
        let mut title_array = [0u8; 128];
        let title_len = title.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(title.as_ptr(), title_array.as_mut_ptr(), title_len);
        }
        SimpleWindow {
            id,
            title: title_array,
            x: AtomicUsize::new(x as usize),
            y: AtomicUsize::new(y as usize),
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            state: AtomicUsize::new(WindowState::Normal as usize),
        }
    }
}

impl Window for SimpleWindow {
    fn id(&self) -> WindowID { self.id }
    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }
    fn x(&self) -> i32 { self.x.load(Ordering::SeqCst) as i32 }
    fn y(&self) -> i32 { self.y.load(Ordering::SeqCst) as i32 }
    fn width(&self) -> u32 { self.width.load(Ordering::SeqCst) as u32 }
    fn height(&self) -> u32 { self.height.load(Ordering::SeqCst) as u32 }
    fn state(&self) -> WindowState {
        match self.state.load(Ordering::SeqCst) {
            0 => WindowState::Normal,
            1 => WindowState::Minimized,
            2 => WindowState::Maximized,
            3 => WindowState::Fullscreen,
            _ => WindowState::Hidden,
        }
    }

    fn set_state(&mut self, state: WindowState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.x.store(x as usize, Ordering::SeqCst);
        self.y.store(y as usize, Ordering::SeqCst);
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width.store(width as usize, Ordering::SeqCst);
        self.height.store(height as usize, Ordering::SeqCst);
    }
}

pub trait WindowManager {
    fn create_window(&mut self, title: &[u8], x: i32, y: i32, width: u32, height: u32) -> Result<WindowID, WindowError>;
    fn destroy_window(&mut self, id: WindowID) -> Result<(), WindowError>;
    fn get_window(&self, id: WindowID) -> Option<&dyn Window>;
    fn focus_window(&mut self, id: WindowID) -> Result<(), WindowError>;
    fn list_windows(&self) -> Vec<WindowID>;
}

#[repr(C)]
pub struct SimpleWindowManager {
    pub windows: Vec<Option<Box<dyn Window>>>,
    pub focused: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleWindowManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleWindowManager {
            windows: Vec::new(),
            focused: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WindowManager for SimpleWindowManager {
    fn create_window(&mut self, title: &[u8], x: i32, y: i32, width: u32, height: u32) -> Result<WindowID, WindowError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let window = SimpleWindow::new(id, title, x, y, width, height);
        self.windows.push(Some(Box::new(window)));
        Ok(id)
    }

    fn destroy_window(&mut self, id: WindowID) -> Result<(), WindowError> {
        for window_option in &mut self.windows {
            if let Some(ref window) = *window_option {
                if window.id() == id {
                    return Ok(());
                }
            }
        }
        Err(WindowError::NotFound)
    }

    fn get_window(&self, id: WindowID) -> Option<&dyn Window> {
        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                if window.id() == id { return Some(window.as_ref()); }
            }
        }
        None
    }

    fn focus_window(&mut self, id: WindowID) -> Result<(), WindowError> {
        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                if window.id() == id {
                    self.focused.store(id, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(WindowError::NotFound)
    }

    fn list_windows(&self) -> Vec<WindowID> {
        let mut ids = Vec::new();
        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                ids.push(window.id());
            }
        }
        ids
    }
}

pub trait WindowDecoration {
    fn set_border(&mut self, window_id: WindowID, width: u32, color: u32) -> Result<(), WindowError>;
    fn set_title_bar(&mut self, window_id: WindowID, height: u32, color: u32) -> Result<(), WindowError>;
    fn set_shadow(&mut self, window_id: WindowID, enabled: bool, blur: u32) -> Result<(), WindowError>;
}

#[repr(C)]
pub struct SimpleWindowDecoration {
    pub manager: SimpleWindowManager,
}

impl SimpleWindowDecoration {
    pub fn new(manager: SimpleWindowManager) -> Self {
        SimpleWindowDecoration { manager }
    }
}

impl WindowDecoration for SimpleWindowDecoration {
    fn set_border(&mut self, _window_id: WindowID, _width: u32, _color: u32) -> Result<(), WindowError> {
        Ok(())
    }

    fn set_title_bar(&mut self, _window_id: WindowID, _height: u32, _color: u32) -> Result<(), WindowError> {
        Ok(())
    }

    fn set_shadow(&mut self, _window_id: WindowID, _enabled: bool, _blur: u32) -> Result<(), WindowError> {
        Ok(())
    }
}

/// Linux (i3/sway/bspwm) & BSD inspired Zenith Dynamic Tiling Layout Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TilingMode {
    HorizontalSplit,
    VerticalSplit,
    BinarySpacePartition,
    FloatingSnap,
}

pub struct ZenithTilingLayout {
    pub screen_width: u32,
    pub screen_height: u32,
    pub gap_px: u32,
    pub mode: TilingMode,
}

impl ZenithTilingLayout {
    pub fn new(screen_width: u32, screen_height: u32, gap_px: u32) -> Self {
        Self {
            screen_width,
            screen_height,
            gap_px,
            mode: TilingMode::HorizontalSplit,
        }
    }

    /// Calculate window bounds dynamically for tiling
    pub fn calculate_bounds(&self, window_count: usize, index: usize) -> (i32, i32, u32, u32) {
        if window_count == 0 {
            return (0, 0, self.screen_width, self.screen_height);
        }

        match self.mode {
            TilingMode::HorizontalSplit => {
                let avail_width = self.screen_width.saturating_sub(self.gap_px * (window_count as u32 + 1));
                let win_width = avail_width / window_count as u32;
                let x = self.gap_px + index as u32 * (win_width + self.gap_px);
                let y = self.gap_px;
                let h = self.screen_height.saturating_sub(self.gap_px * 2);
                (x as i32, y as i32, win_width, h)
            }
            TilingMode::VerticalSplit => {
                let avail_height = self.screen_height.saturating_sub(self.gap_px * (window_count as u32 + 1));
                let win_height = avail_height / window_count as u32;
                let y = self.gap_px + index as u32 * (win_height + self.gap_px);
                let x = self.gap_px;
                let w = self.screen_width.saturating_sub(self.gap_px * 2);
                (x as i32, y as i32, w, win_height)
            }
            TilingMode::BinarySpacePartition | TilingMode::FloatingSnap => {
                let half_w = (self.screen_width / 2).saturating_sub(self.gap_px * 2);
                let half_h = (self.screen_height / 2).saturating_sub(self.gap_px * 2);
                let x = if index % 2 == 0 { self.gap_px as i32 } else { (self.screen_width / 2) as i32 + self.gap_px as i32 };
                let y = if index < 2 { self.gap_px as i32 } else { (self.screen_height / 2) as i32 + self.gap_px as i32 };
                (x, y, half_w, half_h)
            }
        }
    }
}

impl Default for ZenithTilingLayout {
    fn default() -> Self {
        Self::new(1920, 1080, 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zenith_tiling_bounds() {
        let layout = ZenithTilingLayout::new(1920, 1080, 10);
        let (x, y, w, h) = layout.calculate_bounds(2, 0);
        assert_eq!(x, 10);
        assert_eq!(y, 10);
        assert!(w > 900);
        assert_eq!(h, 1060);
    }
}


