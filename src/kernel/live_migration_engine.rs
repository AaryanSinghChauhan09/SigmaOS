//! SigmaOS Live Migration Engine
//!
//! Sovereign process/container/VM live migration without downtime.
//!
//! Inspired by:
//! - CRIU (Checkpoint/Restore In Userspace) — Linux process snapshotting
//! - OpenVZ live migration — container checkpointing + network state transfer
//! - QEMU/KVM live migration — page dirty tracking + iterative pre-copy
//! - Kata Containers pause/resume — lightweight VM migration
//! - MicroVM migration in Firecracker (snapshot + restore)
//!
//! Architecture:
//! - `ProcessSnapshot`: complete serialized process state (regs, memory, fds, net)
//! - `MemoryRegion`: virtual memory mapping with dirty page tracking
//! - `MigrationPhase`: iterative pre-copy phases (hot phase → stop-and-copy)
//! - `LiveMigrationEngine`: orchestrates checkpoint, transfer, restore
//!
//! Migration modes:
//! - `PreCopy`: transfer dirty pages while process runs (QEMU-style)
//! - `PostCopy`: start process on destination, page-fault pages on demand
//! - `StopAndCopy`: pause, transfer all, resume (simple, highest downtime)
//! - `Hybrid`: PreCopy until dirty rate drops, then stop-and-copy for final sync

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// ─── Memory Region ────────────────────────────────────────────────────────────

/// A virtual memory region (mapping)
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    /// Virtual address start
    pub vaddr: u64,
    /// Region size in bytes
    pub size: u64,
    /// Protection flags (R/W/X bitmask)
    pub prot: u8,
    /// Backing data (simulated page contents)
    pub data: Vec<u8>,
    /// Dirty page bitmap: bit N = page N is dirty
    pub dirty_bitmap: Vec<u8>,
    /// Page size (default 4096 bytes)
    pub page_size: u32,
}

impl MemoryRegion {
    pub const PROT_READ: u8 = 1;
    pub const PROT_WRITE: u8 = 2;
    pub const PROT_EXEC: u8 = 4;

    /// Create a new zeroed memory region
    pub fn new(vaddr: u64, size: u64, prot: u8) -> Self {
        let page_size = 4096u32;
        let page_count = ((size + page_size as u64 - 1) / page_size as u64) as usize;
        let bitmap_len = (page_count + 7) / 8;
        MemoryRegion {
            vaddr,
            size,
            prot,
            data: vec![0u8; size as usize],
            dirty_bitmap: vec![0u8; bitmap_len],
            page_size,
        }
    }

    /// Mark page N as dirty
    pub fn mark_dirty(&mut self, page_idx: usize) {
        let byte = page_idx / 8;
        let bit = page_idx % 8;
        if byte < self.dirty_bitmap.len() {
            self.dirty_bitmap[byte] |= 1 << bit;
        }
    }

    /// Check if page N is dirty
    pub fn is_dirty(&self, page_idx: usize) -> bool {
        let byte = page_idx / 8;
        let bit = page_idx % 8;
        if byte >= self.dirty_bitmap.len() { return false; }
        (self.dirty_bitmap[byte] >> bit) & 1 != 0
    }

    /// Clear all dirty bits (after pages have been transferred)
    pub fn clear_dirty(&mut self) {
        for b in &mut self.dirty_bitmap { *b = 0; }
    }

    /// Count total dirty pages
    pub fn dirty_page_count(&self) -> usize {
        self.dirty_bitmap.iter().map(|b| b.count_ones() as usize).sum()
    }

    /// Count total pages in this region
    pub fn total_pages(&self) -> usize {
        ((self.size + self.page_size as u64 - 1) / self.page_size as u64) as usize
    }

    /// Dirty rate: fraction of pages that are dirty [0.0, 1.0]
    pub fn dirty_rate(&self) -> f32 {
        let total = self.total_pages();
        if total == 0 { return 0.0; }
        self.dirty_page_count() as f32 / total as f32
    }
}

