// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/tools/sigma_bench.rs — sigma-benchmark: Standardised Performance Suite
// Language: Rust (std) — OOP via BenchSuite + Benchmark trait

use std::time::{Duration, Instant};
use std::collections::BTreeMap;

// ── Benchmark Trait (OOP) ─────────────────────────────────────────────────────

pub trait Benchmark: Send {
    fn name(&self)        -> &'static str;
    fn description(&self) -> &'static str;
    fn warmup_iters(&self) -> u32 { 3 }
    fn bench_iters(&self) -> u32  { 100 }
    fn run_once(&mut self) -> u64; // returns a work-unit count or score
    fn unit(&self) -> &'static str { "ops" }
}

// ── Result ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct BenchResult {
    pub name:       String,
    pub iters:      u32,
    pub total_ns:   u64,
    pub min_ns:     u64,
    pub max_ns:     u64,
    pub mean_ns:    u64,
    pub p95_ns:     u64,
    pub p99_ns:     u64,
    pub throughput: f64,  // work units / second
    pub unit:       String,
}

impl BenchResult {
    pub fn summary(&self) -> String {
        format!(
            "{:<30} mean={:>9}ns  p95={:>9}ns  p99={:>9}ns  tput={:.2e} {}/s",
            self.name, self.mean_ns, self.p95_ns, self.p99_ns, self.throughput, self.unit
        )
    }
}

// ── Benchmark Runner ──────────────────────────────────────────────────────────

pub struct BenchSuite {
    results:    BTreeMap<String, BenchResult>,
    verbose:    bool,
}

impl BenchSuite {
    pub fn new() -> Self { Self { results: BTreeMap::new(), verbose: false } }
    pub fn set_verbose(&mut self, v: bool) { self.verbose = v; }

    pub fn run<B: Benchmark>(&mut self, mut bench: B) -> &BenchResult {
        let name  = bench.name().to_owned();
        let unit  = bench.unit().to_owned();
        let iters = bench.bench_iters();

        // Warmup
        for _ in 0..bench.warmup_iters() { bench.run_once(); }

        // Timed runs
        let mut samples = Vec::with_capacity(iters as usize);
        let mut total_work = 0u64;
        for _ in 0..iters {
            let start = Instant::now();
            let work  = bench.run_once();
            let ns    = start.elapsed().as_nanos() as u64;
            samples.push(ns);
            total_work += work;
        }

        // Statistics
        samples.sort_unstable();
        let total_ns: u64 = samples.iter().sum();
        let mean_ns  = total_ns / iters as u64;
        let min_ns   = samples[0];
        let max_ns   = *samples.last().unwrap_or(&0);
        let p95_ns   = samples[(iters as usize * 95 / 100).min(iters as usize - 1)];
        let p99_ns   = samples[(iters as usize * 99 / 100).min(iters as usize - 1)];
        let throughput = if total_ns > 0 {
            total_work as f64 / (total_ns as f64 / 1e9)
        } else { 0.0 };

        let result = BenchResult { name: name.clone(), iters, total_ns, min_ns, max_ns,
                                   mean_ns, p95_ns, p99_ns, throughput, unit };
        if self.verbose { println!("{}", result.summary()); }
        self.results.insert(name.clone(), result);
        self.results.get(&name).unwrap()
    }

    pub fn print_report(&self) {
        println!("\n=== sigma-benchmark results ===");
        for r in self.results.values() { println!("{}", r.summary()); }
    }

    pub fn get(&self, name: &str) -> Option<&BenchResult> { self.results.get(name) }
}

// ── Built-in Benchmarks ───────────────────────────────────────────────────────

pub struct MemBandwidthBench { buf: Vec<u8> }
impl MemBandwidthBench {
    pub fn new(size_mb: usize) -> Self { Self { buf: vec![0u8; size_mb * 1024 * 1024] } }
}
impl Benchmark for MemBandwidthBench {
    fn name(&self)        -> &'static str { "mem-bandwidth-write" }
    fn description(&self) -> &'static str { "Sequential memory write throughput" }
    fn bench_iters(&self) -> u32 { 20 }
    fn unit(&self)        -> &'static str { "bytes" }
    fn run_once(&mut self) -> u64 {
        let n = self.buf.len();
        for i in 0..n { self.buf[i] = (i & 0xFF) as u8; }
        n as u64
    }
}

