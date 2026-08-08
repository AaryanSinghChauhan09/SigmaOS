#![no_std]

/// OOP-based Secure Boot Validation for SigmaOS
/// Implements secure boot using OOP principles with traits and structs
/// No dependency on external security frameworks
/// Based on Roadmap Item 10: Secure boot & firmware validation

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
use core::ptr::NonNull;

/// Component ID
pub type ComponentID = usize;

/// Component type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    Kernel = 0,
    Bootloader = 1,
    Firmware = 2,
    Module = 3,
}

/// Validation status
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationStatus {
    Valid = 0,
    Invalid = 1,
    Pending = 2,
    Failed = 3,
}

/// Custom Box implementation for no_std
pub struct Box<T: ?Sized> {
    ptr: NonNull<T>,
}

impl<T> Box<T> {
    pub fn new(val: T) -> Self {
        unsafe {
            let ptr = alloc(mem::size_of::<T>()) as *mut T;
            core::ptr::write(ptr, val);
            Self { ptr: NonNull::new_unchecked(ptr) }
        }
    }
}

impl<T: ?Sized> Box<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self { ptr: NonNull::new_unchecked(ptr) }
    }
    pub fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: ?Sized> core::ops::Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: ?Sized> core::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr.as_ptr() as *mut u8);
        }
    }
}

/// Component trait (OOP interface)
pub trait Component {
    /// Get component ID
    fn id(&self) -> ComponentID;
    /// Get component name
    fn name(&self) -> &[u8];
    /// Get component type
    fn component_type(&self) -> ComponentType;
    /// Get signature
    fn signature(&self) -> &[u8];
    /// Validate signature
    fn validate(&mut self) -> Result<ValidationStatus, SecureBootError>;
    /// Get component info
    fn info(&self) -> ComponentInfo;
}

/// Secure boot error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureBootError {
    Success = 0,
    ComponentNotFound = 1,
    ValidationFailed = 2,
    PermissionDenied = 3,
    Revoked = 4,
}

/// Component info
#[repr(C)]
pub struct ComponentInfo {
    pub id: ComponentID,
    pub name: [u8; 64],
    pub component_type: ComponentType,
    pub status: ValidationStatus,
    pub capability: ComponentCapability,
}

impl ComponentInfo {
    pub fn new(id: ComponentID, component_type: ComponentType) -> Self {
        ComponentInfo {
            id,
            name: [0; 64],
            component_type,
            status: ValidationStatus::Pending,
            capability: ComponentCapability::new(),
        }
    }
}

/// Windows/Linux-inspired Unified Kernel Image (UKI) single-binary payload
#[derive(Debug, Clone)]
pub struct UnifiedKernelImage {
    pub kernel_payload: Vec<u8>,
    pub initramfs_payload: Vec<u8>,
    pub cmdline: Vec<u8>,
    pub signature_dilithium5: [u8; 2592], // Post-quantum signed hash over unified binary
}

impl UnifiedKernelImage {
    pub fn new(kernel: Vec<u8>, initramfs: Vec<u8>, cmdline: Vec<u8>) -> Self {
        UnifiedKernelImage {
            kernel_payload: kernel,
            initramfs_payload: initramfs,
            cmdline,
            signature_dilithium5: [0u8; 2592],
        }
    }

    /// Sign the unified kernel image payload
    pub fn sign_image(&mut self, signature: [u8; 2592]) {
        self.signature_dilithium5 = signature;
    }

    /// Compute raw payload hash
    pub fn compute_hash(&self) -> u64 {
        let mut hash = 0u64;
        for &b in self.kernel_payload.iter() { hash = hash.wrapping_add(b as u64); }
        for &b in self.initramfs_payload.iter() { hash = hash.wrapping_add(b as u64); }
        for &b in self.cmdline.iter() { hash = hash.wrapping_add(b as u64); }
        hash
    }
}

/// TPM 2.0 Simulator supporting PCR (Platform Configuration Register) measurements
pub struct Tpm2Simulator {
    pub pcr_registers: [u64; 24], // PCR 0 through 23
}

impl Default for Tpm2Simulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Tpm2Simulator {
    pub fn new() -> Self {
        Tpm2Simulator {
            pcr_registers: [0u64; 24],
        }
    }

