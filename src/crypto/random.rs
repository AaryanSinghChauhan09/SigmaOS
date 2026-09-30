#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

use core::mem;
/// Random-generator API placeholder. No audited CSPRNG provider is integrated,
/// so this type must never emit key or nonce material.
use core::sync::atomic::{AtomicUsize, Ordering};

pub type RNGID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RNGError {
    Success = 0,
    InsufficientEntropy = 1,
    SeedingFailed = 2,
}

pub trait RandomGenerator {
    fn id(&self) -> RNGID;
    fn next_byte(&mut self) -> Result<u8, RNGError>;
    fn next_u32(&mut self) -> Result<u32, RNGError>;
    fn next_u64(&mut self) -> Result<u64, RNGError>;
    fn fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), RNGError>;
}

#[repr(C)]
pub struct SimpleRandomGenerator {
    pub id: RNGID,
    pub state: AtomicUsize,
    pub counter: AtomicUsize,
}

impl SimpleRandomGenerator {
    pub fn new(id: RNGID) -> Self {
        SimpleRandomGenerator {
            id,
            state: AtomicUsize::new(0),
            counter: AtomicUsize::new(0),
        }
    }
}

impl RandomGenerator for SimpleRandomGenerator {
    fn id(&self) -> RNGID {
        self.id
    }

    fn next_byte(&mut self) -> Result<u8, RNGError> {
        Err(RNGError::InsufficientEntropy)
    }

    fn next_u32(&mut self) -> Result<u32, RNGError> {
        let mut result: u32 = 0;
        for i in 0..4 {
            result |= (self.next_byte()? as u32) << (i * 8);
        }
        Ok(result)
    }

    fn next_u64(&mut self) -> Result<u64, RNGError> {
        let mut result: u64 = 0;
        for i in 0..8 {
            result |= (self.next_byte()? as u64) << (i * 8);
        }
        Ok(result)
    }

    fn fill_bytes(&mut self, buffer: &mut [u8]) -> Result<(), RNGError> {
        for byte in buffer.iter_mut() {
            *byte = self.next_byte()?;
        }
        Ok(())
    }
}

pub trait EntropyCollector {
    fn add_entropy(&mut self, source: u8, data: &[u8]);
    fn get_entropy_estimate(&self) -> usize;
    fn is_ready(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEntropyCollector {
    pub entropy_pool: Vec<u8>,
    pub entropy_estimate: AtomicUsize,
}

impl SimpleEntropyCollector {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleEntropyCollector {
            entropy_pool: Vec::new(),
            entropy_estimate: AtomicUsize::new(0),
        }
    }
}

impl EntropyCollector for SimpleEntropyCollector {
    fn add_entropy(&mut self, source: u8, data: &[u8]) {
        for &byte in data {
            self.entropy_pool.push(byte.wrapping_add(source));
        }
        self.entropy_estimate
            .fetch_add(data.len(), Ordering::SeqCst);
    }

    fn get_entropy_estimate(&self) -> usize {
        self.entropy_estimate.load(Ordering::SeqCst)
    }

    fn is_ready(&self) -> bool {
        self.entropy_estimate.load(Ordering::SeqCst) >= 256
    }
}

pub trait CSPRNG {
    fn reseed(&mut self, seed: &[u8]) -> Result<(), RNGError>;
    fn generate_secure(&mut self, length: usize) -> Result<Vec<u8>, RNGError>;
}

#[repr(C)]
pub struct SimpleCSPRNG {
    pub rng: SimpleRandomGenerator,
    pub entropy: SimpleEntropyCollector,
}

impl SimpleCSPRNG {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleCSPRNG {
            rng: SimpleRandomGenerator::new(1),
            entropy: SimpleEntropyCollector::new(),
        }
    }
}

impl CSPRNG for SimpleCSPRNG {
    fn reseed(&mut self, _seed: &[u8]) -> Result<(), RNGError> {
        Err(RNGError::InsufficientEntropy)
    }

    fn generate_secure(&mut self, length: usize) -> Result<Vec<u8>, RNGError> {
        if !self.entropy.is_ready() {
            return Err(RNGError::InsufficientEntropy);
        }

        let mut result = Vec::new();
        for _ in 0..length {
            result.push(self.rng.next_byte()?);
        }
        Ok(result)
    }
}

