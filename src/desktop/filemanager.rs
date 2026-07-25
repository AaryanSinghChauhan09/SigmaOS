#![no_std]
#![no_main]

/// OOP-based File Manager for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 766
/// Implements file browser and management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FileID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FileType { Directory = 0, File = 1, Symlink = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FileManagerError { Success = 0, NotFound = 1, PermissionDenied = 2 }

pub trait FileEntry {
    fn id(&self) -> FileID;
    fn name(&self) -> &[u8];
    fn file_type(&self) -> FileType;
    fn size(&self) -> u64;
    fn is_hidden(&self) -> bool;
}

#[repr(C)]
pub struct SimpleFileEntry {
    pub id: FileID,
    pub name: [u8; 256],
    pub file_type: AtomicUsize,
    pub size: AtomicUsize,
    pub hidden: AtomicUsize,
}

impl SimpleFileEntry {
    pub fn new(id: FileID, name: &[u8], file_type: FileType, size: u64) -> Self {
        let mut name_array = [0u8; 256];
        let name_len = name.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleFileEntry {
            id,
            name: name_array,
            file_type: AtomicUsize::new(file_type as usize),
            size: AtomicUsize::new(size as usize),
            hidden: AtomicUsize::new(0),
        }
    }
}

impl FileEntry for SimpleFileEntry {
    fn id(&self) -> FileID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(256);
        &self.name[..len]
    }
    fn file_type(&self) -> FileType { unsafe { core::mem::transmute(self.file_type.load(Ordering::SeqCst)) } }
    fn size(&self) -> u64 { self.size.load(Ordering::SeqCst) as u64 }
    fn is_hidden(&self) -> bool { self.hidden.load(Ordering::SeqCst) == 1 }
}

pub trait FileManager {
    fn list_directory(&self, path: &[u8]) -> Result<Vec<&dyn FileEntry>, FileManagerError>;
    fn create_directory(&mut self, path: &[u8], name: &[u8]) -> Result<FileID, FileManagerError>;
    fn delete_file(&mut self, id: FileID) -> Result<(), FileManagerError>;
    fn copy_file(&mut self, source_id: FileID, dest_path: &[u8]) -> Result<(), FileManagerError>;
}

#[repr(C)]
pub struct SimpleFileManager {
    pub files: Vec<Option<Box<dyn FileEntry>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFileManager {
    pub fn new() -> Self {
        SimpleFileManager {
            files: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FileManager for SimpleFileManager {
    fn list_directory(&self, _path: &[u8]) -> Result<Vec<&dyn FileEntry>, FileManagerError> {
        let mut entries = Vec::new();
        for file_option in &self.files {
            if let Some(ref file) = *file_option {
                entries.push(file.as_ref());
            }
        }
        Ok(entries)
    }
    
    fn create_directory(&mut self, _path: &[u8], name: &[u8]) -> Result<FileID, FileManagerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let entry = SimpleFileEntry::new(id, name, FileType::Directory, 4096);
        self.files.push(Some(Box::new(entry)));
        Ok(id)
    }
    
    fn delete_file(&mut self, id: FileID) -> Result<(), FileManagerError> {
        for file_option in &mut self.files {
            if let Some(ref file) = *file_option {
                if file.id() == id {
                    return Ok(());
                }
            }
        }
        Err(FileManagerError::NotFound)
    }
    
    fn copy_file(&mut self, source_id: FileID, _dest_path: &[u8]) -> Result<(), FileManagerError> {
        if self.get_file(source_id).is_some() {
            Ok(())
        } else {
            Err(FileManagerError::NotFound)
        }
    }
    
    fn get_file(&self, id: FileID) -> Option<&dyn FileEntry> {
        for file_option in &self.files {
            if let Some(ref file) *file_option {
                if file.id() == id { return Some(file.as_ref()); }
            }
        }
        None
    }
}

pub trait FileSearch {
    fn search(&self, query: &[u8]) -> Vec<FileID>;
    fn filter_by_type(&self, file_type: FileType) -> Vec<FileID>;
}

#[repr(C)]
pub struct SimpleFileSearch {
    pub manager: SimpleFileManager,
}

impl SimpleFileSearch {
    pub fn new(manager: SimpleFileManager) -> Self {
        SimpleFileSearch { manager }
    }
}

impl FileSearch for SimpleFileSearch {
    fn search(&self, query: &[u8]) -> Vec<FileID> {
        let mut results = Vec::new();
        for file_option in &self.manager.files {
            if let Some(ref file) = *file_option {
                if file.name().contains(query) {
                    results.push(file.id());
                }
            }
        }
        results
    }
    
    fn filter_by_type(&self, file_type: FileType) -> Vec<FileID> {
        let mut results = Vec::new();
        for file_option in &self.manager.files {
            if let Some(ref file) = *file_option {
                if file.file_type() == file_type {
                    results.push(file.id());
                }
            }
        }
        results
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
