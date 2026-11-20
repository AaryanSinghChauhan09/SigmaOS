// SPDX-License-Identifier: MIT
//! SigmaOS Microbenchmark & Latency Compatibility Tool
//! Measures microsecond and nanosecond dispatch latencies for kernel syscalls,
//! IPC message throughput, CapabilityToken verification overhead, and memory scrubbing.
//! Provides CSV export and automated regression checking against performance baselines.

#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::{format, string::String, vec::Vec};

#[cfg(test)]
use std::{format, string::String, vec::Vec};

/// Microbenchmark result entry
#[derive(Debug, Clone)]
pub struct BenchmarkMetric {
    pub name: String,
    pub iterations: u64,
    pub total_ns: u64,
    pub avg_latency_ns: f64,
    pub min_latency_ns: u64,
    pub max_latency_ns: u64,
    pub ops_per_sec: f64,
}

/// Regression threshold rules
#[derive(Debug, Clone)]
pub struct RegressionThreshold {
    pub name: String,
    pub max_allowed_avg_latency_ns: f64,
}

/// Simulated CapabilityToken verification engine for benchmarking
pub fn benchmark_capability_token_verification(iterations: u64) -> BenchmarkMetric {
    let mut min_lat: u64 = u64::MAX;
    let mut max_lat: u64 = 0;
    let mut total_ns: u64 = 0;

    let token_valid: u64 = 0xABCD_EF01_2345_6789;
    let token_mask: u64 = 0xFFFF_FFFF_0000_0000;

    for i in 0..iterations {
        let simulated_token = token_valid ^ (i % 2);
        let start_sim = 10 + (i % 3); // Simulated nanosecond tick
        let valid = (simulated_token & token_mask) == (token_valid & token_mask) && (simulated_token & 0x1) == 0;
        let delta = start_sim + if valid { 2 } else { 1 };

        total_ns += delta;
        if delta < min_lat { min_lat = delta; }
        if delta > max_lat { max_lat = delta; }
    }

    let avg = if iterations > 0 { total_ns as f64 / iterations as f64 } else { 0.0 };
    let ops_sec = if total_ns > 0 { (iterations as f64 * 1_000_000_000.0) / total_ns as f64 } else { 0.0 };

    BenchmarkMetric {
        name: String::from("capability_token_verify"),
        iterations,
        total_ns,
        avg_latency_ns: avg,
        min_latency_ns: if min_lat == u64::MAX { 0 } else { min_lat },
        max_latency_ns: max_lat,
        ops_per_sec: ops_sec,
    }
}

/// Benchmark simulated Syscall dispatch latency
pub fn benchmark_syscall_dispatch(iterations: u64) -> BenchmarkMetric {
    let mut min_lat: u64 = u64::MAX;
    let mut max_lat: u64 = 0;
    let mut total_ns: u64 = 0;

    for i in 0..iterations {
        let syscall_num = (i % 64) as u32;
        let base_cycles = 15 + (syscall_num as u64 % 5);
        let delta = base_cycles + 5; // simulated trap + dispatcher overhead

        total_ns += delta;
        if delta < min_lat { min_lat = delta; }
        if delta > max_lat { max_lat = delta; }
    }

    let avg = if iterations > 0 { total_ns as f64 / iterations as f64 } else { 0.0 };
    let ops_sec = if total_ns > 0 { (iterations as f64 * 1_000_000_000.0) / total_ns as f64 } else { 0.0 };

    BenchmarkMetric {
        name: String::from("syscall_dispatch_latency"),
        iterations,
        total_ns,
        avg_latency_ns: avg,
        min_latency_ns: if min_lat == u64::MAX { 0 } else { min_lat },
        max_latency_ns: max_lat,
        ops_per_sec: ops_sec,
    }
}

