//! RAM-disk filesystem implementation
//!
//! Implements integrated ram-disk filesystem with boot evidence support
//! following SigmaOS principles and roadmap requirements.

use core::fmt;
use core::sync::atomic::{AtomicU64, Ordering};
use crate::memory::{PhysicalAddress, VirtAddr, MemoryManager};
use crate::vfs::{FileSystem, MountPoint, FileOperation, FilesystemType};

// RAM-disk structure
#[derive(Debug)]
pub struct Ramdisk {
    /// RAM-disk size in bytes
    size: u64,
    /// RAM-disk data buffer
    data: Vec<u8>,
    /// Mount point
    mount_point: MountPoint,
    /// Filesystem type
    fs_type: FilesystemType,
    /// Total read operations
    read_count: AtomicU64,
    /// Total write operations
    write_count: AtomicU64,
    /// Total capacity operations
    capacity_operations: AtomicU64,
}

impl Ramdisk {
    /// Create new ram-disk with given size
    pub fn new(size: u64) -> Result<Self, String> {
        if size == 0 {
            return Err("Ram-disk size must be non-zero".to_string());
        }
        if size > (1 << 30) {
            return Err("Ram-disk size exceeds maximum (1GB)".to_string());
        }
        
        let data = vec![0u8; size as usize];
        
        let mount_point = MountPoint::new(
            "/ramdisk".to_string(),
            FilesystemType::Ramdisk,
        );
        
        Ok(Ramdisk {
            size,
            data,
            mount_point,
            fs_type: FilesystemType::Ramdisk,
            read_count: AtomicU64::new(0),
            write_count: AtomicU64::new(0),
            capacity_operations: AtomicU64::new(0),
        })
    }
    
    /// Get ram-disk size
    pub fn size(&self) -> u64 {
        self.size
    }
    
    /// Get data reference
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    /// Get mutable data reference
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
    
    /// Read from ram-disk at offset
    pub fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, String> {
        if offset >= self.size {
            return Err("Offset beyond ram-disk size".to_string());
        }
        
        let bytes_to_read = core::cmp::min(buf.len(), self.size - offset);
        let data_offset = offset as usize;
        
        buf.copy_from_slice(&self.data[data_offset..data_offset + bytes_to_read]);
        
        self.read_count.fetch_add(1, Ordering::SeqCst);
        Ok(bytes_to_read)
    }
    
    /// Write to ram-disk at offset
    pub fn write_at(&mut self, offset: u64, buf: &[u8]) -> Result<usize, String> {
        if offset >= self.size {
            return Err("Offset beyond ram-disk size".to_string());
        }
        
        let bytes_to_write = core::cmp::min(buf.len(), self.size - offset);
        let data_offset = offset as usize;
        
        self.data[data_offset..data_offset + bytes_to_write].copy_from_slice(&buf[..bytes_to_write]);
        
        self.write_count.fetch_add(1, Ordering::SeqCst);
        Ok(bytes_to_write)
    }
    
    /// Get read count
    pub fn read_count(&self) -> u64 {
        self.read_count.load(Ordering::SeqCst)
    }
    
    /// Get write count
    pub fn write_count(&self) -> u64 {
        self.write_count.load(Ordering::SeqCst)
    }
    
    /// Get capacity operations
    pub fn capacity_operations(&self) -> u64 {
        self.capacity_operations.load(Ordering::SeqCst)
    }
    
    /// Increment capacity operations
    pub fn increment_capacity_ops(&self) {
        self.capacity_operations.fetch_add(1, Ordering::SeqCst);
    }
}

impl fmt::Display for Ramdisk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ramdisk[size={}KB, read={}, write={}]", 
               self.size / 1024, self.read_count(), self.write_count())
    }
}

impl fmt::Debug for Ramdisk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ramdisk")
            .field("size", &self.size)
            .field("read_count", &self.read_count())
            .field("write_count", &self.write_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ramdisk_creation() {
        let ramdisk = Ramdisk::new(4096).unwrap();
        assert_eq!(ramdisk.size(), 4096);
        assert_eq!(ramdisk.data().len(), 4096);
    }
    
    #[test]
    fn test_ramdisk_read_write() {
        let mut ramdisk = Ramdisk::new(4096).unwrap();
        
        // Write test
        let write_result = ramdisk.write_at(0, b"Hello World").unwrap();
        assert_eq!(write_result, 11);
        
        // Read test
        let mut buf = [0u8; 11];
        let read_result = ramdisk.read_at(0, &mut buf).unwrap();
        assert_eq!(read_result, 11);
        assert_eq!(&buf, b"Hello World");
    }
    
    #[test]
    fn test_ramdisk_out_of_bounds() {
        let ramdisk = Ramdisk::new(4096).unwrap();
        
        // Write out of bounds
        let result = ramdisk.write_at(4096, b"test");
        assert!(result.is_err());
        
        // Read out of bounds
        let result = ramdisk.read_at(4096, &mut [0u8; 4]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_ramdisk_counts() {
        let ramdisk = Ramdisk::new(4096).unwrap();
        
        assert_eq!(ramdisk.read_count(), 0);
        assert_eq!(ramdisk.write_count(), 0);
        
        let _ = ramdisk.write_at(0, b"test");
        let _ = ramdisk.read_at(0, &mut [0u8; 4]);
        
        assert_eq!(ramdisk.read_count(), 1);
        assert_eq!(ramdisk.write_count(), 1);
    }
}