// ─── CPU Register State ───────────────────────────────────────────────────────

/// Simulated CPU register file for x86_64
#[derive(Debug, Clone, Default)]
pub struct CpuRegisters {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rsp: u64, pub rbp: u64,
    pub r8: u64,  pub r9: u64,  pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64, pub rflags: u64,
    pub cs: u16, pub ss: u16, pub ds: u16, pub es: u16, pub fs: u16, pub gs: u16,
}

// ─── File Descriptor State ────────────────────────────────────────────────────

/// File descriptor state for checkpoint/restore
#[derive(Debug, Clone)]
pub struct FdState {
    pub fd: i32,
    pub fd_type: FdType,
    pub flags: u32,
    pub offset: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FdType {
    RegularFile { path: String },
    Pipe { pipe_id: u32 },
    Socket { domain: u32, sock_type: u32, peer_addr: String },
    Epoll,
    EventFd { count: u64 },
}

// ─── Network Connection State ─────────────────────────────────────────────────

/// TCP connection state for migration
#[derive(Debug, Clone)]
pub struct TcpConnectionState {
    pub src_addr: String,
    pub dst_addr: String,
    pub send_seq: u32,
    pub recv_seq: u32,
    pub state: TcpState,
    pub send_buf: Vec<u8>,
    pub recv_buf: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Established,
    CloseWait,
    TimeWait,
    FinWait1,
}

// ─── Process Snapshot ─────────────────────────────────────────────────────────

/// Complete serialized snapshot of a running process (CRIU-style)
#[derive(Debug, Clone)]
pub struct ProcessSnapshot {
    /// PID of the snapshotted process
    pub pid: u32,
    /// Process name
    pub name: String,
    /// CPU register state at checkpoint
    pub registers: CpuRegisters,
    /// Virtual memory regions
    pub memory_regions: Vec<MemoryRegion>,
    /// Open file descriptors
    pub file_descriptors: Vec<FdState>,
    /// Active TCP connections
    pub tcp_connections: Vec<TcpConnectionState>,
    /// Process environment variables
    pub environ: BTreeMap<String, String>,
    /// Signal mask (blocked signals bitmask)
    pub signal_mask: u64,
    /// Snapshot creation timestamp
    pub snapshot_time_ns: u64,
    /// Total snapshot size in bytes (computed)
    pub snapshot_bytes: u64,
}

impl ProcessSnapshot {
    pub fn new(pid: u32, name: &str) -> Self {
        ProcessSnapshot {
            pid,
            name: String::from(name),
            registers: CpuRegisters::default(),
            memory_regions: Vec::new(),
            file_descriptors: Vec::new(),
            tcp_connections: Vec::new(),
            environ: BTreeMap::new(),
            signal_mask: 0,
            snapshot_time_ns: 0,
            snapshot_bytes: 0,
        }
    }

    /// Add a memory region to the snapshot
    pub fn add_region(&mut self, region: MemoryRegion) {
        self.snapshot_bytes += region.data.len() as u64;
        self.memory_regions.push(region);
    }

    /// Total memory captured in this snapshot
    pub fn total_memory_bytes(&self) -> u64 {
        self.memory_regions.iter().map(|r| r.size).sum()
    }

    /// Total dirty pages across all regions
    pub fn total_dirty_pages(&self) -> usize {
        self.memory_regions.iter().map(|r| r.dirty_page_count()).sum()
    }
}

// ─── Migration Mode ───────────────────────────────────────────────────────────

/// Live migration strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationMode {
    /// Transfer dirty pages iteratively while process runs
    PreCopy,
    /// Start process on dest, page-fault-in pages on demand
    PostCopy,
    /// Pause → transfer all → resume (highest downtime, simplest)
    StopAndCopy,
    /// PreCopy until dirty rate drops below threshold, then stop-and-copy
    Hybrid,
}

