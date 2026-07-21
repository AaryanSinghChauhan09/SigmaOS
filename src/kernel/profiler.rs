//! Kernel Profiling Tools
//!
//! This module provides kernel profiling capabilities for performance analysis,
//! including function call tracing, timing statistics, and hot path identification.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Profiling entry for a function
#[derive(Debug, Clone)]
pub struct ProfileEntry {
    pub name: String,
    pub call_count: u64,
    pub total_time_ns: u64,
    pub self_time_ns: u64,
    pub avg_time_ns: u64,
    pub max_time_ns: u64,
    pub min_time_ns: u64,
}

impl ProfileEntry {
    pub fn new(name: String) -> Self {
        Self {
            name,
            call_count: 0,
            total_time_ns: 0,
            self_time_ns: 0,
            avg_time_ns: 0,
            max_time_ns: 0,
            min_time_ns: u64::MAX,
        }
    }

    pub fn record_call(&mut self, duration_ns: u64) {
        self.call_count += 1;
        self.total_time_ns += duration_ns;
        self.avg_time_ns = self.total_time_ns / self.call_count;
        self.max_time_ns = self.max_time_ns.max(duration_ns);
        self.min_time_ns = self.min_time_ns.min(duration_ns);
    }
}

/// Kernel profiler
pub struct KernelProfiler {
    entries: BTreeMap<String, ProfileEntry>,
    enabled: bool,
    stack: Vec<String>,
}

impl KernelProfiler {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
            enabled: false,
            stack: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enter_function(&mut self, name: String) {
        if !self.enabled {
            return;
        }
        self.stack.push(name);
    }

    pub fn exit_function(&mut self, duration_ns: u64) {
        if !self.enabled {
            return;
        }
        if let Some(name) = self.stack.pop() {
            let entry = self
                .entries
                .entry(name.clone())
                .or_insert_with(|| ProfileEntry::new(name));
            entry.record_call(duration_ns);
        }
    }

    pub fn get_entry(&self, name: &str) -> Option<&ProfileEntry> {
        self.entries.get(name)
    }

    pub fn get_all_entries(&self) -> Vec<&ProfileEntry> {
        self.entries.values().collect()
    }

    pub fn get_hot_paths(&self, threshold_ns: u64) -> Vec<&ProfileEntry> {
        self.entries
            .values()
            .filter(|e| e.avg_time_ns > threshold_ns)
            .collect()
    }

    pub fn get_top_functions(&self, count: usize) -> Vec<&ProfileEntry> {
        let mut entries: Vec<_> = self.entries.values().collect();
        entries.sort_by(|a, b| b.total_time_ns.cmp(&a.total_time_ns));
        entries.into_iter().take(count).collect()
    }

    pub fn reset(&mut self) {
        self.entries.clear();
        self.stack.clear();
    }

    pub fn get_statistics(&self) -> ProfilerStatistics {
        let total_calls: u64 = self.entries.values().map(|e| e.call_count).sum();
        let total_time: u64 = self.entries.values().map(|e| e.total_time_ns).sum();
        let avg_call_time = if total_calls > 0 {
            total_time / total_calls
        } else {
            0
        };

        ProfilerStatistics {
            total_functions: self.entries.len(),
            total_calls,
            total_time_ns: total_time,
            avg_call_time_ns: avg_call_time,
        }
    }
}

impl Default for KernelProfiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Profiling statistics summary
#[derive(Debug, Clone, Copy)]
pub struct ProfilerStatistics {
    pub total_functions: usize,
    pub total_calls: u64,
    pub total_time_ns: u64,
    pub avg_call_time_ns: u64,
}

/// Simple timer for measuring execution time
pub struct Timer {
    start_time: u64,
}

impl Timer {
    pub fn start() -> Self {
        Self {
            start_time: Self::get_time_ns(),
        }
    }

    pub fn elapsed_ns(&self) -> u64 {
        Self::get_time_ns() - self.start_time
    }

    pub fn elapsed_us(&self) -> u64 {
        self.elapsed_ns() / 1000
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.elapsed_us() / 1000
    }

    fn get_time_ns() -> u64 {
        // In a real implementation, this would read from a hardware timer
        // For now, return a placeholder
        0
    }
}

/// Scope-based timer for automatic profiling
pub struct ScopeTimer<'a> {
    profiler: &'a mut KernelProfiler,
    function_name: String,
    start_time: u64,
}

impl<'a> ScopeTimer<'a> {
    pub fn new(profiler: &'a mut KernelProfiler, function_name: String) -> Self {
        profiler.enter_function(function_name.clone());
        Self {
            profiler,
            function_name,
            start_time: Timer::get_time_ns(),
        }
    }
}

impl<'a> Drop for ScopeTimer<'a> {
    fn drop(&mut self) {
        let duration = Timer::get_time_ns() - self.start_time;
        self.profiler.exit_function(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_enable_disable() {
        let mut profiler = KernelProfiler::new();
        assert!(!profiler.is_enabled());

        profiler.enable();
        assert!(profiler.is_enabled());

        profiler.disable();
        assert!(!profiler.is_enabled());
    }

    #[test]
    fn test_profiler_entry() {
        let mut entry = ProfileEntry::new("test_function".to_string());

        entry.record_call(100);
        assert_eq!(entry.call_count, 1);
        assert_eq!(entry.total_time_ns, 100);
        assert_eq!(entry.avg_time_ns, 100);

        entry.record_call(200);
        assert_eq!(entry.call_count, 2);
        assert_eq!(entry.total_time_ns, 300);
        assert_eq!(entry.avg_time_ns, 150);
    }

    #[test]
    fn test_profiler_tracking() {
        let mut profiler = KernelProfiler::new();
        profiler.enable();

        profiler.enter_function("func_a".to_string());
        profiler.exit_function(100);

        profiler.enter_function("func_b".to_string());
        profiler.exit_function(200);

        assert_eq!(profiler.get_all_entries().len(), 2);

        let entry_a = profiler.get_entry("func_a").unwrap();
        assert_eq!(entry_a.call_count, 1);
        assert_eq!(entry_a.total_time_ns, 100);
    }

    #[test]
    fn test_profiler_hot_paths() {
        let mut profiler = KernelProfiler::new();
        profiler.enable();

        profiler.enter_function("hot_func".to_string());
        profiler.exit_function(1000);

        profiler.enter_function("cold_func".to_string());
        profiler.exit_function(10);

        let hot_paths = profiler.get_hot_paths(500);
        assert_eq!(hot_paths.len(), 1);
        assert_eq!(hot_paths[0].name, "hot_func");
    }

    #[test]
    fn test_profiler_statistics() {
        let mut profiler = KernelProfiler::new();
        profiler.enable();

        profiler.enter_function("func_a".to_string());
        profiler.exit_function(100);

        profiler.enter_function("func_b".to_string());
        profiler.exit_function(200);

        let stats = profiler.get_statistics();
        assert_eq!(stats.total_functions, 2);
        assert_eq!(stats.total_calls, 2);
        assert_eq!(stats.total_time_ns, 300);
        assert_eq!(stats.avg_call_time_ns, 150);
    }

    #[test]
    fn test_profiler_reset() {
        let mut profiler = KernelProfiler::new();
        profiler.enable();

        profiler.enter_function("func_a".to_string());
        profiler.exit_function(100);

        assert_eq!(profiler.get_all_entries().len(), 1);

        profiler.reset();
        assert_eq!(profiler.get_all_entries().len(), 0);
    }
}
