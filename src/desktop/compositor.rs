#![no_std]
#![no_main]

/// OOP-based Desktop Environment & Compositor for SigmaOS
/// Based on Roadmap Item: Desktop Environment + Zenith Compositor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WindowID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WindowState { Hidden = 0, Visible = 1, Minimized = 2, Maximized = 3 }

pub trait Window {
    fn id(&self) -> WindowID;
    fn state(&self) -> WindowState;
    fn show(&mut self) -> Result<(), DesktopError>;
    fn hide(&mut self) -> Result<(), DesktopError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DesktopError { Success = 0, InvalidWindow = 1, RenderFailed = 2 }

#[repr(C)]
pub struct SimpleWindow {
    pub id: WindowID,
    pub state: AtomicUsize,
    pub x: AtomicUsize,
    pub y: AtomicUsize,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleWindow {
    pub fn new(id: WindowID, x: usize, y: usize, width: usize, height: usize) -> Self {
        SimpleWindow {
            id,
            state: AtomicUsize::new(WindowState::Hidden as usize),
            x: AtomicUsize::new(x),
            y: AtomicUsize::new(y),
            width: AtomicUsize::new(width),
            height: AtomicUsize::new(height),
        }
    }
}

impl Window for SimpleWindow {
    fn id(&self) -> WindowID { self.id }
    fn state(&self) -> WindowState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn show(&mut self) -> Result<(), DesktopError> {
        self.state.store(WindowState::Visible as usize, Ordering::SeqCst);
        Ok(())
    }
    fn hide(&mut self) -> Result<(), DesktopError> {
        self.state.store(WindowState::Hidden as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait Compositor {
    fn create_window(&mut self, x: usize, y: usize, width: usize, height: usize) -> Result<WindowID, DesktopError>;
    fn destroy_window(&mut self, id: WindowID) -> Result<(), DesktopError>;
    fn render(&mut self) -> Result<(), DesktopError>;
    fn get_window(&self, id: WindowID) -> Option<&dyn Window>;
}

pub struct SimpleCompositor {
    windows: Vec<Option<Box<dyn Window>>>,
    next_id: AtomicUsize,
}

impl SimpleCompositor {
    pub fn new() -> Self { SimpleCompositor { windows: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl Compositor for SimpleCompositor {
    fn create_window(&mut self, x: usize, y: usize, width: usize, height: usize) -> Result<WindowID, DesktopError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let window = SimpleWindow::new(id, x, y, width, height);
        self.windows.push(Some(Box::new(window)));
        Ok(id)
    }
    fn destroy_window(&mut self, id: WindowID) -> Result<(), DesktopError> {
        for window_option in &mut self.windows {
            if let Some(ref window) = *window_option {
                if window.id() == id {
                    self.windows.clear();
                    return Ok(());
                }
            }
        }
        Err(DesktopError::InvalidWindow)
    }
    fn render(&mut self) -> Result<(), DesktopError> {
        Ok(())
    }
    fn get_window(&self, id: WindowID) -> Option<&dyn Window> {
        for window_option in &self.windows {
            if let Some(ref window) = *window_option {
                if window.id() == id { return Some(window.as_ref()); }
            }
        }
        None
    }
}

pub trait DesktopEnvironment {
    fn init(&mut self) -> Result<(), DesktopError>;
    fn run(&mut self) -> Result<(), DesktopError>;
}

pub struct SimpleDesktopEnvironment {
    pub compositor: SimpleCompositor,
}

impl SimpleDesktopEnvironment {
    pub fn new() -> Self { SimpleDesktopEnvironment { compositor: SimpleCompositor::new() } }
}

impl DesktopEnvironment for SimpleDesktopEnvironment {
    fn init(&mut self) -> Result<(), DesktopError> {
        Ok(())
    }
    fn run(&mut self) -> Result<(), DesktopError> {
        self.compositor.render()
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
    fn clear(&mut self) { self.len = 0; }
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