/// Current phase in a pre-copy migration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationPhase {
    /// Not started
    Idle,
    /// Initial checkpoint taken
    Checkpointing,
    /// Iteratively transferring dirty pages
    TransferringPages,
    /// Final stop-and-copy phase
    FinalSync,
    /// Restoring on the destination
    Restoring,
    /// Migration complete
    Complete,
    /// Migration failed
    Failed,
}

/// Migration statistics
#[derive(Debug, Clone, Default)]
pub struct MigrationStats {
    pub pages_transferred: u64,
    pub bytes_transferred: u64,
    pub iterations: u32,
    pub downtime_ms: u64,
    pub total_duration_ms: u64,
}

// ─── Live Migration Engine ────────────────────────────────────────────────────

/// SigmaOS Live Migration Engine
///
/// Orchestrates checkpoint, iterative pre-copy, and restoration of processes/VMs.
pub struct LiveMigrationEngine {
    /// Active migration sessions: pid → (snapshot, phase, stats)
    pub sessions: BTreeMap<u32, (ProcessSnapshot, MigrationPhase, MigrationStats)>,
    /// Configuration: dirty rate threshold to trigger final sync (for Hybrid mode)
    pub hybrid_dirty_threshold: f32,
    /// Configuration: max pre-copy iterations before forcing final sync
    pub max_precopy_iterations: u32,
    /// Total completed migrations
    pub completed_migrations: u64,
    /// Total failed migrations
    pub failed_migrations: u64,
}

impl LiveMigrationEngine {
    pub fn new() -> Self {
        LiveMigrationEngine {
            sessions: BTreeMap::new(),
            hybrid_dirty_threshold: 0.05, // 5% dirty rate triggers final sync
            max_precopy_iterations: 10,
            completed_migrations: 0,
            failed_migrations: 0,
        }
    }

    // ── Checkpoint ────────────────────────────────────────────────────────────

    /// Checkpoint a process: takes a full snapshot of its state.
    ///
    /// Returns the snapshot for inspection or transfer.
    pub fn checkpoint(&mut self, snapshot: ProcessSnapshot) -> u32 {
        let pid = snapshot.pid;
        let stats = MigrationStats::default();
        self.sessions.insert(pid, (snapshot, MigrationPhase::Checkpointing, stats));
        pid
    }

    // ── Pre-Copy Iteration ────────────────────────────────────────────────────

    /// Perform one pre-copy iteration: collect dirty pages and "transfer" them.
    ///
    /// In a real system this would DMA-copy to the destination over RDMA/TCP.
    /// Returns (pages_transferred, dirty_rate) after clearing dirty bits.
    pub fn precopy_iterate(&mut self, pid: u32) -> Option<(usize, f32)> {
        let session = self.sessions.get_mut(&pid)?;
        let (snapshot, phase, stats) = session;

        *phase = MigrationPhase::TransferringPages;
        stats.iterations += 1;

        let mut total_dirty = 0usize;
        let mut total_pages = 0usize;

        for region in &mut snapshot.memory_regions {
            let dirty = region.dirty_page_count();
            total_dirty += dirty;
            total_pages += region.total_pages();
            // Simulate transfer: dirty bytes = dirty_pages * page_size
            stats.pages_transferred += dirty as u64;
            stats.bytes_transferred += (dirty * region.page_size as usize) as u64;
            region.clear_dirty();
        }

        let dirty_rate = if total_pages == 0 { 0.0 }
            else { total_dirty as f32 / total_pages as f32 };

        Some((total_dirty, dirty_rate))
    }

    // ── Final Sync ────────────────────────────────────────────────────────────

    /// Perform the final stop-and-copy sync.
    ///
    /// Process is paused, remaining dirty pages transferred, then control
    /// is handed to the destination.
    pub fn final_sync(&mut self, pid: u32, pause_ms: u64) -> bool {
        let session = match self.sessions.get_mut(&pid) {
            Some(s) => s,
            None => return false,
        };
        let (_, phase, stats) = session;
        *phase = MigrationPhase::FinalSync;
        stats.downtime_ms += pause_ms;
        true
    }

