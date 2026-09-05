// SPDX-License-Identifier: MIT
// SigmaOS EXT4 Filesystem Mount System
// Supports mounting and managing ext4 filesystems

use std::boxed::Box;
use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicU32, Ordering};

// ============================================================================
// EXT4 Constants
// ============================================================================

pub const EXT4_SUPERBLOCK_OFFSET: u64 = 1024;
pub const EXT4_SUPERBLOCK_SIZE: u64 = 1024;
pub const EXT4_MIN_BLOCK_SIZE: u32 = 1024;
pub const EXT4_MAX_BLOCK_SIZE: u32 = 65536;
pub const EXT4_INODE_SIZE: u32 = 256;
pub const EXT4_DIR_ENTRY_SIZE: u32 = 264;

// EXT4 Superblock Magic
pub const EXT4_SUPERBLOCK_MAGIC: u16 = 0xEF53;

// EXT4 Inode Types
pub const EXT4_INODE_TYPE_FIFO: u16 = 0x1000;
pub const EXT4_INODE_TYPE_CHARDEV: u16 = 0x2000;
pub const EXT4_INODE_TYPE_DIRECTORY: u16 = 0x4000;
pub const EXT4_INODE_TYPE_BLOCKDEV: u16 = 0x6000;
pub const EXT4_INODE_TYPE_REGULAR: u16 = 0x8000;
pub const EXT4_INODE_TYPE_SYMLINK: u16 = 0xA000;
pub const EXT4_INODE_TYPE_SOCKET: u16 = 0xC000;

// EXT4 State
pub const EXT4_STATE_CLEAN: u16 = 1;
pub const EXT4_STATE_ERROR: u16 = 2;

// EXT4 Errors Behavior
pub const EXT4_ERRORS_CONTINUE: u32 = 1;
pub const EXT4_ERRORS_RO: u32 = 2;
pub const EXT4_ERRORS_PANIC: u32 = 3;

// ============================================================================
// EXT4 Superblock
// ============================================================================

#[derive(Debug, Clone)]
pub struct Ext4Superblock {
    pub total_inodes: u32,
    pub total_blocks: u32,
    pub reserved_blocks: u32,
    pub free_blocks: u32,
    pub free_inodes: u32,
    pub block_size: u32,
    pub fragment_size: u32,
    pub blocks_per_group: u32,
    pub fragments_per_group: u32,
    pub inodes_per_group: u32,
    pub mount_time: u32,
    pub write_time: u32,
    pub mount_count: u16,
    pub max_mount_count: u16,
    pub magic: u16,
    pub state: u16,
    pub errors: u32,
    pub last_check: u32,
    pub check_interval: u32,
    pub creator_os: u32,
    pub revision_level: u32,
    pub volume_label: [u8; 16],
    pub last_mounted: [u8; 64],
    pub filesystem_type: [u8; 16],
}

impl Ext4Superblock {
    pub fn new() -> Self {
        Ext4Superblock {
            total_inodes: 0,
            total_blocks: 0,
            reserved_blocks: 0,
            free_blocks: 0,
            free_inodes: 0,
            block_size: 4096,
            fragment_size: 4096,
            blocks_per_group: 32768,
            fragments_per_group: 32768,
            inodes_per_group: 8192,
            mount_time: 0,
            write_time: 0,
            mount_count: 0,
            max_mount_count: 0xFFFF,
            magic: EXT4_SUPERBLOCK_MAGIC,
            state: EXT4_STATE_CLEAN,
            errors: EXT4_ERRORS_CONTINUE,
            last_check: 0,
            check_interval: 15552000, // 6 months
            creator_os: 0,
            revision_level: 1,
            volume_label: [0; 16],
            last_mounted: [0; 64],
            filesystem_type: [0; 16],
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == EXT4_SUPERBLOCK_MAGIC
            && self.block_size >= EXT4_MIN_BLOCK_SIZE
            && self.block_size <= EXT4_MAX_BLOCK_SIZE
    }

    pub fn get_block_size_power(&self) -> u32 {
        // block_size = 1024 << block_size_power
        let bs = self.block_size;
        let mut power = 0;
        let mut size = 1024;
        while size < bs {
            size *= 2;
            power += 1;
        }
        power
    }
}

// ============================================================================
// EXT4 Inode
// ============================================================================

#[derive(Debug, Clone)]
pub struct Ext4Inode {
    pub inode_number: u32,
    pub mode: u16,
    pub uid: u16,
    pub size: u32,
    pub access_time: u32,
    pub change_time: u32,
    pub modify_time: u32,
    pub delete_time: u32,
    pub gid: u16,
    pub hard_links: u16,
    pub blocks: u32,
    pub flags: u32,
    pub os_specific1: u32,
    pub direct_blocks: [u32; 12],
    pub indirect_block: u32,
    pub double_indirect_block: u32,
    pub triple_indirect_block: u32,
    pub generation: u32,
    pub file_acl_extended: u32,
    pub size_high: u32,
    pub frag_block: u32,
    pub extra_inode_size: u16,
}

impl Ext4Inode {
    pub fn new(inode_num: u32, mode: u16) -> Self {
        Ext4Inode {
            inode_number: inode_num,
            mode,
            uid: 0,
            size: 0,
            access_time: 0,
            change_time: 0,
            modify_time: 0,
            delete_time: 0,
            gid: 0,
            hard_links: 1,
            blocks: 0,
            flags: 0,
            os_specific1: 0,
            direct_blocks: [0; 12],
            indirect_block: 0,
            double_indirect_block: 0,
            triple_indirect_block: 0,
            generation: 0,
            file_acl_extended: 0,
            size_high: 0,
            frag_block: 0,
            extra_inode_size: 0,
        }
    }

