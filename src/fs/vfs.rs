#![no_std]
#![no_main]

/// OOP-based Virtual Filesystem for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 41
/// Implements VFS layer with mount points and file operations

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type InodeID = usize;
pub type MountID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FileType { Regular = 0, Directory = 1, Symlink = 2, Device = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VFSError { Success = 0, NotFound = 1, PermissionDenied = 2, IsDirectory = 3 }

pub trait Inode {
    fn id(&self) -> InodeID;
    fn file_type(&self) -> FileType;
    fn size(&self) -> usize;
    fn permissions(&self) -> u16;
}

#[repr(C)]
pub struct SimpleInode {
    pub id: InodeID,
    pub file_type: AtomicUsize,
    pub size: AtomicUsize,
    pub permissions: AtomicUsize,
}

impl SimpleInode {
    pub fn new(id: InodeID, file_type: FileType, size: usize, permissions: u16) -> Self {
        SimpleInode {
            id,
            file_type: AtomicUsize::new(file_type as usize),
            size: AtomicUsize::new(size),
            permissions: AtomicUsize::new(permissions as usize),
        }
    }
}

impl Inode for SimpleInode {
    fn id(&self) -> InodeID { self.id }
    fn file_type(&self) -> FileType { unsafe { core::mem::transmute(self.file_type.load(Ordering::SeqCst)) } }
    fn size(&self) -> usize { self.size.load(Ordering::SeqCst) }
    fn permissions(&self) -> u16 { self.permissions.load(Ordering::SeqCst) as u16 }
}

pub trait Filesystem {
    fn mount_id(&self) -> MountID;
    fn read_inode(&self, inode_id: InodeID) -> Option<&dyn Inode>;
    fn read_data(&self, inode_id: InodeID, offset: usize, buffer: &mut [u8]) -> Result<usize, VFSError>;
    fn write_data(&mut self, inode_id: InodeID, offset: usize, data: &[u8]) -> Result<usize, VFSError>;
}

#[repr(C)]
pub struct SimpleFilesystem {
    pub mount_id: MountID,
    pub inodes: Vec<Option<Box<dyn Inode>>>,
    pub data: Vec<Vec<u8>>,
}

impl SimpleFilesystem {
    pub fn new(mount_id: MountID) -> Self {
        SimpleFilesystem {
            mount_id,
            inodes: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl Filesystem for SimpleFilesystem {
    fn mount_id(&self) -> MountID { self.mount_id }
    
    fn read_inode(&self, inode_id: InodeID) -> Option<&dyn Inode> {
        if inode_id > 0 && inode_id <= self.inodes.len() {
            if let Some(ref inode) = *self.inodes[inode_id - 1] {
                return Some(inode.as_ref());
            }
        }
        None
    }
    
    fn read_data(&self, inode_id: InodeID, offset: usize, buffer: &mut [u8]) -> Result<usize, VFSError> {
        if inode_id > 0 && inode_id <= self.data.len() {
            let data = &self.data[inode_id - 1];
            let end = (offset + buffer.len()).min(data.len());
            for i in 0..buffer.len() {
                if offset + i < end {
                    buffer[i] = data[offset + i];
                }
            }
            Ok(end - offset)
        } else {
            Err(VFSError::NotFound)
        }
    }
    
    fn write_data(&mut self, inode_id: InodeID, offset: usize, data: &[u8]) -> Result<usize, VFSError> {
        if inode_id > 0 && inode_id <= self.data.len() {
            let file_data = &mut self.data[inode_id - 1];
            let end = (offset + data.len()).max(file_data.len());
            while file_data.len() < end {
                file_data.push(0u8);
            }
            for i in 0..data.len() {
                if offset + i < file_data.len() {
                    file_data[offset + i] = data[i];
                }
            }
            Ok(data.len())
        } else {
            Err(VFSError::NotFound)
        }
    }
}

pub trait VFS {
    fn mount(&mut self, fs: Box<dyn Filesystem>, mount_point: &[u8]) -> Result<MountID, VFSError>;
    fn unmount(&mut self, mount_id: MountID) -> Result<(), VFSError>;
    fn resolve_path(&self, path: &[u8]) -> Result<(MountID, InodeID), VFSError>;
    fn open_file(&mut self, path: &[u8], flags: u32) -> Result<usize, VFSError>;
}

#[repr(C)]
pub struct SimpleVFS {
    pub filesystems: Vec<Option<Box<dyn Filesystem>>>,
    pub mount_points: Vec<(MountID, [u8; 256])>,
    pub next_id: AtomicUsize,
}

impl SimpleVFS {
    pub fn new() -> Self {
        SimpleVFS {
            filesystems: Vec::new(),
            mount_points: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl VFS for SimpleVFS {
    fn mount(&mut self, fs: Box<dyn Filesystem>, mount_point: &[u8]) -> Result<MountID, VFSError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut mount_array = [0u8; 256];
        let mount_len = mount_point.len().min(255);
        for i in 0..mount_len {
            mount_array[i] = mount_point[i];
        }
        self.filesystems.push(Some(fs));
        self.mount_points.push((id, mount_array));
        Ok(id)
    }
    
    fn unmount(&mut self, mount_id: MountID) -> Result<(), VFSError> {
        for i in 0..self.filesystems.len() {
            if let Some(ref fs) = *self.filesystems[i] {
                if fs.mount_id() == mount_id {
                    return Ok(());
                }
            }
        }
        Err(VFSError::NotFound)
    }
    
    fn resolve_path(&self, path: &[u8]) -> Result<(MountID, InodeID), VFSError> {
        for &(mount_id, ref mount_point) in &self.mount_points {
            let mount_len = mount_point.iter().position(|&b| b == 0).unwrap_or(256);
            if path.starts_with(&mount_point[..mount_len]) {
                return Ok((mount_id, 1));
            }
        }
        Err(VFSError::NotFound)
    }
    
    fn open_file(&mut self, path: &[u8], _flags: u32) -> Result<usize, VFSError> {
        let (mount_id, inode_id) = self.resolve_path(path)?;
        Ok(mount_id * 10000 + inode_id)
    }
}

pub trait FileDescriptor {
    fn fd(&self) -> usize;
    fn mount_id(&self) -> MountID;
    fn inode_id(&self) -> InodeID;
    fn offset(&self) -> usize;
    fn set_offset(&mut self, offset: usize);
}

#[repr(C)]
pub struct SimpleFileDescriptor {
    pub fd: usize,
    pub mount_id: MountID,
    pub inode_id: InodeID,
    pub offset: AtomicUsize,
}

impl SimpleFileDescriptor {
    pub fn new(fd: usize, mount_id: MountID, inode_id: InodeID) -> Self {
        SimpleFileDescriptor {
            fd,
            mount_id,
            inode_id,
            offset: AtomicUsize::new(0),
        }
    }
}

impl FileDescriptor for SimpleFileDescriptor {
    fn fd(&self) -> usize { self.fd }
    fn mount_id(&self) -> MountID { self.mount_id }
    fn inode_id(&self) -> InodeID { self.inode_id }
    fn offset(&self) -> usize { self.offset.load(Ordering::SeqCst) }
    
    fn set_offset(&mut self, offset: usize) {
        self.offset.store(offset, Ordering::SeqCst);
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
