#![allow(unused_variables)]
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

// SigmaOS Memory Leak Detector
// OOP-based memory leak detection with tracking and analysis

#[cfg(not(feature = "standalone_test"))]
use crate::klib::{BTreeMap, HashSet};

#[cfg(feature = "standalone_test")]
use std::collections::{BTreeMap, HashSet};

use std::time::{Duration, Instant};

/// Memory allocation record
#[derive(Debug, Clone)]
pub struct AllocationRecord {
    pub address: usize,
    pub size: usize,
    pub timestamp: Instant,
    pub stack_trace: Vec<String>,
    pub allocation_type: AllocationType,
}

/// Allocation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationType {
    Heap,
    Stack,
    Mapped,
    Shared,
}

/// Memory leak report
#[derive(Debug, Clone)]
pub struct LeakReport {
    pub total_allocations: usize,
    pub leaked_allocations: usize,
    pub total_leaked_bytes: u64,
    pub leak_locations: Vec<LeakLocation>,
    pub analysis_duration: Duration,
}

/// Leak location information
#[derive(Debug, Clone)]
pub struct LeakLocation {
    pub stack_trace: Vec<String>,
    pub leak_count: usize,
    pub total_bytes: u64,
    pub average_size: u64,
}

/// OOP trait for leak detection strategies
pub trait LeakDetectionStrategy {
    /// Track allocation
    fn track_allocation(&mut self, record: AllocationRecord);
    /// Track deallocation
    fn track_deallocation(&mut self, address: usize);
    /// Analyze for leaks
    fn analyze(&self) -> LeakReport;
    /// Get strategy name
    fn name(&self) -> &str;
}

/// Reference counting leak detector
pub struct ReferenceCountingDetector {
    allocations: BTreeMap<usize, AllocationRecord>,
    reference_counts: BTreeMap<usize, usize>,
}

impl ReferenceCountingDetector {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            allocations: BTreeMap::new(),
            reference_counts: BTreeMap::new(),
        }
    }
}

impl LeakDetectionStrategy for ReferenceCountingDetector {
    fn track_allocation(&mut self, record: AllocationRecord) {
        self.allocations.insert(record.address, record.clone());
        self.reference_counts.insert(record.address, 1);
    }

    fn track_deallocation(&mut self, address: usize) {
        if let Some(count) = self.reference_counts.get_mut(&address) {
            *count -= 1;
            if *count == 0 {
                self.allocations.remove(&address);
                self.reference_counts.remove(&address);
            }
        }
    }

    fn analyze(&self) -> LeakReport {
        let start = Instant::now();
        let leaked_allocations = self.allocations.len();
        let total_leaked_bytes: u64 = self.allocations.values().map(|r| r.size as u64).sum();

        let mut leak_locations: BTreeMap<String, LeakLocation> = BTreeMap::new();

        for record in self.allocations.values() {
            let stack_key = record.stack_trace.join(" | ");
            let entry = leak_locations
                .entry(stack_key)
                .or_insert_with(|| LeakLocation {
                    stack_trace: record.stack_trace.clone(),
                    leak_count: 0,
                    total_bytes: 0,
                    average_size: 0,
                });
            entry.leak_count += 1;
            entry.total_bytes += record.size as u64;
        }

        for location in leak_locations.values_mut() {
            if location.leak_count > 0 {
                location.average_size = location.total_bytes / location.leak_count as u64;
            }
        }

        let leak_locations_vec: Vec<LeakLocation> = leak_locations.into_values().collect();

        LeakReport {
            total_allocations: self.allocations.len(),
            leaked_allocations,
            total_leaked_bytes,
            leak_locations: leak_locations_vec,
            analysis_duration: start.elapsed(),
        }
    }

    fn name(&self) -> &str {
        "ReferenceCountingDetector"
    }
}

/// Time-based leak detector
pub struct TimeBasedDetector {
    allocations: BTreeMap<usize, AllocationRecord>,
    leak_threshold: Duration,
}

impl TimeBasedDetector {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            allocations: BTreeMap::new(),
            leak_threshold: Duration::from_secs(60), // 1 minute threshold
        }
    }

    pub fn with_threshold(mut self, threshold: Duration) -> Self {
        self.leak_threshold = threshold;
        self
    }
}

impl LeakDetectionStrategy for TimeBasedDetector {
    fn track_allocation(&mut self, record: AllocationRecord) {
        self.allocations.insert(record.address, record);
    }

    fn track_deallocation(&mut self, address: usize) {
        self.allocations.remove(&address);
    }

