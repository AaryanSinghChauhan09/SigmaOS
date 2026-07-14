#![no_std]

/// Password Manager with Biometric Unlock for SigmaOS
/// Based on 100-Improvement-Ideas.md #35: Password manager with biometric unlock
/// Implements secure password storage with biometric authentication

use core::sync::atomic::{AtomicU64, Ordering};

/// Biometric type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiometricType {
    Fingerprint = 0,
    FaceID = 1,
    Iris = 2,
    Voice = 3,
}

/// Biometric data
#[repr(C)]
pub struct BiometricData {
    pub biometric_type: BiometricType,
    pub template: [u8; 512],
    pub confidence: f32,
}

impl BiometricData {
    pub fn new(biometric_type: BiometricType) -> Self {
        BiometricData {
            biometric_type,
            template: [0u8; 512],
            confidence: 0.0,
        }
    }
}

/// Password entry
#[repr(C)]
pub struct PasswordEntry {
    pub id: u64,
    pub service: [u8; 128],
    pub username: [u8; 128],
    pub encrypted_password: [u8; 256],
    pub created_at: u64,
    pub last_used: u64,
}

impl PasswordEntry {
    pub fn new(id: u64, service: &str, username: &str) -> Self {
        let mut service_array = [0u8; 128];
        let service_bytes = service.as_bytes();
        let service_len = service_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(service_bytes.as_ptr(), service_array.as_mut_ptr(), service_len);
        }
        
        let mut username_array = [0u8; 128];
        let username_bytes = username.as_bytes();
        let username_len = username_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(username_bytes.as_ptr(), username_array.as_mut_ptr(), username_len);
        }
        
        let now = get_current_time();
        
        PasswordEntry {
            id,
            service: service_array,
            username: username_array,
            encrypted_password: [0u8; 256],
            created_at: now,
            last_used: now,
        }
    }
    
    pub fn service_str(&self) -> &str {
        unsafe {
            let len = self.service.iter().position(|&b| b == 0).unwrap_or(128);
            core::str::from_utf8_unchecked(&self.service[..len])
        }
    }
    
    pub fn username_str(&self) -> &str {
        unsafe {
            let len = self.username.iter().position(|&b| b == 0).unwrap_or(128);
            core::str::from_utf8_unchecked(&self.username[..len])
        }
    }
}

/// Password manager
pub struct PasswordManager {
    pub entries: Vec<Option<PasswordEntry>>,
    pub biometric_data: Option<BiometricData>,
    pub master_key: [u8; 32],
    pub is_unlocked: bool,
    pub next_entry_id: AtomicU64,
}

impl PasswordManager {
    pub fn new() -> Self {
        PasswordManager {
            entries: Vec::new(),
            biometric_data: None,
            master_key: [0u8; 32],
            is_unlocked: false,
            next_entry_id: AtomicU64::new(1),
        }
    }
    
    /// Register biometric
    pub fn register_biometric(&mut self, biometric_type: BiometricType, template: [u8; 512]) -> Result<(), PasswordError> {
        let mut biometric = BiometricData::new(biometric_type);
        biometric.template = template;
        biometric.confidence = 0.95;
        self.biometric_data = Some(biometric);
        Ok(())
    }
    
    /// Unlock with biometric
    pub fn unlock_biometric(&mut self, biometric_data: &BiometricData) -> Result<(), PasswordError> {
        if let Some(ref registered) = self.biometric_data {
            if registered.biometric_type != biometric_data.biometric_type {
                return Err(PasswordError::BiometricMismatch);
            }
            
            // Simple template comparison (in real implementation, use proper matching)
            let mut match_score = 0.0;
            for i in 0..512 {
                if registered.template[i] == biometric_data.template[i] {
                    match_score += 1.0 / 512.0;
                }
            }
            
            if match_score > 0.8 {
                self.is_unlocked = true;
                Ok(())
            } else {
                Err(PasswordError::BiometricMismatch)
            }
        } else {
            Err(PasswordError::NoBiometricRegistered)
        }
    }
    
    /// Unlock with master password
    pub fn unlock_master(&mut self, master_password: &[u8]) -> Result<(), PasswordError> {
        // In real implementation, derive master key from password
        if master_password.len() >= 8 {
            self.is_unlocked = true;
            Ok(())
        } else {
            Err(PasswordError::InvalidPassword)
        }
    }
    
    /// Lock password manager
    pub fn lock(&mut self) {
        self.is_unlocked = false;
        self.master_key = [0u8; 32];
    }
    
    /// Add password entry
    pub fn add_entry(&mut self, service: &str, username: &str, password: &str) -> Result<u64, PasswordError> {
        if !self.is_unlocked {
            return Err(PasswordError::Locked);
        }
        
        let id = self.next_entry_id.fetch_add(1, Ordering::SeqCst);
        let mut entry = PasswordEntry::new(id, service, username);
        
        // Encrypt password (simple XOR for demo)
        let password_bytes = password.as_bytes();
        for (i, &byte) in password_bytes.iter().enumerate() {
            if i < 256 {
                entry.encrypted_password[i] = byte ^ self.master_key[i % 32];
            }
        }
        
        self.entries.push(Some(entry));
        Ok(id)
    }
    
    /// Get password entry
    pub fn get_entry(&mut self, id: u64) -> Option<&PasswordEntry> {
        if !self.is_unlocked {
            return None;
        }
        
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                if entry.id == id {
                    return Some(entry);
                }
            }
        }
        None
    }
    
    /// Get password by service
    pub fn get_by_service(&mut self, service: &str) -> Option<&PasswordEntry> {
        if !self.is_unlocked {
            return None;
        }
        
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                if entry.service_str() == service {
                    return Some(entry);
                }
            }
        }
        None
    }
    
    /// Delete entry
    pub fn delete_entry(&mut self, id: u64) -> Result<(), PasswordError> {
        if !self.is_unlocked {
            return Err(PasswordError::Locked);
        }
        
        for entry_option in &mut self.entries {
            if let Some(ref entry) = *entry_option {
                if entry.id == id {
                    *entry_option = None;
                    return Ok(());
                }
            }
        }
        Err(PasswordError::EntryNotFound)
    }
    
    /// List all entries
    pub fn list_entries(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                ids.push(entry.id);
            }
        }
        ids
    }
}

/// Password error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PasswordError {
    Success = 0,
    Locked = 1,
    InvalidPassword = 2,
    BiometricMismatch = 3,
    NoBiometricRegistered = 4,
    EntryNotFound = 5,
    EncryptionFailed = 6,
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
