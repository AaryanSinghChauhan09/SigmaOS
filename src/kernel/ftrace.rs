// SPDX-License-Identifier: MIT
// SigmaOS Ftrace (Function Tracer) Subsystem
// Function tracing and instrumentation inspired by Linux ftrace

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Function trace entry
#[derive(Debug, Clone)]
pub struct FtraceEntry {
    pub id: u64,
    pub function_name: String,
    pub entry_time_ns: u64,
    pub exit_time_ns: u64,
    pub duration_ns: u64,
    pub depth: u32,
    pub parent_id: Option<u64>,
}

impl FtraceEntry {
    pub fn new(id: u64, function_name: String, entry_time_ns: u64, depth: u32, parent_id: Option<u64>) -> Self {
        FtraceEntry {
            id,
            function_name,
            entry_time_ns,
            exit_time_ns: 0,
            duration_ns: 0,
            depth,
            parent_id,
        }
    }

    pub fn mark_exit(&mut self, exit_time_ns: u64) {
        self.exit_time_ns = exit_time_ns;
        self.duration_ns = exit_time_ns.saturating_sub(self.entry_time_ns);
    }
}

/// Ftrace function type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtraceFunctionType {
    Kernel,
    Driver,
    Syscall,
    Scheduler,
    Memory,
    Network,
}

/// Ftrace function descriptor
#[derive(Debug)]
pub struct FtraceFunction {
    pub name: String,
    pub function_type: FtraceFunctionType,
    pub enabled: bool,
    pub call_count: AtomicU64,
    pub total_duration_ns: AtomicU64,
}

impl FtraceFunction {
    pub fn new(name: String, function_type: FtraceFunctionType) -> Self {
        FtraceFunction {
            name,
            function_type,
            enabled: true,
            call_count: AtomicU64::new(0),
            total_duration_ns: AtomicU64::new(0),
        }
    }

    pub fn record_call(&self, duration_ns: u64) {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        self.total_duration_ns.fetch_add(duration_ns, Ordering::SeqCst);
    }

    pub fn get_call_count(&self) -> u64 {
        self.call_count.load(Ordering::SeqCst)
    }

    pub fn get_total_duration(&self) -> u64 {
        self.total_duration_ns.load(Ordering::SeqCst)
    }

    pub fn get_average_duration(&self) -> u64 {
        let count = self.get_call_count();
        if count == 0 {
            return 0;
        }
        self.get_total_duration() / count
    }
}

/// Ftrace subsystem
#[derive(Debug)]
pub struct FtraceSubsystem {
    functions: BTreeMap<String, FtraceFunction>,
    entries: Vec<FtraceEntry>,
    next_entry_id: AtomicU64,
    current_depth: AtomicU32,
    max_entries: usize,
    enabled: AtomicU32, // 0 = disabled, 1 = enabled
}

impl FtraceSubsystem {
    pub fn new(max_entries: usize) -> Self {
        FtraceSubsystem {
            functions: BTreeMap::new(),
            entries: Vec::new(),
            next_entry_id: AtomicU64::new(1),
            current_depth: AtomicU32::new(0),
            max_entries,
            enabled: AtomicU32::new(1),
        }
    }

    /// Register a function for tracing
    pub fn register_function(&mut self, name: String, function_type: FtraceFunctionType) {
        let func = FtraceFunction::new(name.clone(), function_type);
        self.functions.insert(name, func);
    }

