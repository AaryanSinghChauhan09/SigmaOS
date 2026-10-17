//! DragonFly BSD HAMMER2 Transactional Storage Engine for SigmaOS
//!
//! Inspired by DragonFly BSD's HAMMER2 filesystem:
//! - Copy-on-Write (CoW) B-tree block allocation
//! - 64-bit Transaction ID (TID) progression and atomic commits
//! - Subvolume branch snapshots and instant rollback
//! - Checksumming (Fletcher4 / CRC32c) and block integrity validation

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub const HAMMER2_BLOCK_SIZE: usize = 4096;
pub const HAMMER2_MAGIC: u64 = 0x48414d4d45523230; // "HAMMER20"

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumType {
    None,
    Crc32c,
    Fletcher4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    VolumeHeader,
    Inode,
    Data,
    Directory,
    Indirect,
    Freemap,
}

#[derive(Debug, Clone)]
pub struct Hammer2Block {
    pub block_id: u64,
    pub block_type: BlockType,
    pub transaction_id: u64,
    pub data: Vec<u8>,
    pub checksum: u64,
}

#[derive(Debug, Clone)]
pub struct Hammer2Inode {
    pub inum: u64,
    pub parent_inum: u64,
    pub size: u64,
    pub name: String,
    pub direct_blocks: Vec<u64>,
    pub is_dir: bool,
    pub creation_tid: u64,
}

#[derive(Debug, Clone)]
pub struct Hammer2Snapshot {
    pub name: String,
    pub snapshot_tid: u64,
    pub root_inum: u64,
}

pub struct Hammer2Engine {
    pub magic: u64,
    pub current_tid: u64,
    pub blocks: BTreeMap<u64, Hammer2Block>,
    pub inodes: BTreeMap<u64, Hammer2Inode>,
    pub snapshots: BTreeMap<String, Hammer2Snapshot>,
    pub next_block_id: u64,
    pub next_inum: u64,
}

impl Hammer2Engine {
    pub fn new() -> Self {
        let mut engine = Self {
            magic: HAMMER2_MAGIC,
            current_tid: 1,
            blocks: BTreeMap::new(),
            inodes: BTreeMap::new(),
            snapshots: BTreeMap::new(),
            next_block_id: 1,
            next_inum: 1,
        };
        // Initialize root directory inode (inum = 1)
        engine.inodes.insert(
            1,
            Hammer2Inode {
                inum: 1,
                parent_inum: 1,
                size: 0,
                name: String::from("/"),
                direct_blocks: Vec::new(),
                is_dir: true,
                creation_tid: 1,
            },
        );
        engine
    }

    /// Calculate Fletcher-4-like checksum for block integrity
    pub fn compute_checksum(data: &[u8]) -> u64 {
        let mut sum1: u64 = 0;
        let mut sum2: u64 = 0;
        for &byte in data {
            sum1 = (sum1 + byte as u64) % 0xFFFFFFFF;
            sum2 = (sum2 + sum1) % 0xFFFFFFFF;
        }
        (sum2 << 32) | sum1
    }

    /// Allocate a new CoW block associated with the current transaction
    pub fn allocate_block(&mut self, block_type: BlockType, data: Vec<u8>) -> u64 {
        let bid = self.next_block_id;
        self.next_block_id += 1;
        let checksum = Self::compute_checksum(&data);

        let block = Hammer2Block {
            block_id: bid,
            block_type,
            transaction_id: self.current_tid,
            data,
            checksum,
        };
        self.blocks.insert(bid, block);
        bid
    }

    /// Write data to an inode, creating new CoW blocks without mutating old ones
    pub fn write_file(&mut self, name: &str, data: &[u8]) -> Result<u64, &'static str> {
        self.current_tid += 1;
        let inum = self.next_inum;
        self.next_inum += 1;

        let mut direct_blocks = Vec::new();
        for chunk in data.chunks(HAMMER2_BLOCK_SIZE) {
            let bid = self.allocate_block(BlockType::Data, chunk.to_vec());
            direct_blocks.push(bid);
        }

        let inode = Hammer2Inode {
            inum,
            parent_inum: 1,
            size: data.len() as u64,
            name: String::from(name),
            direct_blocks,
            is_dir: false,
            creation_tid: self.current_tid,
        };
        self.inodes.insert(inum, inode);
        Ok(inum)
    }

    /// Read file content assembling CoW blocks
    pub fn read_file(&self, inum: u64) -> Result<Vec<u8>, &'static str> {
        let inode = self.inodes.get(&inum).ok_or("Inode not found")?;
        let mut result = Vec::new();

        for &bid in &inode.direct_blocks {
            let block = self.blocks.get(&bid).ok_or("Block missing from store")?;
            // Verify integrity
            let expected = Self::compute_checksum(&block.data);
            if block.checksum != expected {
                return Err("HAMMER2 block checksum mismatch");
            }
            result.extend_from_slice(&block.data);
        }
        result.truncate(inode.size as usize);
        Ok(result)
    }

    /// Create an instantaneous snapshot branch at current TID
    pub fn create_snapshot(&mut self, name: &str) -> Hammer2Snapshot {
        let snapshot = Hammer2Snapshot {
            name: String::from(name),
            snapshot_tid: self.current_tid,
            root_inum: 1,
        };
        self.snapshots.insert(String::from(name), snapshot.clone());
        snapshot
    }

    /// Rollback active filesystem state to a specified snapshot TID
    pub fn rollback_to_snapshot(&mut self, name: &str) -> Result<u64, &'static str> {
        let snap = self.snapshots.get(name).ok_or("Snapshot not found")?;
        let target_tid = snap.snapshot_tid;

        // Prune inodes created after snapshot TID
        self.inodes.retain(|_, inod| inod.creation_tid <= target_tid);
        // Prune blocks allocated after snapshot TID
        self.blocks.retain(|_, blk| blk.transaction_id <= target_tid);

        self.current_tid = target_tid;
        Ok(target_tid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hammer2_write_and_read() {
        let mut engine = Hammer2Engine::new();
        let payload = b"DragonFly BSD HAMMER2 transactional storage engine for SigmaOS kernel.";
        let inum = engine.write_file("kernel_test.txt", payload).unwrap();

        let read_back = engine.read_file(inum).unwrap();
        assert_eq!(read_back, payload);
    }

    #[test]
    fn test_hammer2_snapshot_and_rollback() {
        let mut engine = Hammer2Engine::new();
        let payload_v1 = b"State Version 1";
        let inum_v1 = engine.write_file("file1.txt", payload_v1).unwrap();

        engine.create_snapshot("snap_v1");

        let payload_v2 = b"State Version 2";
        let inum_v2 = engine.write_file("file2.txt", payload_v2).unwrap();

        assert!(engine.read_file(inum_v2).is_ok());

        // Rollback to snap_v1
        let rolled_tid = engine.rollback_to_snapshot("snap_v1").unwrap();
        assert_eq!(rolled_tid, 2);

        // file1 still exists, file2 is rolled back
        assert!(engine.read_file(inum_v1).is_ok());
        assert!(engine.read_file(inum_v2).is_err());
    }
}
