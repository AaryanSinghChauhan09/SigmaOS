// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Hermes autonomous system debugger
// Principles: FAIL-FAST, DEFENSIVE PROGRAMMING, OBSERVABILITY, INSTRUMENTATION
use alloc::{string::String, vec::Vec};
#[derive(Debug,Clone,PartialEq)] pub enum LogSource { KernelDmesg, Syslog, AppLog { path: String }, CrashDump { path: String }, JournalCtl }
#[derive(Debug,Clone,Copy,PartialEq,PartialOrd)] pub enum LogLevel { Debug=0, Info=1, Notice=2, Warning=3, Error=4, Critical=5 }
impl LogLevel { pub fn as_str(&self) -> &'static str { match self { Self::Debug=>"DEBUG", Self::Info=>"INFO", Self::Notice=>"NOTICE", Self::Warning=>"WARNING", Self::Error=>"ERROR", Self::Critical=>"CRITICAL" } } }
#[derive(Debug,Clone)] pub struct LogEntry { pub timestamp: u64, pub level: LogLevel, pub source: LogSource, pub message: String, pub pid: Option<u32> }
impl LogEntry {
    pub fn new(level: LogLevel, source: LogSource, msg: &str) -> Self { Self { timestamp: 0, level, source, message: msg.into(), pid: Option::None } }
    pub fn is_error(&self) -> bool { self.level >= LogLevel::Error }
}
#[derive(Debug,Clone)] pub struct KernelPanic { pub oops_text: String, pub call_stack: Vec<String>, pub triggered_at: u64, pub cpu_id: u32, pub register_dump: String }
#[derive(Debug,Clone,PartialEq)] pub enum FixRisk { Low, Medium, High, Critical }
#[derive(Debug,Clone)] pub struct Fix { pub description: String, pub patch: Option<String>, pub command: Option<String>, pub risk: FixRisk, pub confidence: u8 }
impl Fix {
    pub fn command_fix(desc: &str, cmd: &str, risk: FixRisk, confidence: u8) -> Self { Self { description: desc.into(), patch: Option::None, command: Some(cmd.into()), risk, confidence } }
    pub fn patch_fix(desc: &str, patch: &str, risk: FixRisk, confidence: u8) -> Self { Self { description: desc.into(), patch: Some(patch.into()), command: Option::None, risk, confidence } }
}
#[derive(Debug,Clone)] pub struct ApprovalGate { pub fix: Fix, pub approved: Option<bool>, pub applied: bool }
impl ApprovalGate {
    pub fn new(fix: Fix) -> Self { Self { fix, approved: Option::None, applied: false } }
    pub fn approve(&mut self) { self.approved = Some(true); }
    pub fn reject(&mut self) { self.approved = Some(false); }
    pub fn is_approved(&self) -> bool { self.approved == Some(true) }
}
pub struct HermesAgent { pub log_buffer: Vec<LogEntry>, pub applied_fixes: Vec<Fix>, pub max_buffer_size: usize }
impl HermesAgent {
    pub fn new() -> Self { Self { log_buffer: Vec::new(), applied_fixes: Vec::new(), max_buffer_size: 10000 } }
    pub fn ingest_log(&mut self, entry: LogEntry) { if self.log_buffer.len() >= self.max_buffer_size { self.log_buffer.remove(0); } self.log_buffer.push(entry); }
    pub fn analyze_crash(&self, panic: &KernelPanic) -> Vec<Fix> {
        let mut fixes = Vec::new();
        if panic.oops_text.contains("NULL pointer") { fixes.push(Fix::patch_fix("Fix null pointer dereference", "--- a/driver.c
+++ b/driver.c
-if (ptr->field)
+if (ptr && ptr->field)", FixRisk::High, 70)); }
        if panic.oops_text.contains("stack overflow") { fixes.push(Fix::command_fix("Increase kernel stack size", "echo 65536 > /proc/sys/kernel/stack_size", FixRisk::Medium, 60)); }
        if panic.oops_text.contains("out of memory") { fixes.push(Fix::command_fix("Drop caches to free memory", "echo 3 > /proc/sys/vm/drop_caches", FixRisk::Low, 85)); }
        fixes.push(Fix::command_fix("Collect full crash info", "journalctl -b -1 --no-pager > /tmp/crash.log", FixRisk::Low, 100));
        fixes
    }
    pub fn propose_fix(&self, entry: &LogEntry) -> Option<Fix> {
        if !entry.is_error() { return Option::None; }
        if entry.message.contains("disk full") || entry.message.contains("No space left") { return Some(Fix::command_fix("Free up disk space", "journalctl --vacuum-size=500M && apt-get clean", FixRisk::Low, 90)); }
        if entry.message.contains("OOM") || entry.message.contains("out of memory") { return Some(Fix::command_fix("Free memory caches", "sync && echo 3 > /proc/sys/vm/drop_caches", FixRisk::Low, 80)); }
        if entry.message.contains("segfault") { return Some(Fix::command_fix("Collect core dump info", "coredumpctl info", FixRisk::Low, 95)); }
        if entry.message.contains("connection refused") { return Some(Fix::command_fix("Check service status", "systemctl status", FixRisk::Low, 85)); }
        Some(Fix::command_fix("Investigate error", "journalctl -p err -n 50", FixRisk::Low, 70))
    }
    pub fn request_approval(&self, fix: Fix) -> ApprovalGate { ApprovalGate::new(fix) }
    pub fn apply_approved(&mut self, gate: &mut ApprovalGate) -> Result<(),&'static str> { if !gate.is_approved() { return Err("fix not approved"); } if gate.fix.risk == FixRisk::Critical { return Err("critical risk fixes require manual application"); } self.applied_fixes.push(gate.fix.clone()); gate.applied = true; Ok(()) }
    pub fn get_recent_errors(&self) -> Vec<&LogEntry> { self.log_buffer.iter().filter(|e| e.is_error()).collect() }
    pub fn get_recent_issues(&self, min_level: LogLevel) -> Vec<&LogEntry> { self.log_buffer.iter().filter(|e| e.level >= min_level).collect() }
    pub fn clear_log_buffer(&mut self) { self.log_buffer.clear(); }
    pub fn error_rate(&self) -> f32 { if self.log_buffer.is_empty() { 0.0 } else { self.log_buffer.iter().filter(|e| e.is_error()).count() as f32 / self.log_buffer.len() as f32 } }
}
#[cfg(test)] mod tests { use super::*;
    #[test] fn test_ingest_and_retrieve() { let mut h = HermesAgent::new(); h.ingest_log(LogEntry::new(LogLevel::Error, LogSource::Syslog, "disk full")); h.ingest_log(LogEntry::new(LogLevel::Info, LogSource::Syslog, "startup complete")); let errors = h.get_recent_errors(); assert_eq!(errors.len(), 1); assert!(errors[0].message.contains("disk full")); }
    #[test] fn test_propose_fix_disk() { let h = HermesAgent::new(); let e = LogEntry::new(LogLevel::Error, LogSource::Syslog, "No space left on device"); let fix = h.propose_fix(&e).unwrap(); assert!(matches!(fix.risk, FixRisk::Low)); assert!(fix.command.is_some()); }
    #[test] fn test_analyze_crash() { let h = HermesAgent::new(); let panic = KernelPanic { oops_text: String::from("NULL pointer dereference in driver"), call_stack: Vec::new(), triggered_at: 0, cpu_id: 0, register_dump: String::new() }; let fixes = h.analyze_crash(&panic); assert!(!fixes.is_empty()); }
    #[test] fn test_approval_gate() { let mut h = HermesAgent::new(); let fix = Fix::command_fix("test fix", "echo ok", FixRisk::Low, 90); let mut gate = h.request_approval(fix); assert!(!gate.is_approved()); gate.reject(); assert!(!gate.is_approved()); gate.approve(); assert!(gate.is_approved()); h.apply_approved(&mut gate).unwrap(); assert!(gate.applied); }
    #[test] fn test_no_fix_for_info() { let h = HermesAgent::new(); let e = LogEntry::new(LogLevel::Info, LogSource::Syslog, "system started"); assert!(h.propose_fix(&e).is_none()); }
}