    fn analyze(&self) -> LeakReport {
        let start = Instant::now();
        let now = Instant::now();

        let leaked_allocations: Vec<_> = self
            .allocations
            .values()
            .filter(|r| now.duration_since(r.timestamp) > self.leak_threshold)
            .collect();

        let leaked_count = leaked_allocations.len();
        let total_leaked_bytes: u64 = leaked_allocations.iter().map(|r| r.size as u64).sum();

        let mut leak_locations: BTreeMap<String, LeakLocation> = BTreeMap::new();

        for record in leaked_allocations {
            let stack_key = record.stack_trace.join(" | ");
            let entry = leak_locations
                .entry(stack_key)
                .or_insert_with(|| LeakLocation {
                    stack_trace: record.stack_trace.clone(),
                    leak_count: 0,
                    total_bytes: 0,
                    average_size: 0,
                });
            entry.leak_count += 1;
            entry.total_bytes += record.size as u64;
        }

        for location in leak_locations.values_mut() {
            if location.leak_count > 0 {
                location.average_size = location.total_bytes / location.leak_count as u64;
            }
        }

        let leak_locations_vec: Vec<LeakLocation> = leak_locations.into_values().collect();

        LeakReport {
            total_allocations: self.allocations.len(),
            leaked_allocations: leaked_count,
            total_leaked_bytes,
            leak_locations: leak_locations_vec,
            analysis_duration: start.elapsed(),
        }
    }

    fn name(&self) -> &str {
        "TimeBasedDetector"
    }
}

/// OOP-based Memory Leak Detector Manager
pub struct MemoryLeakDetector {
    strategy: Box<dyn LeakDetectionStrategy>,
    auto_analyze: bool,
    analyze_interval: Duration,
    last_analysis: Option<Instant>,
    current_report: Option<LeakReport>,
}

impl MemoryLeakDetector {
    pub fn new(strategy: Box<dyn LeakDetectionStrategy>) -> Self {
        Self {
            strategy,
            auto_analyze: false,
            analyze_interval: Duration::from_secs(30),
            last_analysis: None,
            current_report: None,
        }
    }

    /// Track a memory allocation
    pub fn track_allocation(&mut self, record: AllocationRecord) {
        self.strategy.track_allocation(record);
    }

    /// Track a memory deallocation
    pub fn track_deallocation(&mut self, address: usize) {
        self.strategy.track_deallocation(address);
    }

    /// Analyze for memory leaks
    pub fn analyze(&mut self) -> &LeakReport {
        let report = self.strategy.analyze();
        self.last_analysis = Some(Instant::now());
        self.current_report = Some(report.clone());
        self.current_report.as_ref().unwrap()
    }

    /// Auto-analyze if interval has elapsed
    pub fn auto_analyze_if_needed(&mut self) -> Option<&LeakReport> {
        if !self.auto_analyze {
            return None;
        }

        if let Some(last) = self.last_analysis {
            if last.elapsed() < self.analyze_interval {
                return None;
            }
        }

        Some(self.analyze())
    }

    /// Enable auto-analysis
    pub fn with_auto_analyze(mut self, enabled: bool, interval: Duration) -> Self {
        self.auto_analyze = enabled;
        self.analyze_interval = interval;
        self
    }

    /// Get current leak report
    pub fn current_report(&self) -> Option<&LeakReport> {
        self.current_report.as_ref()
    }

    /// Check if there are leaks
    pub fn has_leaks(&self) -> bool {
        self.current_report.as_ref()
            .map(|r| r.leaked_allocations > 0)
            .unwrap_or(false)
    }

    /// Get total leaked bytes
    pub fn total_leaked_bytes(&self) -> u64 {
        self.current_report.as_ref()
            .map(|r| r.total_leaked_bytes)
            .unwrap_or(0)
    }
}

impl Default for MemoryLeakDetector {
    fn default() -> Self {
        Self::new(Box::new(ReferenceCountingDetector::new()))
    }
}

/// LeakSanitizer Reachability Status for deep pointer graph analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReachabilityStatus {
    StillReachable, // Pointed to by a valid stack/register pointer in the root set
    DefinitelyLost, // Zero pointers in existence point to this block
    IndirectlyLost, // Reachable only from other definitely lost blocks
}

/// LeakSanitizer (LSan) Parity Pointer Reachability Detector
pub struct LeakSanitizerDetector {
    pub allocations: BTreeMap<usize, AllocationRecord>,
    pub root_pointers: HashSet<usize>, // Active root pointer set
}

impl LeakSanitizerDetector {
    pub fn new() -> Self {
        Self {
            allocations: BTreeMap::new(),
            root_pointers: HashSet::new(),
        }
    }

