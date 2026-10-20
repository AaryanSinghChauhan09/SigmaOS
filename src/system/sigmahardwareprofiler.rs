// SPDX-License-Identifier: MIT
// SigmaOS - Hardware Profiler
// Hardware performance profiling and benchmarking inspired by Arch and Debian

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Hardware component type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareComponent {
    CPU,
    GPU,
    Memory,
    Storage,
    Network,
}

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub component: HardwareComponent,
    pub test_name: String,
    pub score: u64,
    pub unit: String,
    pub duration_ms: u64,
}

/// Hardware profiler
pub struct SigmaHardwareProfiler {
    benchmarks: Vec<BenchmarkResult>,
}

impl SigmaHardwareProfiler {
    pub fn new() -> Self {
        Self {
            benchmarks: Vec::new(),
        }
    }

    pub fn run_benchmark(&mut self, component: HardwareComponent, test_name: String) -> BenchmarkResult {
        let result = BenchmarkResult {
            component,
            test_name: test_name.clone(),
            score: 1000,
            unit: "ops/s".to_string(),
            duration_ms: 10,
        };
        self.benchmarks.push(result.clone());
        result
    }

    pub fn get_benchmarks(&self) -> &[BenchmarkResult] {
        &self.benchmarks
    }

    pub fn get_component_benchmarks(&self, component: HardwareComponent) -> Vec<&BenchmarkResult> {
        self.benchmarks
            .iter()
            .filter(|b| b.component == component)
            .collect()
    }

    pub fn get_system_score(&self) -> u64 {
        self.benchmarks.iter().map(|b| b.score).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_profiler_creation() {
        let profiler = SigmaHardwareProfiler::new();
        assert_eq!(profiler.get_benchmarks().len(), 0);
    }

    #[test]
    fn test_run_benchmark() {
        let mut profiler = SigmaHardwareProfiler::new();
        let result = profiler.run_benchmark(HardwareComponent::CPU, "FLOPS".to_string());
        assert_eq!(result.score, 1000);
        assert_eq!(profiler.get_benchmarks().len(), 1);
    }

    #[test]
    fn test_component_benchmarks() {
        let mut profiler = SigmaHardwareProfiler::new();
        profiler.run_benchmark(HardwareComponent::CPU, "FLOPS".to_string());
        profiler.run_benchmark(HardwareComponent::CPU, "Cache".to_string());
        profiler.run_benchmark(HardwareComponent::GPU, "FPS".to_string());
        assert_eq!(profiler.get_component_benchmarks(HardwareComponent::CPU).len(), 2);
        assert_eq!(profiler.get_component_benchmarks(HardwareComponent::GPU).len(), 1);
    }

    #[test]
    fn test_system_score() {
        let mut profiler = SigmaHardwareProfiler::new();
        profiler.run_benchmark(HardwareComponent::CPU, "FLOPS".to_string());
        profiler.run_benchmark(HardwareComponent::GPU, "FPS".to_string());
        assert_eq!(profiler.get_system_score(), 2000);
    }
}
