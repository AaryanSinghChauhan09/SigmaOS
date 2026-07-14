#![no_std]

/// Adaptive Profiles for SigmaOS
/// Based on 100-Improvement-Ideas.md #42: Adaptive profiles (developer, gamer, minimalist)
/// Implements system profiles that adapt to user context and usage patterns

use core::sync::atomic::{AtomicU64, Ordering};

/// Profile types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileType {
    Developer = 0,
    Gamer = 1,
    Minimalist = 2,
    Productivity = 3,
    Media = 4,
    Custom = 5,
}

/// Profile state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileState {
    Inactive = 0,
    Active = 1,
    Transitioning = 2,
}

/// Power setting
#[repr(C)]
pub struct PowerSetting {
    pub cpu_performance: u8, // 0-100
    pub gpu_performance: u8, // 0-100
    pub screen_brightness: u8, // 0-100
    pub sleep_timeout: u32, // seconds
}

impl PowerSetting {
    pub fn new() -> Self {
        PowerSetting {
            cpu_performance: 50,
            gpu_performance: 50,
            screen_brightness: 80,
            sleep_timeout: 300,
        }
    }
}

/// Display setting
#[repr(C)]
pub struct DisplaySetting {
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub refresh_rate: u32,
    pub scaling: u8, // 100-200
}

impl DisplaySetting {
    pub fn new() -> Self {
        DisplaySetting {
            resolution_width: 1920,
            resolution_height: 1080,
            refresh_rate: 60,
            scaling: 100,
        }
    }
}

/// Audio setting
#[repr(C)]
pub struct AudioSetting {
    pub master_volume: u8, // 0-100
    pub notification_volume: u8, // 0-100
    pub mute_notifications: bool,
}

impl AudioSetting {
    pub fn new() -> Self {
        AudioSetting {
            master_volume: 80,
            notification_volume: 50,
            mute_notifications: false,
        }
    }
}

/// System profile
pub struct SystemProfile {
    pub id: u64,
    pub profile_type: ProfileType,
    pub name: [u8; 64],
    pub state: ProfileState,
    pub power: PowerSetting,
    pub display: DisplaySetting,
    pub audio: AudioSetting,
    pub enabled_apps: Vec<[u8; 128]>,
    pub disabled_services: Vec<[u8; 128]>,
}

impl SystemProfile {
    pub fn new(id: u64, profile_type: ProfileType, name: &str) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        let (power, display, audio) = match profile_type {
            ProfileType::Developer => (
                PowerSetting { cpu_performance: 100, gpu_performance: 50, screen_brightness: 90, sleep_timeout: 0 },
                DisplaySetting { resolution_width: 2560, resolution_height: 1440, refresh_rate: 144, scaling: 100 },
                AudioSetting { master_volume: 60, notification_volume: 30, mute_notifications: true },
            ),
            ProfileType::Gamer => (
                PowerSetting { cpu_performance: 100, gpu_performance: 100, screen_brightness: 100, sleep_timeout: 0 },
                DisplaySetting { resolution_width: 1920, resolution_height: 1080, refresh_rate: 144, scaling: 100 },
                AudioSetting { master_volume: 100, notification_volume: 20, mute_notifications: true },
            ),
            ProfileType::Minimalist => (
                PowerSetting { cpu_performance: 30, gpu_performance: 30, screen_brightness: 70, sleep_timeout: 600 },
                DisplaySetting { resolution_width: 1920, resolution_height: 1080, refresh_rate: 60, scaling: 100 },
                AudioSetting { master_volume: 50, notification_volume: 20, mute_notifications: false },
            ),
            _ => (PowerSetting::new(), DisplaySetting::new(), AudioSetting::new()),
        };
        