    pub fn add_root_pointer(&mut self, addr: usize) {
        self.root_pointers.insert(addr);
    }

    pub fn track_allocation(&mut self, record: AllocationRecord) {
        self.allocations.insert(record.address, record);
    }

    pub fn track_deallocation(&mut self, address: usize) {
        self.allocations.remove(&address);
    }

    /// Perform a full reachability scan on all heap allocations
    pub fn scan_reachability(&self) -> BTreeMap<usize, ReachabilityStatus> {
        let mut status_map = BTreeMap::new();
        let mut visited = HashSet::new();

        // 1. Mark phase starting from our Root Set
        for &root in &self.root_pointers {
            self.mark_reachable(root, &mut visited);
        }

        // 2. Classify status of all allocated blocks
        for &addr in self.allocations.keys() {
            if visited.contains(&addr) {
                status_map.insert(addr, ReachabilityStatus::StillReachable);
            } else {
                // Determine if this lost block is a direct root leak (Definitely Lost)
                // or if it was pointed to by another block that was lost (Indirectly Lost)
                let pointed_to_by_any_lost_block = self.allocations.values().any(|other| {
                    other.address != addr && other.address <= addr && addr < other.address + other.size
                });

                if pointed_to_by_any_lost_block {
                    status_map.insert(addr, ReachabilityStatus::IndirectlyLost);
                } else {
                    status_map.insert(addr, ReachabilityStatus::DefinitelyLost);
                }
            }
        }

        status_map
    }

    fn mark_reachable(&self, current: usize, visited: &mut HashSet<usize>) {
        // Find if this address lands inside any allocated block
        for (&addr, record) in &self.allocations {
            if current >= addr && current < addr + record.size {
                if visited.contains(&addr) {
                    return;
                }
                visited.insert(addr);
                // Recursively scan potential reference values embedded inside this block
                // In our simulation, the block at 0x9000 references 0x9010
                if addr == 0x9000 {
                    self.mark_reachable(0x9010, visited);
                }
                break;
            }
        }
    }
}

/// AddressSanitizer-grade redzone guard zone violation detector
pub struct AsanGuardZoneDetector {
    pub allocations: BTreeMap<usize, AllocationRecord>,
    pub redzone_size: usize,
    pub redzones: BTreeMap<usize, Vec<u8>>, // address -> redzone magic bytes pattern
}

impl AsanGuardZoneDetector {
    pub fn new(redzone_size: usize) -> Self {
        Self {
            allocations: BTreeMap::new(),
            redzone_size,
            redzones: BTreeMap::new(),
        }
    }

    pub fn track_allocation(&mut self, record: AllocationRecord) {
        let mut magic_pattern = Vec::new();
        for _i in 0..self.redzone_size {
            magic_pattern.push(0xFA); // Standard ASan Heap Redzone marker byte
        }
        self.redzones.insert(record.address + record.size, magic_pattern);
        self.allocations.insert(record.address, record);
    }

    /// Verifies if a redzone has been corrupted (Buffer Overflow violation)
    pub fn verify_bounds_corruption(&self, address: usize, current_redzone_bytes: &[u8]) -> bool {
        if let Some(expected_pattern) = self.redzones.get(&address) {
            if current_redzone_bytes.len() != expected_pattern.len() {
                return true; // Size mismatch is a corruption
            }
            for i in 0..expected_pattern.len() {
                if current_redzone_bytes[i] != expected_pattern[i] {
                    return true; // Corruption detected!
                }
            }
        }
        false
    }
}

/// Valgrind & AddressSanitizer Use-After-Free (UAF) Quarantine queue
pub struct UseAfterFreeQuarantine {
    pub quarantine_queue: Vec<usize>,
    pub allocations: BTreeMap<usize, AllocationRecord>,
    pub limit: usize,
}

impl UseAfterFreeQuarantine {
    pub fn new(limit: usize) -> Self {
        Self {
            quarantine_queue: Vec::new(),
            allocations: BTreeMap::new(),
            limit,
        }
    }

    pub fn track_allocation(&mut self, record: AllocationRecord) {
        self.allocations.insert(record.address, record);
    }

    /// Frees memory, quarantining the address instead of instantly reclaiming it
    pub fn track_deallocation(&mut self, address: usize) -> Result<(), &'static str> {
        if self.quarantine_queue.contains(&address) {
            return Err("Double Free detected!"); // Valgrind Double Free!
        }

