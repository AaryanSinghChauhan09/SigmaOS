#![no_std]

/// Unified Control Center for SigmaOS
/// Based on 100-Improvement-Ideas.md #43: Unified control center
/// Implements centralized system settings and controls

use core::sync::atomic::{AtomicU64, Ordering};

/// Control category
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlCategory {
    Network = 0,
    Bluetooth = 1,
    Audio = 2,
    Display = 3,
    Power = 4,
    Storage = 5,
    Security = 6,
    Accessibility = 7,
}

/// Toggle state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleState {
    Off = 0,
    On = 1,
}

/// Control item
#[repr(C)]
pub struct ControlItem {
    pub id: u64,
    pub category: ControlCategory,
    pub name: [u8; 64],
    pub icon: [u8; 32],
    pub state: ToggleState,
    pub enabled: bool,
}

impl ControlItem {
    pub fn new(id: u64, category: ControlCategory, name: &str) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        ControlItem {
            id,
            category,
            name: name_array,
            icon: [0u8; 32],
            state: ToggleState::Off,
            enabled: true,
        }
    }
    
    pub fn name_str(&self) -> &str {
        unsafe {
            let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
            core::str::from_utf8_unchecked(&self.name[..len])
        }
    }
    
    pub fn toggle(&mut self) {
        if self.enabled {
            self.state = match self.state {
                ToggleState::Off => ToggleState::On,
                ToggleState::On => ToggleState::Off,
            };
        }
    }
    
    pub fn set_state(&mut self, state: ToggleState) {
        if self.enabled {
            self.state = state;
        }
    }
}

/// Quick setting
#[repr(C)]
pub struct QuickSetting {
    pub id: u64,
    pub name: [u8; 64],
    pub value: [u8; 128],
    pub control_type: u8, // 0=slider, 1=toggle, 2=dropdown
}

impl QuickSetting {
    pub fn new(id: u64, name: &str, control_type: u8) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        QuickSetting {
            id,
            name: name_array,
            value: [0u8; 128],
            control_type,
        }
    }
    
    pub fn name_str(&self) -> &str {
        unsafe {
            let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
            core::str::from_utf8_unchecked(&self.name[..len])
        }
    }
    
    pub fn set_value(&mut self, value: &str) {
        let value_bytes = value.as_bytes();
        let len = value_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(value_bytes.as_ptr(), self.value.as_mut_ptr(), len);
        }
    }
    
    pub fn value_str(&self) -> &str {
        unsafe {
            let len = self.value.iter().position(|&b| b == 0).unwrap_or(128);
            core::str::from_utf8_unchecked(&self.value[..len])
        }
    }
}

/// Unified Control Center
pub struct UnifiedControlCenter {
    pub controls: Vec<Option<ControlItem>>,
    pub quick_settings: Vec<Option<QuickSetting>>,
    pub next_control_id: AtomicU64,
    pub next_setting_id: AtomicU64,
}

impl UnifiedControlCenter {
    pub fn new() -> Self {
        UnifiedControlCenter {
            controls: Vec::new(),
            quick_settings: Vec::new(),
            next_control_id: AtomicU64::new(1),
            next_setting_id: AtomicU64::new(1),
        }
    }
    
    /// Add control item
    pub fn add_control(&mut self, category: ControlCategory, name: &str) -> u64 {
        let id = self.next_control_id.fetch_add(1, Ordering::SeqCst);
        let control = ControlItem::new(id, category, name);
        self.controls.push(Some(control));
        id
    }
    
    /// Add quick setting
    pub fn add_quick_setting(&mut self, name: &str, control_type: u8) -> u64 {
        let id = self.next_setting_id.fetch_add(1, Ordering::SeqCst);
        let setting = QuickSetting::new(id, name, control_type);
        self.quick_settings.push(Some(setting));
        id
    }
    
    /// Toggle control
    pub fn toggle_control(&mut self, id: u64) -> Result<(), ControlError> {
        for control_option in &mut self.controls {
            if let Some(ref mut control) = *control_option {
                if control.id == id {
                    control.toggle();
                    return Ok(());
                }
            }
        }
        Err(ControlError::ControlNotFound)
    }
    
    /// Set control state
    pub fn set_control_state(&mut self, id: u64, state: ToggleState) -> Result<(), ControlError> {
        for control_option in &mut self.controls {
            if let Some(ref mut control) = *control_option {
                if control.id == id {
                    control.set_state(state);
                    return Ok(());
                }
            }
        }
        Err(ControlError::ControlNotFound)
    }
    
    /// Get controls by category
    pub fn get_controls_by_category(&self, category: ControlCategory) -> Vec<&ControlItem> {
        let mut result = Vec::new();
        for control_option in &self.controls {
            if let Some(ref control) = *control_option {
                if control.category == category {
                    result.push(control);
                }
            }
        }
        result
    }
    
    /// Get quick setting
    pub fn get_quick_setting(&mut self, id: u64) -> Option<&mut QuickSetting> {
        for setting_option in &mut self.quick_settings {
            if let Some(ref mut setting) = *setting_option {
                if setting.id == id {
                    return Some(setting);
                }
            }
        }
        None
    }
    
    /// Initialize default controls
    pub fn initialize_defaults(&mut self) {
        // Network controls
        self.add_control(ControlCategory::Network, "Wi-Fi");
        self.add_control(ControlCategory::Network, "Ethernet");
        self.add_control(ControlCategory::Network, "Airplane Mode");
        
        // Bluetooth
        self.add_control(ControlCategory::Bluetooth, "Bluetooth");
        
        // Audio
        self.add_control(ControlCategory::Audio, "Mute");
        
        // Display
        self.add_control(ControlCategory::Display, "Night Light");
        self.add_control(ControlCategory::Display, "Do Not Disturb");
        
        // Power
        self.add_control(ControlCategory::Power, "Battery Saver");
        
        // Quick settings
        self.add_quick_setting("Brightness", 0); // slider
        self.add_quick_setting("Volume", 0); // slider
        self.add_quick_setting("Screen Rotation", 2); // dropdown
    }
}

/// Control error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ControlError {
    Success = 0,
    ControlNotFound = 1,
    SettingNotFound = 2,
    InvalidState = 3,
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
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

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
