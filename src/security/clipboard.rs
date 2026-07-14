#![no_std]

/// Secure Clipboard Manager for SigmaOS
/// Based on 100-Improvement-Ideas.md #38: Secure clipboard manager
/// Implements encrypted clipboard history with security controls

use core::sync::atomic::{AtomicU64, Ordering};

/// Clipboard entry type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardEntryType {
    Text = 0,
    Image = 1,
    File = 2,
    HTML = 3,
}

/// Clipboard entry
#[repr(C)]
pub struct ClipboardEntry {
    pub id: u64,
    pub entry_type: ClipboardEntryType,
    pub data: [u8; 4096],
    pub data_size: u32,
    pub timestamp: u64,
    pub source_app: [u8; 64],
    pub is_encrypted: bool,
}

impl ClipboardEntry {
    pub fn new(id: u64, entry_type: ClipboardEntryType, source_app: &str) -> Self {
        let mut app_array = [0u8; 64];
        let app_bytes = source_app.as_bytes();
        let len = app_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(app_bytes.as_ptr(), app_array.as_mut_ptr(), len);
        }
        
        ClipboardEntry {
            id,
            entry_type,
            data: [0u8; 4096],
            data_size: 0,
            timestamp: get_current_time(),
            source_app: app_array,
            is_encrypted: false,
        }
    }
    
    pub fn set_data(&mut self, data: &[u8]) {
        let len = data.len().min(4096);
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.data.as_mut_ptr(), len);
        }
        self.data_size = len as u32;
    }
    
    pub fn get_data(&self) -> &[u8] {
        &self.data[..self.data_size as usize]
    }
}

/// Security policy
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityPolicy {
    AllowAll = 0,
    BlockSensitive = 1,
    BlockAll = 2,
}

/// Secure clipboard manager
pub struct SecureClipboardManager {
    pub history: Vec<Option<ClipboardEntry>>,
    pub current_entry: Option<ClipboardEntry>,
    pub next_entry_id: AtomicU64,
    pub max_history: usize,
    pub security_policy: SecurityPolicy,
    pub encryption_enabled: bool,
    pub encryption_key: [u8; 32],
}

impl SecureClipboardManager {
    pub fn new(max_history: usize) -> Self {
        SecureClipboardManager {
            history: Vec::new(),
            current_entry: None,
            next_entry_id: AtomicU64::new(1),
            max_history,
            security_policy: SecurityPolicy::BlockSensitive,
            encryption_enabled: true,
            encryption_key: [0u8; 32],
        }
    }
    
    /// Set clipboard content
    pub fn set_content(&mut self, entry_type: ClipboardEntryType, data: &[u8], source_app: &str) -> Result<(), ClipboardError> {
        // Check security policy
        if self.security_policy == SecurityPolicy::BlockAll {
            return Err(ClipboardError::BlockedByPolicy);
        }
        
        if self.security_policy == SecurityPolicy::BlockSensitive {
            // Check for sensitive data (passwords, tokens, etc.)
            if self.contains_sensitive_data(data) {
                return Err(ClipboardError::SensitiveDataBlocked);
            }
        }
        
        let id = self.next_entry_id.fetch_add(1, Ordering::SeqCst);
        let mut entry = ClipboardEntry::new(id, entry_type, source_app);
        
        if self.encryption_enabled {
            let mut encrypted_data = [0u8; 4096];
            for (i, &byte) in data.iter().enumerate() {
                if i < 4096 {
                    encrypted_data[i] = byte ^ self.encryption_key[i % 32];
                }
            }
            entry.set_data(&encrypted_data);
            entry.is_encrypted = true;
        } else {
            entry.set_data(data);
        }
        
        self.current_entry = Some(entry.clone());
        self.history.push(Some(entry));
        
        // Remove oldest if over limit
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
        
        Ok(())
    }
    
    /// Get current clipboard content
    pub fn get_content(&mut self) -> Option<&[u8]> {
        if let Some(ref entry) = self.current_entry {
            if entry.is_encrypted {
                let mut decrypted_data = [0u8; 4096];
                for (i, &byte) in entry.data.iter().enumerate() {
                    if i < 4096 {
                        decrypted_data[i] = byte ^ self.encryption_key[i % 32];
                    }
                }
                Some(&decrypted_data[..entry.data_size as usize])
            } else {
                Some(entry.get_data())
            }
        } else {
            None
        }
    }
    
    /// Get history entry by ID
    pub fn get_history_entry(&mut self, id: u64) -> Option<&[u8]> {
        for entry_option in &self.history {
            if let Some(ref entry) = *entry_option {
                if entry.id == id {
                    if entry.is_encrypted {
                        let mut decrypted_data = [0u8; 4096];
                        for (i, &byte) in entry.data.iter().enumerate() {
                            if i < 4096 {
                                decrypted_data[i] = byte ^ self.encryption_key[i % 32];
                            }
                        }
                        return Some(&decrypted_data[..entry.data_size as usize]);
                    } else {
                        return Some(entry.get_data());
                    }
                }
            }
        }
        None
    }
    
    /// Restore from history
    pub fn restore_from_history(&mut self, id: u64) -> Result<(), ClipboardError> {
        for entry_option in &self.history {
            if let Some(ref entry) = *entry_option {
                if entry.id == id {
                    self.current_entry = Some(entry.clone());
                    return Ok(());
                }
            }
        }
        Err(ClipboardError::EntryNotFound)
    }
    
    /// Clear history
    pub fn clear_history(&mut self) {
        self.history = Vec::new();
    }
    
    /// Clear current
    pub fn clear_current(&mut self) {
        self.current_entry = None;
    }
    
    /// Set security policy
    pub fn set_security_policy(&mut self, policy: SecurityPolicy) {
        self.security_policy = policy;
    }
    
    /// Enable/disable encryption
    pub fn set_encryption_enabled(&mut self, enabled: bool) {
        self.encryption_enabled = enabled;
    }
    
    /// Set encryption key
    pub fn set_encryption_key(&mut self, key: [u8; 32]) {
        self.encryption_key = key;
    }
    
    /// List history
    pub fn list_history(&self) -> Vec<u64> {
        let mut ids = Vec::new();
        for entry_option in &self.history {
            if let Some(ref entry) = *entry_option {
                ids.push(entry.id);
            }
        }
        ids
    }
    
    fn contains_sensitive_data(&self, data: &[u8]) -> bool {
        // Simple check for sensitive patterns
        let data_str = unsafe {
            core::str::from_utf8_unchecked(data)
        };
        
        let sensitive_patterns = ["password", "token", "secret", "key", "api_key"];
        
        for pattern in &sensitive_patterns {
            if data_str.contains(pattern) {
                return true;
            }
        }
        
        false
    }
}

/// Clipboard error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClipboardError {
    Success = 0,
    EntryNotFound = 1,
    BlockedByPolicy = 2,
    SensitiveDataBlocked = 3,
    EncryptionFailed = 4,
    DecryptionFailed = 5,
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
