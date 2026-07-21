// SigmaOS Memory Leak Detector
// OOP-based memory leak detection with tracking and analysis

use std::collections::HashMap;
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
    allocations: HashMap<usize, AllocationRecord>,
    reference_counts: HashMap<usize, usize>,
}

impl ReferenceCountingDetector {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            reference_counts: HashMap::new(),
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

        let mut leak_locations: HashMap<String, LeakLocation> = HashMap::new();

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
    allocations: HashMap<usize, AllocationRecord>,
    leak_threshold: Duration,
}

impl TimeBasedDetector {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
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

        let mut leak_locations: HashMap<String, LeakLocation> = HashMap::new();

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
        self.current_report
            .map(|r| r.leaked_allocations > 0)
            .unwrap_or(false)
    }

    /// Get total leaked bytes
    pub fn total_leaked_bytes(&self) -> u64 {
        self.current_report
            .map(|r| r.total_leaked_bytes)
            .unwrap_or(0)
    }
}

impl Default for MemoryLeakDetector {
    fn default() -> Self {
        Self::new(Box::new(ReferenceCountingDetector::new()))
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
}
