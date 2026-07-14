#![no_std]
#![no_main]

/// OOP-based Configuration Loader for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 201
/// Implements system configuration management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ConfigID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ConfigType { String = 0, Integer = 1, Boolean = 2, Float = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ConfigError { Success = 0, NotFound = 1, InvalidType = 2 }

pub trait ConfigValue {
    fn id(&self) -> ConfigID;
    fn key(&self) -> &[u8];
    fn config_type(&self) -> ConfigType;
    fn as_string(&self) -> &[u8];
    fn as_integer(&self) -> i64;
    fn as_boolean(&self) -> bool;
    fn as_float(&self) -> f64;
}

#[repr(C)]
pub struct SimpleConfigValue {
    pub id: ConfigID,
    pub key: [u8; 128],
    pub config_type: AtomicUsize,
    pub string_value: [u8; 256],
    pub int_value: AtomicUsize,
    pub bool_value: AtomicUsize,
}

impl SimpleConfigValue {
    pub fn new(id: ConfigID, key: &[u8], config_type: ConfigType) -> Self {
        let mut key_array = [0u8; 128];
        let key_len = key.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(key.as_ptr(), key_array.as_mut_ptr(), key_len);
        }
        SimpleConfigValue {
            id,
            key: key_array,
            config_type: AtomicUsize::new(config_type as usize),
            string_value: [0u8; 256],
            int_value: AtomicUsize::new(0),
            bool_value: AtomicUsize::new(0),
        }
    }
}

impl ConfigValue for SimpleConfigValue {
    fn id(&self) -> ConfigID { self.id }
    fn key(&self) -> &[u8] {
        let len = self.key.iter().position(|&b| b == 0).unwrap_or(128);
        &self.key[..len]
    }
    fn config_type(&self) -> ConfigType { unsafe { core::mem::transmute(self.config_type.load(Ordering::SeqCst)) } }
    fn as_string(&self) -> &[u8] {
        let len = self.string_value.iter().position(|&b| b == 0).unwrap_or(256);
        &self.string_value[..len]
    }
    fn as_integer(&self) -> i64 { self.int_value.load(Ordering::SeqCst) as i64 }
    fn as_boolean(&self) -> bool { self.bool_value.load(Ordering::SeqCst) == 1 }
    fn as_float(&self) -> f64 { self.int_value.load(Ordering::SeqCst) as f64 / 1000.0 }
}

pub trait ConfigLoader {
    fn load_config(&mut self, config: Box<dyn ConfigValue>) -> Result<ConfigID, ConfigError>;
    fn get_config(&self, key: &[u8]) -> Option<&dyn ConfigValue>;
    fn set_config(&mut self, key: &[u8], value: &[u8]) -> Result<(), ConfigError>;
    fn save_config(&self) -> Result<(), ConfigError>;
}

#[repr(C)]
pub struct SimpleConfigLoader {
    pub configs: Vec<Option<Box<dyn ConfigValue>>>,
    pub next_id: AtomicUsize,
}

impl SimpleConfigLoader {
    pub fn new() -> Self {
        SimpleConfigLoader {
            configs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ConfigLoader for SimpleConfigLoader {
    fn load_config(&mut self, config: Box<dyn ConfigValue>) -> Result<ConfigID, ConfigError> {
        let id = config.id();
        self.configs.push(Some(config));
        Ok(id)
    }

    fn get_config(&self, key: &[u8]) -> Option<&dyn ConfigValue> {
        for config_option in &self.configs {
            if let Some(ref config) = *config_option {
                if config.key() == key { return Some(config.as_ref()); }
            }
        }
        None
    }

    fn set_config(&mut self, key: &[u8], value: &[u8]) -> Result<(), ConfigError> {
        for config_option in &mut self.configs {
            if let Some(ref mut config) = *config_option {
                if config.key() == key {
                    return Ok(());
                }
            }
        }
        Err(ConfigError::NotFound)
    }

    fn save_config(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

pub trait ConfigWatcher {
    fn watch_key(&mut self, key: &[u8], callback: fn());
    fn unwatch_key(&mut self, key: &[u8]);
    fn notify_change(&mut self, key: &[u8]);
}

#[repr(C)]
pub struct SimpleConfigWatcher {
    pub watchers: Vec<([u8; 128], fn())>,
}

impl SimpleConfigWatcher {
    pub fn new() -> Self {
        SimpleConfigWatcher {
            watchers: Vec::new(),
        }
    }
}

impl ConfigWatcher for SimpleConfigWatcher {
    fn watch_key(&mut self, key: &[u8], callback: fn()) {
        let mut key_array = [0u8; 128];
        let key_len = key.len().min(127);
        for i in 0..key_len {
            key_array[i] = key[i];
        }
        self.watchers.push((key_array, callback));
    }

    fn unwatch_key(&mut self, key: &[u8]) {
        for i in 0..self.watchers.len() {
            let len = self.watchers[i].0.iter().position(|&b| b == 0).unwrap_or(128);
            if &self.watchers[i].0[..len] == key {
                self.watchers.remove(i);
                return;
            }
        }
    }

    fn notify_change(&mut self, key: &[u8]) {
        for &(ref k, callback) in &self.watchers {
            let len = k.iter().position(|&b| b == 0).unwrap_or(128);
            if &k[..len] == key {
                callback();
            }
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
