#![no_std]
#![no_main]

/// OOP-based Desktop Panel for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 726
/// Implements taskbar and panel management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PanelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PanelPosition { Top = 0, Bottom = 1, Left = 2, Right = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PanelError { Success = 0, NotFound = 1 }

pub trait Panel {
    fn id(&self) -> PanelID;
    fn position(&self) -> PanelPosition;
    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn is_autohide(&self) -> bool;
}

#[repr(C)]
pub struct SimplePanel {
    pub id: PanelID,
    pub position: AtomicUsize,
    pub height: AtomicUsize,
    pub width: AtomicUsize,
    pub autohide: AtomicUsize,
}

impl SimplePanel {
    pub fn new(id: PanelID, position: PanelPosition, height: u32, width: u32) -> Self {
        SimplePanel {
            id,
            position: AtomicUsize::new(position as usize),
            height: AtomicUsize::new(height as usize),
            width: AtomicUsize::new(width as usize),
            autohide: AtomicUsize::new(0),
        }
    }
}

impl Panel for SimplePanel {
    fn id(&self) -> PanelID { self.id }
    fn position(&self) -> PanelPosition { unsafe { core::mem::transmute(self.position.load(Ordering::SeqCst)) } }
    fn height(&self) -> u32 { self.height.load(Ordering::SeqCst) as u32 }
    fn width(&self) -> u32 { self.width.load(Ordering::SeqCst) as u32 }
    fn is_autohide(&self) -> bool { self.autohide.load(Ordering::SeqCst) == 1 }
}

pub trait PanelManager {
    fn create_panel(&mut self, position: PanelPosition, height: u32, width: u32) -> Result<PanelID, PanelError>;
    fn remove_panel(&mut self, id: PanelID) -> Result<(), PanelError>;
    fn get_panel(&self, id: PanelID) -> Option<&dyn Panel>;
    def add_applet(&mut self, panel_id: PanelID, applet: &[u8]) -> Result<(), PanelError>;
}

#[repr(C)]
pub struct SimplePanelManager {
    pub panels: Vec<Option<Box<dyn Panel>>>,
    pub applets: Vec<(PanelID, [u8; 64])>,
    pub next_id: AtomicUsize,
}

impl SimplePanelManager {
    pub fn new() -> Self {
        SimplePanelManager {
            panels: Vec::new(),
            applets: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PanelManager for SimplePanelManager {
    fn create_panel(&mut self, position: PanelPosition, height: u32, width: u32) -> Result<PanelID, PanelError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let panel = SimplePanel::new(id, position, height, width);
        self.panels.push(Some(Box::new(panel)));
        Ok(id)
    }
    
    fn remove_panel(&mut self, id: PanelID) -> Result<(), PanelError> {
        for panel_option in &mut self.panels {
            if let Some(ref panel) = *panel_option {
                if panel.id() == id {
                    return Ok(());
                }
            }
        }
        Err(PanelError::NotFound)
    }
    
    fn get_panel(&self, id: PanelID) -> Option<&dyn Panel> {
        for panel_option in &self.panels {
            if let Some(ref panel) = *panel_option {
                if panel.id() == id { return Some(panel.as_ref()); }
            }
        }
        None
    }
    
    fn add_applet(&mut self, panel_id: PanelID, applet: &[u8]) -> Result<(), PanelError> {
        let mut applet_array = [0u8; 64];
        let applet_len = applet.len().min(63);
        for i in 0..applet_len {
            applet_array[i] = applet[i];
        }
        self.applets.push((panel_id, applet_array));
        Ok(())
    }
}

pub trait Taskbar {
    fn add_task(&mut self, window_id: usize, title: &[u8]) -> Result<(), PanelError>;
    fn remove_task(&mut self, window_id: usize) -> Result<(), PanelError>;
    fn get_tasks(&self) -> Vec<(usize, &[u8])>;
}

#[repr(C)]
pub struct SimpleTaskbar {
    pub tasks: Vec<(usize, [u8; 128])>,
}

impl SimpleTaskbar {
    pub fn new() -> Self {
        SimpleTaskbar {
            tasks: Vec::new(),
        }
    }
}

impl Taskbar for SimpleTaskbar {
    fn add_task(&mut self, window_id: usize, title: &[u8]) -> Result<(), PanelError> {
        let mut title_array = [0u8; 128];
        let title_len = title.len().min(127);
        for i in 0..title_len {
            title_array[i] = title[i];
        }
        self.tasks.push((window_id, title_array));
        Ok(())
    }
    
    fn remove_task(&mut self, window_id: usize) -> Result<(), PanelError> {
        for i in 0..self.tasks.len() {
            if self.tasks[i].0 == window_id {
                self.tasks.remove(i);
                return Ok(());
            }
        }
        Err(PanelError::NotFound)
    }
    
    fn get_tasks(&self) -> Vec<(usize, &[u8])> {
        let mut result = Vec::new();
        for &(id, ref title) in &self.tasks {
            let len = title.iter().position(|&b| b == 0).unwrap_or(128);
            result.push((id, &title[..len]));
        }
        result
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
