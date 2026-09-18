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
use std::vec::Vec;
use std::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Cloud Storage for SigmaOS
/// Based on Ideas-999-Structured: Cloud & Remote Item 946
/// Implements cloud storage integration

#[cfg(not(target_os = "none"))]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(target_os = "none"))]
use core::mem;
#[cfg(not(target_os = "none"))]
use core::ops::{Deref, DerefMut};

#[cfg(target_os = "none")]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(target_os = "none")]
use core::mem;
#[cfg(target_os = "none")]
use core::ops::{Deref, DerefMut};

pub type FileID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum StorageError { Success = 0, NotFound = 1, UploadFailed = 2 }

pub trait CloudFile {
    fn id(&self) -> FileID;
    fn name(&self) -> &[u8];
    fn size(&self) -> u64;
    fn is_cached(&self) -> bool;
}

#[repr(C)]
pub struct SimpleCloudFile {
    pub id: FileID,
    pub name: [u8; 256],
    pub size: AtomicUsize,
    pub cached: AtomicUsize,
}

impl SimpleCloudFile {
    pub fn new(id: FileID, name: &[u8], size: u64) -> Self {
        let mut name_array = [0u8; 256];
        let name_len = name.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleCloudFile {
            id,
            name: name_array,
            size: AtomicUsize::new(size as usize),
            cached: AtomicUsize::new(0),
        }
    }
}

impl CloudFile for SimpleCloudFile {
    fn id(&self) -> FileID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(256);
        &self.name[..len]
    }
    fn size(&self) -> u64 { self.size.load(Ordering::SeqCst) as u64 }
    fn is_cached(&self) -> bool { self.cached.load(Ordering::SeqCst) == 1 }
}

pub trait CloudStorage {
    fn upload(&mut self, _local_path: &[u8], _remote_path: &[u8]) -> Result<FileID, StorageError>;
    fn download(&self, remote_path: &[u8], local_path: &[u8]) -> Result<(), StorageError>;
    fn list_files(&self, path: &[u8]) -> Result<Vec<&dyn CloudFile>, StorageError>;
}

#[repr(C)]
pub struct SimpleCloudStorage {
    pub files: Vec<Option<Box<dyn CloudFile>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCloudStorage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleCloudStorage {
            files: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CloudStorage for SimpleCloudStorage {
    fn upload(&mut self, _local_path: &[u8], remote_path: &[u8]) -> Result<FileID, StorageError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let file = SimpleCloudFile::new(id, remote_path, 1024);
        self.files.push(Some(Box::new(file)));
        Ok(id)
    }
    
    fn download(&self, _remote_path: &[u8], _local_path: &[u8]) -> Result<(), StorageError> {
        Ok(())
    }
    
    fn list_files(&self, _path: &[u8]) -> Result<Vec<&dyn CloudFile>, StorageError> {
        let mut files = Vec::new();
        for file_option in &self.files {
            if let Some(ref file) = *file_option {
                files.push(file.as_ref());
            }
        }
        Ok(files)
    }
}

pub trait CloudProvider {
    fn connect(&mut self, provider: &[u8], credentials: &[u8]) -> Result<(), StorageError>;
    fn disconnect(&mut self);
    fn is_connected(&self) -> bool;
}

// ================= AWS S3 & GCP Storage-inspired SDK & Lifecycle Engine =================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageClass {
    Standard,
    InfrequentAccess,
    Glacier,
}

#[derive(Clone)]
pub struct S3Object {
    pub bucket: std::string::String,
    pub key: std::string::String,
    pub size_bytes: u64,
    pub storage_class: StorageClass,
    pub last_modified_timestamp: u64,
}

#[derive(Clone)]
pub struct MultipartUploadSession {
    pub upload_id: u32,
    pub bucket: std::string::String,
    pub key: std::string::String,
    pub total_parts_expected: usize,
    pub uploaded_parts_count: usize,
}

pub struct PresignedUrl {
    pub url: std::string::String,
    pub expiration_timestamp: u64,
    pub signature_token: u32,
}

pub struct SovereignS3Bucket {
    pub bucket_name: std::string::String,
    pub objects: std::vec::Vec<S3Object>,
    pub active_multipart_uploads: std::vec::Vec<MultipartUploadSession>,
    pub lifecycle_transition_days_ia: u32,
    pub lifecycle_transition_days_glacier: u32,
}

impl SovereignS3Bucket {
    pub fn new(name: &str) -> Self {
        Self {
            bucket_name: name.to_string(),
            objects: std::vec::Vec::new(),
            active_multipart_uploads: std::vec::Vec::new(),
            lifecycle_transition_days_ia: 30,
            lifecycle_transition_days_glacier: 90,
        }
    }

    pub fn initiate_multipart_upload(&mut self, upload_id: u32, key: &str, parts: usize) {
        self.active_multipart_uploads.push(MultipartUploadSession {
            upload_id,
            bucket: self.bucket_name.clone(),
            key: key.to_string(),
            total_parts_expected: parts,
            uploaded_parts_count: 0,
        });
    }

    pub fn upload_part(&mut self, upload_id: u32, part_num: usize) -> Result<(), &'static str> {
        let session = self.active_multipart_uploads.iter_mut()
            .find(|u| u.upload_id == upload_id)
            .ok_or("S3 SDK: Multipart upload ID not active")?;

