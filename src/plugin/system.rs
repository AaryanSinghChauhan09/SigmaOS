#![no_std]
#![no_main]

/// OOP-based Plugin System for SigmaOS
/// Implements plugin management using OOP principles with traits and structs
/// No dependency on external plugin frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Plugin ID
pub type PluginID = usize;

/// Plugin trait (OOP interface)
pub trait Plugin {
    /// Get plugin ID
    fn id(&self) -> PluginID;
    /// Get plugin name
    fn name(&self) -> &[u8];
    /// Get plugin version
    fn version(&self) -> (u32, u32, u32);
    /// Initialize plugin
    fn initialize(&mut self) -> Result<(), PluginError>;
    /// Shutdown plugin
    fn shutdown(&mut self) -> Result<(), PluginError>;
    /// Get plugin info
    fn info(&self) -> PluginInfo;
}

/// Plugin error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PluginError {
    Success = 0,
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InitializationFailed = 3,
    ShutdownFailed = 4,
    PermissionDenied = 5,
    InvalidState = 6,
}

/// Plugin info
#[repr(C)]
pub struct PluginInfo {
    pub id: PluginID,
    pub name: [u8; 64],
    pub version_major: u32,
    pub version_minor: u32,
    pub version_patch: u32,
    pub state: PluginState,
    pub capability: PluginCapability,
}

impl PluginInfo {
    pub fn new(id: PluginID) -> Self {
        PluginInfo {
            id,
            name: [0; 64],
            version_major: 0,
            version_minor: 0,
            version_patch: 0,
            state: PluginState::Unloaded,
            capability: PluginCapability::new(),
        }
    }
}

/// Plugin state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PluginState {
    Unloaded = 0,
    Loaded = 1,
    Initialized = 2,
    Running = 3,
    Stopped = 4,
    Error = 5,
}

/// Plugin capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PluginCapability {
    pub can_initialize: bool,
    pub can_shutdown: bool,
    pub can_configure: bool,
}

impl PluginCapability {
    pub fn new() -> Self {
        PluginCapability {
            can_initialize: false,
            can_shutdown: false,
            can_configure: false,
        }
    }

    pub fn full() -> Self {
        PluginCapability {
            can_initialize: true,
            can_shutdown: true,
            can_configure: true,
        }
    }
}

/// Simple plugin (OOP: Concrete plugin class)
#[repr(C)]
pub struct SimplePlugin {
    pub id: PluginID,
    pub name: [u8; 64],
    pub version: (u32, u32, u32),
    pub state: AtomicUsize, // PluginState as usize
    pub capability: PluginCapability,
    pub data: Option<NonNull<u8>>,
    pub data_size: usize,
}

impl SimplePlugin {
    pub fn new(id: PluginID, name: &[u8], version: (u32, u32, u32), capability: PluginCapability) -> Self {
        let mut name_array = [0u8; 64];
        let len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }

        SimplePlugin {
            id,
            name: name_array,
            version,
            state: AtomicUsize::new(PluginState::Unloaded as usize),
            capability,
            data: None,
            data_size: 0,
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        let data_ptr = unsafe {
            let ptr = alloc(data.len()) as *mut u8;
            if ptr.is_null() {
                return;
            }
            core::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len());
            NonNull::new_unchecked(ptr)
        };

        if let Some(old_data) = self.data {
            unsafe {
                free(old_data.as_ptr());
            }
        }

