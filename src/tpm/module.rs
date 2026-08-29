#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based TPM Module for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 582
/// Implements Trusted Platform Module operations

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TPMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TPMError { Success = 0, NotFound = 1, OperationFailed = 2 }

pub trait TPM {
    fn id(&self) -> TPMID;
    fn manufacturer(&self) -> &[u8];
    fn version(&self) -> u32;
    fn is_ready(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTPM {
    pub id: TPMID,
    pub manufacturer: [u8; 32],
    pub manufacturer_len: u8,
    pub version: AtomicUsize,
    pub ready: AtomicUsize,
}

impl SimpleTPM {
    pub fn new(id: TPMID, manufacturer: &[u8], version: u32) -> Self {
        let mut manuf_array = [0u8; 32];
        let manuf_len = manufacturer.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(manufacturer.as_ptr(), manuf_array.as_mut_ptr(), manuf_len);
        }
        SimpleTPM {
            id,
            manufacturer: manuf_array,
            manufacturer_len: manuf_len as u8,
            version: AtomicUsize::new(version as usize),
            ready: AtomicUsize::new(1),
        }
    }
}

impl TPM for SimpleTPM {
    fn id(&self) -> TPMID { self.id }
    fn manufacturer(&self) -> &[u8] {
        // Performance optimization: explicit manufacturer_len u8 field avoids $O(N)$ linear byte scans
        &self.manufacturer[..self.manufacturer_len as usize]
    }
    fn version(&self) -> u32 { self.version.load(Ordering::SeqCst) as u32 }
    fn is_ready(&self) -> bool { self.ready.load(Ordering::SeqCst) == 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tpm_manufacturer_len_caching() {
        let manufacturer_bytes = b"Infineon Technologies";
        let tpm = SimpleTPM::new(1, manufacturer_bytes, 2);
        assert_eq!(tpm.id(), 1);
        assert_eq!(tpm.manufacturer(), manufacturer_bytes);
        assert_eq!(tpm.manufacturer_len as usize, manufacturer_bytes.len());
        assert_eq!(tpm.version(), 2);
        assert!(tpm.is_ready());
    }
}

pub trait TPMOperations {
    fn generate_key(&mut self, tpm_id: TPMID) -> Result<Vec<u8>, TPMError>;
    fn seal_data(&mut self, tpm_id: TPMID, data: &[u8]) -> Result<Vec<u8>, TPMError>;
    fn unseal_data(&mut self, tpm_id: TPMID, sealed: &[u8]) -> Result<Vec<u8>, TPMError>;
    fn measure_boot(&mut self, tpm_id: TPMID, pcr: u8, data: &[u8]) -> Result<(), TPMError>;
}

#[repr(C)]
pub struct SimpleTPMOperations {
    pub tpms: Vec<Option<Box<dyn TPM>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTPMOperations {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleTPMOperations {
            tpms: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TPMOperations for SimpleTPMOperations {
    fn generate_key(&mut self, _tpm_id: TPMID) -> Result<Vec<u8>, TPMError> {
        let mut key = Vec::new();
        for i in 0..32 {
            key.push(i as u8);
        }
        Ok(key)
    }

    fn seal_data(&mut self, _tpm_id: TPMID, data: &[u8]) -> Result<Vec<u8>, TPMError> {
        let mut sealed = Vec::new();
        for &byte in data {
            sealed.push(byte.wrapping_add(1));
        }
        Ok(sealed)
    }

    fn unseal_data(&mut self, _tpm_id: TPMID, sealed: &[u8]) -> Result<Vec<u8>, TPMError> {
        let mut data = Vec::new();
        for &byte in sealed {
            data.push(byte.wrapping_sub(1));
        }
        Ok(data)
    }

    fn measure_boot(&mut self, _tpm_id: TPMID, _pcr: u8, _data: &[u8]) -> Result<(), TPMError> {
        Ok(())
    }
}

pub trait Attestation {
    fn generate_attestation(&self, tpm_id: TPMID, nonce: &[u8]) -> Result<Vec<u8>, TPMError>;
    fn verify_attestation(&self, attestation: &[u8], nonce: &[u8]) -> Result<bool, TPMError>;
}

#[repr(C)]
pub struct SimpleAttestation {
    pub tpm_ops: SimpleTPMOperations,
}

impl SimpleAttestation {
    pub fn new(tpm_ops: SimpleTPMOperations) -> Self {
        SimpleAttestation { tpm_ops }
    }
}

impl Attestation for SimpleAttestation {
    fn generate_attestation(&self, _tpm_id: TPMID, nonce: &[u8]) -> Result<Vec<u8>, TPMError> {
        let mut attestation = Vec::new();
        for &byte in nonce {
            attestation.push(byte);
        }
        attestation.push(0xAA);
        attestation.push(0xBB);
        Ok(attestation)
    }

    fn verify_attestation(&self, attestation: &[u8], nonce: &[u8]) -> Result<bool, TPMError> {
        if attestation.len() >= 2 && attestation[attestation.len() - 2] == 0xAA {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
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
    }
}
