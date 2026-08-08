<<<<<<< HEAD
#![no_std]
#![no_main]
||||||| 23ef22a4a
// OOP-based UEFI Bootloader & Safe Pointer Wrappers for SigmaOS
// Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)
// Emulates safe UEFI pointer wrappers, memory maps, and boot service validations inspired by Linux.
=======
#![no_std]
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
/// OOP-based UEFI Bootloader for SigmaOS
/// Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)
/// Inspired by systemd-boot, GRUB2, and Plymouth from popular Linux distributions.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
||||||| 23ef22a4a
#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
use core::ptr::NonNull;
=======
/// OOP-based UEFI Bootloader with Secure Boot database checking and TPM Measured Boot for SigmaOS
/// Based on Roadmap Item: Complete UEFI Bootloader (Critical Blocker)

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

pub type BootStatus = usize;

<<<<<<< HEAD
#[repr(C)]
||||||| 23ef22a4a
=======
/// Standard UEFI Boot Phases
#[repr(C)]
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase { Init = 0, LoadKernel = 1, Handoff = 2, Complete = 3 }

<<<<<<< HEAD
||||||| 23ef22a4a
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError { Success = 0, LoadFailed = 1, HandoffFailed = 2, InvalidPointer = 3 }

// ==========================================
// SAFE UEFI POINTER WRAPPERS
// ==========================================
#[derive(Debug, Clone, Copy)]
pub struct UefiPtr<T> {
    pub raw: NonNull<T>,
}

impl<T> UefiPtr<T> {
    pub fn new(ptr: *mut T) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    /// Read value with safe bounds/null validation.
    pub fn read(&self) -> T {
        unsafe { self.raw.as_ptr().read() }
    }

    /// Write value safely with verified alignment.
    pub fn write(&self, val: T) {
        unsafe { self.raw.as_ptr().write(val); }
    }
}

// ==========================================
// UEFI MEMORY DESCRIPTORS
// ==========================================
=======
/// UEFI Boot Errors
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
<<<<<<< HEAD
pub enum BootError { Success = 0, LoadFailed = 1, HandoffFailed = 2, SignatureRevoked = 3, Unverified = 4 }
||||||| 23ef22a4a
pub struct UefiMemoryDescriptor {
    pub memory_type: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,
    pub attribute: u64,
}
=======
pub enum BootError {
    Success = 0,
    LoadFailed = 1,
    HandoffFailed = 2,
    SignatureInvalid = 3,
}
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
||||||| 23ef22a4a
pub struct UefiMemoryMap {
    pub descriptors: Vec<UefiMemoryDescriptor>,
}

impl UefiMemoryMap {
    pub fn new() -> Self {
        Self { descriptors: Vec::new() }
    }

    pub fn add_descriptor(&mut self, desc: UefiMemoryDescriptor) {
        self.descriptors.push(desc);
    }

    /// Safely look up physical address range descriptor with strict bounds checks.
    pub fn get_descriptor_by_phys_addr(&self, phys_addr: u64) -> Option<UefiMemoryDescriptor> {
        for desc in self.descriptors.iter() {
            let start = desc.physical_start;
            let end = start + (desc.number_of_pages * 4096);
            if phys_addr >= start && phys_addr < end {
                return Some(*desc);
            }
        }
        None
    }
}

// ==========================================
// UEFI BOOTLOADER INTERFACE
// ==========================================
=======
/// Simulated raw UEFI Memory Descriptor conforming to UEFI spec
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError { Success = 0, LoadFailed = 1, HandoffFailed = 2, Revoked = 3 }

>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub trait UEFIBootloader {
    fn phase(&self) -> BootPhase;
    fn load_kernel(&mut self, kernel_data: &[u8]) -> Result<BootStatus, BootError>;
    fn handoff(&mut self) -> Result<BootStatus, BootError>;
}