        if part_num > session.total_parts_expected {
            return Err("S3 SDK: Part number exceeds expected boundary limit");
        }
        session.uploaded_parts_count += 1;
        Ok(())
    }

    pub fn complete_multipart_upload(&mut self, upload_id: u32, size: u64) -> Result<(), &'static str> {
        let pos = self.active_multipart_uploads.iter()
            .position(|u| u.upload_id == upload_id)
            .ok_or("S3 SDK: Multipart upload session not found")?;

        let session = &self.active_multipart_uploads[pos];
        if session.uploaded_parts_count < session.total_parts_expected {
            return Err("S3 SDK: Incomplete multipart upload. Missing parts.");
        }

        let key = session.key.clone();
        self.active_multipart_uploads.remove(pos);

        self.objects.push(S3Object {
            bucket: self.bucket_name.clone(),
            key,
            size_bytes: size,
            storage_class: StorageClass::Standard,
            last_modified_timestamp: 0, // start epoch
        });

        Ok(())
    }

    pub fn generate_presigned_get_url(&self, key: &str, duration_secs: u64, current_time: u64) -> Result<PresignedUrl, &'static str> {
        let obj = self.objects.iter().find(|o| o.key == key).ok_or("S3 SDK: Object key not found")?;

        let expiration = current_time + duration_secs;
        let mut signature: u32 = 5381;
        for byte in key.bytes() {
            signature = signature.wrapping_mul(33).wrapping_add(byte as u32);
        }
        signature = signature.wrapping_mul(33).wrapping_add(expiration as u32);

        Ok(PresignedUrl {
            url: std::format!("https://s3.sigma.os/{}/{}?signature={:x}", self.bucket_name, key, signature),
            expiration_timestamp: expiration,
            signature_token: signature,
        })
    }

    /// Evaluates bucket lifecycle policy to transition cold files to IA or Glacier
    pub fn process_lifecycle_policies(&mut self, current_age_days: u32) -> usize {
        let mut transitioned_count = 0;
        for obj in &mut self.objects {
            if current_age_days >= self.lifecycle_transition_days_glacier && obj.storage_class != StorageClass::Glacier {
                obj.storage_class = StorageClass::Glacier;
                transitioned_count += 1;
            } else if current_age_days >= self.lifecycle_transition_days_ia && obj.storage_class == StorageClass::Standard {
                obj.storage_class = StorageClass::InfrequentAccess;
                transitioned_count += 1;
            }
        }
        transitioned_count
    }
}

#[repr(C)]
pub struct SimpleCloudProvider {
    pub connected: AtomicUsize,
    pub provider: [u8; 32],
}

impl SimpleCloudProvider {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleCloudProvider {
            connected: AtomicUsize::new(0),
            provider: [0u8; 32],
        }
    }
}

impl CloudProvider for SimpleCloudProvider {
    fn connect(&mut self, provider: &[u8], _credentials: &[u8]) -> Result<(), StorageError> {
        let provider_len = provider.len().min(31);
        for i in 0..provider_len {
            self.provider[i] = provider[i];
        }
        self.connected.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn disconnect(&mut self) {
        self.connected.store(0, Ordering::SeqCst);
    }
    
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
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
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s3_multipart_and_presigned_urls() {
        let mut bucket = SovereignS3Bucket::new("sovereign-data");
        bucket.initiate_multipart_upload(101, "kernel-image.bin", 3);

        assert!(bucket.upload_part(101, 1).is_ok());
        assert!(bucket.upload_part(101, 2).is_ok());
        assert!(bucket.upload_part(101, 3).is_ok());
        assert!(bucket.upload_part(101, 4).is_err()); // Exceeds expected parts

        assert!(bucket.complete_multipart_upload(101, 1024 * 1024 * 15).is_ok());
        assert_eq!(bucket.objects.len(), 1);
        assert_eq!(bucket.objects[0].key, "kernel-image.bin");
        assert_eq!(bucket.objects[0].storage_class, StorageClass::Standard);

        // Generate and verify presigned URL
        let presigned = bucket.generate_presigned_get_url("kernel-image.bin", 3600, 1000).unwrap();
        assert!(presigned.url.contains("signature="));
        assert_eq!(presigned.expiration_timestamp, 4600);
    }

    #[test]
    fn test_s3_lifecycle_transitions() {
        let mut bucket = SovereignS3Bucket::new("backup-bucket");
        bucket.objects.push(S3Object {
            bucket: "backup-bucket".to_string(),
            key: "db-dump.sql".to_string(),
            size_bytes: 1024 * 512,
            storage_class: StorageClass::Standard,
            last_modified_timestamp: 0,
        });

        // 10 days - no transition
        assert_eq!(bucket.process_lifecycle_policies(10), 0);
        assert_eq!(bucket.objects[0].storage_class, StorageClass::Standard);

        // 45 days - transition to IA
        assert_eq!(bucket.process_lifecycle_policies(45), 1);
        assert_eq!(bucket.objects[0].storage_class, StorageClass::InfrequentAccess);

        // 100 days - transition to Glacier
        assert_eq!(bucket.process_lifecycle_policies(100), 1);
        assert_eq!(bucket.objects[0].storage_class, StorageClass::Glacier);
    }
}
