// XFS - Linux-style high-performance journaling filesystem
// Supports allocation groups, extent-based allocation, and journaling

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XfsState {
    Clean,
    Dirty,
    Mounted,
    Unmounting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    FirstFit,
    BestFit,
    Near,
    Exact,
}

#[derive(Debug, Clone)]
pub struct XfsAllocationGroup {
    pub id: u32,
    pub start_block: u64,
    pub block_count: u64,
    pub free_blocks: u64,
    pub used_blocks: u64,
}

#[derive(Debug, Clone)]
pub struct XfsInode {
    pub id: u64,
    pub size: u64,
    pub blocks: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub mode: u32,
    pub nlink: u32,
}

#[derive(Debug, Clone)]
pub struct XfsExtent {
    pub start_block: u64,
    pub block_count: u64,
    pub offset: u64,
}

#[derive(Debug, Clone)]
pub struct XfsJournal {
    pub id: u64,
    pub head: u64,
    pub tail: u64,
    pub size: u64,
    pub enabled: bool,
}

pub struct XfsFilesystem {
    allocation_groups: BTreeMap<u32, XfsAllocationGroup>,
    inodes: BTreeMap<u64, XfsInode>,
    extents: BTreeMap<u64, Vec<XfsExtent>>,
    journal: Option<XfsJournal>,
    state: XfsState,
    block_size: u32,
    total_blocks: u64,
    next_inode_id: u64,
}

impl XfsFilesystem {
    pub fn new(total_blocks: u64, block_size: u32, ag_count: u32) -> Self {
        let mut allocation_groups = BTreeMap::new();
        let blocks_per_ag = total_blocks / ag_count as u64;

        for i in 0..ag_count {
            let ag = XfsAllocationGroup {
                id: i,
                start_block: i as u64 * blocks_per_ag,
                block_count: blocks_per_ag,
                free_blocks: blocks_per_ag,
                used_blocks: 0,
            };
            allocation_groups.insert(i, ag);
        }

        Self {
            allocation_groups,
            inodes: BTreeMap::new(),
            extents: BTreeMap::new(),
            journal: None,
            state: XfsState::Clean,
            block_size,
            total_blocks,
            next_inode_id: 1,
        }
    }

    /// Create a new inode
    pub fn create_inode(&mut self, size: u64, mode: u32) -> Result<u64, &'static str> {
        let id = self.next_inode_id;
        self.next_inode_id += 1;

        let inode = XfsInode {
            id,
            size,
            blocks: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
            mode,
            nlink: 1,
        };

        self.inodes.insert(id, inode);
        self.state = XfsState::Dirty;

