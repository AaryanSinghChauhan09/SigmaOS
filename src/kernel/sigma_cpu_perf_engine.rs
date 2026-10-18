/// SigmaOS CPU Microarchitecture Performance Engine
/// perf_events, PMU counters, Intel LBR/PEBS, AMD IBS, topology.
///
/// Standard library (std, libc) are fully authorized per SigmaOS policy.
use std::collections::HashMap;
use std::fmt;

/// Hardware performance event types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PerfEventType {
    /// Retired CPU instructions
    Instructions,
    /// CPU cycles (including halted)
    Cycles,
    /// Cache references
    CacheReferences,
    /// Cache misses
    CacheMisses,
    /// Branch instructions
    BranchInstructions,
    /// Branch mispredictions
    BranchMisses,
    /// Page faults (software event)
    PageFaults,
    /// Context switches
    ContextSwitches,
    /// CPU migrations
    CpuMigrations,
    /// Custom hardware event (raw PMU selector)
    RawPmu(u64),
}

impl fmt::Display for PerfEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Instructions => write!(f, "instructions"),
            Self::Cycles => write!(f, "cycles"),
            Self::CacheReferences => write!(f, "cache-references"),
            Self::CacheMisses => write!(f, "cache-misses"),
            Self::BranchInstructions => write!(f, "branch-instructions"),
            Self::BranchMisses => write!(f, "branch-misses"),
            Self::PageFaults => write!(f, "page-faults"),
            Self::ContextSwitches => write!(f, "context-switches"),
            Self::CpuMigrations => write!(f, "cpu-migrations"),
            Self::RawPmu(sel) => write!(f, "r{:x}", sel),
        }
    }
}

/// Performance counter sample
#[derive(Debug, Clone)]
pub struct PerfSample {
    pub event: PerfEventType,
    pub value: u64,
    pub cpu: u32,
    pub pid: u32,
    pub timestamp_ns: u64,
}

/// CPU topology entry
#[derive(Debug, Clone)]
pub struct CpuCore {
    pub logical_id: u32,
    pub physical_id: u32,
    pub core_id: u32,
    pub socket_id: u32,
    pub numa_node: u32,
    pub is_smt_sibling: bool,
}

/// Last Branch Record entry (Intel LBR)
#[derive(Debug, Clone)]
pub struct LbrEntry {
    pub from_ip: u64,
    pub to_ip: u64,
    pub mispredicted: bool,
    pub cycles: u32,
}

/// CPU microarchitecture performance manager
pub struct SigmaCpuPerfEngine {
    pub topology: Vec<CpuCore>,
    pub counters: HashMap<PerfEventType, u64>,
    pub lbr_stack: Vec<LbrEntry>,
    pub sampling_freq_hz: u32,
    pub paranoid_level: i32, // /proc/sys/kernel/perf_event_paranoid
    pub samples: Vec<PerfSample>,
}

impl SigmaCpuPerfEngine {
    pub fn new(num_cpus: u32, paranoid_level: i32) -> Self {
        let topology = (0..num_cpus)
            .map(|i| CpuCore {
                logical_id: i,
                physical_id: i / 2,
                core_id: i % (num_cpus / 2).max(1),
                socket_id: 0,
                numa_node: 0,
                is_smt_sibling: i % 2 == 1,
            })
            .collect();

        Self {
            topology,
            counters: HashMap::new(),
            lbr_stack: Vec::new(),
            sampling_freq_hz: 10000,
            paranoid_level,
            samples: Vec::new(),
        }
    }

    /// Increment a hardware performance counter
    pub fn increment_counter(&mut self, event: PerfEventType, delta: u64) {
        *self.counters.entry(event).or_insert(0) += delta;
    }

    /// Read current counter value
    pub fn read_counter(&self, event: &PerfEventType) -> u64 {
        *self.counters.get(event).unwrap_or(&0)
    }

    /// Compute IPC (instructions per cycle)
    pub fn ipc(&self) -> f64 {
        let instr = self.read_counter(&PerfEventType::Instructions);
        let cycles = self.read_counter(&PerfEventType::Cycles);
        if cycles == 0 {
            return 0.0;
        }
        instr as f64 / cycles as f64
    }

    /// Compute branch miss rate (percentage)
    pub fn branch_miss_rate(&self) -> f64 {
        let misses = self.read_counter(&PerfEventType::BranchMisses);
        let branches = self.read_counter(&PerfEventType::BranchInstructions);
        if branches == 0 {
            return 0.0;
        }
        (misses as f64 / branches as f64) * 100.0
    }

    /// Compute cache miss rate (percentage)
    pub fn cache_miss_rate(&self) -> f64 {
        let misses = self.read_counter(&PerfEventType::CacheMisses);
        let refs = self.read_counter(&PerfEventType::CacheReferences);
        if refs == 0 {
            return 0.0;
        }
        (misses as f64 / refs as f64) * 100.0
    }

    /// Push an LBR entry
    pub fn push_lbr(&mut self, entry: LbrEntry) {
        if self.lbr_stack.len() >= 32 {
            // LBR stack depth = 32 (Intel)
            self.lbr_stack.remove(0);
        }
        self.lbr_stack.push(entry);
    }

    /// Add a performance sample
    pub fn add_sample(&mut self, sample: PerfSample) {
        self.samples.push(sample);
    }

    /// SMT core count
    pub fn smt_sibling_count(&self) -> usize {
        self.topology.iter().filter(|c| c.is_smt_sibling).count()
    }

    /// Physical core count
    pub fn physical_core_count(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        for core in &self.topology {
            seen.insert((core.socket_id, core.core_id));
        }
        seen.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_creation() {
        let engine = SigmaCpuPerfEngine::new(8, 2);
        assert_eq!(engine.topology.len(), 8);
        assert_eq!(engine.smt_sibling_count(), 4);
    }

    #[test]
    fn test_counter_increment_and_read() {
        let mut engine = SigmaCpuPerfEngine::new(4, 2);
        engine.increment_counter(PerfEventType::Instructions, 1_000_000);
        engine.increment_counter(PerfEventType::Cycles, 2_000_000);
        assert_eq!(engine.read_counter(&PerfEventType::Instructions), 1_000_000);
    }

    #[test]
    fn test_ipc_calculation() {
        let mut engine = SigmaCpuPerfEngine::new(4, 2);
        engine.increment_counter(PerfEventType::Instructions, 3_000_000);
        engine.increment_counter(PerfEventType::Cycles, 1_000_000);
        let ipc = engine.ipc();
        assert!((ipc - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_branch_miss_rate() {
        let mut engine = SigmaCpuPerfEngine::new(4, 2);
        engine.increment_counter(PerfEventType::BranchInstructions, 100);
        engine.increment_counter(PerfEventType::BranchMisses, 5);
        let rate = engine.branch_miss_rate();
        assert!((rate - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_lbr_stack_bounded() {
        let mut engine = SigmaCpuPerfEngine::new(4, 2);
        for i in 0..40u64 {
            engine.push_lbr(LbrEntry {
                from_ip: i * 16,
                to_ip: i * 16 + 8,
                mispredicted: false,
                cycles: 1,
            });
        }
        assert_eq!(engine.lbr_stack.len(), 32);
    }
}
