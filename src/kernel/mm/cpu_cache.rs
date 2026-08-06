// SigmaOS CPU Cache Subsystem Simulator
// High-fidelity modeling of caches inspired by x86, x64, and ARM architectures

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingFunction {
    DirectMapped,
    SetAssociative { ways: usize },
    FullyAssociative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplacementPolicy {
    LRU,
    PLRU, // Pseudo-LRU (MRU-bit tracking as in Intel/AMD and ARM)
    LFU,
    FIFO,
    Random,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WritePolicy {
    WriteBack,
    WriteThrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteAllocation {
    WriteAllocate,
    NoWriteAllocate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheInclusivity {
    Inclusive,
    Exclusive,
    NonInclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MesiState {
    Modified,
    Exclusive,
    Shared,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnoopAction {
    ReadSnoop,
    WriteSnoop,
}

// Dummy display type to prevent unused imports
pub type DrmPlaneType = u32;

#[derive(Debug, Clone)]
pub struct CacheLine {
    pub tag: u64,
    pub valid: bool,
    pub dirty: bool,
    pub state: MesiState,
    pub lru_counter: u64,   // system_time of last access
    pub fifo_counter: u64,  // system_time of insertion
    pub frequency: u64,     // LFU counter
    pub mru_bit: bool,      // Pseudo-LRU MRU tracking bit
    pub data: [u8; 64],     // 64-byte block size
}

impl CacheLine {
    pub fn new() -> Self {
        Self {
            tag: 0,
            valid: false,
            dirty: false,
            state: MesiState::Invalid,
            lru_counter: 0,
            fifo_counter: 0,
            frequency: 0,
            mru_bit: false,
            data: [0u8; 64],
        }
    }
}

impl Default for CacheLine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CacheLevelModel {
    pub level_name: String,
    pub size_bytes: usize,
    pub mapping: MappingFunction,
    pub replacement: ReplacementPolicy,
    pub write_policy: WritePolicy,
    pub write_allocation: WriteAllocation,
    pub lines: Vec<CacheLine>,
    pub num_sets: usize,
    pub ways: usize,

    // Statistics
    pub hits: AtomicUsize,
    pub misses: AtomicUsize,
    pub evictions: AtomicUsize,
    pub writebacks: AtomicUsize,
}

impl CacheLevelModel {
    pub fn new(
        name: &str,
        size_bytes: usize,
        mapping: MappingFunction,
        replacement: ReplacementPolicy,
        write_policy: WritePolicy,
        write_allocation: WriteAllocation,
    ) -> Self {
        let ways = match mapping {
            MappingFunction::DirectMapped => 1,
            MappingFunction::SetAssociative { ways } => ways,
            MappingFunction::FullyAssociative => size_bytes / 64,
        };
        let num_sets = (size_bytes / 64) / ways;
        let total_lines = num_sets * ways;

        let mut lines = Vec::with_capacity(total_lines);
        for _ in 0..total_lines {
            lines.push(CacheLine::new());
        }

        Self {
            level_name: name.to_string(),
            size_bytes,
            mapping,
            replacement,
            write_policy,
            write_allocation,
            lines,
            num_sets,
            ways,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
            evictions: AtomicUsize::new(0),
            writebacks: AtomicUsize::new(0),
        }
    }

    /// Calculate set index and tag for an address
    pub fn decode_address(&self, addr: u64) -> (usize, u64) {
        // Line offset is lower 6 bits (64-byte line size)
        let block_addr = addr >> 6;
        match self.mapping {
            MappingFunction::FullyAssociative => (0, block_addr),
            _ => {
                let set_index = (block_addr as usize) % self.num_sets;
                let num_set_bits = log2_usize(self.num_sets);
                let tag = block_addr >> num_set_bits;
                (set_index, tag)
            }
        }
    }

    /// Perform a cache access operation (read or write)
    /// Returns (is_hit, evicted_address)
    pub fn access(
        &mut self,
        addr: u64,
        write: bool,
        data: Option<&[u8; 64]>,
        system_time: u64,
    ) -> (bool, Option<u64>) {
        let (set_idx, tag) = self.decode_address(addr);
        let start_line = set_idx * self.ways;
        let end_line = start_line + self.ways;

        // 1. Search for Hit
        for i in start_line..end_line {
            let line = &mut self.lines[i];
            if line.valid && line.tag == tag {
                self.hits.fetch_add(1, Ordering::Relaxed);

                // Update replacement metadata
                line.lru_counter = system_time;
                line.frequency += 1;
                line.mru_bit = true;

                // MRU bit maintenance: if all MRU bits in set are true, clear others
                let mut all_mru = true;
                for j in start_line..end_line {
                    if !self.lines[j].mru_bit {
                        all_mru = false;
                        break;
                    }
                }
                if all_mru {
                    for j in start_line..end_line {
                        if j != i {
                            self.lines[j].mru_bit = false;
                        }
                    }
                }

                // Write policy update
                if write {
                    if let Some(buf) = data {
                        line.data.copy_from_slice(buf);
                    }
                    if self.write_policy == WritePolicy::WriteBack {
                        line.dirty = true;
                        line.state = MesiState::Modified;
                    } else {
                        // WriteThrough immediately triggers main memory writeback
                        line.dirty = false;
                        line.state = MesiState::Exclusive;
                        self.writebacks.fetch_add(1, Ordering::Relaxed);
                    }
                } else {
                    // Read hit, keep state or promote
                    if line.state == MesiState::Invalid {
                        line.state = MesiState::Shared;
                    }
                }

                return (true, None);
            }
        }

        // 2. Cache Miss
        self.misses.fetch_add(1, Ordering::Relaxed);

        if write && self.write_allocation == WriteAllocation::NoWriteAllocate {
            // Write directly to memory/lower levels, do not allocate line
            self.writebacks.fetch_add(1, Ordering::Relaxed);
            return (false, None);
        }

        // Find invalid line first to allocate without evicting
        let mut target_idx = None;
        for i in start_line..end_line {
            if !self.lines[i].valid {
                target_idx = Some(i);
                break;
            }
        }

        let mut evicted_addr = None;

        let line_idx = if let Some(idx) = target_idx {
            idx
        } else {
            // All lines valid: must evict one based on ReplacementPolicy
            self.evictions.fetch_add(1, Ordering::Relaxed);
            let mut victim_idx = start_line;

            match self.replacement {
                ReplacementPolicy::LRU => {
                    let mut min_lru = self.lines[start_line].lru_counter;
                    for i in start_line + 1..end_line {
                        if self.lines[i].lru_counter < min_lru {
                            min_lru = self.lines[i].lru_counter;
                            victim_idx = i;
                        }
                    }
                }
                ReplacementPolicy::LFU => {
                    let mut min_freq = self.lines[start_line].frequency;
                    for i in start_line + 1..end_line {
                        if self.lines[i].frequency < min_freq {
                            min_freq = self.lines[i].frequency;
                            victim_idx = i;
                        }
                    }
                }
                ReplacementPolicy::FIFO => {
                    let mut min_fifo = self.lines[start_line].fifo_counter;
                    for i in start_line + 1..end_line {
                        if self.lines[i].fifo_counter < min_fifo {
                            min_fifo = self.lines[i].fifo_counter;
                            victim_idx = i;
                        }
                    }
                }
                ReplacementPolicy::PLRU => {
                    // Evict first block with MRU bit == false
                    let mut found = false;
                    for i in start_line..end_line {
                        if !self.lines[i].mru_bit {
                            victim_idx = i;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        // fallback to first line in set if all MRU
                        victim_idx = start_line;
                    }
                }
                ReplacementPolicy::Random => {
                    let rand = (addr.wrapping_add(system_time) % self.ways as u64) as usize;
                    victim_idx = start_line + rand;
                }
            }

            let victim = &mut self.lines[victim_idx];
            // If victim is dirty, write back to lower level
            if victim.dirty && victim.state == MesiState::Modified {
                self.writebacks.fetch_add(1, Ordering::Relaxed);
                // Reconstruct victim absolute block address
                let num_set_bits = log2_usize(self.num_sets);
                evicted_addr = Some((victim.tag << (6 + num_set_bits)) | ((set_idx as u64) << 6));
            }

            victim_idx
        };

        // Populate new allocated line
        let line = &mut self.lines[line_idx];
        line.valid = true;
        line.dirty = write && self.write_policy == WritePolicy::WriteBack;
        line.tag = tag;
        line.state = if write {
            if self.write_policy == WritePolicy::WriteBack {
                MesiState::Modified
            } else {
                MesiState::Exclusive
            }
        } else {
            MesiState::Exclusive
        };
        line.lru_counter = system_time;
        line.fifo_counter = system_time;
        line.frequency = 1;
        line.mru_bit = true;
        if let Some(buf) = data {
            line.data.copy_from_slice(buf);
        } else {
            line.data = [0u8; 64];
        }

        (false, evicted_addr)
    }

    /// Simulate Coherence snooping from other processors (MESI state transition)
    pub fn snoop(&mut self, addr: u64, snoop_action: SnoopAction) -> Option<[u8; 64]> {
        let (set_idx, tag) = self.decode_address(addr);
        let start_line = set_idx * self.ways;
        let end_line = start_line + self.ways;

        for i in start_line..end_line {
            let line = &mut self.lines[i];
            if line.valid && line.tag == tag {
                match snoop_action {
                    SnoopAction::ReadSnoop => {
                        // Shared read request from other core
                        if line.state == MesiState::Modified {
                            // Writeback dirty block & downgrade to Shared
                            line.dirty = false;
                            line.state = MesiState::Shared;
                            self.writebacks.fetch_add(1, Ordering::Relaxed);
                            return Some(line.data);
                        } else if line.state == MesiState::Exclusive {
                            line.state = MesiState::Shared;
                        }
                    }
                    SnoopAction::WriteSnoop => {
                        // Write broadcast from other core, must invalidate local copy
                        if line.dirty && line.state == MesiState::Modified {
                            self.writebacks.fetch_add(1, Ordering::Relaxed);
                        }
                        line.valid = false;
                        line.dirty = false;
                        line.state = MesiState::Invalid;
                    }
                }
            }
        }
        None
    }
}

/// Orchestrates the full Cache Memory Hierarchy (separate L1I/L1D, unified L2, shared L3)
pub struct CpuCacheHierarchy {
    pub l1_instruction: CacheLevelModel,
    pub l1_data: CacheLevelModel,
    pub l2: CacheLevelModel,
    pub l3: CacheLevelModel,
    pub inclusivity: CacheInclusivity,

    // Latency specifications (simulation clock cycles)
    pub l1_latency: usize,
    pub l2_latency: usize,
    pub l3_latency: usize,
    pub dram_latency: usize,

    // Global Statistics
    pub total_reads: AtomicUsize,
    pub total_writes: AtomicUsize,
    pub dram_traffic_bytes: AtomicUsize,
}

impl CpuCacheHierarchy {
    /// Create standard default x86-64 styled hierarchy
    pub fn new() -> Self {
        Self {
            // L1 Instruction: 32KB, 8-Way Set Associative, Pseudo-LRU
            l1_instruction: CacheLevelModel::new(
                "L1-I",
                32768,
                MappingFunction::SetAssociative { ways: 8 },
                ReplacementPolicy::PLRU,
                WritePolicy::WriteThrough,
                WriteAllocation::NoWriteAllocate,
            ),
            // L1 Data: 32KB, 8-Way Set Associative, PLRU
            l1_data: CacheLevelModel::new(
                "L1-D",
                32768,
                MappingFunction::SetAssociative { ways: 8 },
                ReplacementPolicy::PLRU,
                WritePolicy::WriteBack,
                WriteAllocation::WriteAllocate,
            ),
            // L2 Unified: 512KB, 8-Way, True LRU
            l2: CacheLevelModel::new(
                "L2-U",
                524288,
                MappingFunction::SetAssociative { ways: 8 },
                ReplacementPolicy::LRU,
                WritePolicy::WriteBack,
                WriteAllocation::WriteAllocate,
            ),
            // L3 Unified: 16MB, 16-Way, True LRU, Shared
            l3: CacheLevelModel::new(
                "L3-U",
                16777216,
                MappingFunction::SetAssociative { ways: 16 },
                ReplacementPolicy::LRU,
                WritePolicy::WriteBack,
                WriteAllocation::WriteAllocate,
            ),
            inclusivity: CacheInclusivity::Inclusive,
            l1_latency: 4,
            l2_latency: 12,
            l3_latency: 40,
            dram_latency: 200,
            total_reads: AtomicUsize::new(0),
            total_writes: AtomicUsize::new(0),
            dram_traffic_bytes: AtomicUsize::new(0),
        }
    }

    /// Read data from cache hierarchy
    /// Returns (is_hit, latency_cycles)
    pub fn read(&mut self, addr: u64, is_instruction: bool, system_time: u64) -> (bool, usize) {
        self.total_reads.fetch_add(1, Ordering::Relaxed);

        // 1. Try L1 Cache
        let l1 = if is_instruction {
            &mut self.l1_instruction
        } else {
            &mut self.l1_data
        };

        let (l1_hit, l1_evicted) = l1.access(addr, false, None, system_time);
        if l1_hit {
            return (true, self.l1_latency);
        }

        // L1 eviction tracking (if L1 was inclusive, we snoop/propagate; let's do L2 query)
        if let Some(evicted_addr) = l1_evicted {
            // Writeback L1 evicted dirty block into L2
            self.l2.access(evicted_addr, true, None, system_time);
        }

        // 2. Try L2 Cache
        let (l2_hit, l2_evicted) = self.l2.access(addr, false, None, system_time);
        if l2_hit {
            // Pull into L1
            l1.access(addr, false, None, system_time);
            return (false, self.l1_latency + self.l2_latency);
        }

        if let Some(evicted_addr) = l2_evicted {
            // Writeback L2 evicted dirty block into L3
            self.l3.access(evicted_addr, true, None, system_time);
        }

        // 3. Try L3 Cache
        let (l3_hit, l3_evicted) = self.l3.access(addr, false, None, system_time);
        if l3_hit {
            // Pull into L2 and L1
            self.l2.access(addr, false, None, system_time);
            l1.access(addr, false, None, system_time);
            return (false, self.l1_latency + self.l2_latency + self.l3_latency);
        }

        if let Some(_evicted_addr) = l3_evicted {
            // Evicted dirty block from L3 goes to DRAM
            self.dram_traffic_bytes.fetch_add(64, Ordering::Relaxed);
        }

        // 4. DRAM Memory fetch
        self.dram_traffic_bytes.fetch_add(64, Ordering::Relaxed);

        // Populate throughout the hierarchy: DRAM -> L3 -> L2 -> L1
        self.l3.access(addr, false, None, system_time);
        self.l2.access(addr, false, None, system_time);
        l1.access(addr, false, None, system_time);

        (false, self.l1_latency + self.l2_latency + self.l3_latency + self.dram_latency)
    }

    /// Write data to cache hierarchy
    /// Returns (is_hit, latency_cycles)
    pub fn write(&mut self, addr: u64, data: &[u8; 64], system_time: u64) -> (bool, usize) {
        self.total_writes.fetch_add(1, Ordering::Relaxed);

        let l1 = &mut self.l1_data;

        // 1. Try L1 Data Write
        let (l1_hit, l1_evicted) = l1.access(addr, true, Some(data), system_time);

        if let Some(evicted_addr) = l1_evicted {
            self.l2.access(evicted_addr, true, None, system_time);
        }

        if l1_hit {
            if l1.write_policy == WritePolicy::WriteThrough {
                // Instantly update L2 and L3
                self.l2.access(addr, true, Some(data), system_time);
                self.l3.access(addr, true, Some(data), system_time);
                self.dram_traffic_bytes.fetch_add(64, Ordering::Relaxed);
                return (true, self.l1_latency + self.l2_latency + self.l3_latency + self.dram_latency);
            }
            return (true, self.l1_latency);
        }

        // L1 Miss: Check Write Allocation
        if l1.write_allocation == WriteAllocation::NoWriteAllocate {
            // Bypass L1 allocation, write directly to L2, L3, DRAM
            self.l2.access(addr, true, Some(data), system_time);
            self.l3.access(addr, true, Some(data), system_time);
            self.dram_traffic_bytes.fetch_add(64, Ordering::Relaxed);
            return (false, self.l1_latency + self.l2_latency + self.l3_latency + self.dram_latency);
        }

        // WriteAllocate: fetch block to L1/L2/L3, then apply write
        let (l2_hit, l2_evicted) = self.l2.access(addr, true, Some(data), system_time);
        if let Some(evicted_addr) = l2_evicted {
            self.l3.access(evicted_addr, true, None, system_time);
        }

        if l2_hit {
            l1.access(addr, true, Some(data), system_time);
            return (false, self.l1_latency + self.l2_latency);
        }

        let (l3_hit, l3_evicted) = self.l3.access(addr, true, Some(data), system_time);
        if let Some(evicted_addr) = l3_evicted {
            self.dram_traffic_bytes.fetch_add(64, Ordering::Relaxed);
        }

        if l3_hit {
            self.l2.access(addr, true, Some(data), system_time);
            l1.access(addr, true, Some(data), system_time);
            return (false, self.l1_latency + self.l2_latency + self.l3_latency);
        }

        // DRAM fetch, then allocate and write
        self.dram_traffic_bytes.fetch_add(64, Ordering::Relaxed);
        self.l3.access(addr, true, Some(data), system_time);
        self.l2.access(addr, true, Some(data), system_time);
        l1.access(addr, true, Some(data), system_time);

        (false, self.l1_latency + self.l2_latency + self.l3_latency + self.dram_latency)
    }

    /// Broadcast snoop to support MESI cache coherence protocol (e.g. multi-core invalidation)
    pub fn broadcast_coherence_snoop(&mut self, addr: u64, action: SnoopAction) {
        // Snoop L1 data
        self.l1_data.snoop(addr, action);
        // Snoop L2
        self.l2.snoop(addr, action);
        // Snoop L3
        self.l3.snoop(addr, action);
    }
}

impl Default for CpuCacheHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

fn log2_usize(val: usize) -> u32 {
    if val <= 1 {
        0
    } else {
        usize::BITS - 1 - val.leading_zeros()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_address_decoding() {
        // Direct Mapped cache of 4096 bytes (64 lines of 64 bytes)
        let cache = CacheLevelModel::new(
            "test_direct",
            4096,
            MappingFunction::DirectMapped,
            ReplacementPolicy::LRU,
            WritePolicy::WriteBack,
            WriteAllocation::WriteAllocate,
        );

        // offset bits = 6. num_sets = 64. log2(64) = 6 set index bits.
        // Address 0x000 -> block_addr = 0, set = 0, tag = 0
        let (set1, tag1) = cache.decode_address(0x000);
        assert_eq!(set1, 0);
        assert_eq!(tag1, 0);

        // Address 64 (0x040) -> block_addr = 1, set = 1, tag = 0
        let (set2, tag2) = cache.decode_address(0x040);
        assert_eq!(set2, 1);
        assert_eq!(tag2, 0);

        // Address 4096 (0x1000) -> block_addr = 64, set = 0, tag = 1
        let (set3, tag3) = cache.decode_address(0x1000);
        assert_eq!(set3, 0);
        assert_eq!(tag3, 1);
    }

    #[test]
    fn test_lru_and_plru_replacement() {
        // 8-way Set Associative L1 Data Cache of 512 bytes (1 set of 8 lines)
        let mut cache = CacheLevelModel::new(
            "test_lru",
            512,
            MappingFunction::SetAssociative { ways: 8 },
            ReplacementPolicy::LRU,
            WritePolicy::WriteBack,
            WriteAllocation::WriteAllocate,
        );

        // Fill all 8 ways
        for i in 0..8 {
            let addr = (i * 64) as u64;
            let (hit, evicted) = cache.access(addr, false, None, i as u64 + 1);
            assert!(!hit);
            assert_eq!(evicted, None);
        }

        // Access block 0x00 (line index 0) at time 100, making it MRU (most recently used)
        let (hit, _) = cache.access(0x00, false, None, 100);
        assert!(hit);

        // Access a 9th block, triggering eviction
        // Under LRU, way 1 (address 0x40, last accessed at time 2) should be evicted because it's the oldest
        let (_, evicted) = cache.access(0x200, false, None, 101);
        // Address of way 1 is 0x40 (64)
        assert_eq!(evicted, None); // Not dirty, so no writeback address returned

        // The cache should contain 0x200 and not 0x40
        let (hit_200, _) = cache.access(0x200, false, None, 102);
        assert!(hit_200);
        let (hit_40, _) = cache.access(0x40, false, None, 103);
        assert!(!hit_40); // 0x40 was evicted!
    }

    #[test]
    fn test_write_policy_and_dirty_tagging() {
        // Write-Back Cache
        let mut wb_cache = CacheLevelModel::new(
            "test_wb",
            1024,
            MappingFunction::DirectMapped,
            ReplacementPolicy::LRU,
            WritePolicy::WriteBack,
            WriteAllocation::WriteAllocate,
        );

        wb_cache.access(0x1000, true, None, 1);
        let set_idx = wb_cache.decode_address(0x1000).0;
        assert!(wb_cache.lines[set_idx].dirty);
        assert_eq!(wb_cache.lines[set_idx].state, MesiState::Modified);

        // Write-Through Cache
        let mut wt_cache = CacheLevelModel::new(
            "test_wt",
            1024,
            MappingFunction::DirectMapped,
            ReplacementPolicy::LRU,
            WritePolicy::WriteThrough,
            WriteAllocation::NoWriteAllocate,
        );

        wt_cache.access(0x1000, true, None, 1);
        // Under NoWriteAllocate + WriteThrough, no line is allocated in the cache for write misses
        let set_idx_wt = wt_cache.decode_address(0x1000).0;
        assert!(!wt_cache.lines[set_idx_wt].valid);
    }

    #[test]
    fn test_multilevel_latency_cascade() {
        let mut hierarchy = CpuCacheHierarchy::new();

        // First Access to 0x1000 (DRAM fetch)
        let (hit1, latency1) = hierarchy.read(0x1000, false, 1);
        assert!(!hit1);
        assert_eq!(latency1, 4 + 12 + 40 + 200); // L1 + L2 + L3 + DRAM = 256

        // Second Access (L1 Cache hit)
        let (hit2, latency2) = hierarchy.read(0x1000, false, 2);
        assert!(hit2);
        assert_eq!(latency2, 4); // L1 latency = 4
    }

    #[test]
    fn test_mesi_coherence_snooping() {
        let mut cache = CacheLevelModel::new(
            "test_mesi",
            1024,
            MappingFunction::DirectMapped,
            ReplacementPolicy::LRU,
            WritePolicy::WriteBack,
            WriteAllocation::WriteAllocate,
        );

        // Perform a write -> Line enters Modified state
        cache.access(0x1000, true, None, 1);
        let set_idx = cache.decode_address(0x1000).0;
        assert_eq!(cache.lines[set_idx].state, MesiState::Modified);
        assert!(cache.lines[set_idx].dirty);

        // Read snoop from another core -> downgrade to Shared and writeback
        let data = cache.snoop(0x1000, SnoopAction::ReadSnoop);
        assert!(data.is_some());
        assert_eq!(cache.lines[set_idx].state, MesiState::Shared);
        assert!(!cache.lines[set_idx].dirty); // dirty cleared by writeback

        // Write snoop from another core -> transition to Invalid
        cache.snoop(0x1000, SnoopAction::WriteSnoop);
        assert_eq!(cache.lines[set_idx].state, MesiState::Invalid);
        assert!(!cache.lines[set_idx].valid);
    }
}
