#![no_std]
#![no_main]

/// OOP-based Desktop Settings for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 776
/// Implements desktop settings and preferences

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SettingID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SettingType { String = 0, Integer = 1, Boolean = 2, Color = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SettingsError { Success = 0, NotFound = 1, InvalidType = 2 }

pub trait Setting {
    fn id(&self) -> SettingID;
    fn key(&self) -> &[u8];
    fn setting_type(&self) -> SettingType;
    fn value(&self) -> &[u8];
    fn set_value(&mut self, value: &[u8]);
}

#[repr(C)]
pub struct SimpleSetting {
    pub id: SettingID,
    pub key: [u8; 128],
    pub setting_type: AtomicUsize,
    pub value: [u8; 256],
}

impl SimpleSetting {
    pub fn new(id: SettingID, key: &[u8], setting_type: SettingType, value: &[u8]) -> Self {
        let mut key_array = [0u8; 128];
        let mut value_array = [0u8; 256];
        let key_len = key.len().min(127);
        let value_len = value.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(key.as_ptr(), key_array.as_mut_ptr(), key_len);
            core::ptr::copy_nonoverlapping(value.as_ptr(), value_array.as_mut_ptr(), value_len);
        }
        SimpleSetting {
            id,
            key: key_array,
            setting_type: AtomicUsize::new(setting_type as usize),
            value: value_array,
        }
    }
}

impl Setting for SimpleSetting {
    fn id(&self) -> SettingID { self.id }
    fn key(&self) -> &[u8] {
        let len = self.key.iter().position(|&b| b == 0).unwrap_or(128);
        &self.key[..len]
    }
    fn setting_type(&self) -> SettingType { unsafe { core::mem::transmute(self.setting_type.load(Ordering::SeqCst)) } }
    fn value(&self) -> &[u8] {
        let len = self.value.iter().position(|&b| b == 0).unwrap_or(256);
        &self.value[..len]
    }
    
    fn set_value(&mut self, value: &[u8]) {
        let value_len = value.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(value.as_ptr(), self.value.as_mut_ptr(), value_len);
        }
    }
}

pub trait SettingsManager {
    fn get_setting(&self, key: &[u8]) -> Option<&dyn Setting>;
    fn set_setting(&mut self, key: &[u8], value: &[u8]) -> Result<(), SettingsError>;
    fn reset_default(&mut self, key: &[u8]) -> Result<(), SettingsError>;
    def save_settings(&self) -> Result<(), SettingsError>;
}

#[repr(C)]
pub struct SimpleSettingsManager {
    pub settings: Vec<Option<Box<dyn Setting>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSettingsManager {
    pub fn new() -> Self {
        SimpleSettingsManager {
            settings: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SettingsManager for SimpleSettingsManager {
    fn get_setting(&self, key: &[u8]) -> Option<&dyn Setting> {
        for setting_option in &self.settings {
            if let Some(ref setting) = *setting_option {
                if setting.key() == key { return Some(setting.as_ref()); }
            }
        }
        None
    }
    
    fn set_setting(&mut self, key: &[u8], value: &[u8]) -> Result<(), SettingsError> {
        for setting_option in &mut self.settings {
            if let Some(ref mut setting) = *setting_option {
                if setting.key() == key {
                    setting.set_value(value);
                    return Ok(());
                }
            }
        }
        Err(SettingsError::NotFound)
    }
    
    fn reset_default(&mut self, key: &[u8]) -> Result<(), SettingsError> {
        for setting_option in &mut self.settings {
            if let Some(ref mut setting) = *setting_option {
                if setting.key() == key {
                    setting.set_value(b"default");
                    return Ok(());
                }
            }
        }
        Err(SettingsError::NotFound)
    }
    
    fn save_settings(&self) -> Result<(), SettingsError> {
        Ok(())
    }
}

pub trait SettingsCategory {
    fn get_category(&self, category: &[u8]) -> Vec<&dyn Setting>;
    def add_to_category(&mut self, category: &[u8], setting: Box<dyn Setting>);
}

#[repr(C)]
pub struct SimpleSettingsCategory {
    pub categories: Vec<([u8; 64], Vec<SettingID>)>,
    pub manager: SimpleSettingsManager,
}

impl SimpleSettingsCategory {
    pub fn new(manager: SimpleSettingsManager) -> Self {
        SimpleSettingsCategory {
            categories: Vec::new(),
            manager,
        }
    }
}

impl SettingsCategory for SimpleSettingsCategory {
    fn get_category(&self, category: &[u8]) -> Vec<&dyn Setting> {
        let mut results = Vec::new();
        for &(ref cat, ref ids) in &self.categories {
            let cat_len = cat.iter().position(|&b| b == 0).unwrap_or(64);
            if &cat[..cat_len] == category {
                for &id in ids {
                    if let Some(setting) = self.manager.get_setting(b"") {
                    }
                }
            }
        }
        results
    }
    
    fn add_to_category(&mut self, category: &[u8], setting: Box<dyn Setting>) {
        let id = setting.id();
        self.manager.settings.push(Some(setting));
        
        let mut cat_array = [0u8; 64];
        let cat_len = category.len().min(63);
        for i in 0..cat_len {
            cat_array[i] = category[i];
        }
        
        let mut found = false;
        for &mut (ref cat, ref mut ids) in &mut self.categories {
            let cat_len = cat.iter().position(|&b| b == 0).unwrap_or(64);
            if &cat[..cat_len] == category {
                ids.push(id);
                found = true;
                break;
            }
        }
        if !found {
            let mut ids = Vec::new();
            ids.push(id);
            self.categories.push((cat_array, ids));
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