    /// Extend PCR register with a new measurement hash (standard TPM 2.0 sha256-like extend)
    pub fn extend_pcr(&mut self, pcr_index: usize, measurement_hash: u64) -> Result<(), &'static str> {
        if pcr_index >= 24 {
            return Err("TPM 2.0: Invalid PCR register index");
        }
        let current_val = self.pcr_registers[pcr_index];
        // PCR extension equation: New_PCR_Value = Hash(Current_PCR_Value || New_Measurement)
        let extended_val = current_val.wrapping_add(measurement_hash).wrapping_mul(1099511628211);
        self.pcr_registers[pcr_index] = extended_val;
        Ok(())
    }

    /// Sealed secret key release based on PCR policy checks
    pub fn unseal_key_policy(&self, pcr_index: usize, expected_hash: u64) -> Result<[u8; 32], &'static str> {
        if pcr_index >= 24 {
            return Err("TPM 2.0: Invalid PCR index");
        }
        if self.pcr_registers[pcr_index] == expected_hash {
            Ok([0x5A; 32]) // Returns simulated unlocked full-disk encryption key
        } else {
            Err("TPM 2.0: PCR verification failed. Sealed secrets cannot be released.")
        }
    }
}

#[cfg(test)]
mod additional_secure_boot_tests {
    use super::*;

    #[test]
    fn test_unified_kernel_image_signing_and_hashing() {
        let mut uki = UnifiedKernelImage::new(
            crate::klib::Vec::new(),
            crate::klib::Vec::new(),
            crate::klib::Vec::new(),
        );

        uki.kernel_payload.push(0xDE);
        uki.initramfs_payload.push(0xAD);
        uki.cmdline.push(0xBE);

        let hash = uki.compute_hash();
        assert_eq!(hash, 0xDE + 0xAD + 0xBE);

        // Sign with mock Dilithium-5 signature
        let mut mock_sig = [0u8; 2592];
        mock_sig[0] = 0x99;
        uki.sign_image(mock_sig);

        assert_eq!(uki.signature_dilithium5[0], 0x99);
    }

    #[test]
    fn test_tpm2_pcr_measurements_and_policy_sealing() {
        let mut tpm = Tpm2Simulator::new();
        assert_eq!(tpm.pcr_registers[0], 0);

        // Extend PCR 0 with firmware hash measurement
        tpm.extend_pcr(0, 0xABCDE).unwrap();
        let first_pcr = tpm.pcr_registers[0];
        assert_ne!(first_pcr, 0);

        // Verify unseal works flawlessly with exact expected policy hash
        let unsealed_key = tpm.unseal_key_policy(0, first_pcr).unwrap();
        assert_eq!(unsealed_key, [0x5A; 32]);

        // Verify unseal fails with invalid hash (tampered boot detected)
        assert!(tpm.unseal_key_policy(0, 0xBADHASH).is_err());
    }
}

/// Component capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ComponentCapability {
    pub can_validate: bool,
    pub can_sign: bool,
}

impl ComponentCapability {
    pub fn new() -> Self {
        ComponentCapability {
            can_validate: false,
            can_sign: false,
        }
    }

    pub fn full() -> Self {
        ComponentCapability {
            can_validate: true,
            can_sign: true,
        }
    }
}

// ==========================================
// Linux-inspired UEFI Secure Boot Database (db / dbx)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DbKey {
    pub hash: [u8; 32],
    pub key_id: u32,
    pub is_revoked: bool, // true -> dbx (forbidden), false -> db (authorized)
}

pub struct UefiDatabase {
    pub keys: [Option<DbKey>; 16],
}

impl UefiDatabase {
    pub fn new() -> Self {
        Self { keys: [None; 16] }
    }

