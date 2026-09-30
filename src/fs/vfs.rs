// Virtual Filesystem (VFS) Layer
// Inspired by Linux VFS for unified filesystem abstraction

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    CharacterDevice,
    BlockDevice,
    NamedPipe,
    Socket,
}

/// File permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FilePermissions {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    pub fn as_mode(&self) -> u32 {
        let mut mode = 0u32;
        if self.read {
            mode |= 0o400;
        }
        if self.write {
            mode |= 0o200;
        }
        if self.execute {
            mode |= 0o100;
        }
        mode
    }
}

/// VFS inode
#[derive(Debug, Clone)]
pub struct VfsInode {
    pub inode_number: u64,
    pub file_type: FileType,
    pub permissions: FilePermissions,
    pub size: u64,
    pub links: u32,
    pub uid: u32,
    pub gid: u32,
    pub data: Vec<u8>,
}

/// VFS dentry (directory entry)
#[derive(Debug, Clone)]
pub struct VfsDentry {
    pub name: String,
    pub inode: u64,
    pub parent: Option<u64>,
}

/// VFS superblock
#[derive(Debug, Clone)]
pub struct VfsSuperblock {
    pub filesystem_type: String,
    pub root_inode: u64,
    pub total_inodes: u64,
    pub free_inodes: u64,
    pub total_blocks: u64,
    pub free_blocks: u64,
}

/// VFS file
#[derive(Debug, Clone)]
pub struct VfsFile {
    pub inode: u64,
    pub offset: u64,
    pub flags: u32,
}

/// VFS mount point
#[derive(Debug, Clone)]
pub struct VfsMount {
    pub mount_point: String,
    pub filesystem_type: String,
    pub root_inode: u64,
    pub device: String,
}

/// VFS
pub struct Vfs {
    next_inode: AtomicU64,
    inodes: HashMap<u64, VfsInode>,
    dentries: HashMap<u64, Vec<VfsDentry>>,
    superblocks: HashMap<u64, VfsSuperblock>,
    mounts: Vec<VfsMount>,
    root_inode: u64,
}

impl Vfs {
    pub fn new() -> Self {
        let mut vfs = Self {
            next_inode: AtomicU64::new(1),
            inodes: HashMap::new(),
            dentries: HashMap::new(),
            superblocks: HashMap::new(),
            mounts: Vec::new(),
            root_inode: 1,
        };

        // Create root directory
        let root_inode = VfsInode {
            inode_number: 1,
            file_type: FileType::Directory,
            permissions: FilePermissions::new(true, true, true),
            size: 0,
            links: 2,
            uid: 0,
            gid: 0,
            data: Vec::new(),
        };

        vfs.inodes.insert(1, root_inode);
        vfs.dentries.insert(1, Vec::new());

        vfs
    }

    /// Create a new inode
    pub fn create_inode(&mut self, file_type: FileType, permissions: FilePermissions) -> u64 {
        let inode_number = self.next_inode.fetch_add(1, Ordering::SeqCst);

        let inode = VfsInode {
            inode_number,
            file_type,
            permissions,
            size: 0,
            links: 1,
            uid: 0,
            gid: 0,
            data: Vec::new(),
        };

        self.inodes.insert(inode_number, inode);
        self.dentries.insert(inode_number, Vec::new());

        inode_number
    }

    /// Lookup inode by path
    pub fn lookup(&self, path: &str) -> Option<u64> {
        if path == "/" || path == "" {
            return Some(self.root_inode);
        }

        let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current_inode = self.root_inode;

        for component in components {
            let dentries = self.dentries.get(&current_inode)?;

            let dentry = dentries.iter().find(|d| d.name == component)?;
            current_inode = dentry.inode;
        }

        Some(current_inode)
    }

    /// Create a directory
    pub fn mkdir(&mut self, path: &str, permissions: FilePermissions) -> Result<u64, &'static str> {
        let parent_path = self.get_parent_path(path)?;
        let parent_inode = self.lookup(&parent_path).ok_or("Parent not found")?;

        let dir_name = self.get_basename(path).to_string();
        let new_inode = self.create_inode(FileType::Directory, permissions);

        let dentry = VfsDentry {
            name: dir_name,
            inode: new_inode,
            parent: Some(parent_inode),
        };

        if let Some(dentries) = self.dentries.get_mut(&parent_inode) {
            dentries.push(dentry);
        }

