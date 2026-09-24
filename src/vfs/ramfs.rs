/// SigmaOS RAMFS / Tmpfs Implementation
/// Inspired by Linux's tmpfs and FreeBSD's tmpfs, this module provides
/// the volatile memory-backed file system required for early boot and /dev/.

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    CharacterDevice,
    BlockDevice,
}

#[derive(Debug, Clone)]
pub struct RamfsNode {
    pub name: String,
    pub node_type: FileType,
    pub size: usize,
    pub data: Vec<u8>,
    pub children: HashMap<String, usize>, // maps child name to inode number
    pub mode: u16,
    pub uid: u32,
    pub gid: u32,
}

pub struct RamFileSystem {
    inodes: HashMap<usize, RamfsNode>,
    next_inode: usize,
}

impl RamFileSystem {
    pub fn new() -> Self {
        let mut fs = Self {
            inodes: HashMap::new(),
            next_inode: 2, // 1 is root
        };
        // Create root directory
        let root = RamfsNode {
            name: String::from("/"),
            node_type: FileType::Directory,
            size: 0,
            data: Vec::new(),
            children: HashMap::new(),
            mode: 0o755,
            uid: 0,
            gid: 0,
        };
        fs.inodes.insert(1, root);
        fs
    }

    pub fn create_file(&mut self, parent_inode: usize, name: &str, mode: u16) -> Result<usize, &'static str> {
        let inode_num = self.next_inode;
        self.next_inode += 1;

        let new_file = RamfsNode {
            name: String::from(name),
            node_type: FileType::Regular,
            size: 0,
            data: Vec::new(),
            children: HashMap::new(),
            mode,
            uid: 0,
            gid: 0,
        };

        if let Some(parent) = self.inodes.get_mut(&parent_inode) {
            if matches!(parent.node_type, FileType::Directory) {
                parent.children.insert(String::from(name), inode_num);
            } else {
                return Err("Parent is not a directory");
            }
        } else {
            return Err("Parent inode not found");
        }

        self.inodes.insert(inode_num, new_file);
        Ok(inode_num)
    }

    pub fn write_file(&mut self, inode: usize, data: &[u8]) -> Result<usize, &'static str> {
        if let Some(node) = self.inodes.get_mut(&inode) {
            if matches!(node.node_type, FileType::Regular) {
                node.data.extend_from_slice(data);
                node.size = node.data.len();
                Ok(data.len())
            } else {
                Err("Not a regular file")
            }
        } else {
            Err("Inode not found")
        }
    }

    pub fn read_file(&self, inode: usize) -> Result<&[u8], &'static str> {
        if let Some(node) = self.inodes.get(&inode) {
            if matches!(node.node_type, FileType::Regular) {
                Ok(&node.data)
            } else {
                Err("Not a regular file")
            }
        } else {
            Err("Inode not found")
        }
    }
}