    // ── Restore ───────────────────────────────────────────────────────────────

    /// Restore a snapshot on the destination.
    ///
    /// Returns Ok(new_pid) on success, or Err(reason) on failure.
    pub fn restore(&mut self, pid: u32) -> Result<u32, String> {
        let session = self.sessions.get_mut(&pid)
            .ok_or_else(|| format!("No migration session for pid={}", pid))?;
        let (snapshot, phase, stats) = session;

        *phase = MigrationPhase::Restoring;

        // Validate snapshot integrity
        if snapshot.memory_regions.is_empty() {
            *phase = MigrationPhase::Failed;
            self.failed_migrations += 1;
            return Err(format!("Snapshot for pid={} has no memory regions", pid));
        }

        // Simulate restore: process gets a new PID on the destination
        let new_pid = snapshot.pid + 100_000; // offset for destination
        *phase = MigrationPhase::Complete;
        stats.total_duration_ms = stats.downtime_ms + stats.iterations as u64 * 50; // simulated

        self.completed_migrations += 1;
        Ok(new_pid)
    }

    // ── Full Migration Simulation ─────────────────────────────────────────────

    /// Execute a full hybrid migration simulation for a snapshot.
    ///
    /// Runs pre-copy iterations until dirty rate drops, then does final sync + restore.
    pub fn migrate_hybrid(&mut self, mut snapshot: ProcessSnapshot) -> Result<u32, String> {
        // Simulate some initial dirty pages
        for region in &mut snapshot.memory_regions {
            for page in 0..region.total_pages().min(32) {
                region.mark_dirty(page);
            }
        }

        let pid = self.checkpoint(snapshot);

        // Pre-copy iterations
        for _iter in 0..self.max_precopy_iterations {
            match self.precopy_iterate(pid) {
                None => return Err(format!("Pre-copy iteration failed for pid={}", pid)),
                Some((_dirty, dirty_rate)) => {
                    // Simulate new writes between iterations
                    if let Some(session) = self.sessions.get_mut(&pid) {
                        for region in &mut session.0.memory_regions {
                            // Each iteration: mark ~2% of pages dirty (simulated writes)
                            let new_dirty = (region.total_pages() as f32 * 0.02) as usize;
                            for page in 0..new_dirty {
                                region.mark_dirty(page);
                            }
                        }
                    }
                    if dirty_rate <= self.hybrid_dirty_threshold {
                        break;
                    }
                }
            }
        }

        // Final sync (10ms downtime)
        self.final_sync(pid, 10);

        // Restore
        self.restore(pid)
    }

    // ── Status ────────────────────────────────────────────────────────────────

    /// Returns engine status string
    pub fn status(&self) -> String {
        format!(
            "LiveMigration | {} active sessions | {} completed | {} failed | threshold={:.0}%",
            self.sessions.len(),
            self.completed_migrations,
            self.failed_migrations,
            self.hybrid_dirty_threshold * 100.0
        )
    }

