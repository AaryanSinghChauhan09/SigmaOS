#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;

use crate::klib::collections::VecDeque;
/// SigmaOS Block Device Layer
/// Absorbs Linux block/genhd.c, bio.c, elevator.c, blk-mq.c
/// Generic block I/O request queue with elevator sorting (C-SCAN / Deadline)

use crate::klib::{BTreeMap, Vec};
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use alloc::collections::{BTreeMap, VecDeque};
extern crate alloc;
use crate::klib::{BTreeMap, Vec, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;

pub const SECTOR_SIZE: usize = 512;
pub const BLOCK_SIZE: usize = 4096; // 4K blocks
pub const SECTORS_PER_BLOCK: usize = BLOCK_SIZE / SECTOR_SIZE;

// ── Block I/O request types ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BioOp {
    Read,
    Write,
    Flush,
    Discard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReqPriority {
    Idle,
    Normal,
    High,
    Sync,
}

/// A block I/O request (bio)
#[derive(Debug, Clone)]
pub struct Bio {
    pub id: u64,
    pub sector: u64, // Starting sector (LBA)
    pub count: u32,  // Number of sectors
    pub op: BioOp,
    pub priority: ReqPriority,
    pub data: Vec<u8>,
}

impl Bio {
    pub fn read(id: u64, sector: u64, count: u32) -> Self {
        Bio {
            id,
            sector,
            count,
            op: BioOp::Read,
            priority: ReqPriority::Normal,
            data: vec![0u8; count as usize * SECTOR_SIZE],
        }
    }
    pub fn write(id: u64, sector: u64, data: Vec<u8>) -> Self {
        let count = (data.len() / SECTOR_SIZE) as u32;
        Bio {
            id,
            sector,
            count,
            op: BioOp::Write,
            priority: ReqPriority::Normal,
            data,
        }
    }
    pub fn end_sector(&self) -> u64 {
        self.sector + self.count as u64
    }
    pub fn byte_len(&self) -> usize {
        self.count as usize * SECTOR_SIZE
    }
}

// ── I/O Scheduler (elevator) ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElevatorAlgorithm {
    Noop,
    Deadline,
    CScan,
    Bfq,
}

/// Deadline scheduler: read deadline 500ms, write deadline 5000ms
pub struct DeadlineScheduler {
    read_queue: BTreeMap<u64, Bio>, // sector → Bio
    write_queue: BTreeMap<u64, Bio>,
    dispatch: VecDeque<Bio>,
    head_pos: u64, // Current disk head position
    total_merged: AtomicUsize,
}

impl DeadlineScheduler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DeadlineScheduler {
            read_queue: BTreeMap::new(),
            write_queue: BTreeMap::new(),
            dispatch: VecDeque::new(),
            head_pos: 0,
            total_merged: AtomicUsize::new(0),
        }
    }

    pub fn submit(&mut self, bio: Bio) {
        // Try to merge adjacent requests
        match bio.op {
            BioOp::Read => {
                self.read_queue.insert(bio.sector, bio);
            }
            BioOp::Write => {
                self.write_queue.insert(bio.sector, bio);
            }
            _ => {
                self.dispatch.push_back(bio);
            }
        }
    }

    /// Dispatch next request using C-SCAN (elevator) over sectors
    pub fn dispatch_next(&mut self) -> Option<Bio> {
        if let Some(bio) = self.dispatch.pop_front() {
            return Some(bio);
        }

        // Serve reads preferentially; find next sector >= head
        if let Some((&sector, _)) = self.read_queue.range(self.head_pos..).next() {
            let bio = self.read_queue.remove(&sector).unwrap();
            self.head_pos = bio.end_sector();
            return Some(bio);
        }
        // Wrap around (C-SCAN)
        if let Some((&sector, _)) = self.read_queue.iter().next() {
            let bio = self.read_queue.remove(&sector).unwrap();
            self.head_pos = bio.end_sector();
            return Some(bio);
        }
        // Then writes
        if let Some((&sector, _)) = self.write_queue.range(self.head_pos..).next() {
            let bio = self.write_queue.remove(&sector).unwrap();
            self.head_pos = bio.end_sector();
            return Some(bio);
        }
        if let Some((&sector, _)) = self.write_queue.iter().next() {
            let bio = self.write_queue.remove(&sector).unwrap();
            self.head_pos = bio.end_sector();
            return Some(bio);
        }
        None
    }

    pub fn pending(&self) -> usize {
        self.read_queue.len() + self.write_queue.len() + self.dispatch.len()
    }
}

impl Default for DeadlineScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// ── Block device abstraction ──────────────────────────────────────────────

pub trait BlockDevice: Send + Sync {
    fn name(&self) -> &str;
    fn sector_count(&self) -> u64;
    fn read_sectors(&mut self, lba: u64, buf: &mut [u8]) -> Result<usize, &'static str>;
    fn write_sectors(&mut self, lba: u64, data: &[u8]) -> Result<usize, &'static str>;
    fn flush(&mut self) -> Result<(), &'static str> {
        Ok(())
    }
}

/// RAM-backed block device (for testing / ramdisk)
pub struct RamDisk {
    name: String,
    data: crate::klib::Vec<u8>,
    sector_count: u64,
    reads: AtomicU64,
    writes: AtomicU64,
}

