//! Kernel Boot and Panic Diagnostics Subsystem for SigmaOS
//!
//! Provides structured boot initialization reporting, panic reporting formats,
//! CPU register/context serialization, and greppable event logging.

use std::vec::Vec;
use std::string::{String, ToString};
use std::collections::BTreeMap;

/// Kernel Subsystem Boot Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubsystemBootStatus {
    Success,
    Warning(String),
    Failed(String),
    Skipped,
}

/// Single Kernel Subsystem Initialization Result
#[derive(Debug, Clone)]
pub struct SubsystemInitResult {
    pub subsystem_name: String,
    pub status: SubsystemBootStatus,
    pub duration_us: u64,
    pub boot_stage: String,
    pub details: BTreeMap<String, String>,
}

/// Structured Kernel Boot Diagnostic Report
#[derive(Debug, Clone)]
pub struct KernelBootDiagnosticReport {
    pub os_version: String,
    pub boot_timestamp_us: u64,
    pub results: Vec<SubsystemInitResult>,
    pub overall_success: bool,
}

impl KernelBootDiagnosticReport {
    pub fn new(os_version: &str) -> Self {
        Self {
            os_version: os_version.to_string(),
            boot_timestamp_us: 1_000_000,
            results: Vec::new(),
            overall_success: true,
        }
    }

    pub fn record_subsystem(
        &mut self,
        subsystem_name: &str,
        status: SubsystemBootStatus,
        duration_us: u64,
        boot_stage: &str,
    ) {
        if matches!(status, SubsystemBootStatus::Failed(_)) {
            self.overall_success = false;
        }
        self.results.push(SubsystemInitResult {
            subsystem_name: subsystem_name.to_string(),
            status,
            duration_us,
            boot_stage: boot_stage.to_string(),
            details: BTreeMap::new(),
        });
    }

    pub fn generate_greppable_log(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("[BOOT_REPORT] os={} success={}\n", self.os_version, self.overall_success));
        for res in &self.results {
            let status_str = match &res.status {
                SubsystemBootStatus::Success => "SUCCESS",
                SubsystemBootStatus::Warning(w) => &format!("WARN({})", w),
                SubsystemBootStatus::Failed(f) => &format!("FAIL({})", f),
                SubsystemBootStatus::Skipped => "SKIPPED",
            };
            out.push_str(&format!(
                "[BOOT_SUB] stage={} name={} status={} duration_us={}\n",
                res.boot_stage, res.subsystem_name, status_str, res.duration_us
            ));
        }
        out
    }
}

/// Structured Kernel Panic Diagnostic Report
#[derive(Debug, Clone)]
pub struct KernelPanicDiagnosticReport {
    pub component: String,
    pub operation: String,
    pub panic_message: String,
    pub fault_address: Option<u64>,
    pub registers: BTreeMap<String, u64>,
    pub stack_trace: Vec<u64>,
    pub timestamp_us: u64,
}

impl KernelPanicDiagnosticReport {
    pub fn new(component: &str, operation: &str, panic_message: &str) -> Self {
        Self {
            component: component.to_string(),
            operation: operation.to_string(),
            panic_message: panic_message.to_string(),
            fault_address: None,
            registers: BTreeMap::new(),
            stack_trace: Vec::new(),
            timestamp_us: 2_000_000,
        }
    }

    pub fn set_register(&mut self, reg_name: &str, val: u64) {
        self.registers.insert(reg_name.to_string(), val);
    }

    pub fn add_stack_frame(&mut self, rip: u64) {
        self.stack_trace.push(rip);
    }

    pub fn generate_stable_report(&self) -> String {
        let mut out = String::new();
        out.push_str("--- SIGMAOS KERNEL PANIC REPORT ---\n");
        out.push_str(&format!("Component: {}\n", self.component));
        out.push_str(&format!("Operation: {}\n", self.operation));
        out.push_str(&format!("Message: {}\n", self.panic_message));
        if let Some(addr) = self.fault_address {
            out.push_str(&format!("Fault Address: {:#018x}\n", addr));
        }
        out.push_str("Registers:\n");
        for (reg, val) in &self.registers {
            out.push_str(&format!("  {}: {:#018x}\n", reg, val));
        }
        out.push_str("Backtrace:\n");
        for (idx, frame) in self.stack_trace.iter().enumerate() {
            out.push_str(&format!("  #{:02}: {:#018x}\n", idx, frame));
        }
        out.push_str("--- END KERNEL PANIC REPORT ---\n");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_diagnostic_report() {
        let mut report = KernelBootDiagnosticReport::new("SigmaOS 1.0");
        report.record_subsystem("Paging", SubsystemBootStatus::Success, 150, "memory_init");
        report.record_subsystem("Scheduler", SubsystemBootStatus::Success, 300, "scheduler_init");

        assert!(report.overall_success);
        let log = report.generate_greppable_log();
        assert!(log.contains("[BOOT_REPORT] os=SigmaOS 1.0 success=true"));
        assert!(log.contains("name=Paging status=SUCCESS"));
    }

    #[test]
    fn test_panic_diagnostic_report() {
        let mut panic_report = KernelPanicDiagnosticReport::new("VFS", "mount_root", "Null vnode pointer");
        panic_report.fault_address = Some(0x0000000000000000);
        panic_report.set_register("RIP", 0xFFFFFFFF80100050);
        panic_report.add_stack_frame(0xFFFFFFFF80100050);

        let formatted = panic_report.generate_stable_report();
        assert!(formatted.contains("Component: VFS"));
        assert!(formatted.contains("Message: Null vnode pointer"));
        assert!(formatted.contains("RIP: 0xffffffff80100050"));
    }
}