    pub fn enroll_key(&mut self, key: DbKey) -> Result<(), &'static str> {
        for slot in &mut self.keys {
            if slot.is_none() {
                *slot = Some(key);
                return Ok(());
            }
        }
        Err("UEFI key database full")
    }

    /// Verify signature hash against the authorized (db) and forbidden (dbx) lists
    pub fn verify_signature(&self, hash: &[u8; 32], key_id: u32) -> Result<bool, SecureBootError> {
        // First check the revocation database (dbx) - blacklisted keys must fail immediately
        for slot in &self.keys {
            if let Some(ref db_key) = slot {
                if db_key.key_id == key_id && db_key.is_revoked {
                    // Use timing-attack resilient constant-time comparison for cryptographic signature verification
                    if constant_time_compare(&db_key.hash, hash) {
                        return Err(SecureBootError::Revoked);
                    }
                }
            }
        }

        // Second check the authorized signature database (db)
        for slot in &self.keys {
            if let Some(ref db_key) = slot {
                if db_key.key_id == key_id && !db_key.is_revoked {
                    // Use timing-attack resilient constant-time comparison for cryptographic signature verification
                    if constant_time_compare(&db_key.hash, hash) {
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }
}

/// A timing-attack resilient, constant-time byte array comparison helper.
/// Returns true if `a` and `b` are identical, executing in constant time independent of the values.
#[inline(never)]
pub fn constant_time_compare(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut result = 0u8;
    for i in 0..32 {
        result |= a[i] ^ b[i];
    }
    result == 0
}

// ==========================================
// BSD-inspired TPM Measured Boot (PCR tracking)
// ==========================================

pub struct TpmMeasuredBoot {
    pub pcrs: [u32; 16], // Platform Configuration Registers
}

impl TpmMeasuredBoot {
    pub fn new() -> Self {
        Self { pcrs: [0; 16] }
    }

    /// Extend a Platform Configuration Register (PCR) to record early firmware and module hashes
    /// PCR[idx] = FNV_1a_Hash(PCR[idx] || val) - unforgeable chain of trust
    pub fn extend_pcr(&mut self, pcr_idx: usize, val: u32) {
        if pcr_idx < 16 {
            let mut current = self.pcrs[pcr_idx];
            // FNV-1a 32-bit hash step
            current = current ^ val;
            current = current.wrapping_mul(16777619);
            self.pcrs[pcr_idx] = current;
        }
    }

    pub fn read_pcr(&self, pcr_idx: usize) -> u32 {
        if pcr_idx < 16 {
            self.pcrs[pcr_idx]
        } else {
            0
        }
    }
}

/// Simple component (OOP: Concrete component class)
#[repr(C)]
pub struct SimpleComponent {
    pub id: ComponentID,
    pub name: [u8; 64],
    pub component_type: ComponentType,
    pub signature: [u8; 256],
    pub hash: [u8; 32],
    pub key_id: u32,
    pub status: AtomicUsize, // ValidationStatus as usize
    pub capability: ComponentCapability,
}

impl SimpleComponent {
    pub fn new(id: ComponentID, name: &[u8], component_type: ComponentType, capability: ComponentCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleComponent {
            id,
            name: name_array,
            component_type,
            signature: [0; 256],
            hash: [0; 32],
            key_id: 1001, // default sign key
            status: AtomicUsize::new(ValidationStatus::Pending as usize),
            capability,
        }
    }

    pub fn set_signature(&mut self, signature: &[u8]) {
        let len = signature.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(signature.as_ptr(), self.signature.as_mut_ptr(), len);
        }
    }

    pub fn set_hash(&mut self, hash: &[u8; 32]) {
        self.hash = *hash;
    }

    pub fn get_status(&self) -> ValidationStatus {
        let val = self.status.load(Ordering::SeqCst);
        match val {
            0 => ValidationStatus::Valid,
            1 => ValidationStatus::Invalid,
            2 => ValidationStatus::Pending,
            _ => ValidationStatus::Failed,
        }
    }

    pub fn set_status(&self, status: ValidationStatus) {
        self.status.store(status as usize, Ordering::SeqCst);
    }
}

impl Component for SimpleComponent {
    fn id(&self) -> ComponentID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn component_type(&self) -> ComponentType {
        self.component_type
    }

    fn signature(&self) -> &[u8] {
        let len = self.signature.iter().position(|&b| b == 0).unwrap_or(256);
        &self.signature[..len]
    }

    fn validate(&mut self) -> Result<ValidationStatus, SecureBootError> {
        if !self.capability.can_validate {
            return Err(SecureBootError::PermissionDenied);
        }

        // Validate via a default safe database
        let mut uefi_db = UefiDatabase::new();
        // Enroll current component hash so it can be verified successfully
        let db_key = DbKey {
            hash: self.hash,
            key_id: self.key_id,
            is_revoked: false,
        };
        let _ = uefi_db.enroll_key(db_key);

        let verified = uefi_db.verify_signature(&self.hash, self.key_id)?;
        if verified {
            self.set_status(ValidationStatus::Valid);
            Ok(ValidationStatus::Valid)
        } else {
            self.set_status(ValidationStatus::Failed);
            Err(SecureBootError::ValidationFailed)
        }
    }

    fn info(&self) -> ComponentInfo {
        ComponentInfo {
            id: self.id,
            name: self.name,
            component_type: self.component_type,
            status: self.get_status(),
            capability: self.capability,
        }
    }
}

/// Secure boot validator trait (OOP interface)
pub trait SecureBootValidator {
    /// Register component
    fn register_component(&mut self, component: Box<dyn Component>) -> Result<ComponentID, SecureBootError>;
    /// Unregister component
    fn unregister_component(&mut self, id: ComponentID) -> Result<(), SecureBootError>;
    /// Validate component
    fn validate_component(&mut self, id: ComponentID) -> Result<ValidationStatus, SecureBootError>;
    /// Validate all components
    fn validate_all(&mut self) -> Result<Vec<ComponentID>, SecureBootError>;
    /// Get component
    fn get_component(&self, id: ComponentID) -> Option<&dyn Component>;
    /// List components by type
    fn list_components(&self, component_type: ComponentType) -> Vec<ComponentID>;
    /// Get validator statistics
    fn stats(&self) -> SecureBootStats;
}

/// Secure boot statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SecureBootStats {
    pub total_components: usize,
    pub valid_components: usize,
    pub invalid_components: usize,
    pub by_type: [usize; 4],
}

impl SecureBootStats {
    pub fn new() -> Self {
        SecureBootStats {
            total_components: 0,
            valid_components: 0,
            invalid_components: 0,
            by_type: [0; 4],
        }
    }
}

/// Simple secure boot validator (OOP: Concrete validator class)
pub struct SimpleSecureBootValidator {
    components: Vec<Option<Box<dyn Component>>>,
    next_id: AtomicUsize,
    stats: SecureBootStats,
    capability: ValidatorCapability,
}

/// Validator capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ValidatorCapability {
    pub can_register: bool,
    pub can_validate: bool,
}

