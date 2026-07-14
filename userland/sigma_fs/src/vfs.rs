use crate::inode::{Inode, InodeKind};

/// Sovereign VFS — absorbs SerenityOS VFS, Linux VFS, and Haiku BFS design.
/// No libc, no stdlib fs module, no external crates.
#[derive(Debug, Default)]
pub struct FileEntry {
    pub path: String,
    pub inode_id: u64,
}

pub struct Vfs {
    pub entries: Vec<FileEntry>,
    pub inodes: Vec<Inode>,
    next_inode: u64,
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}

impl Vfs {
    pub fn new() -> Self {
        Self { entries: Vec::new(), inodes: Vec::new(), next_inode: 1 }
    }

    pub fn create_file(&mut self, path: &str, size: u64) -> u64 {
        let id = self.next_inode;
        self.next_inode += 1;
        self.inodes.push(Inode { id, kind: InodeKind::File, size });
        self.entries.push(FileEntry { path: path.to_string(), inode_id: id });
        id
    }

    pub fn lookup(&self, path: &str) -> Option<&Inode> {
        let inode_id = self.entries.iter().find(|e| e.path == path)?.inode_id;
        self.inodes.iter().find(|i| i.id == inode_id)
    }
}
