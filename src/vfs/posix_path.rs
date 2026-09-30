// POSIX Path Resolution and VFS (Virtual File System)
// Implements POSIX path resolution, normalization, and VFS abstraction

use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// VFS inode identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VfsInode {
    id: u64,
}

impl VfsInode {
    pub fn new(id: u64) -> Self {
        VfsInode { id }
    }
}

/// VFS file type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfsFileType {
    RegularFile,
    Directory,
    SymbolicLink,
    CharacterDevice,
    BlockDevice,
    NamedPipe,
    Socket,
}

/// VFS file attributes
#[derive(Debug, Clone)]
pub struct VfsFileAttr {
    pub inode: VfsInode,
    pub file_type: VfsFileType,
    pub size: u64,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}

/// VFS directory entry
#[derive(Debug, Clone)]
pub struct VfsDirEntry {
    pub name: String,
    pub inode: VfsInode,
    pub file_type: VfsFileType,
}

/// VFS filesystem operations trait
pub trait VfsOperations {
    fn lookup(&self, parent: VfsInode, name: &str) -> Result<VfsInode, String>;
    fn create(&mut self, parent: VfsInode, name: &str, mode: u32) -> Result<VfsInode, String>;
    fn mkdir(&mut self, parent: VfsInode, name: &str, mode: u32) -> Result<VfsInode, String>;
    fn unlink(&mut self, parent: VfsInode, name: &str) -> Result<(), String>;
    fn rmdir(&mut self, parent: VfsInode, name: &str) -> Result<(), String>;
    fn rename(&mut self, old_parent: VfsInode, old_name: &str, new_parent: VfsInode, new_name: &str) -> Result<(), String>;
    fn getattr(&self, inode: VfsInode) -> Result<VfsFileAttr, String>;
    fn setattr(&mut self, inode: VfsInode, attr: VfsFileAttr) -> Result<(), String>;
    fn read(&self, inode: VfsInode, offset: u64, buf: &mut [u8]) -> Result<usize, String>;
    fn write(&mut self, inode: VfsInode, offset: u64, buf: &[u8]) -> Result<usize, String>;
    fn readdir(&self, inode: VfsInode) -> Result<Vec<VfsDirEntry>, String>;
    fn symlink(&mut self, parent: VfsInode, name: &str, target: &str) -> Result<VfsInode, String>;
    fn readlink(&self, inode: VfsInode) -> Result<String, String>;
}

/// In-memory VFS implementation for testing
#[derive(Debug)]
pub struct MemoryVfs {
    next_inode: u64,
    inodes: HashMap<VfsInode, VfsFileAttr>,
    data: HashMap<VfsInode, Vec<u8>>,
    directories: HashMap<VfsInode, Vec<VfsDirEntry>>,
    symlinks: HashMap<VfsInode, String>,
}

impl MemoryVfs {
    pub fn new() -> Self {
        let mut vfs = MemoryVfs {
            next_inode: 1,
            inodes: HashMap::new(),
            data: HashMap::new(),
            directories: HashMap::new(),
            symlinks: HashMap::new(),
        };

        // Create root directory
        let root_inode = VfsInode::new(0);
        vfs.inodes.insert(root_inode, VfsFileAttr {
            inode: root_inode,
            file_type: VfsFileType::Directory,
            size: 0,
            mode: 0o755,
            uid: 0,
            gid: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        });
        vfs.directories.insert(root_inode, Vec::new());

        vfs
    }

    fn allocate_inode(&mut self) -> VfsInode {
        let inode = VfsInode::new(self.next_inode);
        self.next_inode += 1;
        inode
    }
}

impl Default for MemoryVfs {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsOperations for MemoryVfs {
    fn lookup(&self, parent: VfsInode, name: &str) -> Result<VfsInode, String> {
        let entries = self.directories.get(&parent)
            .ok_or_else(|| format!("Parent inode not found"))?;

        for entry in entries {
            if entry.name == name {
                return Ok(entry.inode);
            }
        }

        Err(format!("Entry '{}' not found", name))
    }

    fn create(&mut self, parent: VfsInode, name: &str, mode: u32) -> Result<VfsInode, String> {
        let inode = self.allocate_inode();

        self.inodes.insert(inode, VfsFileAttr {
            inode,
            file_type: VfsFileType::RegularFile,
            size: 0,
            mode,
            uid: 0,
            gid: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        });

        self.data.insert(inode, Vec::new());

        if let Some(entries) = self.directories.get_mut(&parent) {
            entries.push(VfsDirEntry {
                name: name.to_string(),
                inode,
                file_type: VfsFileType::RegularFile,
            });
        }

        Ok(inode)
    }