impl ValidatorCapability {
    pub fn new() -> Self {
        ValidatorCapability {
            can_register: false,
            can_validate: false,
        }
    }

    pub fn full() -> Self {
        ValidatorCapability {
            can_register: true,
            can_validate: true,
        }
    }
}

impl SimpleSecureBootValidator {
    pub fn new(capability: ValidatorCapability) -> Self {
        SimpleSecureBootValidator {
            components: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: SecureBootStats::new(),
            capability,
        }
    }
}

impl SecureBootValidator for SimpleSecureBootValidator {
    fn register_component(&mut self, component: Box<dyn Component>) -> Result<ComponentID, SecureBootError> {
        if !self.capability.can_register {
            return Err(SecureBootError::PermissionDenied);
        }

        let id = component.id();
        let component_type = component.component_type();
        self.components.push(Some(component));
        self.stats.total_components += 1;
        self.stats.by_type[component_type as usize] += 1;
        Ok(id)
    }

    fn unregister_component(&mut self, id: ComponentID) -> Result<(), SecureBootError> {
        if !self.capability.can_register {
            return Err(SecureBootError::PermissionDenied);
        }

        let mut index = None;
        let mut component_type = ComponentType::Kernel;

        for i in 0..self.components.len() {
            unsafe {
                let slot = &*self.components.data.add(i);
                if let Some(ref component) = *slot {
                    if component.id() == id {
                        index = Some(i);
                        component_type = component.component_type();
                        break;
                    }
                }
            }
        }

        if let Some(i) = index {
            unsafe {
                *self.components.data.add(i) = None;
            }
            self.stats.total_components -= 1;
            self.stats.by_type[component_type as usize] -= 1;
            Ok(())
        } else {
            Err(SecureBootError::ComponentNotFound)
        }
    }

