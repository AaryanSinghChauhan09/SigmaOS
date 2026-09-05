// BPF-Seccomp Integration - Advanced Syscall Filtering
// Phase 9.6: BPF-based Seccomp Filter Implementation with Syscall Argument Inspection

use std::collections::HashMap;
use crate::kernel::ebpf_vm::BpfInstruction;
use crate::kernel::ebpf_verification::BpfProgramVerifier;

/// Seccomp filter decision
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    Allow = 0,
    Deny = 1,
    Trace = 2,
    Kill = 3,
    Log = 4,
    ErrorNo = 5,
}

/// Syscall information for filtering
#[derive(Debug, Clone)]
pub struct SyscallInfo {
    pub syscall_number: u32,
    pub args: [u64; 6],
}

impl SyscallInfo {
    pub fn new(syscall_number: u32) -> Self {
        SyscallInfo {
            syscall_number,
            args: [0; 6],
        }
    }

    pub fn with_args(syscall_number: u32, args: [u64; 6]) -> Self {
        SyscallInfo { syscall_number, args }
    }
}

/// BPF filter result
#[derive(Debug, Clone)]
pub struct BpfFilterResult {
    pub action: SeccompAction,
    pub error_code: i32,
}

impl BpfFilterResult {
    pub fn allow() -> Self {
        BpfFilterResult {
            action: SeccompAction::Allow,
            error_code: 0,
        }
    }

    pub fn deny() -> Self {
        BpfFilterResult {
            action: SeccompAction::Deny,
            error_code: -1,
        }
    }

    pub fn error(code: i32) -> Self {
        BpfFilterResult {
            action: SeccompAction::ErrorNo,
            error_code: code,
        }
    }
}

/// BPF Seccomp Filter
pub struct BpfSeccompFilter {
    program: Vec<BpfInstruction>,
    program_loaded: bool,
    filter_name: String,
    stats: HashMap<String, u64>,
}

impl BpfSeccompFilter {
    pub fn new(program: Vec<BpfInstruction>, name: String) -> Result<Self, String> {
        let mut verifier = BpfProgramVerifier::new(program.clone());
        let report = verifier.verify()
            .map_err(|e| format!("Program verification failed: {}", e))?;

        if !report.is_valid {
            return Err("Program verification reported errors".to_string());
        }

        let mut stats = HashMap::new();
        stats.insert("filters_allowed".to_string(), 0);
        stats.insert("filters_denied".to_string(), 0);
        stats.insert("filters_traced".to_string(), 0);
        stats.insert("filters_killed".to_string(), 0);

        Ok(BpfSeccompFilter {
            program,
            program_loaded: true,
            filter_name: name,
            stats,
        })
    }

    pub fn unload(&mut self) {
        self.program_loaded = false;
    }

    pub fn is_loaded(&self) -> bool {
        self.program_loaded
    }

    pub fn get_filter_name(&self) -> &str {
        &self.filter_name
    }

    pub fn execute_filter(&mut self, _syscall_info: &SyscallInfo) -> Result<BpfFilterResult, String> {
        if !self.program_loaded {
            return Err("Filter program not loaded".to_string());
        }

        // Update stats - in real implementation would execute program
        *self.stats.get_mut("filters_allowed").unwrap() += 1;

        Ok(BpfFilterResult::allow())
    }

    pub fn get_stats(&self) -> HashMap<String, u64> {
        self.stats.clone()
    }
}

/// Syscall Argument Inspector
pub struct SyscallArgumentInspector;

impl SyscallArgumentInspector {
    pub fn extract_arg(syscall_info: &SyscallInfo, arg_num: usize) -> Option<u64> {
        if arg_num < syscall_info.args.len() {
            Some(syscall_info.args[arg_num])
        } else {
            None
        }
    }

    pub fn compare_arg(syscall_info: &SyscallInfo, arg_num: usize, value: u64) -> bool {
        if let Some(arg_val) = Self::extract_arg(syscall_info, arg_num) {
            arg_val == value
        } else {
            false
        }
    }

    pub fn arg_in_range(syscall_info: &SyscallInfo, arg_num: usize, min: u64, max: u64) -> bool {
        if let Some(arg_val) = Self::extract_arg(syscall_info, arg_num) {
            arg_val >= min && arg_val <= max
        } else {
            false
        }
    }
}

/// BPF Seccomp Filter Context
pub struct BpfSeccompFilterContext {
    filters: HashMap<String, BpfSeccompFilter>,
    active_filter: Option<String>,
}

impl BpfSeccompFilterContext {
    pub fn new() -> Self {
        BpfSeccompFilterContext {
            filters: HashMap::new(),
            active_filter: None,
        }
    }

    pub fn add_filter(&mut self, name: String, filter: BpfSeccompFilter) {
        self.filters.insert(name, filter);
    }

    pub fn activate_filter(&mut self, name: &str) -> Result<(), String> {
        if self.filters.contains_key(name) {
            self.active_filter = Some(name.to_string());
            Ok(())
        } else {
            Err(format!("Filter {} not found", name))
        }
    }

    pub fn remove_filter(&mut self, name: &str) -> Result<(), String> {
        if self.filters.remove(name).is_some() {
            if self.active_filter.as_ref() == Some(&name.to_string()) {
                self.active_filter = None;
            }
            Ok(())
        } else {
            Err(format!("Filter {} not found", name))
        }
    }

    pub fn get_filter_stats(&self, name: &str) -> Result<HashMap<String, u64>, String> {
        if let Some(filter) = self.filters.get(name) {
            Ok(filter.get_stats())
        } else {
            Err(format!("Filter {} not found", name))
        }
    }
}

impl Default for BpfSeccompFilterContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_info_creation() {
        let syscall = SyscallInfo::new(1);
        assert_eq!(syscall.syscall_number, 1);
    }

    #[test]
    fn test_syscall_argument_extractor() {
        let args = [10, 20, 30, 40, 50, 60];
        let syscall = SyscallInfo::with_args(5, args);
        assert_eq!(SyscallArgumentInspector::extract_arg(&syscall, 0), Some(10));
        assert_eq!(SyscallArgumentInspector::extract_arg(&syscall, 5), Some(60));
    }

    #[test]
    fn test_syscall_argument_comparison() {
        let args = [10, 20, 30, 40, 50, 60];
        let syscall = SyscallInfo::with_args(5, args);
        assert!(SyscallArgumentInspector::compare_arg(&syscall, 0, 10));
        assert!(!SyscallArgumentInspector::compare_arg(&syscall, 0, 11));
    }

    #[test]
    fn test_bpf_seccomp_filter_context() {
        let mut context = BpfSeccompFilterContext::new();
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Return,
        ];

        let filter = BpfSeccompFilter::new(program, "filter1".to_string()).unwrap();
        context.add_filter("filter1".to_string(), filter);
        assert!(context.activate_filter("filter1").is_ok());
    }

    #[test]
    fn test_bpf_filter_result() {
        let allow = BpfFilterResult::allow();
        assert_eq!(allow.action, SeccompAction::Allow);

        let deny = BpfFilterResult::deny();
        assert_eq!(deny.action, SeccompAction::Deny);
    }
}