        self.data = Some(data_ptr);
        self.data_size = data.len();
    }

    pub fn get_state(&self) -> PluginState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state(&self, state: PluginState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Plugin for SimplePlugin {
    fn id(&self) -> PluginID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn version(&self) -> (u32, u32,-u32) {
        self.version
    }

    fn initialize(&mut self) -> Result<(), PluginError> {
        if !self.capability.can_initialize {
            return Err(PluginError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == PluginState::Initialized || current_state == PluginState::Running {
            return Err(PluginError::AlreadyInitialized);
        }

        self.set_state(PluginState::Initialized);
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PluginError> {
        if !self.capability.can_shutdown {
            return Err(PluginError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == PluginState::Unloaded {
            return Err(PluginError::NotInitialized);
        }

        self.set_state(PluginState::Stopped);
        Ok(())
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            id: self.id,
            name: self.name,
            version_major: self.version.0,
            version_minor: self.version.1,
            version_patch: self.version.2,
            state: self.get_state(),
            capability: self.capability,
        }
    }
}

impl Drop for SimplePlugin {
    fn drop(&mut self) {
        unsafe {
            if let Some(data) = self.data {
                free(data.as_ptr());
            }
        }
    }
}

/// Plugin manager trait (OOP interface)
pub trait PluginManager {
    /// Load plugin
    fn load_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<PluginID, PluginError>;
    /// Unload plugin
    fn unload_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
    /// Initialize plugin
    fn initialize_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
    /// Shutdown plugin
    fn shutdown_plugin(&mut self, id: PluginID) -> Result<(), PluginError>;
    /// Get plugin
    fn get_plugin(&self, id: PluginID) -> Option<&dyn Plugin>;
    /// Get plugin mutable
    fn get_plugin_mut(&mut self, id: PluginID) -> Option<&mut Box<dyn Plugin>>;
    /// Get manager statistics
    fn stats(&self) -> PluginStats;
}

/// Plugin statistics
#[repr(C)]
pub struct PluginStats {
    pub total_plugins: usize,
    pub loaded_plugins: usize,
    pub initialized_plugins: usize,
    pub running_plugins: usize,
}

impl PluginStats {
    pub fn new() -> Self {
        PluginStats {
            total_plugins: 0,
            loaded_plugins: 0,
            initialized_plugins: 0,
            running_plugins: 0,
        }
    }
}

/// Simple plugin manager (OOP: Concrete manager class)
pub struct SimplePluginManager {
    plugins: Vec<Option<Box<dyn Plugin>>>,
    next_id: AtomicUsize,
    stats: PluginStats,
    capability: ManagerCapability,
}

/// Manager capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ManagerCapability {
    pub can_load: bool,
    pub can_unload: bool,
    pub can_initialize: bool,
    pub can_shutdown: bool,
}

impl ManagerCapability {
    pub fn new() -> Self {
        ManagerCapability {
            can_load: false,
            can_unload: false,
            can_initialize: false,
            can_shutdown: false,
        }
    }

    pub fn full() -> Self {
        ManagerCapability {
            can_load: true,
            can_unload: true,
            can_initialize: true,
            can_shutdown: true,
        }
    }
}

impl SimplePluginManager {
    pub fn new(capability: ManagerCapability) -> Self {
        SimplePluginManager {
            plugins: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: PluginStats::new(),
            capability,
        }
    }
}

impl PluginManager for SimplePluginManager {
    fn load_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<PluginID, PluginError> {
        if !self.capability.can_load {
            return Err(PluginError::PermissionDenied);
        }

        let id = plugin.id();
        self.plugins.push(Some(plugin));
        self.stats.total_plugins += 1;
        self.stats.loaded_plugins += 1;
        Ok(id)
    }

    fn unload_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        if !self.capability.can_unload {
            return Err(PluginError::PermissionDenied);
        }

        let mut index = None;
        for (i, plugin_option) in self.plugins.iter().enumerate() {
            if let Some(ref plugin) = *plugin_option {
                if plugin.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.plugins[i] = None;
            self.stats.loaded_plugins -= 1;
            Ok(())
        } else {
            Err(PluginError::InvalidState)
        }
    }

    fn initialize_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        if !self.capability.can_initialize {
            return Err(PluginError::PermissionDenied);
        }

        if let Some(ref mut plugin) = self.get_plugin_mut(id) {
            let result = plugin.initialize();
            if result.is_ok() {
                self.stats.initialized_plugins += 1;
            }
            result
        } else {
            Err(PluginError::InvalidState)
        }
    }

    fn shutdown_plugin(&mut self, id: PluginID) -> Result<(), PluginError> {
        if !self.capability.can_shutdown {
            return Err(PluginError::PermissionDenied);
        }

        if let Some(ref mut plugin) = self.get_plugin_mut(id) {
            let result = plugin.shutdown();
            if result.is_ok() {
                self.stats.initialized_plugins -= 1;
            }
            result
        } else {
            Err(PluginError::InvalidState)
        }
    }

    fn get_plugin(&self, id: PluginID) -> Option<&dyn Plugin> {
        for plugin_option in &self.plugins {
            if let Some(ref plugin) = *plugin_option {
                if plugin.id() == id {
                    return Some(plugin.as_ref());
                }
            }
        }
        None
    }

    fn get_plugin_mut(&mut self, id: PluginID) -> Option<&mut Box<dyn Plugin>> {
        for plugin_option in &mut self.plugins {
            if let Some(ref mut plugin) = *plugin_option {
                if plugin.id() == id {
                    return Some(plugin);
                }
            }
        }
        None
    }

    fn stats(&self) -> PluginStats {
        self.stats
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
