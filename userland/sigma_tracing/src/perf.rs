/// Represents a hardware performance metric, displacing `perf`.
#[derive(Debug, Clone)]
pub enum PerfMetric {
    CpuCycles(u64),
    Instructions(u64),
    CacheMisses(u64),
    BranchMispredictions(u64),
}

/// PerfCounter reads hardware performance counters via MSR/PMU registers.
pub struct PerfCounter {
    counters: Vec<PerfMetric>,
}

impl Default for PerfCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl PerfCounter {
    pub fn new() -> Self {
        Self {
            counters: Vec::new(),
        }
    }

    /// Start counting a specific metric.
    pub fn start(&mut self, metric_type: &str) -> Result<(), String> {
        // Real implementation: write to IA32_PERFEVTSELx MSR
        match metric_type {
            "cycles" => self.counters.push(PerfMetric::CpuCycles(0)),
            "instructions" => self.counters.push(PerfMetric::Instructions(0)),
            "cache-misses" => self.counters.push(PerfMetric::CacheMisses(0)),
            "branch-misses" => self.counters.push(PerfMetric::BranchMispredictions(0)),
            _ => return Err(format!("Unknown metric: {}", metric_type)),
        }
        Ok(())
    }

    /// Read and return all active counters.
    pub fn read_all(&self) -> &[PerfMetric] {
        &self.counters
    }
}
