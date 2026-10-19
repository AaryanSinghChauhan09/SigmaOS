//! Virtual filesystem module
//! 
//! Provides core VFS layer for SigmaOS file system integration.
//! Following principles: COMPONENT_ISOLATION, SINGLE_RESPONSIBILITY,
//! and VIRTUAL_MEMORY_ABSTRACTION.

pub mod ramdisk;
pub mod mount;
pub mod file_operation;
pub mod filesystem_type;

pub use self::ramdisk::Ramdisk;
pub use self::mount::MountPoint;
pub use self::filesystem_type::FilesystemType;

/// Filesystem type enumeration
#[derive(Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum FilesystemType {
    /// RAM-disk filesystem
    Ramdisk = 1,
    /// Virtual filesystem root
    Root = 2,
}

/// Mount point structure
pub struct MountPoint {
    /// Filesystem type
    fs_type: FilesystemType,
    /// Mount path
    path: String,
    /// Underlying filesystem
    filesystem: Option<Box<dyn FilesystemTrait>>,
    /// Read-only flag
    read_only: bool,
}

impl MountPoint {
    /// Create new mount point
    pub fn new(path: String, fs_type: FilesystemType) -> Self {
        MountPoint {
            fs_type,
            path,
            filesystem: None,
            read_only: false,
        }
    }
    
    /// Get filesystem type
    pub fn filesystem_type(&self) -> FilesystemType {
        self.fs_type
    }
    
    /// Get mount path
    pub fn path(&self) -> &str {
        &self.path
    }
    
    /// Check if read-only
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }
    
    /// Set read-only flag
    pub fn set_read_only(&mut self, readonly: bool) {
        self.read_only = readonly;
    }
}

/// Trait for filesystem implementations
pub trait FilesystemTrait: Send + Sync {
    /// Read from filesystem
    fn read(&self, path: &str, offset: u64, buf: &mut [u8]) -> Result<usize, String>;
    
    /// Write to filesystem
    fn write(&self, path: &str, offset: u64, buf: &[u8]) -> Result<usize, String>;
    
    /// Get filesystem information
    fn info(&self) -> String;
}

/// Default filesystem trait implementation
impl dyn FilesystemTrait {
    /// Create ram-disk filesystem
    pub fn ramdisk(size: u64) -> Result<Ramdisk, String> {
        Ramdisk::new(size)
    }
}