        Ok(new_inode)
    }

    /// Create a file
    pub fn create(
        &mut self,
        path: &str,
        permissions: FilePermissions,
    ) -> Result<u64, &'static str> {
        let parent_path = self.get_parent_path(path)?;
        let parent_inode = self.lookup(&parent_path).ok_or("Parent not found")?;

        let file_name = self.get_basename(path).to_string();
        let new_inode = self.create_inode(FileType::Regular, permissions);

        let dentry = VfsDentry {
            name: file_name,
            inode: new_inode,
            parent: Some(parent_inode),
        };

        if let Some(dentries) = self.dentries.get_mut(&parent_inode) {
            dentries.push(dentry);
        }

        Ok(new_inode)
    }

    /// Read from file
    pub fn read(&self, inode: u64, offset: u64, size: usize) -> Result<Vec<u8>, &'static str> {
        let file = self.inodes.get(&inode).ok_or("Inode not found")?;

        if file.file_type != FileType::Regular {
            return Err("Not a regular file");
        }

        let start = offset as usize;
        let end = (offset as usize + size).min(file.data.len());

        if start >= file.data.len() {
            return Ok(Vec::new());
        }

        Ok(file.data[start..end].to_vec())
    }

    /// Write to file
    pub fn write(&mut self, inode: u64, offset: u64, data: &[u8]) -> Result<usize, &'static str> {
        let file = self.inodes.get_mut(&inode).ok_or("Inode not found")?;

        if file.file_type != FileType::Regular {
            return Err("Not a regular file");
        }

        let start = offset as usize;

        if start + data.len() > file.data.len() {
            file.data.resize(start + data.len(), 0);
        }

        file.data[start..start + data.len()].copy_from_slice(data);
        file.size = file.data.len() as u64;

        Ok(data.len())
    }

    /// List directory entries
    pub fn readdir(&self, inode: u64) -> Result<Vec<String>, &'static str> {
        let file = self.inodes.get(&inode).ok_or("Inode not found")?;

        if file.file_type != FileType::Directory {
            return Err("Not a directory");
        }

        let dentries = self.dentries.get(&inode).ok_or("No dentries")?;

        Ok(dentries.iter().map(|d| d.name.clone()).collect())
    }

    /// Get inode attributes
    pub fn getattr(&self, inode: u64) -> Result<&VfsInode, &'static str> {
        self.inodes.get(&inode).ok_or("Inode not found")
    }

    /// Mount a filesystem
    pub fn mount(
        &mut self,
        mount_point: String,
        filesystem_type: String,
        device: String,
    ) -> Result<(), &'static str> {
        let parent_inode = self.lookup(&mount_point).ok_or("Mount point not found")?;

        let superblock = VfsSuperblock {
            filesystem_type: filesystem_type.clone(),
            root_inode: parent_inode,
            total_inodes: 1000,
            free_inodes: 1000,
            total_blocks: 10000,
            free_blocks: 10000,
        };

        let sb_id = self.next_inode.fetch_add(1, Ordering::SeqCst);
        self.superblocks.insert(sb_id, superblock);

        let mount = VfsMount {
            mount_point,
            filesystem_type,
            root_inode: parent_inode,
            device,
        };

        self.mounts.push(mount);
        Ok(())
    }

    /// Get parent path
    fn get_parent_path(&self, path: &str) -> Result<String, &'static str> {
        if path == "/" {
            return Err("Root has no parent");
        }

        let last_slash = path.rfind('/').unwrap_or(0);
        if last_slash == 0 {
            Ok("/".to_string())
        } else {
            Ok(path[..last_slash].to_string())
        }
    }

    /// Get basename
    fn get_basename<'a>(&self, path: &'a str) -> &'a str {
        path.rfind('/').map(|i| &path[i + 1..]).unwrap_or(path)
    }

    /// Get inode count
    pub fn inode_count(&self) -> usize {
        self.inodes.len()
    }

    /// Get mount count
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_create() {
        let vfs = Vfs::new();

        assert_eq!(vfs.inode_count(), 1);
    }

    #[test]
    fn test_mkdir() {
        let mut vfs = Vfs::new();

        let perms = FilePermissions::new(true, true, true);
        let inode = vfs.mkdir("/test", perms).unwrap();

        assert!(inode > 1);
        assert_eq!(vfs.inode_count(), 2);
    }

    #[test]
    fn test_create_file() {
        let mut vfs = Vfs::new();

        let perms = FilePermissions::new(true, true, false);
        let inode = vfs.create("/test.txt", perms).unwrap();

        assert!(inode > 1);
    }

    #[test]
    fn test_write_read() {
        let mut vfs = Vfs::new();

        let perms = FilePermissions::new(true, true, false);
        let inode = vfs.create("/test.txt", perms).unwrap();

        vfs.write(inode, 0, b"Hello, World!").unwrap();

        let data = vfs.read(inode, 0, 13).unwrap();
        assert_eq!(data, b"Hello, World!");
    }

    #[test]
    fn test_readdir() {
        let mut vfs = Vfs::new();

        let perms = FilePermissions::new(true, true, true);
        vfs.mkdir("/test", perms).unwrap();

        let entries = vfs.readdir(1).unwrap();
        assert!(entries.contains(&"test".to_string()));
    }

    #[test]
    fn test_lookup() {
        let mut vfs = Vfs::new();

        let perms = FilePermissions::new(true, true, true);
        vfs.mkdir("/test", perms).unwrap();

        let inode = vfs.lookup("/test").unwrap();
        assert!(inode > 1);
    }

    #[test]
    fn test_mount() {
        let mut vfs = Vfs::new();

        vfs.mount("/".to_string(), "ext4".to_string(), "/dev/sda1".to_string())
            .unwrap();
        assert_eq!(vfs.mount_count(), 1);
    }
}
