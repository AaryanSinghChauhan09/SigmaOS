//! Tmpfs (In-Memory Virtual File System) Subsystem
//! Inspired by Linux's on-demand VM allocations and FreeBSD's swap-backed tmpfs mechanics.

#![no_std]

pub const MAX_TMPFS_INODES: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmpfsFileType {
    Regular,
    Directory,
    Symlink,
}

#[derive(Debug, Clone, Copy)]
pub struct TmpfsConfig {
    pub max_bytes: usize,  // Maximum memory bytes allocation limit
    pub max_inodes: usize, // Maximum inode registry slots
    pub uid: u32,          // Owner UID (default 0/root)
    pub gid: u32,          // Group GID (default 0/root)
    pub mode: u32,         // Permissions (e.g. 0o1777 sticky-bit tmpfs)
}

#[derive(Debug, Clone, Copy)]
pub struct TmpfsInode {
    pub id: usize,
    pub file_type: TmpfsFileType,
    pub apparent_size: usize,  // Size of file content
    pub is_swapbacked: bool,   // True if paged out to anonymous swap space
    pub link_count: u32,       // Hard link count reference
    pub mtime: u64,            // Modification time
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
}

pub struct TmpfsFileSystem {
    pub config: TmpfsConfig,
    pub current_bytes_used: usize,
    pub current_inodes_used: usize,
    pub inodes: [Option<TmpfsInode>; MAX_TMPFS_INODES],
    pub next_inode_id: usize,
}

impl TmpfsFileSystem {
    pub fn new(config: TmpfsConfig) -> Self {
        Self {
            config,
            current_bytes_used: 0,
            current_inodes_used: 0,
            inodes: [None; MAX_TMPFS_INODES],
            next_inode_id: 1,
        }
    }

    /// Create a new in-memory file or directory node (on-demand dynamic RAM allocation)
    pub fn create_node(
        &mut self,
        file_type: TmpfsFileType,
        size_bytes: usize,
        mtime: u64,
    ) -> Result<usize, &'static str> {
        // 1. Enforce max inodes boundary limit
        if self.current_inodes_used >= self.config.max_inodes || self.current_inodes_used >= MAX_TMPFS_INODES {
            return Err("Tmpfs reached maximum inode capacity limit");
        }

        // 2. Enforce memory byte allocation limit (on-demand page reservation)
        if self.current_bytes_used + size_bytes > self.config.max_bytes {
            return Err("Tmpfs allocation request exceeds memory limit");
        }

        let id = self.next_inode_id;
        self.next_inode_id += 1;

        let inode = TmpfsInode {
            id,
            file_type,
            apparent_size: size_bytes,
            is_swapbacked: false,
            link_count: 1, // baseline single hard link reference
            mtime,
            uid: self.config.uid,
            gid: self.config.gid,
            mode: self.config.mode,
        };

        for slot in &mut self.inodes {
            if slot.is_none() {
                *slot = Some(inode);
                self.current_bytes_used += size_bytes;
                self.current_inodes_used += 1;
                return Ok(id);
            }
        }

