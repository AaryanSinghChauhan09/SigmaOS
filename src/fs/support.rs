#![no_std]
#![no_main]

/// OOP-based Filesystem Support for SigmaOS
/// Implements filesystem using OOP principles with traits and structs
/// No dependency on external filesystem frameworks
/// Based on Roadmap Item 7: Filesystem support

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// File ID
pub type FileID = usize;

/// File type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Regular = 0,
    Directory = 1,
    Symlink = 2,
    Device = 3,
}

/// File trait (OOP interface)
pub trait File {
    /// Get file ID
    fn id(&self) -> FileID;
    /// Get file name
    fn name(&self) -> &[u8];
    /// Get file type
    fn file_type(&self) -> FileType;
    /// Read file
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, FSError>;
    /// Write file
    fn write(&mut self, data: &[u8]) -> Result<usize, FSError>;
    /// Get file info
    fn info(&self) -> FileInfo;
}

/// FS error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FSError {
    Success = 0,
    NotFound = 1,
    PermissionDenied = 2,
    InvalidOperation = 3,
}

/// File info
#[repr(C)]
pub struct FileInfo {
    pub id: FileID,
    pub name: [u8; 64],
    pub file_type: FileType,
    pub size: u64,
    pub capability: FileCapability,
}

impl FileInfo {
    pub fn new(id: FileID, file_type: FileType) -> Self {
        FileInfo {
            id,
            name: [0; 64],
            file_type,
            size: 0,
            capability: FileCapability::new(),
        }
    }
}

/// File capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FileCapability {
    pub can_read: bool,
    pub can_write: bool,
}

impl FileCapability {
    pub fn new() -> Self {
        FileCapability {
            can_read: false,
            can_write: false,
        }
    }

    pub fn full() -> Self {
        FileCapability {
            can_read: true,
            can_write: true,
        }
    }
}

/// Simple file (OOP: Concrete file class)
#[repr(C)]
pub struct SimpleFile {
    pub id: FileID,
    pub name: [u8; 64],
    pub file_type: FileType,
    pub data: [u8; 4096],
    pub size: AtomicUsize,
    pub capability: FileCapability,
}

impl SimpleFile {
    pub fn new(id: FileID, name: &[u8], file_type: FileType, capability: FileCapability) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }

        SimpleFile {
            id,
            name: name_array,
            file_type,
            data: [0; 4096],
            size: AtomicUsize::new(0),
            capability,
        }
    }
}

impl File for SimpleFile {
    fn id(&self) -> FileID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, FSError> {
        if !self.capability.can_read {
            return Err(FSError::PermissionDenied);
        }

        let current_size = self.size.load(Ordering::SeqCst);
        let bytes_to_read = buffer.len().min(current_size);

        unsafe {
            core::ptr::copy_nonoverlapping(self.data.as_ptr(), buffer.as_mut_ptr(), bytes_to_read);
        }

        Ok(bytes_to_read)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, FSError> {
        if !self.capability.can_write {
            return Err(FSError::PermissionDenied);
        }

        let bytes_to_write = data.len().min(4096);
        let current_size = self.size.load(Ordering::SeqCst);

        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), self.data.as_mut_ptr(), bytes_to_write);
        }

        self.size.store(bytes_to_write, Ordering::SeqCst);
        Ok(bytes_to_write)
    }

    fn info(&self) -> FileInfo {
        FileInfo {
            id: self.id,
            name: self.name,
            file_type: self.file_type,
            size: self.size.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

/// Filesystem trait (OOP interface)
pub trait Filesystem {
    /// Create file
    fn create_file(&mut self, name: &[u8], file_type: FileType) -> Result<FileID, FSError>;
    /// Delete file
    fn delete_file(&mut self, id: FileID) -> Result<(), FSError>;
    /// Get file
    fn get_file(&self, id: FileID) -> Option<&dyn File>;
    /// List files
    fn list_files(&self) -> Vec<FileID>;
    /// Get filesystem statistics
    fn stats(&self) -> FSStats;
}

/// FS statistics
#[repr(C)]
pub struct FSStats {
    pub total_files: usize,
    pub total_size: u64,
    pub by_type: [usize; 4],
}

impl FSStats {
    pub fn new() -> Self {
        FSStats {
            total_files: 0,
            total_size: 0,
            by_type: [0; 4],
        }
    }
}

/// Simple filesystem (OOP: Concrete filesystem class)
pub struct SimpleFilesystem {
    files: Vec<Option<Box<dyn File>>>,
    next_id: AtomicUsize,
    stats: FSStats,
    capability: FSCapability,
}

/// FS capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FSCapability {
    pub can_create: bool,
    pub can_delete: bool,
}

impl FSCapability {
    pub fn new() -> Self {
        FSCapability {
            can_create: false,
            can_delete: false,
        }
    }

    pub fn full() -> Self {
        FSCapability {
            can_create: true,
            can_delete: true,
        }
    }
}

impl SimpleFilesystem {
    pub fn new(capability: FSCapability) -> Self {
        SimpleFilesystem {
            files: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: FSStats::new(),
            capability,
        }
    }
}

impl Filesystem for SimpleFilesystem {
    fn create_file(&mut self, name: &[u8], file_type: FileType) -> Result<FileID, FSError> {
        if !self.capability.can_create {
            return Err(FSError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let file = SimpleFile::new(id, name, file_type, FileCapability::full());
        self.files.push(Some(Box::new(file)));
        self.stats.total_files += 1;
        self.stats.by_type[file_type as usize] += 1;
        Ok(id)
    }

    fn delete_file(&mut self, id: FileID) -> Result<(), FSError> {
        if !self.capability.can_delete {
            return Err(FSError::PermissionDenied);
        }

        let mut index = None;
        let mut file_type = FileType::Regular;

        for (i, file_option) in self.files.iter().enumerate() {
            if let Some(ref file) = *file_option {
                if file.id() == id {
                    index = Some(i);
                    file_type = file.file_type();
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.files[i] = None;
            self.stats.total_files -= 1;
            self.stats.by_type[file_type as usize] -= 1;
            Ok(())
        } else {
            Err(FSError::NotFound)
        }
    }

    fn get_file(&self, id: FileID) -> Option<&dyn File> {
        for file_option in &self.files {
            if let Some(ref file) = *file_option {
                if file.id() == id {
                    return Some(file.as_ref());
                }
            }
        }
        None
    }

    fn list_files(&self) -> Vec<FileID> {
        let mut ids = Vec::new();
        for file_option in &self.files {
            if let Some(ref file) = *file_option {
                ids.push(file.id());
            }
        }
        ids
    }

    fn stats(&self) -> FSStats {
        self.stats
    }
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
