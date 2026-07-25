#![no_std]
#![no_main]

/// OOP-based Filesystem Abstraction for SigmaOS
/// Implements filesystem using OOP principles with traits and structs
/// No dependency on std::fs

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Filesystem trait (OOP interface)
pub trait Filesystem {
    /// Open a file
    fn open(&self, path: &[u8], flags: FileFlags) -> Result<FileHandle, FilesystemError>;
    /// Close a file
    fn close(&self, handle: FileHandle) -> Result<(), FilesystemError>;
    /// Read from file
    fn read(&self, handle: FileHandle, buffer: &mut [u8]) -> Result<usize, FilesystemError>;
    /// Write to file
    fn write(&self, handle: FileHandle, buffer: &[u8]) -> Result<usize, FilesystemError>;
    /// Seek in file
    fn seek(&self, handle: FileHandle, offset: isize, origin: SeekOrigin) -> Result<isize, FilesystemError>;
    /// Create directory
    fn mkdir(&self, path: &[u8]) -> Result<(), FilesystemError>;
    /// Remove directory
    fn rmdir(&self, path: &[u8]) -> Result<(), FilesystemError>;
    /// Delete file
    fn unlink(&self, path: &[u8]) -> Result<(), FilesystemError>;
    /// Get file info
    fn stat(&self, path: &[u8]) -> Result<FileInfo, FilesystemError>;
}

/// Filesystem error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FilesystemError {
    Success = 0,
    NotFound = 1,
    PermissionDenied = 2,
    AlreadyExists = 3,
    InvalidPath = 4,
    NotDirectory = 5,
    IsDirectory = 6,
    NoSpace = 7,
    IoError = 8,
}

/// File flags
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FileFlags {
    pub read: bool,
    pub write: bool,
    pub create: bool,
    pub truncate: bool,
    pub append: bool,
}

impl FileFlags {
    pub fn new() -> Self {
        FileFlags {
            read: false,
            write: false,
            create: false,
            truncate: false,
            append: false,
        }
    }

    pub fn read_only() -> Self {
        let mut flags = FileFlags::new();
        flags.read = true;
        flags
    }

    pub fn write_only() -> Self {
        let mut flags = FileFlags::new();
        flags.write = true;
        flags
    }

    pub fn read_write() -> Self {
        let mut flags = FileFlags::new();
        flags.read = true;
        flags.write = true;
        flags
    }
}

/// Seek origin
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SeekOrigin {
    Set = 0,
    Current = 1,
    End = 2,
}

/// File handle
#[repr(C)]
pub struct FileHandle {
    pub id: usize,
    pub offset: AtomicUsize,
    pub flags: FileFlags,
}

impl FileHandle {
    pub fn new(id: usize, flags: FileFlags) -> Self {
        FileHandle {
            id,
            offset: AtomicUsize::new(0),
            flags,
        }
    }
}

/// File info
#[repr(C)]
pub struct FileInfo {
    pub size: u64,
    pub is_directory: bool,
    pub is_file: bool,
    pub permissions: u32,
    pub modified_time: u64,
    pub created_time: u64,
}

impl FileInfo {
    pub fn new() -> Self {
        FileInfo {
            size: 0,
            is_directory: false,
            is_file: true,
            permissions: 0o644,
            modified_time: 0,
            created_time: 0,
        }
    }
}

/// Inode (OOP: Filesystem node object)
#[repr(C)]
pub struct Inode {
    pub id: u64,
    pub file_info: FileInfo,
    pub data: Option<NonNull<u8>>,
    pub data_size: usize,
    pub ref_count: AtomicUsize,
}

impl Inode {
    pub fn new(id: u64) -> Self {
        Inode {
            id,
            file_info: FileInfo::new(),
            data: None,
            data_size: 0,
            ref_count: AtomicUsize::new(0),
        }
    }

    pub unsafe fn allocate_data(&mut self, size: usize) -> bool {
        let data = alloc(size);
        if data.is_null() {
            return false;
        }

        if let Some(old_data) = self.data {
            free(old_data.as_ptr());
        }

        self.data = Some(NonNull::new_unchecked(data));
        self.data_size = size;
        self.file_info.size = size as u64;
        true
    }