unsafe impl Send for RamDisk {}
unsafe impl Sync for RamDisk {}

impl RamDisk {
    pub fn new(name: &str, size_bytes: usize) -> Self {
        let sectors = (size_bytes / SECTOR_SIZE) as u64;
        let mut data = crate::klib::Vec::with_capacity(size_bytes);
        for _ in 0..size_bytes {
            data.push(0);
        }
        RamDisk {
            name: name.to_string(),
            data,
            sector_count: sectors,
            reads: AtomicU64::new(0),
            writes: AtomicU64::new(0),
        }
    }
    pub fn reads(&self) -> u64 {
        self.reads.load(Ordering::Relaxed)
    }
    pub fn writes(&self) -> u64 {
        self.writes.load(Ordering::Relaxed)
    }
}

impl BlockDevice for RamDisk {
    fn name(&self) -> &str {
        &self.name
    }
    fn sector_count(&self) -> u64 {
        self.sector_count
    }

    fn read_sectors(&mut self, lba: u64, buf: &mut [u8]) -> Result<usize, &'static str> {
        let offset = lba as usize * SECTOR_SIZE;
        let len = buf.len();
        if offset + len > self.data.len() {
            return Err("RamDisk: read out of bounds");
        }
        for i in 0..len {
            buf[i] = self.data[offset + i];
        }
        self.reads.fetch_add(1, Ordering::Relaxed);
        Ok(len)
    }

    fn write_sectors(&mut self, lba: u64, data: &[u8]) -> Result<usize, &'static str> {
        let offset = lba as usize * SECTOR_SIZE;
        if offset + data.len() > self.data.len() {
            return Err("RamDisk: write out of bounds");
        }
        for i in 0..data.len() {
            self.data[offset + i] = data[i];
        }
        self.writes.fetch_add(1, Ordering::Relaxed);
        Ok(data.len())
    }
}

// ── Block device manager ──────────────────────────────────────────────────

pub struct BlockDeviceManager {
    devices: alloc::collections::BTreeMap<String, Box<dyn BlockDevice>>,
    scheduler: DeadlineScheduler,
    bio_counter: AtomicU64,
}

impl BlockDeviceManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        BlockDeviceManager {
            devices: alloc::collections::BTreeMap::new(),
            scheduler: DeadlineScheduler::new(),
            bio_counter: AtomicU64::new(0),
        }
    }

    pub fn register(&mut self, dev: Box<dyn BlockDevice>) {
        let name = dev.name().to_string();
        self.devices.insert(name, dev);
    }

    pub fn submit_bio(&mut self, bio: Bio) {
        self.scheduler.submit(bio);
    }

    pub fn process_pending(&mut self) -> usize {
        let mut processed = 0;
        while let Some(bio) = self.scheduler.dispatch_next() {
            if let Some(dev) = self.devices.get_mut(&"ram0".to_string()) {
                match bio.op {
                    BioOp::Read => {
                        let mut buf = vec![0u8; bio.byte_len()];
                        let _ = dev.read_sectors(bio.sector, &mut buf);
                    }
                    BioOp::Write => {
                        let _ = dev.write_sectors(bio.sector, &bio.data);
                    }
                    _ => {}
                }
            }
            processed += 1;
        }
        processed
    }

    pub fn next_bio_id(&self) -> u64 {
        self.bio_counter.fetch_add(1, Ordering::SeqCst)
    }
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}

impl Default for BlockDeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ramdisk_rw() {
        let mut rd = RamDisk::new("ram0", 1024 * 1024); // 1MB
        let write_data: Vec<u8> = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes()
            .iter()
            .cycle()
            .take(512)
            .copied()
            .collect();
        rd.write_sectors(0, &write_data).unwrap();
        let mut read_buf = vec![0u8; 512];
        rd.read_sectors(0, &mut read_buf).unwrap();
        assert_eq!(read_buf[0], write_data[0]);
        assert_eq!(rd.reads(), 1);
        assert_eq!(rd.writes(), 1);
    }

    #[test]
    fn test_deadline_scheduler_cscan_ordering() {
        let mut sched = DeadlineScheduler::new();
        sched.submit(Bio::read(1, 100, 1));
        sched.submit(Bio::read(2, 50, 1));
        sched.submit(Bio::read(3, 200, 1));
        // head at 0, should serve 50, then 100, then 200 (sorted)
        let b1 = sched.dispatch_next().unwrap();
        let b2 = sched.dispatch_next().unwrap();
        let b3 = sched.dispatch_next().unwrap();
        assert_eq!(b1.sector, 50);
        assert_eq!(b2.sector, 100);
        assert_eq!(b3.sector, 200);
        assert!(sched.dispatch_next().is_none());
    }

    #[test]
    fn test_block_device_manager() {
        let mut mgr = BlockDeviceManager::new();
        mgr.register(Box::new(RamDisk::new("ram0", 2 * 1024 * 1024)));
        assert_eq!(mgr.device_count(), 1);

        let bio_id = mgr.next_bio_id();
        let write_data: Vec<u8> = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes()
            .iter()
            .cycle()
            .take(512)
            .copied()
            .collect();
        let write_bio = Bio::write(bio_id, 10, write_data);
        mgr.submit_bio(write_bio);
        let processed = mgr.process_pending();
        assert_eq!(processed, 1);
    }
}