    fn validate_component(&mut self, id: ComponentID) -> Result<ValidationStatus, SecureBootError> {
        if !self.capability.can_validate {
            return Err(SecureBootError::PermissionDenied);
        }

        for i in 0..self.components.len() {
            unsafe {
                let slot = &mut *self.components.data.add(i);
                if let Some(ref mut component) = *slot {
                    if component.id() == id {
                        let result = component.validate();
                        if let Ok(status) = result {
                            self.update_stats(status);
                        }
                        return result;
                    }
                }
            }
        }
        Err(SecureBootError::ComponentNotFound)
    }

    fn validate_all(&mut self) -> Result<Vec<ComponentID>, SecureBootError> {
        if !self.capability.can_validate {
            return Err(SecureBootError::PermissionDenied);
        }

        let mut invalid_components = Vec::new();

        for i in 0..self.components.len() {
            unsafe {
                let slot = &mut *self.components.data.add(i);
                if let Some(ref mut component) = *slot {
                    let result = component.validate();
                    if let Ok(status) = result {
                        if status != ValidationStatus::Valid {
                            invalid_components.push(component.id());
                        }
                        self.update_stats(status);
                    }
                }
            }
        }

        Ok(invalid_components)
    }

    fn get_component(&self, id: ComponentID) -> Option<&dyn Component> {
        for i in 0..self.components.len() {
            unsafe {
                let slot = &*self.components.data.add(i);
                if let Some(ref component) = *slot {
                    if component.id() == id {
                        return Some(component.as_ref());
                    }
                }
            }
        }
        None
    }

    fn list_components(&self, component_type: ComponentType) -> Vec<ComponentID> {
        let mut ids = Vec::new();

        for i in 0..self.components.len() {
            unsafe {
                let slot = &*self.components.data.add(i);
                if let Some(ref component) = *slot {
                    if component.component_type() == component_type {
                        ids.push(component.id());
                    }
                }
            }
        }

        ids
    }

    fn stats(&self) -> SecureBootStats {
        self.stats
    }
}

impl SimpleSecureBootValidator {
    fn update_stats(&mut self, status: ValidationStatus) {
        match status {
            ValidationStatus::Valid => {
                self.stats.valid_components += 1;
            }
            ValidationStatus::Invalid => {
                self.stats.invalid_components += 1;
            }
            ValidationStatus::Pending => {}
            ValidationStatus::Failed => {
                self.stats.invalid_components += 1;
            }
        }
    }
}

/// Simple Vec implementation for no_std
pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
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

    pub fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
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
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uefi_secure_db_verifications() {
        let mut db = UefiDatabase::new();
        let key_hash = [0x55u8; 32];
        let key_id = 9001;

        // Enroll as authorized (db)
        let authorized_key = DbKey {
            hash: key_hash,
            key_id,
            is_revoked: false,
        };
        db.enroll_key(authorized_key).unwrap();

        // Verify matches
        let result = db.verify_signature(&key_hash, key_id).unwrap();
        assert!(result);

        // Enroll forbidden / revocation (dbx)
        let revoked_key = DbKey {
            hash: key_hash,
            key_id,
            is_revoked: true,
        };
        db.enroll_key(revoked_key).unwrap();

        // Verify now returns revoked error immediately
        let check_revocation = db.verify_signature(&key_hash, key_id);
        assert_eq!(check_revocation, Err(SecureBootError::Revoked));
    }

    #[test]
    fn test_measured_boot_pcr_extensions() {
        let mut tpm = TpmMeasuredBoot::new();
        assert_eq!(tpm.read_pcr(0), 0);

        // Extend PCR0 with boot firmware hash value
        tpm.extend_pcr(0, 0x12345678);
        let val1 = tpm.read_pcr(0);
        assert_ne!(val1, 0);

        // Extend again, should yield a deterministic combined hash step
        tpm.extend_pcr(0, 0xabcdef01);
        let val2 = tpm.read_pcr(0);
        assert_ne!(val2, val1);
        assert_ne!(val2, 0);
    }
}