    pub unsafe fn free_data(&mut self) {
        if let Some(data) = self.data {
            free(data.as_ptr());
            self.data = None;
            self.data_size = 0;
            self.file_info.size = 0;
        }
    }

    pub fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_ref(&self) -> usize {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}

impl Drop for Inode {
    fn drop(&mut self) {
        unsafe {
            self.free_data();
        }
    }
}

/// Directory entry
#[repr(C)]
pub struct DirectoryEntry {
   pub name: [u8; 256],
    pub inode_id: u64,
}

impl DirectoryEntry {
    pub fn new(name: &[u8], inode_id: u64) -> Self {
        let mut entry = DirectoryEntry {
            name: [0; 256],
            inode_id,
        };

        let len = name.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), entry.name.as_mut_ptr(), len);
        }

        entry
    }
}

/// Directory (OOP: Directory object)
#[repr(C)]
pub struct Directory {
    pub inode_id: u64,
    pub entries: Vec<DirectoryEntry>,
}

impl Directory {
    pub fn new(inode_id: u64) -> Self {
        Directory {
            inode_id,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: DirectoryEntry) {
        self.entries.push(entry);
    }

    pub fn find_entry(&self, name: &[u8]) -> Option<&DirectoryEntry> {
        for entry in &self.entries {
            let entry_name_len = entry.name.iter().position(|&b| b == 0).unwrap_or(256);
            if &entry.name[..entry_name_len] == name {
                return Some(entry);
            }
        }
        None
    }

    pub fn remove_entry(&mut self, name: &[u8]) -> bool {
        let mut index = None;
        for (i, entry) in self.entries.iter().enumerate() {
            let entry_name_len = entry.name.iter().position(|&b| b == 0).unwrap_or(256);
            if &entry.name[..entry_name_len] == name {
                index = Some(i);
                break;
            }
        }

        if let Some(i) = index {
            self.entries.remove(i);
            true
        } else {
            false
        }
    }
}

/// Simple Vec implementation for no_std
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
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

    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            core::ptr::copy(self.data.add(index + 1), self.data.add(index), self.len - index - 1);
            self.len -= 1;
            item
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            data: self.data,
            len: self.len,
            index: 0,
        }
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

pub struct Iter<T> {
    data: *const T,
    len: usize,
    index: usize,
}

impl<T> Iterator for Iter<T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            unsafe {
                let item = &*self.data.add(self.index);
                self.index += 1;
                Some(item)
            }
        } else {
            None
        }
    }
}

/// Memory filesystem (OOP: Concrete Filesystem implementation)
pub struct MemoryFilesystem {
    inodes: Vec<Option<NonNull<Inode>>>,
    directories: Vec<Option<NonNull<Directory>>>,
    next_inode_id: AtomicUsize,
    next_handle_id: AtomicUsize,
}

impl MemoryFilesystem {
    pub fn new() -> Self {
        let mut fs = MemoryFilesystem {
            inodes: Vec::new(),
            directories: Vec::new(),
            next_inode_id: AtomicUsize::new(1),
            next_handle_id: AtomicUsize::new(1),
        };

        // Create root directory
        unsafe {
            let root_inode = Inode::new(0);
            root_inode.file_info.is_directory = true;
            root_inode.file_info.is_file = false;

            let root_inode_ptr = alloc(mem::size_of::<Inode>()) as *mut Inode;
            if !root_inode_ptr.is_null() {
                core::ptr::write(root_inode_ptr, root_inode);
                fs.inodes.push(Some(NonNull::new_unchecked(root_inode_ptr)));

                let root_dir = Directory::new(0);
                let root_dir_ptr = alloc(mem::size_of::<Directory>()) as *mut Directory;
                if !root_dir_ptr.is_null() {
                    core::ptr::write(root_dir_ptr, root_dir);
                    fs.directories.push(Some(NonNull::new_unchecked(root_dir_ptr)));
                }
            }
        }

        fs
    }

