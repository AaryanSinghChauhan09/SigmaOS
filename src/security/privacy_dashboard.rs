#![no_std]

/// Privacy Dashboard (Telemetry Control) for SigmaOS
/// Based on 100-Improvement-Ideas.md #37: Privacy dashboard (telemetry control)
/// Implements centralized privacy settings and telemetry management

use core::sync::atomic::{AtomicU64, Ordering};

/// Telemetry category
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TelemetryCategory {
    Usage = 0,
    Performance = 1,
    CrashReports = 2,
    Updates = 3,
    Personalization = 4,
    Location = 5,
}

/// Permission level
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionLevel {
    Denied = 0,
    Minimal = 1,
    Standard = 2,
    Full = 3,
}

/// Telemetry setting
#[repr(C)]
pub struct TelemetrySetting {
    pub category: TelemetryCategory,
    pub permission: PermissionLevel,
    pub enabled: bool,
    pub data_retention_days: u32,
}

impl TelemetrySetting {
    pub fn new(category: TelemetryCategory) -> Self {
        TelemetrySetting {
            category,
            permission: PermissionLevel::Standard,
            enabled: true,
            data_retention_days: 30,
        }
    }
}

/// Data collection entry
#[repr(C)]
pub struct DataCollectionEntry {
    pub id: u64,
    pub category: TelemetryCategory,
    pub timestamp: u64,
    pub data_size: u32,
    pub description: [u8; 128],
}

impl DataCollectionEntry {
    pub fn new(id: u64, category: TelemetryCategory, description: &str) -> Self {
        let mut desc_array = [0u8; 128];
        let desc_bytes = description.as_bytes();
        let len = desc_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(desc_bytes.as_ptr(), desc_array.as_mut_ptr(), len);
        }
        
        DataCollectionEntry {
            id,
            category,
            timestamp: get_current_time(),
            data_size: 0,
            description: desc_array,
        }
    }
}

/// Privacy dashboard
pub struct PrivacyDashboard {
    pub settings: [TelemetrySetting; 6],
    pub collection_log: Vec<Option<DataCollectionEntry>>,
    pub next_entry_id: AtomicU64,
    pub telemetry_enabled: bool,
}

impl PrivacyDashboard {
    pub fn new() -> Self {
        PrivacyDashboard {
            settings: [
                TelemetrySetting::new(TelemetryCategory::Usage),
                TelemetrySetting::new(TelemetryCategory::Performance),
                TelemetrySetting::new(TelemetryCategory::CrashReports),
                TelemetrySetting::new(TelemetryCategory::Updates),
                TelemetrySetting::new(TelemetryCategory::Personalization),
                TelemetrySetting::new(TelemetryCategory::Location),
            ],
            collection_log: Vec::new(),
            next_entry_id: AtomicU64::new(1),
            telemetry_enabled: true,
        }
    }
    
    /// Set permission for category
    pub fn set_permission(&mut self, category: TelemetryCategory, permission: PermissionLevel) {
        for setting in &mut self.settings {
            if setting.category == category {
                setting.permission = permission;
                break;
            }
        }
    }
    
    /// Enable/disable category
    pub fn set_category_enabled(&mut self, category: TelemetryCategory, enabled: bool) {
        for setting in &mut self.settings {
            if setting.category == category {
                setting.enabled = enabled;
                break;
            }
        }
    }
    
    /// Set data retention days
    pub fn set_retention_days(&mut self, category: TelemetryCategory, days: u32) {
        for setting in &mut self.settings {
            if setting.category == category {
                setting.data_retention_days = days;
                break;
            }
        }
    }
    
    /// Get setting for category
    pub fn get_setting(&self, category: TelemetryCategory) -> Option<&TelemetrySetting> {
        for setting in &self.settings {
            if setting.category == category {
                return Some(setting);
            }
        }
        None
    }
    
    /// Enable/disable all telemetry
    pub fn set_telemetry_enabled(&mut self, enabled: bool) {
        self.telemetry_enabled = enabled;
    }
    
    /// Log data collection
    pub fn log_collection(&mut self, category: TelemetryCategory, description: &str) {
        if !self.telemetry_enabled {
            return;
        }
        
        let setting = self.get_setting(category);
        if let Some(setting) = setting {
            if !setting.enabled || setting.permission == PermissionLevel::Denied {
                return;
            }
        }
        
        let id = self.next_entry_id.fetch_add(1, Ordering::SeqCst);
        let entry = DataCollectionEntry::new(id, category, description);
        self.collection_log.push(Some(entry));
    }
    
    /// Get collection log
    pub fn get_collection_log(&self) -> Vec<&DataCollectionEntry> {
        let mut entries = Vec::new();
        for entry_option in &self.collection_log {
            if let Some(ref entry) = *entry_option {
                entries.push(entry);
            }
        }
        entries
    }
    
    /// Clear collection log
    pub fn clear_log(&mut self) {
        self.collection_log = Vec::new();
    }
    
    /// Export privacy report
    pub fn export_report(&self) -> PrivacyReport {
        let mut report = PrivacyReport::new();
        
        for setting in &self.settings {
            report.categories.push(setting.category);
            report.permissions.push(setting.permission);
            report.enabled.push(setting.enabled);
        }
        
        report.total_entries = self.collection_log.len() as u32;
        report.telemetry_enabled = self.telemetry_enabled;
        
        report
    }
    
    /// Initialize default settings
    pub fn initialize_defaults(&mut self) {
        self.set_permission(TelemetryCategory::Usage, PermissionLevel::Minimal);
        self.set_permission(TelemetryCategory::Performance, PermissionLevel::Standard);
        self.set_permission(TelemetryCategory::CrashReports, PermissionLevel::Standard);
        self.set_permission(TelemetryCategory::Updates, PermissionLevel::Standard);
        self.set_permission(TelemetryCategory::Personalization, PermissionLevel::Denied);
        self.set_permission(TelemetryCategory::Location, PermissionLevel::Denied);
        
        self.set_retention_days(TelemetryCategory::Usage, 7);
        self.set_retention_days(TelemetryCategory::Performance, 14);
        self.set_retention_days(TelemetryCategory::CrashReports, 30);
        self.set_retention_days(TelemetryCategory::Updates, 90);
    }
}

/// Privacy report
pub struct PrivacyReport {
    pub categories: Vec<TelemetryCategory>,
    pub permissions: Vec<PermissionLevel>,
    pub enabled: Vec<bool>,
    pub total_entries: u32,
    pub telemetry_enabled: bool,
}

impl PrivacyReport {
    pub fn new() -> Self {
        PrivacyReport {
            categories: Vec::new(),
            permissions: Vec::new(),
            enabled: Vec::new(),
            total_entries: 0,
            telemetry_enabled: false,
        }
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

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