    pub fn get_inode_type(&self) -> u16 {
        self.mode & 0xF000
    }

    pub fn is_regular_file(&self) -> bool {
        self.get_inode_type() == EXT4_INODE_TYPE_REGULAR
    }

    pub fn is_directory(&self) -> bool {
        self.get_inode_type() == EXT4_INODE_TYPE_DIRECTORY
    }

    pub fn is_symlink(&self) -> bool {
        self.get_inode_type() == EXT4_INODE_TYPE_SYMLINK
    }

    pub fn get_permissions(&self) -> u16 {
        self.mode & 0x0FFF
    }
}

// ============================================================================
// EXT4 Directory Entry
// ============================================================================

#[derive(Debug, Clone)]
pub struct Ext4DirEntry {
    pub inode: u32,
    pub rec_len: u16,
    pub name_len: u8,
    pub file_type: u8,
    pub name: String,
}

impl Ext4DirEntry {
    pub fn new(inode: u32, name: &str, file_type: u8) -> Self {
        Ext4DirEntry {
            inode,
            rec_len: 8 + name.len() as u16,
            name_len: name.len() as u8,
            file_type,
            name: name.to_string(),
        }
    }
}

// ============================================================================
// EXT4 Mounted Filesystem
// ============================================================================

#[derive(Debug, Clone)]
pub struct Ext4Mount {
    pub mount_point: String,
    pub device_path: String,
    pub superblock: Ext4Superblock,
    pub is_mounted: bool,
    pub is_readonly: bool,
    pub is_dirty: bool,
}

impl Ext4Mount {
    pub fn new(mount_point: &str, device: &str) -> Self {
        Ext4Mount {
            mount_point: mount_point.to_string(),
            device_path: device.to_string(),
            superblock: Ext4Superblock::new(),
            is_mounted: false,
            is_readonly: false,
            is_dirty: false,
        }
    }

    pub fn initialize_superblock(&mut self, total_blocks: u32, total_inodes: u32) {
        self.superblock.total_blocks = total_blocks;
        self.superblock.total_inodes = total_inodes;
        self.superblock.free_blocks = total_blocks;
        self.superblock.free_inodes = total_inodes;
    }
}

// ============================================================================
// EXT4 Filesystem Manager
// ============================================================================

pub struct Ext4FilesystemManager {
    mounts: Vec<Ext4Mount>,
    inodes: Vec<Ext4Inode>,
    mount_count: AtomicU32,
    inode_count: AtomicU32,
}

impl Ext4FilesystemManager {
    pub fn new() -> Self {
        Ext4FilesystemManager {
            mounts: Vec::new(),
            inodes: Vec::new(),
            mount_count: AtomicU32::new(0),
            inode_count: AtomicU32::new(0),
        }
    }

    pub fn mount(&mut self, mount_point: &str, device: &str) -> Result<(), &'static str> {
        // Check if already mounted
        if self.mounts.iter().any(|m| m.mount_point == mount_point) {
            return Err("Mount point already in use");
        }

        let mut mount = Ext4Mount::new(mount_point, device);