    /// Enable tracing for a specific function
    pub fn enable_function(&mut self, name: &str) -> Result<(), &'static str> {
        let func = self.functions.get_mut(name).ok_or("Function not found")?;
        func.enabled = true;
        Ok(())
    }

    /// Disable tracing for a specific function
    pub fn disable_function(&mut self, name: &str) -> Result<(), &'static str> {
        let func = self.functions.get_mut(name).ok_or("Function not found")?;
        func.enabled = false;
        Ok(())
    }

    /// Enable all tracing
    pub fn enable_all(&self) {
        self.enabled.store(1, Ordering::SeqCst);
    }

    /// Disable all tracing
    pub fn disable_all(&self) {
        self.enabled.store(0, Ordering::SeqCst);
    }

    /// Check if tracing is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }

    /// Enter a function (trace entry)
    pub fn function_entry(&mut self, name: &str, timestamp_ns: u64) -> Option<u64> {
        if !self.is_enabled() {
            return None;
        }

        let func = self.functions.get(name)?;
        if !func.enabled {
            return None;
        }

        let depth = self.current_depth.load(Ordering::SeqCst);
        let parent_id = self.entries.last().map(|e| e.id);
        let id = self.next_entry_id.fetch_add(1, Ordering::SeqCst);

        let entry = FtraceEntry::new(id, name.to_string(), timestamp_ns, depth, parent_id);
        self.entries.push(entry);

        self.current_depth.fetch_add(1, Ordering::SeqCst);
        Some(id)
    }

    /// Exit a function (trace exit)
    pub fn function_exit(&mut self, id: u64, timestamp_ns: u64) -> Result<(), &'static str> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.mark_exit(timestamp_ns);
            
            if let Some(func) = self.functions.get(&entry.function_name) {
                func.record_call(entry.duration_ns);
            }

            self.current_depth.fetch_sub(1, Ordering::SeqCst);
            Ok(())
        } else {
            Err("Entry not found")
        }
    }

    /// Get function statistics
    pub fn get_function_stats(&self, name: &str) -> Option<(u64, u64, u64)> {
        let func = self.functions.get(name)?;
        Some((
            func.get_call_count(),
            func.get_total_duration(),
            func.get_average_duration(),
        ))
    }

    /// Get all entries
    pub fn get_entries(&self) -> &[FtraceEntry] {
        &self.entries
    }

    /// Clear all entries
    pub fn clear_entries(&mut self) {
        self.entries.clear();
        self.current_depth.store(0, Ordering::SeqCst);
    }

    /// Get function count
    pub fn function_count(&self) -> usize {
        self.functions.len()
    }

    /// Get entry count
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for FtraceSubsystem {
    fn default() -> Self {
        Self::new(10000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftrace_function_registration() {
        let mut ftrace = FtraceSubsystem::new(100);
        
        ftrace.register_function("test_func".to_string(), FtraceFunctionType::Kernel);
        assert_eq!(ftrace.function_count(), 1);
    }

    #[test]
    fn test_ftrace_function_entry_exit() {
        let mut ftrace = FtraceSubsystem::new(100);
        
        ftrace.register_function("test_func".to_string(), FtraceFunctionType::Kernel);
        
        let id = ftrace.function_entry("test_func", 1000).unwrap();
        ftrace.function_exit(id, 2000).unwrap();
        
        assert_eq!(ftrace.entry_count(), 1);
    }

    #[test]
    fn test_ftrace_function_stats() {
        let mut ftrace = FtraceSubsystem::new(100);
        
        ftrace.register_function("test_func".to_string(), FtraceFunctionType::Kernel);
        
        let id = ftrace.function_entry("test_func", 1000).unwrap();
        ftrace.function_exit(id, 2000).unwrap();
        
        let stats = ftrace.get_function_stats("test_func").unwrap();
        assert_eq!(stats.0, 1); // call count
        assert_eq!(stats.1, 1000); // total duration
        assert_eq!(stats.2, 1000); // average duration
    }

    #[test]
    fn test_ftrace_enable_disable() {
        let mut ftrace = FtraceSubsystem::new(100);
        
        ftrace.register_function("test_func".to_string(), FtraceFunctionType::Kernel);
        ftrace.disable_function("test_func").unwrap();
        
        let id = ftrace.function_entry("test_func", 1000);
        assert!(id.is_none());
        
        ftrace.enable_function("test_func").unwrap();
        let id = ftrace.function_entry("test_func", 1000);
        assert!(id.is_some());
    }

    #[test]
    fn test_ftrace_nesting() {
        let mut ftrace = FtraceSubsystem::new(100);
        
        ftrace.register_function("outer".to_string(), FtraceFunctionType::Kernel);
        ftrace.register_function("inner".to_string(), FtraceFunctionType::Kernel);
        
        let outer_id = ftrace.function_entry("outer", 1000).unwrap();
        let inner_id = ftrace.function_entry("inner", 1500).unwrap();
        ftrace.function_exit(inner_id, 2000).unwrap();
        ftrace.function_exit(outer_id, 3000).unwrap();
        
        assert_eq!(ftrace.entry_count(), 2);
    }

    #[test]
    fn test_ftrace_clear_entries() {
        let mut ftrace = FtraceSubsystem::new(100);
        
        ftrace.register_function("test_func".to_string(), FtraceFunctionType::Kernel);
        
        let id = ftrace.function_entry("test_func", 1000).unwrap();
        ftrace.function_exit(id, 2000).unwrap();
        
        ftrace.clear_entries();
        assert_eq!(ftrace.entry_count(), 0);
    }
}
