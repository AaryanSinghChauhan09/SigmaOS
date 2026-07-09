// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Performance benchmark suite for SigmaOS
// Continuous performance monitoring and regression detection

#![no_std]
#![cfg_attr(test, no_main)]

use core::time::Duration;

/// Benchmark result structure
#[derive(Debug, Clone, Copy)]
pub struct BenchmarkResult {
    pub name: &'static str,
    pub iterations: u64,
    pub total_time_ns: u64,
    pub avg_time_ns: u64,
    pub min_time_ns: u64,
    pub max_time_ns: u64,
}

impl BenchmarkResult {
    pub fn ops_per_second(&self) -> f64 {
        if self.total_time_ns == 0 {
            0.0
        } else {
            (self.iterations as f64) / (self.total_time_ns as f64) * 1_000_000_000.0
        }
    }
}

/// Benchmark runner
pub struct BenchmarkRunner {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkRunner {
    pub const fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn run<F>(&mut self, name: &'static str, iterations: u64, benchmark: F)
    where
        F: Fn() -> u64,
    {
        let mut total_time = 0u64;
        let mut min_time = u64::MAX;
        let mut max_time = 0u64;

        for _ in 0..iterations {
            let time = benchmark();
            total_time += time;
            if time < min_time {
                min_time = time;
            }
            if time > max_time {
                max_time = time;
            }
        }

        let result = BenchmarkResult {
            name,
            iterations,
            total_time_ns: total_time,
            avg_time_ns: total_time / iterations,
            min_time_ns: min_time,
            max_time_ns: max_time,
        };

        self.results.push(result);
    }

    pub fn print_results(&self) {
        println!("\n=== Benchmark Results ===");
        for result in &self.results {
            println!(
                "{}: {} iterations, avg: {} ns, min: {} ns, max: {} ns, ops/s: {:.2}",
                result.name,
                result.iterations,
                result.avg_time_ns,
                result.min_time_ns,
                result.max_time_ns,
                result.ops_per_second()
            );
        }
    }

    pub fn get_results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}

/// Collection benchmarks
pub mod collection_benchmarks {
    use super::BenchmarkRunner;
    use crate::collections::{SigmaMap, SigmaVec, SigmaStringBuilder};

    pub fn benchmark_sigma_map(runner: &mut BenchmarkRunner) {
        runner.run("SigmaMap insert", 10000, || {
            let start = rdtsc();
            let mut map: SigmaMap<u32, u32, 64> = SigmaMap::new();
            for i in 0..64 {
                map.insert(i, i * 2);
            }
            rdtsc() - start
        });

        runner.run("SigmaMap lookup", 10000, || {
            let mut map: SigmaMap<u32, u32, 64> = SigmaMap::new();
            for i in 0..64 {
                map.insert(i, i * 2);
            }
            let start = rdtsc();
            for i in 0..64 {
                let _ = map.get(i);
            }
            rdtsc() - start
        });
    }

    pub fn benchmark_sigma_vec(runner: &mut BenchmarkRunner) {
        runner.run("SigmaVec push", 10000, || {
            let start = rdtsc();
            let mut vec: SigmaVec<u32, 64> = SigmaVec::new();
            for i in 0..64 {
                vec.push(i);
            }
            rdtsc() - start
        });

        runner.run("SigmaVec pop", 10000, || {
            let mut vec: SigmaVec<u32, 64> = SigmaVec::new();
            for i in 0..64 {
                vec.push(i);
            }
            let start = rdtsc();
            while !vec.is_empty() {
                vec.pop();
            }
            rdtsc() - start
        });
    }

    pub fn benchmark_string_builder(runner: &mut BenchmarkRunner) {
        runner.run("SigmaStringBuilder push_str", 10000, || {
            let start = rdtsc();
            let mut builder: SigmaStringBuilder<256> = SigmaStringBuilder::new();
            builder.push_str("Hello, SigmaOS! ");
            builder.push_str("This is a performance test.");
            rdtsc() - start
        });
    }
}

/// SIMD string benchmarks
pub mod simd_benchmarks {
    use super::BenchmarkRunner;
    use crate::simd_string::{strcmp, strlen, memcpy, to_lowercase};

    pub fn benchmark_strcmp(runner: &mut BenchmarkRunner) {
        let a = b"Hello, SigmaOS!";
        let b = b"Hello, SigmaOS!";

        runner.run("SIMD strcmp", 100000, || {
            let start = rdtsc();
            let _ = strcmp(a, b);
            rdtsc() - start
        });
    }

    pub fn benchmark_strlen(runner: &mut BenchmarkRunner) {
        let s = b"Hello, SigmaOS! This is a test string for performance benchmarking.";

        runner.run("SIMD strlen", 100000, || {
            let start = rdtsc();
            let _ = strlen(s);
            rdtsc() - start
        });
    }