pub struct IntArithBench;
impl Benchmark for IntArithBench {
    fn name(&self)        -> &'static str { "int-arithmetic" }
    fn description(&self) -> &'static str { "Integer arithmetic throughput" }
    fn bench_iters(&self) -> u32 { 1000 }
    fn unit(&self)        -> &'static str { "mops" }
    fn run_once(&mut self) -> u64 {
        let mut acc = 0u64;
        for i in 0u64..1_000_000 {
            acc = acc.wrapping_mul(6364136223846793005).wrapping_add(i ^ 0xDEAD);
        }
        let _ = acc;
        1_000_000
    }
}

pub struct ContextSwitchBench { count: u32 }
impl ContextSwitchBench { pub fn new() -> Self { Self { count: 0 } } }
impl Benchmark for ContextSwitchBench {
    fn name(&self)        -> &'static str { "context-switch-latency" }
    fn description(&self) -> &'static str { "Thread context switch round-trip" }
    fn bench_iters(&self) -> u32 { 10 }
    fn unit(&self)        -> &'static str { "switches" }
    fn run_once(&mut self) -> u64 {
        // Measure via pipe round-trip (proxy for context switch)
        use std::io::{Read, Write};
        let (mut r, mut w) = os_pipe::pipe().ok()
            .map(|(r,w)| (r,w))
            .unwrap_or_else(|| {
                // Fallback: just yield N times
                for _ in 0..100 { std::thread::yield_now(); }
                return;
            });
        self.count += 1;
        100
    }
}

pub struct HashBench;
impl Benchmark for HashBench {
    fn name(&self)        -> &'static str { "sha256-throughput" }
    fn description(&self) -> &'static str { "SHA-256 hashing throughput" }
    fn bench_iters(&self) -> u32 { 200 }
    fn unit(&self)        -> &'static str { "hashes" }
    fn run_once(&mut self) -> u64 {
        // Use sigma's own SHA-256 (no stdlib crypto)
        let data = [0xABu8; 4096];
        let mut h = crate::kernel::crypto::sigma_sha256::Sha256::new();
        h.update(&data);
        let _ = h.finalize();
        1
    }
}

pub struct DiskSeqBench { path: String, buf: Vec<u8> }
impl DiskSeqBench {
    pub fn new(path: &str, size_kb: usize) -> Self {
        Self { path: path.to_owned(), buf: vec![0xA5u8; size_kb * 1024] }
    }
}
impl Benchmark for DiskSeqBench {
    fn name(&self)        -> &'static str { "disk-seq-write" }
    fn description(&self) -> &'static str { "Sequential disk write throughput" }
    fn bench_iters(&self) -> u32 { 10 }
    fn unit(&self)        -> &'static str { "bytes" }
    fn run_once(&mut self) -> u64 {
        let path = format!("{}/sigma_bench_tmp", self.path);
        let n = self.buf.len();
        let _ = std::fs::write(&path, &self.buf);
        let _ = std::fs::remove_file(&path);
        n as u64
    }
}

pub struct NetworkLatencyBench { addr: String }
impl NetworkLatencyBench { pub fn new(addr: &str) -> Self { Self { addr: addr.to_owned() } } }
impl Benchmark for NetworkLatencyBench {
    fn name(&self)        -> &'static str { "network-ping-latency" }
    fn description(&self) -> &'static str { "UDP echo round-trip latency" }
    fn bench_iters(&self) -> u32 { 50 }
    fn unit(&self)        -> &'static str { "roundtrips" }
    fn run_once(&mut self) -> u64 {
        use std::net::UdpSocket;
        if let Ok(sock) = UdpSocket::bind("0.0.0.0:0") {
            let _ = sock.set_read_timeout(Some(Duration::from_millis(100)));
            let _ = sock.send_to(b"ping", &self.addr);
            let mut buf = [0u8; 8];
            let _ = sock.recv_from(&mut buf);
        }
        1
    }
}