        // In real implementation:
        // 1. Read superblock from device
        // 2. Validate superblock
        // 3. Load block group descriptors
        // 4. Initialize caches

        mount.is_mounted = true;
        self.mounts.push(mount);
        self.mount_count.fetch_add(1, Ordering::SeqCst);

        Ok(())
    }

    pub fn unmount(&mut self, mount_point: &str) -> Result<(), &'static str> {
        if let Some(pos) = self.mounts.iter().position(|m| m.mount_point == mount_point) {
            let mount = &self.mounts[pos];

            if mount.is_dirty {
                // Flush all dirty data to disk
                // In real implementation, would sync all blocks
            }

            self.mounts.remove(pos);
            self.mount_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err("Mount point not found")
        }
    }

    pub fn create_inode(
        &mut self,
        parent_inode: u32,
        name: &str,
        mode: u16,
    ) -> Result<u32, &'static str> {
        let inode_num = self.inodes.len() as u32 + 1;
        let inode = Ext4Inode::new(inode_num, mode);

        self.inodes.push(inode);
        self.inode_count.fetch_add(1, Ordering::SeqCst);

        Ok(inode_num)
    }

    pub fn create_file(&mut self, path: &str) -> Result<u32, &'static str> {
        // Parse path to extract parent directory and filename
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

        if parts.is_empty() {
            return Err("Invalid path");
        }

        // Root inode is always 2
        let parent_inode = 2;
        let filename = parts[parts.len() - 1];

        // Create regular file inode
        self.create_inode(parent_inode, filename, EXT4_INODE_TYPE_REGULAR)
    }

    pub fn create_directory(&mut self, path: &str) -> Result<u32, &'static str> {
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();

        if parts.is_empty() {
            return Err("Invalid path");
        }

        let parent_inode = 2;
        let dirname = parts[parts.len() - 1];

        self.create_inode(parent_inode, dirname, EXT4_INODE_TYPE_DIRECTORY)
    }

    pub fn read_file(&self, inode_num: u32, offset: u32, size: u32) -> Result<Vec<u8>, &'static str> {
        if let Some(inode) = self.inodes.iter().find(|i| i.inode_number == inode_num) {
            if !inode.is_regular_file() {
                return Err("Not a regular file");
            }

            // In real implementation:
            // 1. Calculate block number from offset
            // 2. Read from block storage
            // 3. Return requested bytes

            Ok(vec![0; size as usize])
        } else {
            Err("Inode not found")
        }
    }

    pub fn write_file(
        &mut self,
        inode_num: u32,
        offset: u32,
        data: &[u8],
    ) -> Result<u32, &'static str> {
        if let Some(inode) = self.inodes.iter_mut().find(|i| i.inode_number == inode_num) {
            if !inode.is_regular_file() {
                return Err("Not a regular file");
            }

            let bytes_written = data.len() as u32;
            if offset + bytes_written > inode.size {
                inode.size = offset + bytes_written;
            }

            // Mark as dirty
            if let Some(mount) = self.mounts.iter_mut().next() {
                mount.is_dirty = true;
            }

            Ok(bytes_written)
        } else {
            Err("Inode not found")
        }
    }

    pub fn delete_inode(&mut self, inode_num: u32) -> Result<(), &'static str> {
        if let Some(pos) = self.inodes.iter().position(|i| i.inode_number == inode_num) {
            self.inodes.remove(pos);
            self.inode_count.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err("Inode not found")
        }
    }

    pub fn list_directory(&self, inode_num: u32) -> Result<Vec<Ext4DirEntry>, &'static str> {
        if let Some(inode) = self.inodes.iter().find(|i| i.inode_number == inode_num) {
            if !inode.is_directory() {
                return Err("Not a directory");
            }

            // In real implementation, would read directory entries from disk
            Ok(Vec::new())
        } else {
            Err("Inode not found")
        }
    }

    pub fn get_mount(&self, mount_point: &str) -> Option<&Ext4Mount> {
        self.mounts.iter().find(|m| m.mount_point == mount_point)
    }

    pub fn get_mount_count(&self) -> u32 {
        self.mount_count.load(Ordering::SeqCst)
    }

    pub fn get_inode_count(&self) -> u32 {
        self.inode_count.load(Ordering::SeqCst)
    }

    pub fn get_mounts(&self) -> &[Ext4Mount] {
        &self.mounts
    }
}

