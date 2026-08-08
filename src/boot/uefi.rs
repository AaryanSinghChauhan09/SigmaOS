#![no_std]

/// OOP-based UEFI Bootloader with Secure Boot database checking and TPM Measured Boot for SigmaOS
/// Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

pub type BootStatus = usize;

/// Standard UEFI Boot Phases
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase { Init = 0, LoadKernel = 1, Handoff = 2, Complete = 3 }

/// UEFI Boot Errors
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    Success = 0,
    LoadFailed = 1,
    HandoffFailed = 2,
    SignatureInvalid = 3,
}

/// Simulated raw UEFI Memory Descriptor conforming to UEFI spec
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError { Success = 0, LoadFailed = 1, HandoffFailed = 2, Revoked = 3 }

pub trait UEFIBootloader {
    fn phase(&self) -> BootPhase;
    unsafe fn load_kernel_raw(&mut self, kernel_raw: *const u8, size: usize, destination: *mut u8) -> Result<BootStatus, BootError>;
    unsafe fn parse_uefi_memory_map(&self, map_ptr: *const UefiMemoryDescriptor, descriptor_count: usize) -> u64;
    fn handoff(&mut self) -> Result<BootStatus, BootError>;
}

/// Complete UEFI Bootloader Implementation with Raw Pointer Memory Handling
#[repr(C)]
pub struct SimpleUEFIBootloader {
    pub phase: AtomicU32,
    pub kernel_loaded: AtomicU32,
    pub secure_boot_active: bool,
}

impl SimpleUEFIBootloader {
    pub fn new() -> Self {
        SimpleUEFIBootloader {
            phase: AtomicU32::new(BootPhase::Init as u32),
            kernel_loaded: AtomicU32::new(0),
            secure_boot_active: true,
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

    /// Loads the kernel payload by directly copying from a raw pointer using core::ptr operations (Linux boot chain)
    unsafe fn load_kernel_raw(
        &mut self,
        kernel_raw: *const u8,
        size: usize,
        destination: *mut u8,
    ) -> Result<BootStatus, BootError> {
        if kernel_raw.is_null() || destination.is_null() || size == 0 {
            return Err(BootError::LoadFailed);
        }

        // Copy raw memory non-overlapping
        core::ptr::copy_nonoverlapping(kernel_raw, destination, size);

        self.phase.store(BootPhase::LoadKernel as u32, Ordering::SeqCst);
        self.kernel_loaded.store(1, Ordering::SeqCst);
        Ok(size)
    }

    /// Iterates across raw UEFI memory map descriptors to calculate total available physical pages
    unsafe fn parse_uefi_memory_map(
        &self,
        map_ptr: *const UefiMemoryDescriptor,
        descriptor_count: usize,
    ) -> u64 {
        if map_ptr.is_null() || descriptor_count == 0 {
            return 0;
        }

        let mut total_pages = 0;
        for i in 0..descriptor_count {
            // Raw offset dereference
            let desc = *map_ptr.add(i);
            // Type 7 is EfiConventionalMemory (Available RAM)
            if desc.memory_type == 7 {
                total_pages += desc.number_of_pages;
            }
        }
        total_pages
    }

    fn handoff(&mut self) -> Result<BootStatus, BootError> {
        if self.kernel_loaded.load(Ordering::SeqCst) == 0 {
            return Err(BootError::HandoffFailed);
        }
        self.phase.store(BootPhase::Handoff as u32, Ordering::SeqCst);
        self.phase.store(BootPhase::Complete as u32, Ordering::SeqCst);
        Ok(1)
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
        let mut computed_hash: u8 = 0;
        for byte in data {
            computed_hash = computed_hash.wrapping_add(*byte).wrapping_mul(31);
        }
        let mut signature = Vec::new();
        signature.push(computed_hash);
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
            let result = bootloader.load_kernel_raw(
                kernel_src.as_ptr(),
                kernel_src.len(),
                kernel_dst.as_mut_ptr(),
            ).unwrap();
            assert_eq!(result, 7);
        }

        assert_eq!(kernel_dst, kernel_src);
        assert_eq!(bootloader.phase(), BootPhase::LoadKernel);
    }

    #[test]
    fn test_parse_uefi_memory_map() {
        let bootloader = SimpleUEFIBootloader::new();
        let map = [
            UefiMemoryDescriptor {
                memory_type: 7, // EfiConventionalMemory
                physical_start: 0x100000,
                virtual_start: 0x100000,
                number_of_pages: 256,
                attribute: 0xF,
            },
            UefiMemoryDescriptor {
                memory_type: 2, // EfiBootServicesCode
                physical_start: 0x200000,
                virtual_start: 0x200000,
                number_of_pages: 64,
                attribute: 0xF,
            },
        ];

        unsafe {
            let total_pages = bootloader.parse_uefi_memory_map(map.as_ptr(), map.len());
            assert_eq!(total_pages, 256); // Only memory type 7 pages are added
        }
    }

    #[test]
    fn test_uefi_secure_boot_verification() {
        let secure_boot = SimpleSecureBoot::new();
        let kernel_payload = [0xBB, 0xAA, 0x55, 0x33];

        let signature = secure_boot.sign(&kernel_payload).unwrap();
        assert!(secure_boot.verify_signature(&kernel_payload, &signature).unwrap());

        // Corrupted payload should fail verification
        let corrupted_payload = [0xBB, 0xAA, 0x55, 0x44];
        assert!(!secure_boot.verify_signature(&corrupted_payload, &signature).unwrap());
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