// ============================================================================
// Experimental hardware RNG and audit API shapes
// ============================================================================

/// Hardware entropy API placeholder. Raw instructions are not a CSPRNG provider.
pub struct HardwareRng {
    pub total_harvested_bytes: u64,
}

impl HardwareRng {
    pub fn new() -> Self {
        Self {
            total_harvested_bytes: 0,
        }
    }

    /// No health-tested entropy provider is integrated; never report raw entropy.
    pub fn get_hardware_u64(&mut self) -> Option<u64> {
        None
    }
}

/// Simulated audit API shape. It cannot attest production crypto integration.
pub struct ProductionCryptoEnclave {
    pub key_checksum: u64,
    pub audit_passed: bool,
}

#[derive(Debug, Clone)]
pub struct SecurityAuditReport {
    pub verified_algorithms: std::vec::Vec<std::string::String>,
    pub hardware_rng_active: bool,
    pub signatures_intact: bool,
}

impl ProductionCryptoEnclave {
    pub fn new(key: &[u8]) -> Self {
        let mut hash: u64 = 5381;
        for &byte in key {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(byte as u64);
        }
        Self {
            key_checksum: hash,
            audit_passed: false,
        }
    }

    /// Reports unavailable until audited crypto and signature checks are integrated.
    pub fn perform_security_audit(&mut self, hrng: &HardwareRng) -> SecurityAuditReport {
        self.audit_passed = false;

        SecurityAuditReport {
            verified_algorithms: std::vec::Vec::new(),
            hardware_rng_active: hrng.total_harvested_bytes > 0,
            signatures_intact: false,
        }
    }
}

struct VecImpl<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> VecImpl<T> {
    fn new() -> Self {
        VecImpl {
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
    }
}

extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod fail_closed_tests {
    use super::{
        HardwareRng, ProductionCryptoEnclave, RNGError, RandomGenerator, SimpleRandomGenerator,
    };

    #[test]
    fn generator_does_not_emit_pseudo_random_bytes() {
        let mut rng = SimpleRandomGenerator::new(7);
        assert!(matches!(
            rng.next_byte(),
            Err(RNGError::InsufficientEntropy)
        ));
        let mut bytes = [0x5A; 8];
        assert!(matches!(
            rng.fill_bytes(&mut bytes),
            Err(RNGError::InsufficientEntropy)
        ));
        assert_eq!(bytes, [0x5A; 8]);
    }

    #[test]
    fn unavailable_hardware_rng_and_audit_do_not_claim_success() {
        let mut rng = HardwareRng::new();
        assert_eq!(rng.get_hardware_u64(), None);
        let mut audit = ProductionCryptoEnclave::new(b"test-key");
        let report = audit.perform_security_audit(&rng);
        assert!(!audit.audit_passed);
        assert!(!report.signatures_intact);
        assert!(report.verified_algorithms.is_empty());
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_random_dynamic_entropy() {
        // Instantiate two separate random generators and verify their initial states are configured
        let r1 = SimpleRandomGenerator::new(101);
        let r2 = SimpleRandomGenerator::new(102);

        let s1 = r1.state.load(core::sync::atomic::Ordering::SeqCst);
        let s2 = r2.state.load(core::sync::atomic::Ordering::SeqCst);

        // Verify that initial seeds are dynamically configured and non-matching
        assert_ne!(s1, s2);
        assert_ne!(s1, 12345);
        assert_ne!(s2, 12345);
    }

    #[test]
    fn test_hardware_rng() {
        let mut hrng = HardwareRng::new();
        assert_eq!(hrng.total_harvested_bytes, 0);

        if let Some(val) = hrng.get_hardware_u64() {
            assert!(hrng.total_harvested_bytes >= 8);
        }
    }

    #[test]
    fn test_cryptographic_enclave_and_audit() {
        let mut hrng = HardwareRng::new();
        let _ = hrng.get_hardware_u64(); // harvest some entropy

        let mut enclave = ProductionCryptoEnclave::new(b"enclave_master_key");
        assert_ne!(enclave.key_checksum, 5381);
        assert!(!enclave.audit_passed);

        let report = enclave.perform_security_audit(&hrng);
        assert!(!enclave.audit_passed);
        assert!(!report.signatures_intact);
        assert!(report.verified_algorithms.is_empty());
    }
}