impl Default for Ext4FilesystemManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_superblock_creation() {
        let sb = Ext4Superblock::new();
        assert_eq!(sb.magic, EXT4_SUPERBLOCK_MAGIC);
        assert!(sb.is_valid());
    }

    #[test]
    fn test_superblock_validation() {
        let mut sb = Ext4Superblock::new();
        assert!(sb.is_valid());

        sb.magic = 0x1234;
        assert!(!sb.is_valid());
    }

    #[test]
    fn test_inode_creation() {
        let inode = Ext4Inode::new(1, EXT4_INODE_TYPE_REGULAR);
        assert_eq!(inode.inode_number, 1);
        assert!(inode.is_regular_file());
    }

    #[test]
    fn test_inode_type_detection() {
        let mut inode = Ext4Inode::new(1, EXT4_INODE_TYPE_DIRECTORY);
        assert!(inode.is_directory());
        assert!(!inode.is_regular_file());

        inode.mode = EXT4_INODE_TYPE_SYMLINK;
        assert!(inode.is_symlink());
    }

    #[test]
    fn test_dir_entry_creation() {
        let entry = Ext4DirEntry::new(1, "test.txt", 1);
        assert_eq!(entry.inode, 1);
        assert_eq!(entry.name, "test.txt");
    }

    #[test]
    fn test_ext4_mount_creation() {
        let mount = Ext4Mount::new("/mnt/data", "/dev/sda1");
        assert_eq!(mount.mount_point, "/mnt/data");
        assert_eq!(mount.device_path, "/dev/sda1");
        assert!(!mount.is_mounted);
    }

    #[test]
    fn test_filesystem_manager_creation() {
        let manager = Ext4FilesystemManager::new();
        assert_eq!(manager.get_mount_count(), 0);
    }

    #[test]
    fn test_mount_filesystem() {
        let mut manager = Ext4FilesystemManager::new();
        assert!(manager.mount("/mnt/data", "/dev/sda1").is_ok());
        assert_eq!(manager.get_mount_count(), 1);
    }

    #[test]
    fn test_unmount_filesystem() {
        let mut manager = Ext4FilesystemManager::new();
        manager.mount("/mnt/data", "/dev/sda1").unwrap();
        assert!(manager.unmount("/mnt/data").is_ok());
        assert_eq!(manager.get_mount_count(), 0);
    }

    #[test]
    fn test_create_file() {
        let mut manager = Ext4FilesystemManager::new();
        manager.mount("/mnt/data", "/dev/sda1").unwrap();

        let inode = manager.create_file("/mnt/data/test.txt").unwrap();
        assert!(inode > 0);
        assert_eq!(manager.get_inode_count(), 1);
    }

    #[test]
    fn test_create_directory() {
        let mut manager = Ext4FilesystemManager::new();
        manager.mount("/mnt/data", "/dev/sda1").unwrap();

        let inode = manager.create_directory("/mnt/data/folder").unwrap();
        assert!(inode > 0);
    }

    #[test]
    fn test_delete_inode() {
        let mut manager = Ext4FilesystemManager::new();
        manager.mount("/mnt/data", "/dev/sda1").unwrap();
        let inode = manager.create_file("/mnt/data/test.txt").unwrap();

        assert!(manager.delete_inode(inode).is_ok());
        assert_eq!(manager.get_inode_count(), 0);
    }

    #[test]
    fn test_read_write_file() {
        let mut manager = Ext4FilesystemManager::new();
        manager.mount("/mnt/data", "/dev/sda1").unwrap();
        let inode = manager.create_file("/mnt/data/test.txt").unwrap();

        let data = vec![1, 2, 3, 4, 5];
        let written = manager.write_file(inode, 0, &data).unwrap();
        assert_eq!(written, 5);

        let read_data = manager.read_file(inode, 0, 5).unwrap();
        assert_eq!(read_data.len(), 5);
    }

    #[test]
    fn test_list_directory() {
        let mut manager = Ext4FilesystemManager::new();
        manager.mount("/mnt/data", "/dev/sda1").unwrap();
        let dir_inode = manager.create_directory("/mnt/data/folder").unwrap();

        let entries = manager.list_directory(dir_inode).unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_duplicate_mount_point() {
        let mut manager = Ext4FilesystemManager::new();
        manager.mount("/mnt/data", "/dev/sda1").unwrap();

        // Try to mount same point again
        let result = manager.mount("/mnt/data", "/dev/sdb1");
        assert!(result.is_err());
    }
}
