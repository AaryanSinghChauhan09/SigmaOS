#![no_std]

/// OOP-based UEFI Bootloader with Secure Boot database checking and TPM Measured Boot for SigmaOS
/// Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BootStatus = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase { Init = 0, LoadKernel = 1, Handoff = 2, Complete = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError { Success = 0, LoadFailed = 1, HandoffFailed = 2, Revoked = 3 }

pub trait UEFIBootloader {
    fn phase(&self) -> BootPhase;
    fn load_kernel(&mut self, kernel_data: &[u8]) -> Result<BootStatus, BootError>;
    fn handoff(&mut self) -> Result<BootStatus, BootError>;
}

#[repr(C)]
pub struct SimpleUEFIBootloader {
    pub phase: AtomicUsize,
    pub kernel_loaded: AtomicUsize,
}

impl SimpleUEFIBootloader {
    pub fn new() -> Self {
        SimpleUEFIBootloader {
            phase: AtomicUsize::new(BootPhase::Init as usize),
            kernel_loaded: AtomicUsize::new(0),
        }
    }
}

impl UEFIBootloader for SimpleUEFIBootloader {
    fn phase(&self) -> BootPhase {
        let val = self.phase.load(Ordering::SeqCst);
        match val {
            0 => BootPhase::Init,
            1 => BootPhase::LoadKernel,
            2 => BootPhase::Handoff,
            _ => BootPhase::Complete,
        }
    }
    fn load_kernel(&mut self, _kernel_data: &[u8]) -> Result<BootStatus, BootError> {
        self.phase.store(BootPhase::LoadKernel as usize, Ordering::SeqCst);
        self.kernel_loaded.store(1, Ordering::SeqCst);
        Ok(1)
    }
    fn handoff(&mut self) -> Result<BootStatus, BootError> {
        if self.kernel_loaded.load(Ordering::SeqCst) == 0 {
            return Err(BootError::LoadFailed);
        }
        self.phase.store(BootPhase::Handoff as usize, Ordering::SeqCst);
        self.phase.store(BootPhase::Complete as usize, Ordering::SeqCst);
        Ok(2)
    }
}

// UEFI db / dbx Databases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DbKey {
    pub hash: [u8; 32],
    pub key_id: u32,
    pub is_revoked: bool,
}

pub struct UefiDatabase {
    pub keys: [Option<DbKey>; 8],
}

impl UefiDatabase {
    pub fn new() -> Self {
        Self { keys: [None; 8] }
    }

    pub fn enroll_key(&mut self, key: DbKey) -> Result<(), &'static str> {
        for slot in &mut self.keys {
            if slot.is_none() {
                *slot = Some(key);
                return Ok(());
            }
        }
        Err("UEFI db full")
    }

    pub fn verify_signature(&self, hash: &[u8; 32], key_id: u32) -> Result<bool, BootError> {
        // Check dbx (revocation) first
        for slot in &self.keys {
            if let Some(ref db_key) = slot {
                if db_key.key_id == key_id && db_key.hash == *hash && db_key.is_revoked {
                    return Err(BootError::Revoked);
                }
            }
        }

        // Check db (authorized)
        for slot in &self.keys {
            if let Some(ref db_key) = slot {
                if db_key.key_id == key_id && db_key.hash == *hash && !db_key.is_revoked {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

// TPM Platform Configuration Registers (Measured Boot)
pub struct TpmMeasuredBoot {
    pub pcrs: [u32; 16],
}

impl TpmMeasuredBoot {
    pub fn new() -> Self {
        Self { pcrs: [0; 16] }
    }

    pub fn extend_pcr(&mut self, pcr_idx: usize, val: u32) {
        if pcr_idx < 16 {
            let mut current = self.pcrs[pcr_idx];
            current = current ^ val;
            current = current.wrapping_mul(16777619);
            self.pcrs[pcr_idx] = current;
        }
    }
}

pub trait SecureBoot {
    fn verify_signature(&self, data: &[u8], key_id: u32) -> Result<bool, BootError>;
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError>;
}

pub struct SimpleSecureBoot {
    pub bootloader: SimpleUEFIBootloader,
    pub db: UefiDatabase,
    pub tpm: TpmMeasuredBoot,
}

impl SimpleSecureBoot {
    pub fn new() -> Self {
        SimpleSecureBoot {
            bootloader: SimpleUEFIBootloader::new(),
            db: UefiDatabase::new(),
            tpm: TpmMeasuredBoot::new(),
        }
    }
}

impl SecureBoot for SimpleSecureBoot {
    fn verify_signature(&self, data: &[u8], key_id: u32) -> Result<bool, BootError> {
        // Hash data (simple deterministic checksum for #![no_std])
        let mut hash = [0u8; 32];
        for (i, &byte) in data.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }

        let verified = self.db.verify_signature(&hash, key_id)?;
        Ok(verified)
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError> {
        let mut signature = Vec::new();
        for byte in data {
            signature.push(byte.wrapping_add(0x42));
        }
        Ok(signature)
    }
}

pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uefi_bootloader_lifecycle() {
        let mut boot = SimpleUEFIBootloader::new();
        assert_eq!(boot.phase(), BootPhase::Init);

        boot.load_kernel(&[0x1, 0x2, 0x3]).unwrap();
        assert_eq!(boot.phase(), BootPhase::LoadKernel);

        boot.handoff().unwrap();
        assert_eq!(boot.phase(), BootPhase::Complete);
    }

    #[test]
    fn test_uefi_secure_db_signature_validation() {
        let mut sb = SimpleSecureBoot::new();
        let kernel_data = [0xAA; 64];

        // Hash kernel data
        let mut hash = [0u8; 32];
        for (i, &byte) in kernel_data.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }

        // Enroll kernel signing key as revoked in dbx
        let revoked_key = DbKey {
            hash,
            key_id: 2002,
            is_revoked: true,
        };
        sb.db.enroll_key(revoked_key).unwrap();

        // Enforcing signature check fails on revoked keys immediately
        let check_revoked = sb.verify_signature(&kernel_data, 2002);
        assert_eq!(check_revoked, Err(BootError::Revoked));

        // Enroll authorized key in db
        let authorized_key = DbKey {
            hash,
            key_id: 2001,
            is_revoked: false,
        };
        sb.db.enroll_key(authorized_key).unwrap();

        // Check authorized succeeds
        let check_auth = sb.verify_signature(&kernel_data, 2001).unwrap();
        assert!(check_auth);
    }
}
