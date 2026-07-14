#![no_std]
#![no_main]

/// OOP-based Window Manager for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 686
/// Implements window creation, management, and composition

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
    fn state(&self) -> WindowState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

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
    def list_windows(&self) -> Vec<WindowID>;
}

#[repr(C)]
pub struct SimpleWindowManager {
    pub windows: Vec<Option<Box<dyn Window>>>,
    pub focused: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleWindowManager {
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