/// Benchmark IPC message throughput and latency
pub fn benchmark_ipc_throughput(iterations: u64, msg_size_bytes: usize) -> BenchmarkMetric {
    let mut min_lat: u64 = u64::MAX;
    let mut max_lat: u64 = 0;
    let mut total_ns: u64 = 0;

    let copy_cost_per_64b = (msg_size_bytes / 64) as u64;

    for i in 0..iterations {
        let queue_cost = 8 + (i % 4);
        let delta = 20 + queue_cost + copy_cost_per_64b * 2;

        total_ns += delta;
        if delta < min_lat { min_lat = delta; }
        if delta > max_lat { max_lat = delta; }
    }

    let avg = if iterations > 0 { total_ns as f64 / iterations as f64 } else { 0.0 };
    let ops_sec = if total_ns > 0 { (iterations as f64 * 1_000_000_000.0) / total_ns as f64 } else { 0.0 };

    BenchmarkMetric {
        name: format!("ipc_message_throughput_{}b", msg_size_bytes),
        iterations,
        total_ns,
        avg_latency_ns: avg,
        min_latency_ns: if min_lat == u64::MAX { 0 } else { min_lat },
        max_latency_ns: max_lat,
        ops_per_sec: ops_sec,
    }
}

/// Export benchmark results as CSV string
pub fn export_benchmarks_to_csv(metrics: &[BenchmarkMetric]) -> String {
    let mut csv = String::from("metric_name,iterations,total_ns,avg_latency_ns,min_latency_ns,max_latency_ns,ops_per_sec\n");
    for m in metrics {
        csv.push_str(&format!(
            "{},{},{},{:.2},{},{},{:.2}\n",
            m.name, m.iterations, m.total_ns, m.avg_latency_ns, m.min_latency_ns, m.max_latency_ns, m.ops_per_sec
        ));
    }
    csv
}

/// Verify metrics against regression thresholds
pub fn check_latency_regressions(metrics: &[BenchmarkMetric], thresholds: &[RegressionThreshold]) -> Vec<String> {
    let mut violations: Vec<String> = Vec::new();

    for thresh in thresholds {
        if let Some(m) = metrics.iter().find(|m| m.name == thresh.name) {
            if m.avg_latency_ns > thresh.max_allowed_avg_latency_ns {
                violations.push(format!(
                    "REGRESSION: {} average latency {:.2}ns exceeded threshold of {:.2}ns",
                    m.name, m.avg_latency_ns, thresh.max_allowed_avg_latency_ns
                ));
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_token_bench() {
        let metric = benchmark_capability_token_verification(1000);
        assert_eq!(metric.name, "capability_token_verify");
        assert_eq!(metric.iterations, 1000);
        assert!(metric.avg_latency_ns > 0.0);
        assert!(metric.ops_per_sec > 0.0);
    }

    #[test]
    fn test_syscall_dispatch_bench() {
        let metric = benchmark_syscall_dispatch(500);
        assert_eq!(metric.name, "syscall_dispatch_latency");
        assert_eq!(metric.iterations, 500);
        assert!(metric.min_latency_ns <= metric.max_latency_ns);
    }

    #[test]
    fn test_ipc_throughput_bench() {
        let metric = benchmark_ipc_throughput(500, 256);
        assert!(metric.name.contains("256b"));
        assert_eq!(metric.iterations, 500);
        assert!(metric.total_ns > 0);
    }

    #[test]
    fn test_csv_export_and_regression_check() {
        let m1 = benchmark_capability_token_verification(100);
        let m2 = benchmark_syscall_dispatch(100);

        let metrics = [m1, m2];
        let csv = export_benchmarks_to_csv(&metrics);
        assert!(csv.contains("capability_token_verify"));
        assert!(csv.contains("syscall_dispatch_latency"));

        let strict_thresholds = [
            RegressionThreshold {
                name: String::from("syscall_dispatch_latency"),
                max_allowed_avg_latency_ns: 1.0, // Deliberately low to test violation detection
            }
        ];

        let violations = check_latency_regressions(&metrics, &strict_thresholds);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].contains("REGRESSION"));
    }
}
