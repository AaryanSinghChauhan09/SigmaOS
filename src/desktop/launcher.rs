#![no_std]
#![no_main]

/// OOP-based Application Launcher for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 756
/// Implements application launcher and search

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AppID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LauncherError { Success = 0, NotFound = 1, LaunchFailed = 2 }

pub trait Application {
    fn id(&self) -> AppID;
    fn name(&self) -> &[u8];
    fn executable(&self) -> &[u8];
    fn icon(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleApplication {
    pub id: AppID,
    pub name: [u8; 64],
    pub executable: [u8; 256],
    pub icon: [u8; 256],
}

impl SimpleApplication {
    pub fn new(id: AppID, name: &[u8], executable: &[u8], icon: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut exec_array = [0u8; 256];
        let mut icon_array = [0u8; 256];
        let name_len = name.len().min(63);
        let exec_len = executable.len().min(255);
        let icon_len = icon.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(executable.as_ptr(), exec_array.as_mut_ptr(), exec_len);
            core::ptr::copy_nonoverlapping(icon.as_ptr(), icon_array.as_mut_ptr(), icon_len);
        }
        SimpleApplication {
            id,
            name: name_array,
            executable: exec_array,
            icon: icon_array,
        }
    }
}

impl Application for SimpleApplication {
    fn id(&self) -> AppID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn executable(&self) -> &[u8] {
        let len = self.executable.iter().position(|&b| b == 0).unwrap_or(256);
        &self.executable[..len]
    }
    fn icon(&self) -> &[u8] {
        let len = self.icon.iter().position(|&b| b == 0).unwrap_or(256);
        &self.icon[..len]
    }
}

pub trait ApplicationLauncher {
    fn register_app(&mut self, app: Box<dyn Application>) -> Result<AppID, LauncherError>;
    fn unregister_app(&mut self, id: AppID) -> Result<(), LauncherError>;
    def launch_app(&self, id: AppID) -> Result<(), LauncherError>;
    fn search_apps(&self, query: &[u8]) -> Vec<AppID>;
}

#[repr(C)]
pub struct SimpleApplicationLauncher {
    pub apps: Vec<Option<Box<dyn Application>>>,
    pub next_id: AtomicUsize,
}

impl SimpleApplicationLauncher {
    pub fn new() -> Self {
        SimpleApplicationLauncher {
            apps: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ApplicationLauncher for SimpleApplicationLauncher {
    fn register_app(&mut self, app: Box<dyn Application>) -> Result<AppID, LauncherError> {
        let id = app.id();
        self.apps.push(Some(app));
        Ok(id)
    }
    
    fn unregister_app(&mut self, id: AppID) -> Result<(), LauncherError> {
        for app_option in &mut self.apps {
            if let Some(ref app) = *app_option {
                if app.id() == id {
                    return Ok(());
                }
            }
        }
        Err(LauncherError::NotFound)
    }
    
    fn launch_app(&self, id: AppID) -> Result<(), LauncherError> {
        if self.get_app(id).is_some() {
            Ok(())
        } else {
            Err(LauncherError::NotFound)
        }
    }
    
    fn search_apps(&self, query: &[u8]) -> Vec<AppID> {
        let mut results = Vec::new();
        for app_option in &self.apps {
            if let Some(ref app) = *app_option {
                if app.name().starts_with(query) {
                    results.push(app.id());
                }
            }
        }
        results
    }
    
    fn get_app(&self, id: AppID) -> Option<&dyn Application> {
        for app_option in &self.apps {
            if let Some(ref app) = *app_option {
                if app.id() == id { return Some(app.as_ref()); }
            }
        }
        None
    }
}

pub trait RecentApps {
    def add_recent(&mut self, app_id: AppID);
    def get_recent(&self) -> Vec<AppID>;
}

#[repr(C)]
pub struct SimpleRecentApps {
    pub recent: Vec<AppID>,
}

impl SimpleRecentApps {
    pub fn new() -> Self {
        SimpleRecentApps {
            recent: Vec::new(),
        }
    }
}

impl RecentApps for SimpleRecentApps {
    fn add_recent(&mut self, app_id: AppID) {
        self.recent.push(app_id);
        if self.recent.len() > 10 {
            self.recent.remove(0);
        }
    }
    
    fn get_recent(&self) -> Vec<AppID> {
        self.recent.clone()
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
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
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
