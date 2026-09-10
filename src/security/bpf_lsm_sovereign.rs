#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]


#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── LSM Hook Point Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmHookPoint {
    BprmCheckSecurity, // Binary execution authorization
    FileOpen,          // File opening and path access
    SocketConnect,     // Network connection initiation
    TaskKill,          // Signal dispatch between processes
    PtraceAccessCheck, // Debugger attachment verification
}

// ─── BPF-LSM Action ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfLsmDecision {
    Allow,
    Deny(i32), // Returns negative errno e.g. -13 (-EACCES), -1 (-EPERM)
    AuditOnly,
}

// ─── BPF-LSM Program Descriptor ───────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BpfLsmProgram {
    pub id: u32,
    pub name: String,
    pub hook: LsmHookPoint,
    pub target_pattern: String, // Path or name match pattern
    pub action: BpfLsmDecision,
    pub invocation_count: u64,
    pub violations_blocked: u64,
}

impl BpfLsmProgram {
    pub fn new(id: u32, name: &str, hook: LsmHookPoint, pattern: &str, action: BpfLsmDecision) -> Self {
        BpfLsmProgram {
            id,
            name: name.to_string(),
            hook,
            target_pattern: pattern.to_string(),
            action,
            invocation_count: 0,
            violations_blocked: 0,
        }
    }
}

// ─── Sovereign BPF-LSM Engine ─────────────────────────────────────────────────

pub struct SovereignBpfLsmEngine {
    pub programs: Vec<BpfLsmProgram>,
    pub next_program_id: u32,
    pub total_evaluations: u64,
    pub total_denials: u64,
}

impl SovereignBpfLsmEngine {
    pub fn new() -> Self {
        SovereignBpfLsmEngine {
            programs: Vec::new(),
            next_program_id: 1,
            total_evaluations: 0,
            total_denials: 0,
        }
    }

    pub fn attach_program(&mut self, name: &str, hook: LsmHookPoint, pattern: &str, action: BpfLsmDecision) -> u32 {
        let id = self.next_program_id;
        self.next_program_id = self.next_program_id.saturating_add(1);
        self.programs.push(BpfLsmProgram::new(id, name, hook, pattern, action));
        id
    }

    pub fn detach_program(&mut self, id: u32) -> bool {
        if let Some(pos) = self.programs.iter().position(|p| p.id == id) {
            self.programs.remove(pos);
            true
        } else {
            false
        }
    }

    /// Evaluate binary execution against BprmCheckSecurity hook
    pub fn check_bprm(&mut self, binary_path: &str) -> Result<(), i32> {
        self.evaluate_hook(LsmHookPoint::BprmCheckSecurity, binary_path)
    }

    /// Evaluate file access against FileOpen hook
    pub fn check_file_open(&mut self, file_path: &str) -> Result<(), i32> {
        self.evaluate_hook(LsmHookPoint::FileOpen, file_path)
    }

    /// Evaluate network connection against SocketConnect hook
    pub fn check_socket_connect(&mut self, target_addr: &str) -> Result<(), i32> {
        self.evaluate_hook(LsmHookPoint::SocketConnect, target_addr)
    }

    /// Evaluate process debugging against PtraceAccessCheck hook
    pub fn check_ptrace(&mut self, target_pid: u32) -> Result<(), i32> {
        let pid_str = target_pid.to_string();
        self.evaluate_hook(LsmHookPoint::PtraceAccessCheck, &pid_str)
    }

    fn evaluate_hook(&mut self, hook: LsmHookPoint, target: &str) -> Result<(), i32> {
        self.total_evaluations = self.total_evaluations.saturating_add(1);
        for prog in &mut self.programs {
            if prog.hook == hook {
                prog.invocation_count = prog.invocation_count.saturating_add(1);
                let matched = if prog.hook == LsmHookPoint::PtraceAccessCheck {
                    target == prog.target_pattern
                } else {
                    target.contains(&prog.target_pattern)
                };
                if matched {
                    match prog.action {
                        BpfLsmDecision::Deny(errno) => {
                            prog.violations_blocked = prog.violations_blocked.saturating_add(1);
                            self.total_denials = self.total_denials.saturating_add(1);
                            return Err(errno);
                        }
                        BpfLsmDecision::Allow | BpfLsmDecision::AuditOnly => {}
                    }
                }
            }
        }
        Ok(())
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpf_lsm_attach_detach() {
        let mut lsm = SovereignBpfLsmEngine::new();
        let id = lsm.attach_program("block_etc_shadow", LsmHookPoint::FileOpen, "/etc/shadow", BpfLsmDecision::Deny(-13));
        assert_eq!(id, 1);
        assert_eq!(lsm.programs.len(), 1);
        assert!(lsm.detach_program(id));
        assert_eq!(lsm.programs.len(), 0);
    }

    #[test]
    fn test_bpf_lsm_file_open_denial() {
        let mut lsm = SovereignBpfLsmEngine::new();
        lsm.attach_program("protect_secrets", LsmHookPoint::FileOpen, "/secret", BpfLsmDecision::Deny(-13));

        assert!(lsm.check_file_open("/usr/bin/ls").is_ok());
        assert_eq!(lsm.check_file_open("/secret/keys.txt"), Err(-13));
        assert_eq!(lsm.total_denials, 1);
    }

    #[test]
    fn test_bpf_lsm_bprm_exec_protection() {
        let mut lsm = SovereignBpfLsmEngine::new();
        lsm.attach_program("no_suid_bash", LsmHookPoint::BprmCheckSecurity, "/tmp/evil_sh", BpfLsmDecision::Deny(-1));

        assert!(lsm.check_bprm("/bin/sh").is_ok());
        assert_eq!(lsm.check_bprm("/tmp/evil_sh"), Err(-1));
    }

    #[test]
    fn test_bpf_lsm_socket_filter() {
        let mut lsm = SovereignBpfLsmEngine::new();
        lsm.attach_program("block_c2_ip", LsmHookPoint::SocketConnect, "198.51.100.", BpfLsmDecision::Deny(-111)); // -ECONNREFUSED

        assert!(lsm.check_socket_connect("1.1.1.1:53").is_ok());
        assert_eq!(lsm.check_socket_connect("198.51.100.4:4444"), Err(-111));
    }

    #[test]
    fn test_bpf_lsm_ptrace_defense() {
        let mut lsm = SovereignBpfLsmEngine::new();
        // Protect PID 1 (init) from ptrace attachment
        lsm.attach_program("protect_init", LsmHookPoint::PtraceAccessCheck, "1", BpfLsmDecision::Deny(-1));

        assert!(lsm.check_ptrace(100).is_ok());
        assert_eq!(lsm.check_ptrace(1), Err(-1));
    }

    #[test]
    fn test_bpf_lsm_audit_decision() {
        let mut lsm = SovereignBpfLsmEngine::new();
        lsm.attach_program("audit_root", LsmHookPoint::FileOpen, "/root", BpfLsmDecision::AuditOnly);

        assert!(lsm.check_file_open("/root/.bashrc").is_ok());
        assert_eq!(lsm.programs[0].invocation_count, 1);
        assert_eq!(lsm.total_denials, 0);
    }
}
