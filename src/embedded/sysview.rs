#![no_std]
#![no_main]

/// OOP-based SystemView for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2176
/// Implements SystemView tracing

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SysViewID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SysViewError { Success = 0, NotFound = 1 }

pub trait SysView {
    fn id(&self) -> SysViewID;
    fn is_recording(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSysView {
    pub id: SysViewID,
    pub recording: AtomicUsize,
}

impl SimpleSysView {
    pub fn new(id: SysViewID) -> Self {
        SimpleSysView {
            id,
            recording: AtomicUsize::new(0),
        }
    }
}

impl SysView for SimpleSysView {
    fn id(&self) -> SysViewID { self.id }
    fn is_recording(&self) -> bool { self.recording.load(Ordering::SeqCst) == 1 }
}

pub trait SysViewController {
    fn start(&mut self, sv_id: SysViewID) -> Result<(), SysViewError>;
    fn stop(&mut self, sv_id: SysViewID) -> Result<(), SysViewError>;
    def log(&self, sv_id: SysViewID, event: u32, data: u32) -> Result<(), SysViewError>;
}

#[repr(C)]
pub struct SimpleSysViewController {
    pub views: Vec<Option<Box<dyn SysView>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSysViewController {
    pub fn new() -> Self {
        SimpleSysViewController {
            views: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SysViewController for SimpleSysViewController {
    fn start(&mut self, sv_id: SysViewID) -> Result<(), SysViewError> {
        for view_option in &mut self.views {
            if let Some(ref mut view) = *view_option {
                if view.id() == sv_id {
                    view.recording.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SysViewError::NotFound)
    }
    
    fn stop(&mut self, sv_id: SysViewID) -> Result<(), SysViewError> {
        for view_option in &mut self.views {
            if let Some(ref mut view) = *view_option {
                if view.id() == sv_id {
                    view.recording.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SysViewError::NotFound)
    }
    
    fn log(&self, sv_id: SysViewID, _event: u32, _data: u32) -> Result<(), SysViewError> {
        if self.get_view(sv_id).is_some() {
            Ok(())
        } else {
            Err(SysViewError::NotFound)
        }
    }
    
    fn get_view(&self, id: SysViewID) -> Option<&dyn SysView> {
        for view_option in &self.views {
            if let Some(ref view) = *view_option {
                if view.id() == id { return Some(view.as_ref()); }
            }
        }
        None
    }
}

pub trait TaskTrace {
    def task_start(&self, sv_id: SysViewID, task_id: u32) -> Result<(), SysViewError>;
    def task_stop(&self, sv_id: SysViewID, task_id: u32) -> Result<(), SysViewError>;
}

#[repr(C)]
pub struct SimpleTaskTrace {
    pub controller: SimpleSysViewController,
}

impl SimpleTaskTrace {
    pub fn new(controller: SimpleSysViewController) -> Self {
        SimpleTaskTrace { controller }
    }
}

impl TaskTrace for SimpleTaskTrace {
    fn task_start(&self, sv_id: SysViewID, _task_id: u32) -> Result<(), SysViewError> {
        if self.controller.get_view(sv_id).is_some() {
            Ok(())
        } else {
            Err(SysViewError::NotFound)
        }
    }
    
    fn task_stop(&self, sv_id: SysViewID, _task_id: u32) -> Result<(), SysViewError> {
        if self.controller.get_view(sv_id).is_some() {
            Ok(())
        } else {
            Err(SysViewError::NotFound)
        }
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