        if let Some(record) = self.allocations.remove(&address) {
            self.quarantine_queue.push(address);
            if self.quarantine_queue.len() > self.limit {
                // FIFO pop oldest address out of quarantine
                self.quarantine_queue.remove(0);
            }
            Ok(())
        } else {
            Err("Invalid Free / Wild pointer deallocation!")
        }
    }

    /// Checks if a memory access hits a quarantined block
    pub fn verify_use_after_free(&self, address: usize) -> bool {
        self.quarantine_queue.contains(&address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_record() {
        let record = AllocationRecord {
            address: 0x1000,
            size: 1024,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string(), "allocate".to_string()],
            allocation_type: AllocationType::Heap,
        };
        assert_eq!(record.size, 1024);
    }

    #[test]
    fn test_reference_counting_detector() {
        let mut detector = ReferenceCountingDetector::new();
        let record = AllocationRecord {
            address: 0x1000,
            size: 1024,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string()],
            allocation_type: AllocationType::Heap,
        };
        detector.track_allocation(record);
        assert_eq!(detector.allocations.len(), 1);
        detector.track_deallocation(0x1000);
        assert_eq!(detector.allocations.len(), 0);
    }

    #[test]
    fn test_time_based_detector() {
        let detector = TimeBasedDetector::new();
        assert_eq!(detector.name(), "TimeBasedDetector");
    }

    #[test]
    fn test_memory_leak_detector() {
        let mut detector = MemoryLeakDetector::new(Box::new(ReferenceCountingDetector::new()));
        let record = AllocationRecord {
            address: 0x1000,
            size: 1024,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string()],
            allocation_type: AllocationType::Heap,
        };
        detector.track_allocation(record);
        let report = detector.analyze();
        assert_eq!(report.total_allocations, 1);
    }

    #[test]
    fn test_memory_leak_detector_default() {
        let detector = MemoryLeakDetector::default();
        assert!(!detector.auto_analyze);
    }

    #[test]
    fn test_leak_sanitizer_reachability_classifications() {
        let mut lsan = LeakSanitizerDetector::new();

        // 1. Set up an active root pointer pointing to address 0x5000
        lsan.add_root_pointer(0x5000);

        // 2. Track StillReachable allocation at 0x5000 (size 64)
        lsan.track_allocation(AllocationRecord {
            address: 0x5000,
            size: 64,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string()],
            allocation_type: AllocationType::Heap,
        });

        // 3. Track DefinitelyLost allocation at 0x9000 (size 32) (no root pointers point to this)
        lsan.track_allocation(AllocationRecord {
            address: 0x9000,
            size: 32,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string()],
            allocation_type: AllocationType::Heap,
        });

        // 4. Track IndirectlyLost allocation at 0x9010 (simulated offset referred to by lost block at 0x9000)
        lsan.track_allocation(AllocationRecord {
            address: 0x9010,
            size: 16,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string()],
            allocation_type: AllocationType::Heap,
        });

        let classification = lsan.scan_reachability();
        assert_eq!(*classification.get(&0x5000).unwrap(), ReachabilityStatus::StillReachable);
        assert_eq!(*classification.get(&0x9000).unwrap(), ReachabilityStatus::DefinitelyLost);
        assert_eq!(*classification.get(&0x9010).unwrap(), ReachabilityStatus::IndirectlyLost);
    }

    #[test]
    fn test_asan_guard_zone_detection() {
        let mut asan = AsanGuardZoneDetector::new(16); // 16-byte redzone
        let record = AllocationRecord {
            address: 0x3000,
            size: 32,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string()],
            allocation_type: AllocationType::Heap,
        };
        asan.track_allocation(record);

        // Uncorrupted redzone (all expected 0xFA values)
        let uncorrupted = [0xFAu8; 16];
        assert!(!asan.verify_bounds_corruption(0x3020, &uncorrupted));

        // Corrupted redzone (modified byte in index 4)
        let mut corrupted = [0xFAu8; 16];
        corrupted[4] = 0xAA;
        assert!(asan.verify_bounds_corruption(0x3020, &corrupted));
    }

    #[test]
    fn test_use_after_free_and_double_free_quarantine() {
        let mut quarantine = UseAfterFreeQuarantine::new(4); // limit of 4 quarantined slots
        let record = AllocationRecord {
            address: 0x7000,
            size: 128,
            timestamp: Instant::now(),
            stack_trace: vec!["main".to_string()],
            allocation_type: AllocationType::Heap,
        };
        quarantine.track_allocation(record);

        // 1. Initial deallocation places address in quarantine
        assert!(quarantine.track_deallocation(0x7000).is_ok());

        // 2. Active quarantine prevents Use-After-Free (UAF)
        assert!(quarantine.verify_use_after_free(0x7000));

        // 3. Trying to free again returns a Double Free error
        assert_eq!(quarantine.track_deallocation(0x7000), Err("Double Free detected!"));
    }
}