    unsafe fn allocate_inode(&mut self) -> Option<u64> {
        let id = self.next_inode_id.fetch_add(1, Ordering::SeqCst);
        let inode = Inode::new(id);
        let inode_ptr = alloc(mem::size_of::<Inode>()) as *mut Inode;

        if inode_ptr.is_null() {
            return None;
        }

        core::ptr::write(inode_ptr, inode);
        self.inodes.push(Some(NonNull::new_unchecked(inode_ptr)));

        Some(id)
    }

    unsafe fn get_inode(&self, id: u64) -> Option<&Inode> {
        if id as usize >= self.inodes.len() {
            return None;
        }

        self.inodes[id as usize].map(|ptr| &*ptr.as_ptr())
    }

    unsafe fn get_inode_mut(&mut self, id: u64) -> Option<&mut Inode> {
        if id as usize >= self.inodes.len() {
            return None;
        }

        self.inodes[id as usize].map(|mut ptr| &mut *ptr.as_ptr())
    }

    unsafe fn get_directory(&self, inode_id: u64) -> Option<&Directory> {
        for dir_option in &self.directories {
            if let Some(dir_ptr) = *dir_option {
                let dir = &*dir_ptr.as_ptr();
                if dir.inode_id == inode_id {
                    return Some(dir);
                }
            }
        }
        None
    }

    unsafe fn get_directory_mut(&mut self, inode_id: u64) -> Option<&mut Directory> {
        for dir_option in &mut self.directories {
            if let Some(mut dir_ptr) = *dir_option {
                let dir = &mut *dir_ptr.as_ptr();
                if dir.inode_id == inode_id {
                    return Some(dir);
                }
            }
        }
        None
    }

    unsafe fn resolve_path(&self, path: &[u8]) -> Result<u64, FilesystemError> {
        if path.is_empty() || path[0] != b'/' {
            return Err(FilesystemError::InvalidPath);
        }

        let mut current_inode_id = 0u64; // Root
        let mut path_start = 1;

        while path_start < path.len() {
            // Find next path component
            let mut path_end = path_start;
            while path_end < path.len() && path[path_end] != b'/' {
                path_end += 1;
            }

            let component = &path[path_start..path_end];

            if component.is_empty() {
                path_start = path_end + 1;
                continue;
            }

            // Look up in current directory
            if let Some(dir) = self.get_directory(current_inode_id) {
                if let Some(entry) = dir.find_entry(component) {
                    current_inode_id = entry.inode_id;
                } else {
                    return Err(FilesystemError::NotFound);
                }
            } else {
                return Err(FilesystemError::NotDirectory);
            }

            path_start = path_end + 1;
        }

        Ok(current_inode_id)
    }
}

impl Filesystem for MemoryFilesystem {
    fn open(&self, path: &[u8], flags: FileFlags) -> Result<FileHandle, FilesystemError> {
        unsafe {
            let inode_id = self.resolve_path(path)?;

            if let Some(inode) = self.get_inode(inode_id) {
                if inode.file_info.is_directory {
                    return Err(FilesystemError::IsDirectory);
                }

                let handle_id = self.next_handle_id.fetch_add(1, Ordering::SeqCst);
                Ok(FileHandle::new(handle_id, flags))
            } else {
                Err(FilesystemError::NotFound)
            }
        }
    }

    fn close(&self, _handle: FileHandle) -> Result<(), FilesystemError> {
        Ok(())
    }

    fn read(&self, handle: FileHandle, buffer: &mut [u8]) -> Result<usize, FilesystemError> {
        unsafe {
            // In a real implementation, this would read from the inode data
            // For now, return success
            Ok(buffer.len())
        }
    }

    fn write(&self, handle: FileHandle, buffer: &[u8]) -> Result<usize, FilesystemError> {
        unsafe {
            // In a real implementation, this would write to the inode data
            // For now, return success
            Ok(buffer.len())
        }
    }

