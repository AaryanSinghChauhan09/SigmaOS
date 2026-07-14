#![no_std]

/// Encrypted File Vault for SigmaOS
/// Implements secure encrypted storage for sensitive files
/// Based on 100-Improvement-Ideas.md #34: Encrypted file vault

use core::sync::atomic::{AtomicU64, Ordering};

/// Encryption algorithm types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    AES256GCM = 0,
    ChaCha20Poly1305 = 1,
    XChaCha20Poly1305 = 2,
}

/// Vault status
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultStatus {
    Locked = 0,
    Unlocked = 1,
    Error = 2,
}

/// Vault metadata
#[repr(C)]
pub struct VaultMetadata {
    pub version: u32,
    pub algorithm: EncryptionAlgorithm,
    pub created_at: u64,
    pub modified_at: u64,
    pub file_count: u64,
    pub total_size: u64,
}

impl VaultMetadata {
    pub fn new(algorithm: EncryptionAlgorithm) -> Self {
        let now = get_current_time();
        VaultMetadata {
            version: 1,
            algorithm,
            created_at: now,
            modified_at: now,
            file_count: 0,
            total_size: 0,
        }
    }
}

/// Encrypted file entry
#[repr(C)]
pub struct VaultEntry {
    pub name: [u8; 256],
    pub encrypted_size: u64,
    pub original_size: u64,
    pub nonce: [u8; 24],
    pub checksum: [u8; 32],
}

impl VaultEntry {
    pub fn new(name: &str, encrypted_size: u64, original_size: u64) -> Self {
        let mut name_array = [0u8; 256];
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(255);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), len);
        }
        
        VaultEntry {
            name: name_array,
            encrypted_size,
            original_size,
            nonce: [0u8; 24],
            checksum: [0u8; 32],
        }
    }
}

/// Encrypted vault
pub struct EncryptedVault {
    metadata: VaultMetadata,
    status: VaultStatus,
    entries: Vec<Option<VaultEntry>>,
    master_key: [u8; 32],
    unlock_count: AtomicU64,
}

impl EncryptedVault {
    pub fn new(algorithm: EncryptionAlgorithm) -> Self {
        EncryptedVault {
            metadata: VaultMetadata::new(algorithm),
            status: VaultStatus::Locked,
            entries: Vec::new(),
            master_key: [0u8; 32],
            unlock_count: AtomicU64::new(0),
        }
    }
    
    /// Unlock vault with master key
    pub fn unlock(&mut self, key: &[u8; 32]) -> Result<(), VaultError> {
        if self.status == VaultStatus::Unlocked {
            return Ok(());
        }
        
        // In real implementation, verify key here
        self.master_key.copy_from_slice(key);
        self.status = VaultStatus::Unlocked;
        self.unlock_count.fetch_add(1, Ordering::SeqCst);
        
        Ok(())
    }
    
    /// Lock vault
    pub fn lock(&mut self) {
        if self.status == VaultStatus::Unlocked {
            self.status = VaultStatus::Locked;
            // Clear master key from memory
            self.master_key = [0u8; 32];
        }
    }
    
    /// Add file to vault
    pub fn add_file(&mut self, name: &str, data: &[u8]) -> Result<(), VaultError> {
        if self.status != VaultStatus::Unlocked {
            return Err(VaultError::VaultLocked);
        }
        
        let encrypted_data = self.encrypt(data);
        let entry = VaultEntry::new(name, encrypted_data.len() as u64, data.len() as u64);
        
        self.entries.push(Some(entry));
        self.metadata.file_count += 1;
        self.metadata.total_size += encrypted_data.len() as u64;
        self.metadata.modified_at = get_current_time();
        
        Ok(())
    }
    
    /// Get file from vault
    pub fn get_file(&self, name: &str) -> Result<Vec<u8>, VaultError> {
        if self.status != VaultStatus::Unlocked {
            return Err(VaultError::VaultLocked);
        }
        
        for entry_option in &self.entries {
            if let Some(ref entry) = *entry_option {
                let entry_name = unsafe {
                    let len = entry.name.iter().position(|&b| b == 0).unwrap_or(256);
                    core::str::from_utf8_unchecked(&entry.name[..len])
                };
                
                if entry_name == name {
                    // In real implementation, decrypt and return data
                    return Ok(Vec::new());
                }
            }
        }
        
        Err(VaultError::FileNotFound)
    }
    
    /// Remove file from vault
    pub fn remove_file(&mut self, name: &str) -> Result<(), VaultError> {
        if self.status != VaultStatus::Unlocked {
            return Err(VaultError::VaultLocked);
        }
        
        let mut index = None;
        for (i, entry_option) in self.entries.iter().enumerate() {
            if let Some(ref entry) = *entry_option {
                let entry_name = unsafe {
                    let len = entry.name.iter().position(|&b| b == 0).unwrap_or(256);
                    core::str::from_utf8_unchecked(&entry.name[..len])
                };
                
                if entry_name == name {
                    index = Some(i);
                    break;
                }
            }
        }
        
        if let Some(i) = index {
            if let Some(ref entry) = self.entries[i] {
                self.metadata.file_count -= 1;
                self.metadata.total_size -= entry.encrypted_size;
                self.metadata.modified_at = get_current_time();
            }
            self.entries[i] = None;
            Ok(())
        } else {
            Err(VaultError::FileNotFound)
        }
    }
    
    /// Get vault status
    pub fn status(&self) -> VaultStatus {
        self.status
    }
    
    /// Get vault metadata
    pub fn metadata(&self) -> &VaultMetadata {
        &self.metadata
    }
    
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // In real implementation, perform actual encryption
        let mut encrypted = Vec::new();
        for &byte in data {
            encrypted.push(byte);
        }
        encrypted
    }
    
    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        // In real implementation, perform actual decryption
        let mut decrypted = Vec::new();
        for &byte in data {
            decrypted.push(byte);
        }
        decrypted
    }
}

/// Vault error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VaultError {
    Success = 0,
    VaultLocked = 1,
    InvalidKey = 2,
    FileNotFound = 3,
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
