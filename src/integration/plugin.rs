#![no_std]
#![no_main]

/// OOP-based Plugin System for SigmaOS
/// Based on Ideas-999-Structured: Integration & Interoperability Item 906
/// Implements plugin loading and management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PluginID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PluginState { Unloaded = 0, Loaded = 1, Active = 2, Error = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PluginError { Success = 0, NotFound = 1, LoadFailed = 2 }

pub trait Plugin {
    fn id(&self) -> PluginID;
    fn name(&self) -> &[u8];
    fn version(&self) -> &[u8];
    fn state(&self) -> PluginState;
}

#[repr(C)]
pub struct SimplePlugin {
    pub id: PluginID,
    pub name: [u8; 64],
    pub version: [u8; 16],
    pub state: AtomicUsize,
}

impl SimplePlugin {
    pub fn new(id: PluginID, name: &[u8], version: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut ver_array = [0u8; 16];
        let name_len = name.len().min(63);
        let ver_len = version.len().min(15);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(version.as_ptr(), ver_array.as_mut_ptr(), ver_len);
        }
        SimplePlugin {
            id,
            name: name_array,
            version: ver_array,
            state: AtomicUsize::new(PluginState::Unloaded as usize),
        }
    }
}

impl Plugin for SimplePlugin {
    fn id(&self) -> PluginID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn version(&self) -> &[u8] {
        let len = self.version.iter().position(|&b| b == 0).unwrap_or(16);
        &self.version[..len]
    }
    fn state(&self) -> PluginState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
}

pub trait PluginManager {
    fn load_plugin(&mut self, path: &[u8]) -> Result<PluginID, PluginError>;
    fn unload_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
    fn get_plugin(&self, id: PluginID) -> Option<&dyn Plugin>;
    def enable_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
}

#[repr(C)]
pub struct SimplePluginManager {
    pub plugins: Vec<Option<Box<dyn Plugin>>>,
    pub next_id: AtomicUsize,
}

impl SimplePluginManager {
    pub fn new() -> Self {
        SimplePluginManager {
            plugins: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PluginManager for SimplePluginManager {
    fn load_plugin(&mut self, path: &[u8]) -> Result<PluginID, PluginError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let name_len = path.len().min(63);
        let mut name_array = [0u8; 64];
        for i in 0..name_len {
            name_array[i] = path[i];
        }
        let plugin = SimplePlugin::new(id, &name_array, b"1.0.0");
        plugin.state.store(PluginState::Loaded as usize, Ordering::SeqCst);
        self.plugins.push(Some(Box::new(plugin)));
        Ok(id)
    }
    
    fn unload_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        for plugin_option in &mut self.plugins {
            if let Some(ref mut plugin) = *plugin_option {
                if plugin.id() == id {
                    plugin.state.store(PluginState::Unloaded as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PluginError::NotFound)
    }
    
    fn get_plugin(&self, id: PluginID) -> Option<&dyn Plugin> {
        for plugin_option in &self.plugins {
            if let Some(ref plugin) = *plugin_option {
                if plugin.id() == id { return Some(plugin.as_ref()); }
            }
        }
        None
    }
    
    fn enable_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        for plugin_option in &mut self.plugins {
            if let Some(ref mut plugin) = *plugin_option {
                if plugin.id() == id {
                    plugin.state.store(PluginState::Active as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PluginError::NotFound)
    }
}

pub trait PluginAPI {
    fn register_extension(&mut self, plugin_id: PluginID, extension: &[u8]);
    def get_extension(&self, plugin_id: PluginID, extension: &[u8]) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimplePluginAPI {
    pub extensions: Vec<(PluginID, [u8; 64], Vec<u8>)>,
}

impl SimplePluginAPI {
    pub fn new() -> Self {
        SimplePluginAPI {
            extensions: Vec::new(),
        }
    }
}

impl PluginAPI for SimplePluginAPI {
    fn register_extension(&mut self, plugin_id: PluginID, extension: &[u8]) {
        let mut ext_array = [0u8; 64];
        let ext_len = extension.len().min(63);
        for i in 0..ext_len {
            ext_array[i] = extension[i];
        }
        self.extensions.push((plugin_id, ext_array, Vec::new()));
    }
    
    fn get_extension(&self, plugin_id: PluginID, extension: &[u8]) -> Option<&[u8]> {
        for &(id, ref ext, ref data) in &self.extensions {
            if id == plugin_id {
                let ext_len = ext.iter().position(|&b| b == 0).unwrap_or(64);
                if &ext[..ext_len] == extension {
                    return Some(data);
                }
            }
        }
        None
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