    fn mkdir(&mut self, parent: VfsInode, name: &str, mode: u32) -> Result<VfsInode, String> {
        let inode = self.allocate_inode();

        self.inodes.insert(inode, VfsFileAttr {
            inode,
            file_type: VfsFileType::Directory,
            size: 0,
            mode,
            uid: 0,
            gid: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        });

        self.directories.insert(inode, Vec::new());

        if let Some(entries) = self.directories.get_mut(&parent) {
            entries.push(VfsDirEntry {
                name: name.to_string(),
                inode,
                file_type: VfsFileType::Directory,
            });
        }

        Ok(inode)
    }

    fn unlink(&mut self, parent: VfsInode, name: &str) -> Result<(), String> {
        let inode = self.lookup(parent, name)?;
        self.inodes.remove(&inode);
        self.data.remove(&inode);

        if let Some(entries) = self.directories.get_mut(&parent) {
            entries.retain(|e| e.name != name);
        }

        Ok(())
    }

    fn rmdir(&mut self, parent: VfsInode, name: &str) -> Result<(), String> {
        let inode = self.lookup(parent, name)?;
        self.inodes.remove(&inode);
        self.directories.remove(&inode);

        if let Some(entries) = self.directories.get_mut(&parent) {
            entries.retain(|e| e.name != name);
        }

        Ok(())
    }

    fn rename(&mut self, old_parent: VfsInode, old_name: &str, new_parent: VfsInode, new_name: &str) -> Result<(), String> {
        let inode = self.lookup(old_parent, old_name)?;

        // Remove from old parent
        if let Some(entries) = self.directories.get_mut(&old_parent) {
            entries.retain(|e| e.name != old_name);
        }

        // Add to new parent
        let file_type = self.inodes.get(&inode)
            .map(|attr| attr.file_type)
            .unwrap_or(VfsFileType::RegularFile);

        if let Some(entries) = self.directories.get_mut(&new_parent) {
            entries.push(VfsDirEntry {
                name: new_name.to_string(),
                inode,
                file_type,
            });
        }

        Ok(())
    }

    fn getattr(&self, inode: VfsInode) -> Result<VfsFileAttr, String> {
        self.inodes.get(&inode)
            .cloned()
            .ok_or_else(|| format!("Inode not found"))
    }

    fn setattr(&mut self, inode: VfsInode, attr: VfsFileAttr) -> Result<(), String> {
        self.inodes.insert(inode, attr);
        Ok(())
    }

    fn read(&self, inode: VfsInode, offset: u64, buf: &mut [u8]) -> Result<usize, String> {
        let data = self.data.get(&inode)
            .ok_or_else(|| format!("Inode not found"))?;

        let offset = offset as usize;
        if offset >= data.len() {
            return Ok(0);
        }

        let len = std::cmp::min(buf.len(), data.len() - offset);
        buf[..len].copy_from_slice(&data[offset..offset + len]);

        Ok(len)
    }

    fn write(&mut self, inode: VfsInode, offset: u64, buf: &[u8]) -> Result<usize, String> {
        let data = self.data.get_mut(&inode)
            .ok_or_else(|| format!("Inode not found"))?;

        let offset = offset as usize;
        if offset + buf.len() > data.len() {
            data.resize(offset + buf.len(), 0);
        }

        data[offset..offset + buf.len()].copy_from_slice(buf);

        // Update file size
        if let Some(attr) = self.inodes.get_mut(&inode) {
            attr.size = data.len() as u64;
        }

        Ok(buf.len())
    }

    fn readdir(&self, inode: VfsInode) -> Result<Vec<VfsDirEntry>, String> {
        self.directories.get(&inode)
            .cloned()
            .ok_or_else(|| format!("Directory not found"))
    }

    fn symlink(&mut self, parent: VfsInode, name: &str, target: &str) -> Result<VfsInode, String> {
        let inode = self.allocate_inode();

        self.inodes.insert(inode, VfsFileAttr {
            inode,
            file_type: VfsFileType::SymbolicLink,
            size: target.len() as u64,
            mode: 0o777,
            uid: 0,
            gid: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
        });

        self.symlinks.insert(inode, target.to_string());

        if let Some(entries) = self.directories.get_mut(&parent) {
            entries.push(VfsDirEntry {
                name: name.to_string(),
                inode,
                file_type: VfsFileType::SymbolicLink,
            });
        }

        Ok(inode)
    }

