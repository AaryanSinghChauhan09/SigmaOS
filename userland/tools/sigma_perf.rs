// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/tools/sigma_perf.rs — sigma-perf: Performance Profiler + Flamegraph
// Language: Rust (std) — OOP via Profiler + CallStack

use std::collections::BTreeMap;
use std::time::Instant;
use std::io::Write;

// ── Sample ────────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct Sample {
    pub stack:    Vec<String>,
    pub count:    u64,
    pub cpu_ns:   u64,
}

// ── Symbol ────────────────────────────────────────────────────────────────────
#[derive(Clone, Debug, Default)]
pub struct Symbol {
    pub name:     String,
    pub file:     String,
    pub line:     u32,
    pub addr:     u64,
}

// ── Call Tree Node ────────────────────────────────────────────────────────────
#[derive(Debug, Default)]
pub struct CallNode {
    pub name:     String,
    pub self_ns:  u64,
    pub total_ns: u64,
    pub count:    u64,
    pub children: BTreeMap<String, CallNode>,
}

impl CallNode {
    pub fn new(name: &str) -> Self { Self { name: name.to_owned(), ..Default::default() } }

    pub fn insert(&mut self, stack: &[String], ns: u64) {
        self.total_ns += ns; self.count += 1;
        if let Some((top, rest)) = stack.split_first() {
            let child = self.children.entry(top.clone()).or_insert_with(|| CallNode::new(top));
            child.insert(rest, ns);
        } else {
            self.self_ns += ns;
        }
    }

    /// Render as folded stacks (for FlameGraph tool compatibility)
    pub fn to_folded(&self, prefix: &str, out: &mut Vec<String>) {
        let path = if prefix.is_empty() { self.name.clone() } else { format!("{};{}", prefix, self.name) };
        if self.self_ns > 0 { out.push(format!("{} {}", path, self.self_ns)); }
        for child in self.children.values() { child.to_folded(&path, out); }
    }

    /// ASCII flamegraph (simplified tree)
    pub fn print_tree(&self, depth: usize, out: &mut impl Write) {
        let indent = "  ".repeat(depth);
        let pct = if depth == 0 || self.total_ns == 0 { "".to_owned() }
                  else { format!(" ({:.1}%)", self.self_ns as f64 / self.total_ns as f64 * 100.0) };
        let _ = writeln!(out, "{}{} — {}µs total{}",
            indent, self.name, self.total_ns/1000, pct);
        // Sort by total_ns desc
        let mut sorted: Vec<&CallNode> = self.children.values().collect();
        sorted.sort_by(|a,b| b.total_ns.cmp(&a.total_ns));
        for child in sorted.iter().take(10) { child.print_tree(depth+1, out); }
    }
}

// ── Profiler ──────────────────────────────────────────────────────────────────
pub struct Profiler {
    root:       CallNode,
    samples:    Vec<Sample>,
    active:     Vec<(String, Instant)>, // stack of (func, start_time)
    pub total_ns: u64,
    pub sample_count: u64,
    enabled:    bool,
}

impl Profiler {
    pub fn new() -> Self {
        Self { root: CallNode::new("root"), samples: Vec::new(),
               active: Vec::new(), total_ns: 0, sample_count: 0, enabled: true }
    }

    pub fn enable(&mut self) { self.enabled = true; }
    pub fn disable(&mut self) { self.enabled = false; }

    /// Mark entry into a function
    pub fn enter(&mut self, name: &str) {
        if !self.enabled { return; }
        self.active.push((name.to_owned(), Instant::now()));
    }

    /// Mark exit from a function
    pub fn exit(&mut self) -> u64 {
        if !self.enabled { return 0; }
        if let Some((name, start)) = self.active.pop() {
            let ns = start.elapsed().as_nanos() as u64;
            // Build stack for this exit
            let mut stack: Vec<String> = self.active.iter().map(|(n,_)| n.clone()).collect();
            stack.push(name);
            self.root.insert(&stack, ns);
            self.total_ns += ns;
            self.sample_count += 1;
            return ns;
        }
        0
    }

    /// Add a pre-collected sample (from perf_event / PMU)
    pub fn add_sample(&mut self, stack: Vec<String>, cpu_ns: u64) {
        self.root.insert(&stack, cpu_ns);
        self.total_ns += cpu_ns;
        self.sample_count += 1;
        self.samples.push(Sample { stack, count: 1, cpu_ns });
    }

    /// Generate folded stacks string (compatible with FlameGraph tools)
    pub fn folded_stacks(&self) -> String {
        let mut lines = Vec::new();
        for child in self.root.children.values() { child.to_folded("", &mut lines); }
        lines.join("\n")
    }

    /// Print ASCII call tree to stdout
    pub fn print_report(&self) {
        let mut buf = Vec::new();
        let _ = writeln!(&mut buf, "\n=== sigma-perf report ===");
        let _ = writeln!(&mut buf, "Total time: {:.3}ms  Samples: {}",
                         self.total_ns as f64/1e6, self.sample_count);
        let _ = writeln!(&mut buf, "");
        let mut sorted: Vec<&CallNode> = self.root.children.values().collect();
        sorted.sort_by(|a,b| b.total_ns.cmp(&a.total_ns));
        for child in sorted.iter().take(20) {
            child.print_tree(0, &mut buf);
        }
        println!("{}", String::from_utf8_lossy(&buf));
    }

    /// Top-N functions by self time
    pub fn top_n(&self, n: usize) -> Vec<(&str, u64, u64)> {
        let mut flat: Vec<(&str, u64, u64)> = Vec::new();
        fn collect<'a>(node: &'a CallNode, flat: &mut Vec<(&'a str, u64, u64)>) {
            flat.push((&node.name, node.self_ns, node.total_ns));
            for child in node.children.values() { collect(child, flat); }
        }
        for child in self.root.children.values() { collect(child, &mut flat); }
        flat.sort_by(|a,b| b.1.cmp(&a.1));
        flat.truncate(n);
        flat
    }
}

// ── Scoped Profiler Guard ─────────────────────────────────────────────────────
pub struct ProfGuard<'a> { profiler: &'a mut Profiler }
impl<'a> ProfGuard<'a> {
    pub fn new(profiler: &'a mut Profiler, name: &str) -> Self {
        profiler.enter(name);
        Self { profiler }
    }
}
impl<'a> Drop for ProfGuard<'a> { fn drop(&mut self) { self.profiler.exit(); } }