        Ok(id)
    }

    /// Allocate blocks for an inode
    pub fn allocate_blocks(&mut self, inode_id: u64, block_count: u64, strategy: AllocationStrategy) -> Result<Vec<XfsExtent>, &'static str> {
        if !self.inodes.contains_key(&inode_id) {
            return Err("Inode not found");
        }

        let mut allocated_extents = Vec::new();
        let mut remaining = block_count;

        // Find allocation groups with free space
        for ag in self.allocation_groups.values_mut() {
            if remaining == 0 {
                break;
            }

            if ag.free_blocks >= remaining {
                // Allocate from this AG
                let extent = XfsExtent {
                    start_block: ag.start_block + (ag.block_count - ag.free_blocks),
                    block_count: remaining,
                    offset: 0,
                };

                allocated_extents.push(extent);
                ag.free_blocks -= remaining;
                ag.used_blocks += remaining;
                remaining = 0;
            } else if ag.free_blocks > 0 {
                // Allocate partial from this AG
                let extent = XfsExtent {
                    start_block: ag.start_block + (ag.block_count - ag.free_blocks),
                    block_count: ag.free_blocks,
                    offset: 0,
                };

                allocated_extents.push(extent);
                remaining -= ag.free_blocks;
                ag.used_blocks += ag.free_blocks;
                ag.free_blocks = 0;
            }
        }

        if remaining > 0 {
            return Err("Insufficient free space");
        }

        // Update inode
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            inode.blocks += block_count;
        }

        // Store extents for inode
        self.extents.insert(inode_id, allocated_extents.clone());
        self.state = XfsState::Dirty;

        Ok(allocated_extents)
    }

    /// Get inode by ID
    pub fn get_inode(&self, id: u64) -> Option<&XfsInode> {
        self.inodes.get(&id)
    }

    /// Get extents for an inode
    pub fn get_extents(&self, inode_id: u64) -> Option<&Vec<XfsExtent>> {
        self.extents.get(&inode_id)
    }

    /// Delete an inode
    pub fn delete_inode(&mut self, id: u64) -> Result<(), &'static str> {
        let inode = self.inodes.remove(&id)
            .ok_or("Inode not found")?;

        // Free blocks
        if let Some(extents) = self.extents.remove(&id) {
            for extent in extents {
                self.free_blocks(extent.start_block, extent.block_count)?;
            }
        }

        self.state = XfsState::Dirty;
        Ok(())
    }

    /// Free blocks back to allocation groups
    fn free_blocks(&mut self, start_block: u64, block_count: u64) -> Result<(), &'static str> {
        for ag in self.allocation_groups.values_mut() {
            if start_block >= ag.start_block && start_block < ag.start_block + ag.block_count {
                ag.free_blocks += block_count;
                ag.used_blocks -= block_count;
                return Ok(());
            }
        }
        Err("Allocation group not found for block")
    }

    /// Enable journaling
    pub fn enable_journal(&mut self, journal_size: u64) -> Result<(), &'static str> {
        if self.journal.is_some() {
            return Err("Journal already exists");
        }

        let journal = XfsJournal {
            id: 1,
            head: 0,
            tail: 0,
            size: journal_size,
            enabled: true,
        };

        self.journal = Some(journal);
        Ok(())
    }

    /// Disable journaling
    pub fn disable_journal(&mut self) -> Result<(), &'static str> {
        self.journal = None;
        Ok(())
    }

    /// Sync filesystem (flush journal)
    pub fn sync(&mut self) -> Result<(), &'static str> {
        if let Some(ref mut journal) = self.journal {
            if journal.enabled {
                journal.head = journal.tail;
            }
        }
        self.state = XfsState::Clean;
        Ok(())
    }

    /// Get filesystem state
    pub fn state(&self) -> XfsState {
        self.state
    }

    /// Get allocation group by ID
    pub fn get_allocation_group(&self, id: u32) -> Option<&XfsAllocationGroup> {
        self.allocation_groups.get(&id)
    }

    /// Get free space
    pub fn free_space(&self) -> u64 {
        self.allocation_groups.values().map(|ag| ag.free_blocks).sum()
    }

    /// Get used space
    pub fn used_space(&self) -> u64 {
        self.total_blocks - self.free_space()
    }

    /// Get inode count
    pub fn inode_count(&self) -> usize {
        self.inodes.len()
    }

    /// Get allocation group count
    pub fn allocation_group_count(&self) -> usize {
        self.allocation_groups.len()
    }
}

impl Default for XfsFilesystem {
    fn default() -> Self {
        Self::new(1024 * 1024, 4096, 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_inode() {
        let mut fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        let id = fs.create_inode(4096, 0o644).unwrap();
        assert_eq!(fs.inode_count(), 1);
        
        let inode = fs.get_inode(id).unwrap();
        assert_eq!(inode.size, 4096);
    }

    #[test]
    fn test_allocate_blocks() {
        let mut fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        let id = fs.create_inode(4096, 0o644).unwrap();
        let extents = fs.allocate_blocks(id, 8, AllocationStrategy::FirstFit).unwrap();
        
        assert_eq!(extents.len(), 1);
        assert_eq!(extents[0].block_count, 8);
    }

    #[test]
    fn test_delete_inode() {
        let mut fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        let id = fs.create_inode(4096, 0o644).unwrap();
        fs.allocate_blocks(id, 8, AllocationStrategy::FirstFit).unwrap();
        fs.delete_inode(id).unwrap();
        
        assert_eq!(fs.inode_count(), 0);
    }

    #[test]
    fn test_journal() {
        let mut fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        fs.enable_journal(1024 * 1024).unwrap();
        assert!(fs.journal.is_some());
        
        fs.disable_journal().unwrap();
        assert!(fs.journal.is_none());
    }

    #[test]
    fn test_sync() {
        let mut fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        fs.create_inode(4096, 0o644).unwrap();
        assert_eq!(fs.state(), XfsState::Dirty);
        
        fs.sync().unwrap();
        assert_eq!(fs.state(), XfsState::Clean);
    }

    #[test]
    fn test_free_space() {
        let fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        assert_eq!(fs.free_space(), 1024 * 1024);
        assert_eq!(fs.used_space(), 0);
    }

    #[test]
    fn test_allocation_groups() {
        let fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        assert_eq!(fs.allocation_group_count(), 4);
        
        let ag = fs.get_allocation_group(0).unwrap();
        assert_eq!(ag.free_blocks, 256 * 1024);
    }

    #[test]
    fn test_get_extents() {
        let mut fs = XfsFilesystem::new(1024 * 1024, 4096, 4);
        
        let id = fs.create_inode(4096, 0o644).unwrap();
        fs.allocate_blocks(id, 8, AllocationStrategy::FirstFit).unwrap();
        
        let extents = fs.get_extents(id).unwrap();
        assert_eq!(extents.len(), 1);
    }
}