    /// Returns stats for a migration session
    pub fn session_stats(&self, pid: u32) -> Option<String> {
        let (_, phase, stats) = self.sessions.get(&pid)?;
        Some(format!(
            "pid={}: phase={:?} iters={} pages={} bytes={} downtime={}ms",
            pid,
            phase,
            stats.iterations,
            stats.pages_transferred,
            stats.bytes_transferred,
            stats.downtime_ms
        ))
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod migration_tests {
    use super::*;

    fn make_snapshot(pid: u32) -> ProcessSnapshot {
        let mut snap = ProcessSnapshot::new(pid, "test-process");
        snap.registers.rip = 0x400000;
        snap.registers.rsp = 0x7FFF0000;

        let mut region1 = MemoryRegion::new(0x400000, 4 * 4096, MemoryRegion::PROT_READ | MemoryRegion::PROT_EXEC);
        region1.mark_dirty(0);
        region1.mark_dirty(1);
        snap.add_region(region1);

        let mut region2 = MemoryRegion::new(0x7FFF0000, 8 * 4096, MemoryRegion::PROT_READ | MemoryRegion::PROT_WRITE);
        for p in 0..4 { region2.mark_dirty(p); }
        snap.add_region(region2);

        snap
    }

    #[test]
    fn test_memory_region_dirty_tracking() {
        let mut region = MemoryRegion::new(0x1000, 8 * 4096, MemoryRegion::PROT_READ | MemoryRegion::PROT_WRITE);
        region.mark_dirty(0);
        region.mark_dirty(3);
        region.mark_dirty(7);
        assert_eq!(region.dirty_page_count(), 3);
        assert!(region.is_dirty(0));
        assert!(region.is_dirty(3));
        assert!(!region.is_dirty(1));
        region.clear_dirty();
        assert_eq!(region.dirty_page_count(), 0);
    }

    #[test]
    fn test_dirty_rate() {
        let mut region = MemoryRegion::new(0x0, 4 * 4096, MemoryRegion::PROT_WRITE);
        region.mark_dirty(0);
        region.mark_dirty(1);
        // 2 of 4 pages dirty = 50%
        assert!((region.dirty_rate() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_checkpoint_creates_session() {
        let mut engine = LiveMigrationEngine::new();
        let snap = make_snapshot(1234);
        let pid = engine.checkpoint(snap);
        assert_eq!(pid, 1234);
        assert!(engine.sessions.contains_key(&1234));
    }

    #[test]
    fn test_precopy_transfers_dirty_pages() {
        let mut engine = LiveMigrationEngine::new();
        let snap = make_snapshot(2000);
        engine.checkpoint(snap);

        let (pages, rate) = engine.precopy_iterate(2000).expect("should succeed");
        assert!(pages > 0, "should transfer some dirty pages");
        assert!(rate >= 0.0 && rate <= 1.0);

        // After iteration, dirty pages should be cleared
        let (snap, _, _) = engine.sessions.get(&2000).unwrap();
        assert_eq!(snap.total_dirty_pages(), 0);
    }

    #[test]
    fn test_restore_success() {
        let mut engine = LiveMigrationEngine::new();
        let snap = make_snapshot(3000);
        engine.checkpoint(snap);
        engine.final_sync(3000, 5);
        let new_pid = engine.restore(3000).expect("restore should succeed");
        assert!(new_pid > 3000);
        assert_eq!(engine.completed_migrations, 1);
    }

    #[test]
    fn test_restore_empty_snapshot_fails() {
        let mut engine = LiveMigrationEngine::new();
        let snap = ProcessSnapshot::new(4000, "empty");
        engine.checkpoint(snap);
        let result = engine.restore(4000);
        assert!(result.is_err());
        assert_eq!(engine.failed_migrations, 1);
    }

    #[test]
    fn test_full_hybrid_migration() {
        let mut engine = LiveMigrationEngine::new();
        let snap = make_snapshot(5000);
        let new_pid = engine.migrate_hybrid(snap).expect("hybrid migration should succeed");
        assert!(new_pid > 5000);
        assert_eq!(engine.completed_migrations, 1);
        assert_eq!(engine.failed_migrations, 0);
    }

    #[test]
    fn test_migration_stats_tracked() {
        let mut engine = LiveMigrationEngine::new();
        let snap = make_snapshot(6000);
        engine.checkpoint(snap);
        engine.precopy_iterate(6000);
        let stats_str = engine.session_stats(6000).unwrap();
        assert!(stats_str.contains("pid=6000"));
        assert!(stats_str.contains("iters=1"));
    }

    #[test]
    fn test_snapshot_total_memory() {
        let snap = make_snapshot(7000);
        // region1: 4*4096 = 16384, region2: 8*4096 = 32768
        assert_eq!(snap.total_memory_bytes(), 49152);
    }
}