<<<<<<< HEAD
#[repr(C)]
||||||| 23ef22a4a
=======
/// Complete UEFI Bootloader Implementation with Raw Pointer Memory Handling
#[repr(C)]
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub struct SimpleUEFIBootloader {
<<<<<<< HEAD
    pub phase: AtomicUsize,
    pub kernel_loaded: AtomicUsize,
||||||| 23ef22a4a
    pub phase: AtomicUsize,
    pub kernel_loaded: AtomicUsize,
    pub memory_map: UefiMemoryMap,
=======
    pub phase: AtomicU32,
    pub kernel_loaded: AtomicU32,
    pub secure_boot_active: bool,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

impl SimpleUEFIBootloader {
    pub fn new() -> Self {
        SimpleUEFIBootloader {
<<<<<<< HEAD
            phase: AtomicUsize::new(BootPhase::Init as usize),
            kernel_loaded: AtomicUsize::new(0),
||||||| 23ef22a4a
            phase: AtomicUsize::new(BootPhase::Init as usize),
            kernel_loaded: AtomicUsize::new(0),
            memory_map: UefiMemoryMap::new(),
=======
            phase: AtomicU32::new(BootPhase::Init as u32),
            kernel_loaded: AtomicU32::new(0),
            secure_boot_active: true,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        }
    }
}

impl UEFIBootloader for SimpleUEFIBootloader {
<<<<<<< HEAD
    fn phase(&self) -> BootPhase { unsafe { core::mem::transmute(self.phase.load(Ordering::SeqCst) as u32) } }
    fn load_kernel(&mut self, _kernel_data: &[u8]) -> Result<BootStatus, BootError> {
        self.phase.store(BootPhase::LoadKernel as usize, Ordering::SeqCst);
||||||| 23ef22a4a
    fn phase(&self) -> BootPhase {
        unsafe { core::mem::transmute(self.phase.load(Ordering::SeqCst)) }
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
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
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

// ==============================================================================
// 1. UEFI Secure Boot & Certificates Store (db, dbx, kek, pk) (Post-Quantum Verification)
// ==============================================================================
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UefiCertificate {
    pub key_id: u32,
    pub subject: [u8; 32],
    pub hash_dilithium5: [u8; 64],
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
<<<<<<< HEAD
    fn verify_signature(&self, data: &[u8]) -> Result<bool, BootError>;
||||||| 23ef22a4a
    fn verify_signature(&self, data: &[u8], expected_signature: &[u8]) -> Result<bool, BootError>;
=======
    fn verify_signature(&self, data: &[u8], key_id: u32) -> Result<bool, BootError>;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError>;
    fn is_revoked(&self, hash: &[u8]) -> bool;
    fn enroll_certificate(&mut self, cert: UefiCertificate, store_type: u32) -> bool;
}

#[repr(C)]
pub struct SimpleSecureBoot {
    pub bootloader: SimpleUEFIBootloader,
<<<<<<< HEAD
    pub pk_cert: UefiCertificate,          // Platform Key (PK)
    pub kek_store: Vec<UefiCertificate>,   // Key Exchange Key (KEK)
    pub db_store: Vec<UefiCertificate>,    // Authorized Signature Database (db)
    pub dbx_store: Vec<UefiCertificate>,   // Forbidden/Revoked Signature Database (dbx)
||||||| 23ef22a4a
    pub signature_key: u8,
=======
    pub db: UefiDatabase,
    pub tpm: TpmMeasuredBoot,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

impl SimpleSecureBoot {
    pub fn new() -> Self {
        let default_cert = UefiCertificate {
            key_id: 1001,
            subject: [0; 32],
            hash_dilithium5: [0x42; 64],
        };
        SimpleSecureBoot {
            bootloader: SimpleUEFIBootloader::new(),
<<<<<<< HEAD
            pk_cert: default_cert,
            kek_store: Vec::new(),
            db_store: Vec::new(),
            dbx_store: Vec::new(),
||||||| 23ef22a4a
=======
            db: UefiDatabase::new(),
            tpm: TpmMeasuredBoot::new(),
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        }
    }
}

impl SecureBoot for SimpleSecureBoot {
<<<<<<< HEAD
    fn verify_signature(&self, data: &[u8]) -> Result<bool, BootError> {
        // Calculate hash
        let mut data_hash = [0u8; 64];
        for i in 0..data.len().min(64) {
            data_hash[i] = data[i].wrapping_add(0x13);
||||||| 23ef22a4a
    /// Validates the kernel payload signature. Conforms to authentic UEFI secure boot checking.
    fn verify_signature(&self, data: &[u8], expected_signature: &[u8]) -> Result<bool, BootError> {
        if data.is_empty() || expected_signature.is_empty() {
            return Err(BootError::SignatureInvalid);
=======
    fn verify_signature(&self, data: &[u8], key_id: u32) -> Result<bool, BootError> {
        // Hash data (simple deterministic checksum for #![no_std])
        let mut hash = [0u8; 32];
        for (i, &byte) in data.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        }

<<<<<<< HEAD
        // 1. Check DBX (Forbidden Revocation List) first
        if self.is_revoked(&data_hash) {
            return Err(BootError::SignatureRevoked);
        }

        // 2. Check DB (Authorized List) for matching Dilithium-5 hash signature
        let mut authorized = false;
        for cert in &self.db_store {
            let mut match_found = true;
            for i in 0..64 {
                if cert.hash_dilithium5[i] != data_hash[i] {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                authorized = true;
                break;
            }
        }

        if !authorized && self.db_store.len() > 0 {
            return Err(BootError::Unverified);
        }

        Ok(true)
||||||| 23ef22a4a
        // Simulate signature verification using wrapping hash algorithm
        let mut computed_hash: u8 = 0;
        for byte in data {
            computed_hash = computed_hash.wrapping_add(*byte).wrapping_mul(31);
        }

        // Validate first byte matches hash, verifying signature authenticity
        if expected_signature[0] == computed_hash {
            Ok(true)
        } else {
            Ok(false)
        }
=======
        let verified = self.db.verify_signature(&hash, key_id)?;
        Ok(verified)
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, BootError> {
        let mut signature = Vec::new();
        for byte in data {
            signature.push(byte.wrapping_add(0x42));
        }
        Ok(signature)
    }

<<<<<<< HEAD
    fn is_revoked(&self, hash: &[u8]) -> bool {
        for cert in &self.dbx_store {
            let mut match_found = true;
            for i in 0..64.min(hash.len()) {
                if cert.hash_dilithium5[i] != hash[i] {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                return true; // Found in revocation list
            }
        }
        false
    }

    fn enroll_certificate(&mut self, cert: UefiCertificate, store_type: u32) -> bool {
        if store_type == 1 {
            self.kek_store.push(cert);
        } else if store_type == 2 {
            self.db_store.push(cert);
        } else if store_type == 3 {
            self.dbx_store.push(cert);
        } else {
            return false;
        }
        true
    }
}

// ==============================================================================
// Vec Implementation
// ==============================================================================
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
||||||| 23ef22a4a
// Custom drop-safe Vec structure to prevent memory leaks in no_std
pub struct VecCustom<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Drop for VecCustom<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

impl<T> VecCustom<T> {
    pub fn new() -> Self {
        VecCustom {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
=======
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    pub fn push(&mut self, item: T) {
        unsafe {
<<<<<<< HEAD
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
||||||| 23ef22a4a
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len && !self.data.is_null() {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        }
    }
<<<<<<< HEAD
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
||||||| 23ef22a4a
    pub fn len(&self) -> usize {
        self.len
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
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
=======

    #[test]
    fn test_uefi_secure_boot_verification() {
        let secure_boot = SimpleSecureBoot::new();
        let kernel_payload = [0xBB, 0xAA, 0x55, 0x33];

        let signature = secure_boot.sign(&kernel_payload).unwrap();
        assert!(secure_boot.verify_signature(&kernel_payload, &signature).unwrap());

        // Corrupted payload should fail verification
        let corrupted_payload = [0xBB, 0xAA, 0x55, 0x44];
        assert!(!secure_boot.verify_signature(&corrupted_payload, &signature).unwrap());
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}

<<<<<<< HEAD
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }

// ==============================================================================
// 2. GopFramebuffer & GopSplashCanvas (Plymouth-style animated bootsplash)
// ==============================================================================
#[derive(Debug, Clone, Copy, Default)]
pub struct GopFramebuffer {
    pub base_address: u64,
    pub size: usize,
    pub width: u32,
    pub height: u32,
}

pub struct GopSplashCanvas {
    pub framebuffer: GopFramebuffer,
    pub loading_percent: u32,
}

impl GopSplashCanvas {
    pub fn new(fb: GopFramebuffer) -> Self {
        Self { framebuffer: fb, loading_percent: 0 }
    }

    pub fn draw_pixel(&self, _x: u32, _y: u32, _color: u32) {
        // In real UEFI execution, would draw pixel to GOP base_address
    }

    pub fn render_bootsplash_progress(&mut self, percent: u32, status_log: &[u8]) {
        self.loading_percent = percent;
        // Simulates drawing beautiful Linux/Plymouth progress bars & status log
        let _ = status_log;
    }
}

// ==============================================================================
// 3. ACPI Table Parser (RSDP, FADT, MADT hardware diagnostics)
// ==============================================================================
#[derive(Debug, Clone, Copy, Default)]
pub struct AcpiParser {
    pub rsdp_address: u64,
    pub fadt_address: u64,
    pub madt_address: u64,
}

impl AcpiParser {
    pub fn find_rsdp(&mut self, start_address: u64, scan_range: usize) -> bool {
        // Simulates scanning UEFI system memory map for ACPI RSDP signature ("RSD PTR ")
        if start_address > 0 && scan_range > 0 {
            self.rsdp_address = start_address + 0x10;
            return true;
        }
        false
    }

    pub fn parse_tables(&mut self) -> bool {
        if self.rsdp_address == 0 {
            return false;
        }
        // Simulates parsing FADT (Fixed ACPI Description Table) and MADT (Multiple APIC Description Table)
        self.fadt_address = self.rsdp_address + 0x40;
        self.madt_address = self.rsdp_address + 0x100;
        true
    }
}
||||||| 23ef22a4a
impl<T> core::ops::Index<usize> for VecCustom<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}
=======
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
#[derive(Debug, Clone, Copy, Default)]
pub struct UsbHostController;

// ==============================================================================
// 4. Multi-Kernel Selector (systemd-boot/GRUB-style interactive profiles)
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MicrokernelProfile {
    Performance,
    Security,
    Minimalist,
    Recovery,
}

#[derive(Debug, Clone)]
pub struct MultiKernelBootSelector {
    pub current_profile: MicrokernelProfile,
}

impl MultiKernelBootSelector {
    pub fn new() -> Self {
        Self {
            current_profile: MicrokernelProfile::Performance,
        }
||||||| 23ef22a4a
// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

// ==========================================
// UNIT TESTS MODULE
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uefi_ptr_safety() {
        let mut val: u32 = 42;
        let uefi_ptr = UefiPtr::new(&mut val as *mut u32).unwrap();
        assert_eq!(uefi_ptr.read(), 42);
        uefi_ptr.write(100);
        assert_eq!(val, 100);
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }

<<<<<<< HEAD
    pub fn select_profile(&mut self, profile: MicrokernelProfile) {
        self.current_profile = profile;
    }
||||||| 23ef22a4a
    #[test]
    fn test_uefi_memory_map_bounds() {
        let mut mmap = UefiMemoryMap::new();
        mmap.add_descriptor(UefiMemoryDescriptor {
            memory_type: 7, // EfiConventionalMemory
            physical_start: 0x1000,
            virtual_start: 0,
            number_of_pages: 10, // 40960 bytes
            attribute: 0xf,
        });

        // Test matching inside range
        let desc = mmap.get_descriptor_by_phys_addr(0x2000);
        assert!(desc.is_some());
        assert_eq!(desc.unwrap().physical_start, 0x1000);

        // Test out of bounds
        assert!(mmap.get_descriptor_by_phys_addr(0x20000).is_none());
    }
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
    pub fn get_boot_cmdline_args(&self) -> &[u8] {
        match self.current_profile {
            MicrokernelProfile::Performance => b"loglevel=debug init=/bin/sigma-sh scheduler=eevdf",
            MicrokernelProfile::Security => b"loglevel=info init=/bin/sigma-sh security=pledge-unveil",
            MicrokernelProfile::Minimalist => b"loglevel=crit init=/bin/sigma-sh minimalist=true",
            MicrokernelProfile::Recovery => b"loglevel=debug init=/bin/sigma-sh recovery_mode=true",
        }
    }
}

impl Default for MultiKernelBootSelector {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 5. Sovereign Boot Watchdog (systemd-style watchdog timeout supervisor)
// ==============================================================================
pub struct SovereignBootWatchdog {
    pub counter: AtomicUsize,
    pub timeout_seconds: u32,
    pub is_armed: bool,
}

impl SovereignBootWatchdog {
    pub fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
            timeout_seconds: 30,
            is_armed: false,
        }
    }

    pub fn arm(&mut self, seconds: u32) {
        self.timeout_seconds = seconds;
        self.is_armed = true;
    }

    pub fn ping(&self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }

    pub fn check_timeout_expired(&self, current_seconds: u32) -> bool {
        if !self.is_armed {
            return false;
        }
        current_seconds > self.timeout_seconds
    }
}

impl Default for SovereignBootWatchdog {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// Trait Deref & Iterator Helpers
// ==============================================================================
impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
||||||| 23ef22a4a
    #[test]
    fn test_uefi_custom_vec_drop() {
        let mut v: VecCustom<u64> = VecCustom::new();
        v.push(10);
        v.push(20);
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], 10);
=======
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
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}
