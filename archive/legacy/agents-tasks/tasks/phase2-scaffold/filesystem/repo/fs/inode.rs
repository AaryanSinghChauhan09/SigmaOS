// Inode Lifecycle and Metadata Representation
// Part of SigmaOS File Management Subsystem

extern crate alloc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use super::fd::FsError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeType {
    RegularFile,
    Directory,
    SymbolicLink,
    CharacterDevice,
    BlockDevice,
    FifoPipe,
    Socket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum InodeState {
    Allocated = 0,
    Active = 1,
    Unlinked = 2,
    Reclaimed = 3,
}

impl InodeState {
    pub fn from_u32(val: u32) -> Self {
        match val {
            0 => InodeState::Allocated,
            1 => InodeState::Active,
            2 => InodeState::Unlinked,
            3 => InodeState::Reclaimed,
            _ => InodeState::Active,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InodeMetadata {
    pub inode_id: u64,
    pub inode_type: InodeType,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub nlink: u32,
    pub state: InodeState,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}

#[derive(Debug)]
pub struct Inode {
    pub metadata: InodeMetadata,
    pub state_atomic: AtomicU32,
    pub ref_count: AtomicU32,
    pub size_atomic: AtomicU64,
    pub content: Vec<u8>,
}

impl Inode {
    pub fn new(
        inode_id: u64,
        inode_type: InodeType,
        mode: u32,
        uid: u32,
        gid: u32,
        now: u64,
    ) -> Self {
        Self {
            metadata: InodeMetadata {
                inode_id,
                inode_type,
                mode,
                uid,
                gid,
                size: 0,
                nlink: 1,
                state: InodeState::Active,
                atime: now,
                mtime: now,
                ctime: now,
            },
            state_atomic: AtomicU32::new(InodeState::Active as u32),
            ref_count: AtomicU32::new(1),
            size_atomic: AtomicU64::new(0),
            content: Vec::new(),
        }
    }

    pub fn get_state(&self) -> InodeState {
        InodeState::from_u32(self.state_atomic.load(Ordering::Relaxed))
    }

    pub fn set_state(&self, state: InodeState) {
        self.state_atomic.store(state as u32, Ordering::Relaxed);
    }

    pub fn inc_ref(&self) -> u32 {
        self.ref_count.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn dec_ref(&self) -> u32 {
        let prev = self.ref_count.fetch_sub(1, Ordering::SeqCst);
        if prev > 0 { prev - 1 } else { 0 }
    }

    pub fn get_ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::Relaxed)
    }

    pub fn set_size(&mut self, new_size: u64) {
        self.metadata.size = new_size;
        self.size_atomic.store(new_size, Ordering::Relaxed);
    }

    pub fn get_size(&self) -> u64 {
        self.size_atomic.load(Ordering::Relaxed)
    }
}

#[derive(Debug)]
pub struct InodeManager {
    inodes: Vec<Option<Arc<Inode>>>,
    next_inode_id: u64,
}

impl InodeManager {
    pub fn new() -> Self {
        Self {
            inodes: Vec::new(),
            next_inode_id: 1,
        }
    }

    pub fn create_inode(
        &mut self,
        inode_type: InodeType,
        mode: u32,
        uid: u32,
        gid: u32,
        now: u64,
    ) -> Result<Arc<Inode>, FsError> {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        let inode = Arc::new(Inode::new(id, inode_type, mode, uid, gid, now));
        self.inodes.push(Some(inode.clone()));
        Ok(inode)
    }

    pub fn get_inode(&self, inode_id: u64) -> Result<Arc<Inode>, FsError> {
        for item in &self.inodes {
            if let Some(ref inode) = item {
                if inode.metadata.inode_id == inode_id {
                    return Ok(inode.clone());
                }
            }
        }
        Err(FsError::NotFound)
    }

    pub fn unlink_inode(&mut self, inode_id: u64) -> Result<(), FsError> {
        let inode = self.get_inode(inode_id)?;
        let state = inode.get_state();
        if state == InodeState::Unlinked || state == InodeState::Reclaimed {
            return Err(FsError::NotFound);
        }
        inode.set_state(InodeState::Unlinked);
        let mut idx_to_remove = None;
        for (i, item) in self.inodes.iter().enumerate() {
            if let Some(ref in_node) = item {
                if in_node.metadata.inode_id == inode_id {
                    idx_to_remove = Some(i);
                    break;
                }
            }
        }
        if let Some(i) = idx_to_remove {
            self.inodes[i] = None;
            Ok(())
        } else {
            Err(FsError::NotFound)
        }
    }

    pub fn active_inode_count(&self) -> usize {
        self.inodes.iter().filter(|i| i.is_some()).count()
    }
}