    pub fn benchmark_memcpy(runner: &mut BenchmarkRunner) {
        let src = b"This is test data for memory copy benchmarking.";
        let mut dst = [0u8; 64];

        runner.run("SIMD memcpy", 100000, || {
            let start = rdtsc();
            unsafe { memcpy(dst.as_mut_ptr(), src.as_ptr(), src.len()) };
            rdtsc() - start
        });
    }

    pub fn benchmark_to_lowercase(runner: &mut BenchmarkRunner) {
        let mut s = *b"HELLO, SIGMAOS! THIS IS A TEST STRING.";

        runner.run("SIMD to_lowercase", 100000, || {
            let start = rdtsc();
            to_lowercase(&mut s);
            rdtsc() - start
        });
    }
}

/// Lock-free structure benchmarks
pub mod lockfree_benchmarks {
    use super::BenchmarkRunner;
    use crate::lockfree::{SpscQueue, LockFreeStack};

    pub fn benchmark_spsc_queue(runner: &mut BenchmarkRunner) {
        runner.run("SPSC Queue push/pop", 10000, || {
            let queue = SpscQueue::<u32, 64>::new();
            let start = rdtsc();
            for i in 0..64 {
                queue.push(i);
            }
            while !queue.is_empty() {
                queue.pop();
            }
            rdtsc() - start
        });
    }

    pub fn benchmark_lockfree_stack(runner: &mut BenchmarkRunner) {
        runner.run("Lock-free Stack push/pop", 10000, || {
            let stack = LockFreeStack::new();
            let start = rdtsc();
            for i in 0..64 {
                stack.push(i);
            }
            while !stack.is_empty() {
                stack.pop();
            }
            rdtsc() - start
        });
    }
}

/// Memory allocator benchmarks
pub mod allocator_benchmarks {
    use super::BenchmarkRunner;
    use crate::custom_allocators::{PoolAllocator, SlabAllocator};

    pub fn benchmark_pool_allocator(runner: &mut BenchmarkRunner) {
        runner.run("PoolAllocator allocate/deallocate", 10000, || {
            let mut pool: PoolAllocator<u32, 64> = PoolAllocator::new();
            let start = rdtsc();
            for i in 0..64 {
                if let Some(slot) = pool.allocate() {
                    pool.deallocate(slot);
                }
            }
            rdtsc() - start
        });
    }

    pub fn benchmark_slab_allocator(runner: &mut BenchmarkRunner) {
        runner.run("SlabAllocator allocate/deallocate", 10000, || {
            let mut slab: SlabAllocator<64, 128> = SlabAllocator::new();
            let start = rdtsc();
            for i in 0..128 {
                if let Some(block) = slab.allocate() {
                    slab.deallocate(block);
                }
            }
            rdtsc() - start
        });
    }
}

/// Read Time-Stamp Counter for high-precision timing
#[inline(always)]
pub fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let mut hi: u32;
        let mut lo: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nostack, nomem, preserves_flags)
        );
        ((hi as u64) << 32) | (lo as u64)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // Fallback for non-x86 architectures
        0
    }
}

/// Main benchmark runner
pub fn run_all_benchmarks() {
    let mut runner = BenchmarkRunner::new();

    println!("Running SigmaOS Performance Benchmarks...");
    println!("==========================================");

    // Collection benchmarks
    println!("\n--- Collection Benchmarks ---");
    collection_benchmarks::benchmark_sigma_map(&mut runner);
    collection_benchmarks::benchmark_sigma_vec(&mut runner);
    collection_benchmarks::benchmark_string_builder(&mut runner);

    // SIMD benchmarks
    println!("\n--- SIMD Benchmarks ---");
    simd_benchmarks::benchmark_strcmp(&mut runner);
    simd_benchmarks::benchmark_strlen(&mut runner);
    simd_benchmarks::benchmark_memcpy(&mut runner);
    simd_benchmarks::benchmark_to_lowercase(&mut runner);

    // Lock-free benchmarks
    println!("\n--- Lock-free Benchmarks ---");
    lockfree_benchmarks::benchmark_spsc_queue(&mut runner);
    lockfree_benchmarks::benchmark_lockfree_stack(&mut runner);

    // Allocator benchmarks
    println!("\n--- Allocator Benchmarks ---");
    allocator_benchmarks::benchmark_pool_allocator(&mut runner);
    allocator_benchmarks::benchmark_slab_allocator(&mut runner);

    // Print results
    runner.print_results();

    println!("\n==========================================");
    println!("Benchmark suite completed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_runner() {
        let mut runner = BenchmarkRunner::new();
        runner.run("test", 100, || 42);
        assert_eq!(runner.results.len(), 1);
    }

    #[test]
    fn test_rdtsc() {
        let t1 = rdtsc();
        let t2 = rdtsc();
        assert!(t2 >= t1);
    }
}
