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
use std::boxed::Box;
use std::string::{String, ToString};
use std::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Fingerprint Scanner for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 562
/// Implements fingerprint capture and authentication

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FingerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ScanError { Success = 0, NotFound = 1, ScanFailed = 2, NoMatch = 3 }

pub trait FingerprintTemplate {
    fn id(&self) -> FingerID;
    fn data(&self) -> &[u8];
    fn quality(&self) -> u32;
}

#[repr(C)]
pub struct SimpleFingerprintTemplate {
    pub id: FingerID,
    pub data: [u8; 512],
    pub data_len: u16,
    pub quality: AtomicUsize,
}

impl SimpleFingerprintTemplate {
    pub fn new(id: FingerID, data: &[u8], quality: u32) -> Self {
        let mut data_array = [0u8; 512];
        let data_len = data.len().min(511);
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), data_array.as_mut_ptr(), data_len);
        }
        SimpleFingerprintTemplate {
            id,
            data: data_array,
            data_len: data_len as u16,
            quality: AtomicUsize::new(quality as usize),
        }
    }
}

impl FingerprintTemplate for SimpleFingerprintTemplate {
    fn id(&self) -> FingerID { self.id }
    fn data(&self) -> &[u8] {
        // Bolt ⚡ Optimization: Store explicit template byte length on instantiation to eliminate
        // O(N) zero-byte linear scanning (.position(|&b| b == 0)) on every fingerprint template data access,
        // reducing slice lookup to instantaneous O(1) constant time.
        &self.data[..self.data_len as usize]
    }
    fn quality(&self) -> u32 { self.quality.load(Ordering::SeqCst) as u32 }
}

pub trait FingerprintScanner {
    fn scan(&mut self) -> Result<Box<dyn FingerprintTemplate>, ScanError>;
    fn enroll(&mut self, _user_id: usize) -> Result<FingerID, ScanError>;
    fn verify(&self, template: &dyn FingerprintTemplate) -> Result<bool, ScanError>;
}

#[repr(C)]
pub struct SimpleFingerprintScanner {
    pub templates: Vec<Option<Box<dyn FingerprintTemplate>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFingerprintScanner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleFingerprintScanner {
            templates: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FingerprintScanner for SimpleFingerprintScanner {
    fn scan(&mut self) -> Result<Box<dyn FingerprintTemplate>, ScanError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let template = SimpleFingerprintTemplate::new(id, b"fingerprint_data", 95);
        Ok(Box::new(template))
    }

    fn enroll(&mut self, _user_id: usize) -> Result<FingerID, ScanError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let template = SimpleFingerprintTemplate::new(id, b"enrolled_template", 90);
        self.templates.push(Some(Box::new(template)));
        Ok(id)
    }

    fn verify(&self, _template: &dyn FingerprintTemplate) -> Result<bool, ScanError> {
        for stored_option in &self.templates {
            if let Some(ref stored) = *stored_option {
                if stored.quality() > 80 {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

pub trait BiometricAuth {
    fn authenticate(&mut self, _fingerprint: &dyn FingerprintTemplate) -> Result<usize, ScanError>;
    fn register_user(&mut self, user_id: usize, template: Box<dyn FingerprintTemplate>) -> Result<(), ScanError>;
}

#[repr(C)]
pub struct SimpleBiometricAuth {
    pub users: Vec<(usize, Box<dyn FingerprintTemplate>)>,
}

impl SimpleBiometricAuth {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleBiometricAuth {
            users: Vec::new(),
        }
    }
}

impl BiometricAuth for SimpleBiometricAuth {
    fn authenticate(&mut self, _fingerprint: &dyn FingerprintTemplate) -> Result<usize, ScanError> {
        for &(user_id, ref template) in &self.users {
            if template.quality() > 80 {
                return Ok(user_id);
            }
        }
        Err(ScanError::NoMatch)
    }

    fn register_user(&mut self, user_id: usize, template: Box<dyn FingerprintTemplate>) -> Result<(), ScanError> {
        self.users.push((user_id, template));
        Ok(())
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
        let new_layout = core::alloc::Layout::array::<T>(new_capacity).unwrap();
        let new_data = alloc::alloc::alloc(new_layout) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 {
                let old_layout = core::alloc::Layout::array::<T>(self.capacity).unwrap();
                alloc::alloc::dealloc(self.data as *mut u8, old_layout);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_fingerprint_template_o1_lookup() {
        let raw_data = b"fingerprint_template_sample_bytes_12345";
        let template = SimpleFingerprintTemplate::new(1, raw_data, 95);
        assert_eq!(template.id(), 1);
        assert_eq!(template.data(), raw_data);
        assert_eq!(template.quality(), 95);
    }

    #[test]
    fn test_simple_fingerprint_scanner() {
        let mut scanner = SimpleFingerprintScanner::new();
        let scanned = scanner.scan().unwrap();
        assert_eq!(scanned.data(), b"fingerprint_data");

        let enrolled_id = scanner.enroll(42).unwrap();
        assert!(enrolled_id > 0);
        assert!(scanner.verify(scanned.as_ref()).unwrap());
    }
}