    fn seek(&self, handle: FileHandle, offset: isize, origin: SeekOrigin) -> Result<isize, FilesystemError> {
        let current = handle.offset.load(Ordering::SeqCst) as isize;
        let new_offset = match origin {
            SeekOrigin::Set => offset,
            SeekOrigin::Current => current + offset,
            SeekOrigin::End => {
                // In a real implementation, this would get file size
                current + offset
            }
        };

        if new_offset < 0 {
            return Err(FilesystemError::IoError);
        }

        handle.offset.store(new_offset as usize, Ordering::SeqCst);
        Ok(new_offset)
    }

    fn mkdir(&mut self, path: &[u8]) -> Result<(), FilesystemError> {
        unsafe {
            let parent_path = self.get_parent_path(path);
            let dir_name = self.get_last_component(path);

            let parent_inode_id = self.resolve_path(parent_path)?;
            let dir_inode_id = self.allocate_inode().ok_or(FilesystemError::NoSpace)?;

            if let Some(inode) = self.get_inode_mut(dir_inode_id) {
                inode.file_info.is_directory = true;
                inode.file_info.is_file = false;
            }

            let dir = Directory::new(dir_inode_id);
            let dir_ptr = alloc(mem::size_of::<Directory>()) as *mut Directory;
            if dir_ptr.is_null() {
                return Err(FilesystemError::NoSpace);
            }

            core::ptr::write(dir_ptr, dir);
            self.directories.push(Some(NonNull::new_unchecked(dir_ptr)));

            if let Some(parent_dir) = self.get_directory_mut(parent_inode_id) {
                let entry = DirectoryEntry::new(dir_name, dir_inode_id);
                parent_dir.add_entry(entry);
            }

            Ok(())
        }
    }

    fn rmdir(&mut self, path: &[u8]) -> Result<(), FilesystemError> {
        unsafe {
            let inode_id = self.resolve_path(path)?;

            if let Some(inode) = self.get_inode(inode_id) {
                if !inode.file_info.is_directory {
                    return Err(FilesystemError::NotDirectory);
                }

                let parent_path = self.get_parent_path(path);
                let dir_name = self.get_last_component(path);
                let parent_inode_id = self.resolve_path(parent_path)?;

                if let Some(parent_dir) = self.get_directory_mut(parent_inode_id) {
                    parent_dir.remove_entry(dir_name);
                }

                Ok(())
            } else {
                Err(FilesystemError::NotFound)
            }
        }
    }

    fn unlink(&mut self, path: &[u8]) -> Result<(), FilesystemError> {
        unsafe {
            let inode_id = self.resolve_path(path)?;

            if let Some(inode) = self.get_inode(inode_id) {
                if inode.file_info.is_directory {
                    return Err(FilesystemError::IsDirectory);
                }

                let parent_path = self.get_parent_path(path);
                let file_name = self.get_last_component(path);
                let parent_inode_id = self.resolve_path(parent_path)?;

                if let Some(parent_dir) = self.get_directory_mut(parent_inode_id) {
                    parent_dir.remove_entry(file_name);
                }

                Ok(())
            } else {
                Err(FilesystemError::NotFound)
            }
        }
    }

    fn stat(&self, path: &[u8]) -> Result<FileInfo, FilesystemError> {
        unsafe {
            let inode_id = self.resolve_path(path)?;

            if let Some(inode) = self.get_inode(inode_id) {
                Ok(inode.file_info)
            } else {
                Err(FilesystemError::NotFound)
            }
        }
    }
}

impl MemoryFilesystem {
    unsafe fn get_parent_path(&self, path: &[u8]) -> &[u8] {
        let last_slash = path.iter().rposition(|&b| b == b'/').unwrap_or(0);
        if last_slash == 0 {
            b"/"
        } else {
            &path[..last_slash]
        }
    }

    unsafe fn get_last_component(&self, path: &[u8]) -> &[u8] {
        let last_slash = path.iter().rposition(|&b| b == b'/').unwrap_or(0);
        &path[last_slash + 1..]
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