        Err("Tmpfs inode registry table error")
    }

    /// Link an existing inode (Linux/BSD style hard links)
    pub fn link_node(&mut self, id: usize) -> Result<(), &'static str> {
        for slot in &mut self.inodes {
            if let Some(ref mut inode) = slot {
                if inode.id == id {
                    inode.link_count += 1;
                    return Ok(());
                }
            }
        }
        Err("Inode not found to create link")
    }

    /// Unlink an inode. Reclaims memory when link count drops to zero
    pub fn unlink_node(&mut self, id: usize) -> Result<(), &'static str> {
        let mut index = None;
        let mut reclaim_bytes = 0;
        let mut is_deleted = false;

        for (i, slot) in self.inodes.iter_mut().enumerate() {
            if let Some(ref mut inode) = slot {
                if inode.id == id {
                    if inode.link_count > 1 {
                        inode.link_count -= 1;
                        return Ok(());
                    } else {
                        // link_count is 1, so decrementing drops it to 0 (delete/reclaim)
                        reclaim_bytes = if inode.is_swapbacked { 0 } else { inode.apparent_size };
                        index = Some(i);
                        is_deleted = true;
                        break;
                    }
                }
            }
        }

        if is_deleted {
            if let Some(idx) = index {
                self.inodes[idx] = None;
                self.current_bytes_used = self.current_bytes_used.saturating_sub(reclaim_bytes);
                self.current_inodes_used = self.current_inodes_used.saturating_sub(1);
                return Ok(());
            }
        }

        Err("Inode not found to unlink")
    }

    /// Simulate FreeBSD/Linux dynamic swap backing page-outs under extreme memory pressure
    pub fn swap_out_node(&mut self, id: usize) -> Result<(), &'static str> {
        for slot in &mut self.inodes {
            if let Some(ref mut inode) = slot {
                if inode.id == id {
                    if inode.is_swapbacked {
                        return Err("Node is already swapped out");
                    }
                    inode.is_swapbacked = true;
                    // Reclaim physical RAM footprint by paging out to disk block swap mappings
                    self.current_bytes_used = self.current_bytes_used.saturating_sub(inode.apparent_size);
                    return Ok(());
                }
            }
        }
        Err("Inode not found to swap out")
    }

    /// Page the swapped-out node back into active physical memory
    pub fn swap_in_node(&mut self, id: usize) -> Result<(), &'static str> {
        for slot in &mut self.inodes {
            if let Some(ref mut inode) = slot {
                if inode.id == id {
                    if !inode.is_swapbacked {
                        return Err("Node is already resident in memory");
                    }
                    // Check if bringing back to RAM exceeds limits
                    if self.current_bytes_used + inode.apparent_size > self.config.max_bytes {
                        return Err("Insufficient Tmpfs memory to page in node");
                    }
                    inode.is_swapbacked = false;
                    self.current_bytes_used += inode.apparent_size;
                    return Ok(());
                }
            }
        }
        Err("Inode not found to swap in")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmpfs_dynamic_allocation_limits() {
        let config = TmpfsConfig {
            max_bytes: 8192, // 8KB limit
            max_inodes: 4,
            uid: 0,
            gid: 0,
            mode: 0o1777,
        };

        let mut fs = TmpfsFileSystem::new(config);

        // Allocate a 4KB file - succeeds
        let f1 = fs.create_node(TmpfsFileType::Regular, 4096, 1625100000).unwrap();
        assert_eq!(fs.current_bytes_used, 4096);
        assert_eq!(fs.current_inodes_used, 1);

        // Allocate another 4KB file - succeeds
        let f2 = fs.create_node(TmpfsFileType::Regular, 4096, 1625100005).unwrap();
        assert_eq!(fs.current_bytes_used, 8192);

        // Allocate a third 1KB file - fails with exceeds memory limits
        let f3 = fs.create_node(TmpfsFileType::Regular, 1024, 1625100010);
        assert_eq!(f3, Err("Tmpfs allocation request exceeds memory limit"));

        // Unlink f1 to reclaim memory
        fs.unlink_node(f1).unwrap();
        assert_eq!(fs.current_bytes_used, 4096);

        // Try allocating f3 again - now succeeds!
        let f3_ok = fs.create_node(TmpfsFileType::Regular, 1024, 1625100010).unwrap();
        assert_eq!(fs.current_bytes_used, 5120);
    }

    #[test]
    fn test_tmpfs_link_unlink_lifecycle() {
        let config = TmpfsConfig {
            max_bytes: 10240,
            max_inodes: 10,
            uid: 0,
            gid: 0,
            mode: 0o1777,
        };

        let mut fs = TmpfsFileSystem::new(config);
        let fid = fs.create_node(TmpfsFileType::Regular, 1024, 0).unwrap();

        // Initial link count = 1
        assert_eq!(fs.inodes[0].unwrap().link_count, 1);

        // Hard link the node
        fs.link_node(fid).unwrap();
        assert_eq!(fs.inodes[0].unwrap().link_count, 2);

        // Unlink first reference - does not delete inode because link_count = 1
        fs.unlink_node(fid).unwrap();
        assert_eq!(fs.inodes[0].unwrap().link_count, 1);
        assert_eq!(fs.current_bytes_used, 1024);

        // Unlink second reference - drops link_count to 0, deleting the node and reclaiming memory
        fs.unlink_node(fid).unwrap();
        assert_eq!(fs.current_bytes_used, 0);
        assert!(fs.inodes[0].is_none());
    }

    #[test]
    fn test_tmpfs_swap_backing() {
        let config = TmpfsConfig {
            max_bytes: 4096, // 4KB limit
            max_inodes: 10,
            uid: 0,
            gid: 0,
            mode: 0o1777,
        };

        let mut fs = TmpfsFileSystem::new(config);
        let fid = fs.create_node(TmpfsFileType::Regular, 4096, 0).unwrap();
        assert_eq!(fs.current_bytes_used, 4096);

        // Swap out the file - physical RAM is freed (0 bytes used), but inode is retained!
        fs.swap_out_node(fid).unwrap();
        assert_eq!(fs.current_bytes_used, 0);
        assert!(fs.inodes[0].unwrap().is_swapbacked);

        // We can now allocate another 4KB file in RAM!
        let fid2 = fs.create_node(TmpfsFileType::Regular, 4096, 0).unwrap();
        assert_eq!(fs.current_bytes_used, 4096);

        // Trying to swap in fid must fail because it would exceed limits (4KB + 4KB > 4KB max_bytes)
        let swap_in_res = fs.swap_in_node(fid);
        assert_eq!(swap_in_res, Err("Insufficient Tmpfs memory to page in node"));

        // Delete fid2
        fs.unlink_node(fid2).unwrap();

        // Swap in succeeds now!
        fs.swap_in_node(fid).unwrap();
        assert_eq!(fs.current_bytes_used, 4096);
        assert!(!fs.inodes[0].unwrap().is_swapbacked);
    }
}