    fn readlink(&self, inode: VfsInode) -> Result<String, String> {
        self.symlinks.get(&inode)
            .cloned()
            .ok_or_else(|| format!("Symlink not found"))
    }
}

/// POSIX path resolver
pub struct PosixPathResolver {
    vfs: Box<dyn VfsOperations>,
    current_dir: VfsInode,
    root_dir: VfsInode,
}

impl PosixPathResolver {
    pub fn new(vfs: Box<dyn VfsOperations>) -> Self {
        PosixPathResolver {
            vfs,
            current_dir: VfsInode::new(0),
            root_dir: VfsInode::new(0),
        }
    }

    /// Resolve a POSIX path to an inode
    pub fn resolve(&self, path: &str) -> Result<VfsInode, String> {
        let path = Path::new(path);

        if path.is_absolute() {
            self.resolve_absolute(path)
        } else {
            self.resolve_relative(path)
        }
    }

    fn resolve_absolute(&self, path: &Path) -> Result<VfsInode, String> {
        let mut current = self.root_dir;

        for component in path.components() {
            use std::path::Component;
            match component {
                Component::RootDir => continue,
                Component::CurDir => continue,
                Component::ParentDir => {
                    // Go to parent directory
                    // Simplified - in real implementation would track parent
                }
                Component::Normal(name) => {
                    let name = name.to_str().ok_or_else(|| "Invalid UTF-8 in path")?;
                    current = self.vfs.lookup(current, name)?;
                }
                Component::Prefix(_) => return Err("Prefix paths not supported".to_string()),
            }
        }

        Ok(current)
    }

    fn resolve_relative(&self, path: &Path) -> Result<VfsInode, String> {
        let mut current = self.current_dir;

        for component in path.components() {
            use std::path::Component;
            match component {
                Component::RootDir => current = self.root_dir,
                Component::CurDir => continue,
                Component::ParentDir => {
                    // Go to parent directory
                }
                Component::Normal(name) => {
                    let name = name.to_str().ok_or_else(|| "Invalid UTF-8 in path")?;
                    current = self.vfs.lookup(current, name)?;
                }
                Component::Prefix(_) => return Err("Prefix paths not supported".to_string()),
            }
        }

        Ok(current)
    }

    /// Normalize a POSIX path (remove . and .. components)
    pub fn normalize(path: &str) -> String {
        let path = Path::new(path);
        let mut result = PathBuf::new();

        for component in path.components() {
            use std::path::Component;
            match component {
                Component::RootDir => result.push("/"),
                Component::CurDir => continue,
                Component::ParentDir => {
                    result.pop();
                }
                Component::Normal(name) => {
                    result.push(name);
                }
                Component::Prefix(_) => continue,
            }
        }

        result.to_string_lossy().to_string()
    }

    /// Check if a path is absolute
    pub fn is_absolute(path: &str) -> bool {
        Path::new(path).is_absolute()
    }

    /// Get current working directory inode
    pub fn current_dir(&self) -> VfsInode {
        self.current_dir
    }

    /// Change current working directory
    pub fn chdir(&mut self, path: &str) -> Result<(), String> {
        let inode = self.resolve(path)?;
        self.current_dir = inode;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_vfs_create() {
        let mut vfs = MemoryVfs::new();
        let root = VfsInode::new(0);

        let file = vfs.create(root, "test.txt", 0o644).unwrap();
        assert!(vfs.lookup(root, "test.txt").is_ok());
    }

    #[test]
    fn test_memory_vfs_mkdir() {
        let mut vfs = MemoryVfs::new();
        let root = VfsInode::new(0);

        let dir = vfs.mkdir(root, "testdir", 0o755).unwrap();
        assert!(vfs.lookup(root, "testdir").is_ok());
    }

    #[test]
    fn test_memory_vfs_read_write() {
        let mut vfs = MemoryVfs::new();
        let root = VfsInode::new(0);

        let file = vfs.create(root, "test.txt", 0o644).unwrap();
        vfs.write(file, 0, b"hello").unwrap();

        let mut buf = [0u8; 5];
        let n = vfs.read(file, 0, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");
    }

    #[test]
    fn test_path_normalize() {
        assert_eq!(PosixPathResolver::normalize("/foo/../bar"), "/bar");
        assert_eq!(PosixPathResolver::normalize("/foo/./bar"), "/foo/bar");
        assert_eq!(PosixPathResolver::normalize("/foo/bar/../baz"), "/foo/baz");
    }

    #[test]
    fn test_path_is_absolute() {
        assert!(PosixPathResolver::is_absolute("/foo/bar"));
        assert!(!PosixPathResolver::is_absolute("foo/bar"));
    }
}