        SystemProfile {
            id,
            profile_type,
            name: name_array,
            state: ProfileState::Inactive,
            power,
            display,
            audio,
            enabled_apps: Vec::new(),
            disabled_services: Vec::new(),
        }
    }
    
    pub fn name_str(&self) -> &str {
        unsafe {
            let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
            core::str::from_utf8_unchecked(&self.name[..len])
        }
    }
    
    pub fn activate(&mut self) {
        self.state = ProfileState::Active;
    }
    
    pub fn deactivate(&mut self) {
        self.state = ProfileState::Inactive;
    }
    
    pub fn add_enabled_app(&mut self, app_name: &str) {
        let mut app_array = [0u8; 128];
        let app_bytes = app_name.as_bytes();
        let len = app_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(app_bytes.as_ptr(), app_array.as_mut_ptr(), len);
        }
        
        self.enabled_apps.push(app_array);
    }
    
    pub fn add_disabled_service(&mut self, service_name: &str) {
        let mut service_array = [0u8; 128];
        let service_bytes = service_name.as_bytes();
        let len = service_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(service_bytes.as_ptr(), service_array.as_mut_ptr(), len);
        }
        
        self.disabled_services.push(service_array);
    }
}

/// Adaptive profile manager
pub struct AdaptiveProfileManager {
    pub profiles: Vec<Option<SystemProfile>>,
    pub current_profile: AtomicU64,
    pub next_profile_id: AtomicU64,
    pub auto_switch_enabled: bool,
}

impl AdaptiveProfileManager {
    pub fn new() -> Self {
        AdaptiveProfileManager {
            profiles: Vec::new(),
            current_profile: AtomicU64::new(0),
            next_profile_id: AtomicU64::new(1),
            auto_switch_enabled: false,
        }
    }
    
    /// Create default profiles
    pub fn create_default_profiles(&mut self) {
        let dev_id = self.next_profile_id.fetch_add(1, Ordering::SeqCst);
        let dev_profile = SystemProfile::new(dev_id, ProfileType::Developer, "Developer");
        self.profiles.push(Some(dev_profile));
        
        let gamer_id = self.next_profile_id.fetch_add(1, Ordering::SeqCst);
        let gamer_profile = SystemProfile::new(gamer_id, ProfileType::Gamer, "Gamer");
        self.profiles.push(Some(gamer_profile));
        
        let minimal_id = self.next_profile_id.fetch_add(1, Ordering::SeqCst);
        let minimal_profile = SystemProfile::new(minimal_id, ProfileType::Minimalist, "Minimalist");
        self.profiles.push(Some(minimal_profile));
    }
    
    /// Create custom profile
    pub fn create_profile(&mut self, profile_type: ProfileType, name: &str) -> u64 {
        let id = self.next_profile_id.fetch_add(1, Ordering::SeqCst);
        let profile = SystemProfile::new(id, profile_type, name);
        self.profiles.push(Some(profile));
        id
    }
    
    /// Switch to profile
    pub fn switch_profile(&mut self, profile_id: u64) -> Result<(), ProfileError> {
        // Deactivate current profile
        let current_id = self.current_profile.load(Ordering::SeqCst);
        if current_id > 0 {
            for profile_option in &mut self.profiles {
                if let Some(ref mut profile) = *profile_option {
                    if profile.id == current_id {
                        profile.deactivate();
                    }
                }
            }
        }
        
        // Activate new profile
        for profile_option in &mut self.profiles {
            if let Some(ref mut profile) = *profile_option {
                if profile.id == profile_id {
                    profile.activate();
                    self.current_profile.store(profile_id, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        
        Err(ProfileError::ProfileNotFound)
    }
    
    /// Get current profile
    pub fn get_current_profile(&self) -> Option<&SystemProfile> {
        let current_id = self.current_profile.load(Ordering::SeqCst);
        if current_id == 0 {
            return None;
        }
        
        for profile_option in &self.profiles {
            if let Some(ref profile) = *profile_option {
                if profile.id == current_id {
                    return Some(profile);
                }
            }
        }
        None
    }
    
    /// Auto-detect and switch profile based on context
    pub fn auto_detect_profile(&mut self) -> Option<u64> {
        if !self.auto_switch_enabled {
            return None;
        }
        
        // Simple heuristic-based profile detection
        // In real implementation, use ML or more sophisticated detection
        let detected_profile = self.detect_running_apps();
        
        if let Some(profile_id) = detected_profile {
            let _ = self.switch_profile(profile_id);
            Some(profile_id)
        } else {
            None
        }
    }
    
    fn detect_running_apps(&self) -> Option<u64> {
        // Placeholder for app detection
        // In real implementation, check running processes
        None
    }
}

/// Profile error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProfileError {
    Success = 0,
    ProfileNotFound = 1,
    ProfileActive = 2,
    SwitchFailed = 3,
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
