// SigmaOS `sigma-strace` / `truss` Syscall Interception & Signal Inspector CLI
// Implements process attachment, syscall argument formatting, microsecond latency tracking,
// and execution summaries.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct SyscallTraceRecord {
    pub syscall_id: usize,
    pub syscall_name: String,
    pub args_formatted: Vec<String>,
    pub return_value: i64,
    pub latency_us: u64,
}

pub struct SovereignStraceInspectorEngine {
    pub target_pid: usize,
    pub is_attached: bool,
    pub trace_records: Vec<SyscallTraceRecord>,
    pub syscall_latency_totals: BTreeMap<String, (u64, u64)>, // name -> (count, total_us)
}

impl SovereignStraceInspectorEngine {
    pub fn new(pid: usize) -> Self {
        Self {
            target_pid: pid,
            is_attached: false,
            trace_records: Vec::new(),
            syscall_latency_totals: BTreeMap::new(),
        }
    }

    pub fn attach(&mut self) -> Result<String, &'static str> {
        self.is_attached = true;
        Ok(format!("strace: Attached to process PID {}", self.target_pid))
    }

    pub fn record_syscall(
        &mut self,
        syscall_id: usize,
        name: &str,
        args: &[&str],
        ret_val: i64,
        duration_us: u64,
    ) {
        if !self.is_attached {
            return;
        }

        let record = SyscallTraceRecord {
            syscall_id,
            syscall_name: name.to_string(),
            args_formatted: args.iter().map(|s| s.to_string()).collect(),
            return_value: ret_val,
            latency_us: duration_us,
        };

        let entry = self
            .syscall_latency_totals
            .entry(name.to_string())
            .or_insert((0, 0));
        entry.0 += 1;
        entry.1 += duration_us;

        self.trace_records.push(record);
    }

    pub fn generate_summary_report(&self) -> String {
        let mut report = format!("=== sigma-strace Summary for PID {} ===\n", self.target_pid);
        report.push_str("Syscall         Calls   Total(us)\n");
        for (name, (count, total_us)) in &self.syscall_latency_totals {
            report.push_str(&format!("{:<15} {:<7} {}\n", name, count, total_us));
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_strace_inspector() {
        let mut strace = SovereignStraceInspectorEngine::new(4096);
        assert!(strace.attach().is_ok());

        strace.record_syscall(0, "read", &["fd=3", "buf=0x7fff", "count=1024"], 1024, 15);
        strace.record_syscall(1, "write", &["fd=1", "buf=0x7fff", "count=1024"], 1024, 25);
        strace.record_syscall(0, "read", &["fd=3", "buf=0x7fff", "count=512"], 512, 10);

        assert_eq!(strace.trace_records.len(), 3);
        let summary = strace.generate_summary_report();
        assert!(summary.contains("read"));
        assert!(summary.contains("write"));
    }
}
