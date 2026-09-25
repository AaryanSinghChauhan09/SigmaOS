// SPDX-License-Identifier: MIT
// SigmaOS x86_64 User Mode (Ring 3) & Task State Segment (TSS) Protection Engine
// Inspired by Linux (arch/x86/kernel/process_64.c, entry_64.S) and FreeBSD (sys/amd64/amd64/machdep.c)
// Enhanced with additional BSD security features and Linux scheduler integration

use std::vec::Vec;
use std::string::String;
use std::collections::BTreeMap;

// ============================================================================
// 1. 64-bit Task State Segment (TSS) Hardware Layout
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SovereignTaskStateSegment64 {
    pub reserved_0: u32,
    pub privilege_stack_table: [u64; 3], // rsp0, rsp1, rsp2
    pub reserved_1: u64,
    pub interrupt_stack_table: [u64; 7], // ist1 .. ist7
    pub reserved_2: u64,
    pub reserved_3: u16,
    pub iopb_offset: u16, // I/O Permission Bitmap Base Offset
}

impl SovereignTaskStateSegment64 {
    pub fn new() -> Self {
        Self {
            reserved_0: 0,
            privilege_stack_table: [0; 3],
            interrupt_stack_table: [0; 7],
            reserved_1: 0,
            reserved_2: 0,
            reserved_3: 0,
            iopb_offset: core::mem::size_of::<SovereignTaskStateSegment64>() as u16,
        }
    }

    pub fn set_privilege_stack(&mut self, level: usize, stack_top: u64) {
        if level < 3 {
            self.privilege_stack_table[level] = stack_top;
        }
    }

    pub fn set_interrupt_stack(&mut self, ist_index: usize, stack_top: u64) {
        if ist_index > 0 && ist_index <= 7 {
            self.interrupt_stack_table[ist_index - 1] = stack_top;
        }
    }
}

impl Default for SovereignTaskStateSegment64 {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Hardware iretq Ring 3 Transition Stack Frame
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IretqStackFrame {
    pub rip: u64,    // Instruction pointer
    pub cs: u64,     // Code selector (Ring 3: 0x2B with RPL=3)
    pub rflags: u64, // CPU RFLAGS (IF=1 bit 9 set: 0x202)
    pub rsp: u64,    // User Stack pointer
    pub ss: u64,     // Data selector (Ring 3: 0x33 with RPL=3)
}

impl IretqStackFrame {
    pub fn new_user_mode(entry_point: u64, user_rsp: u64) -> Self {
        Self {
            rip: entry_point,
            cs: 0x2B,     // Ring 3 Code Segment Selector (0x28 | 3)
            rflags: 0x202, // Enable Interrupts (IF)
            rsp: user_rsp,
            ss: 0x33,     // Ring 3 Data Segment Selector (0x30 | 3)
        }
    }
}

// ============================================================================
// 3. User Mode Process Context & KPTI Shadow Page Table Engine
// ============================================================================

/// Security sandbox level inspired by BSD pledge/unveil and Linux seccomp
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecuritySandboxLevel {
    None,
    Minimal,
    Restricted,
    Strict,
}

/// Process state inspired by Linux task_struct and FreeBSD proc
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
}

#[derive(Debug, Clone)]
pub struct UserModeProcessContext {
    pub pid: u64,
    pub ppid: u64, // Parent PID for process hierarchy
    pub user_cr3_page_table: u64, // KPTI Shadow Page Table for User Space
    pub kernel_cr3_page_table: u64, // Full Kernel Page Table
    pub kernel_stack_top: u64,
    pub user_stack_top: u64,
    pub entry_point: u64,
    pub is_ring3_active: bool,
    pub allowed_io_ports: Vec<u16>,
    pub security_level: SecuritySandboxLevel,
    pub process_state: ProcessState,
    pub cgroup_id: Option<u64>, // Linux CGroup integration
    pub allowed_syscalls: Vec<u64>, // seccomp-style syscall filtering
    pub signal_mask: u64, // Signal mask inspired by Linux sigprocmask
}

pub struct SovereignRing3UserModeEngine {
    pub tss: SovereignTaskStateSegment64,
    pub active_processes: Vec<UserModeProcessContext>,
    pub current_pid: Option<u64>,
    pub process_hierarchy: BTreeMap<u64, Vec<u64>>, // Parent -> Children mapping
    pub security_contexts: BTreeMap<u64, SecuritySandboxLevel>,
}

impl SovereignRing3UserModeEngine {
    pub fn new() -> Self {
        Self {
            tss: SovereignTaskStateSegment64::new(),
            active_processes: Vec::new(),
            current_pid: None,
            process_hierarchy: BTreeMap::new(),
            security_contexts: BTreeMap::new(),
        }
    }

    pub fn register_user_process(
        &mut self,
        pid: u64,
        ppid: u64,
        user_cr3: u64,
        kernel_cr3: u64,
        kernel_stack: u64,
        user_stack: u64,
        entry: u64,
        security_level: SecuritySandboxLevel,
    ) {
        let ctx = UserModeProcessContext {
            pid,
            ppid,
            user_cr3_page_table: user_cr3,
            kernel_cr3_page_table: kernel_cr3,
            kernel_stack_top: kernel_stack,
            user_stack_top: user_stack,
            entry_point: entry,
            is_ring3_active: false,
            allowed_io_ports: Vec::new(),
            security_level,
            process_state: ProcessState::Running,
            cgroup_id: None,
            allowed_syscalls: Vec::new(),
            signal_mask: 0,
        };

        // Update process hierarchy
        if ppid != 0 {
            self.process_hierarchy.entry(ppid).or_insert_with(Vec::new).push(pid);
        }

        // Set security context
        self.security_contexts.insert(pid, security_level);

        self.active_processes.push(ctx);
    }

    pub fn prepare_ring3_transition(&mut self, pid: u64) -> Result<IretqStackFrame, &'static str> {
        let process = self
            .active_processes
            .iter_mut()
            .find(|p| p.pid == pid)
            .ok_or("User process PID not found")?;

        // 1. Update TSS rsp0 for privilege level switches (Ring 3 -> Ring 0 interrupts)
        self.tss.set_privilege_stack(0, process.kernel_stack_top);

        // 2. Activate Ring 3 user execution state
        process.is_ring3_active = true;
        self.current_pid = Some(pid);

        // 3. Construct iretq hardware transition stack frame
        Ok(IretqStackFrame::new_user_mode(
            process.entry_point,
            process.user_stack_top,
        ))
    }

    pub fn authorize_io_port(&mut self, pid: u64, port: u16) -> Result<(), &'static str> {
        let process = self
            .active_processes
            .iter_mut()
            .find(|p| p.pid == pid)
            .ok_or("User process PID not found")?;

        if !process.allowed_io_ports.contains(&port) {
            process.allowed_io_ports.push(port);
        }
        Ok(())
    }

    /// Linux-inspired signal handling with BSD signal semantics
    pub fn set_signal_mask(&mut self, pid: u64, mask: u64) -> Result<(), &'static str> {
        let process = self
            .active_processes
            .iter_mut()
            .find(|p| p.pid == pid)
            .ok_or("User process PID not found")?;

        process.signal_mask = mask;
        Ok(())
    }

    /// seccomp-style syscall filtering
    pub fn add_allowed_syscall(&mut self, pid: u64, syscall: u64) -> Result<(), &'static str> {
        let process = self
            .active_processes
            .iter_mut()
            .find(|p| p.pid == pid)
            .ok_or("User process PID not found")?;

        if !process.allowed_syscalls.contains(&syscall) {
            process.allowed_syscalls.push(syscall);
        }
        Ok(())
    }

    /// Check if syscall is allowed for process (seccomp enforcement)
    pub fn is_syscall_allowed(&self, pid: u64, syscall: u64) -> bool {
        if let Some(process) = self.active_processes.iter().find(|p| p.pid == pid) {
            match process.security_level {
                SecuritySandboxLevel::None => true,
                SecuritySandboxLevel::Minimal => process.allowed_syscalls.is_empty() || process.allowed_syscalls.contains(&syscall),
                SecuritySandboxLevel::Restricted => process.allowed_syscalls.contains(&syscall),
                SecuritySandboxLevel::Strict => process.allowed_syscalls.contains(&syscall),
            }
        } else {
            false
        }
    }

    /// Linux CGroup integration - assign process to control group
    pub fn assign_cgroup(&mut self, pid: u64, cgroup_id: u64) -> Result<(), &'static str> {
        let process = self
            .active_processes
            .iter_mut()
            .find(|p| p.pid == pid)
            .ok_or("User process PID not found")?;

        process.cgroup_id = Some(cgroup_id);
        Ok(())
    }

    /// Get child processes of a given PID (process tree traversal)
    pub fn get_child_processes(&self, ppid: u64) -> Vec<u64> {
        self.process_hierarchy.get(&ppid).cloned().unwrap_or_default()
    }

    /// Change process state (inspired by Linux task state management)
    pub fn set_process_state(&mut self, pid: u64, state: ProcessState) -> Result<(), &'static str> {
        let process = self
            .active_processes
            .iter_mut()
            .find(|p| p.pid == pid)
            .ok_or("User process PID not found")?;

        process.process_state = state;
        Ok(())
    }

    /// BSD pledge-inspired security level escalation
    pub fn set_security_level(&mut self, pid: u64, level: SecuritySandboxLevel) -> Result<(), &'static str> {
        let process = self
            .active_processes
            .iter_mut()
            .find(|p| p.pid == pid)
            .ok_or("User process PID not found")?;

        // Only allow moving to more restrictive levels (pledge principle)
        if level as u8 >= process.security_level as u8 {
            process.security_level = level;
            self.security_contexts.insert(pid, level);
            Ok(())
        } else {
            Err("Cannot downgrade security level")
        }
    }
}

impl Default for SovereignRing3UserModeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Standalone Unit Test Suite
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_state_segment_64_layout() {
        let mut tss = SovereignTaskStateSegment64::new();
        tss.set_privilege_stack(0, 0xFFFF_8000_1000_0000);
        assert_eq!(tss.privilege_stack_table[0], 0xFFFF_8000_1000_0000);

        tss.set_interrupt_stack(1, 0xFFFF_8000_2000_0000);
        assert_eq!(tss.interrupt_stack_table[0], 0xFFFF_8000_2000_0000);
    }

    #[test]
    fn test_iretq_user_mode_stack_frame() {
        let frame = IretqStackFrame::new_user_mode(0x0000_7FFF_0000_1000, 0x0000_7FFF_FFFF_0000);
        assert_eq!(frame.rip, 0x0000_7FFF_0000_1000);
        assert_eq!(frame.cs, 0x2B);
        assert_eq!(frame.rflags, 0x202);
        assert_eq!(frame.rsp, 0x0000_7FFF_FFFF_0000);
        assert_eq!(frame.ss, 0x33);
    }

    #[test]
    fn test_ring3_user_mode_engine_transition() {
        let mut engine = SovereignRing3UserModeEngine::new();
        engine.register_user_process(
            101,
            1, // parent PID
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::Minimal,
        );

        let frame = engine.prepare_ring3_transition(101).unwrap();
        assert_eq!(frame.rip, 0x0000_7FFF_0000_0000);
        assert_eq!(engine.tss.privilege_stack_table[0], 0xFFFF_8000_8000_0000);

        assert!(engine.authorize_io_port(101, 0x3F8).is_ok()); // COM1 serial port
    }

    #[test]
    fn test_security_level_escalation() {
        let mut engine = SovereignRing3UserModeEngine::new();
        engine.register_user_process(
            102,
            1,
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::None,
        );

        // Should allow escalation to more restrictive levels
        assert!(engine.set_security_level(102, SecuritySandboxLevel::Minimal).is_ok());
        assert!(engine.set_security_level(102, SecuritySandboxLevel::Restricted).is_ok());

        // Should not allow downgrade
        assert!(engine.set_security_level(102, SecuritySandboxLevel::None).is_err());
    }

    #[test]
    fn test_seccomp_syscall_filtering() {
        let mut engine = SovereignRing3UserModeEngine::new();
        engine.register_user_process(
            103,
            1,
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::Restricted,
        );

        engine.add_allowed_syscall(103, 1).unwrap(); // Allow syscall 1
        engine.add_allowed_syscall(103, 2).unwrap(); // Allow syscall 2

        assert!(engine.is_syscall_allowed(103, 1));
        assert!(engine.is_syscall_allowed(103, 2));
        assert!(!engine.is_syscall_allowed(103, 3)); // Not allowed
    }

    #[test]
    fn test_process_hierarchy() {
        let mut engine = SovereignRing3UserModeEngine::new();
        engine.register_user_process(
            1,
            0, // init process
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::None,
        );

        engine.register_user_process(
            2,
            1, // child of init
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::Minimal,
        );

        engine.register_user_process(
            3,
            1, // another child of init
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::Minimal,
        );

        let children = engine.get_child_processes(1);
        assert_eq!(children.len(), 2);
        assert!(children.contains(&2));
        assert!(children.contains(&3));
    }

    #[test]
    fn test_process_state_management() {
        let mut engine = SovereignRing3UserModeEngine::new();
        engine.register_user_process(
            104,
            1,
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::None,
        );

        assert!(engine.set_process_state(104, ProcessState::Sleeping).is_ok());
        assert!(engine.set_process_state(104, ProcessState::Stopped).is_ok());
        assert!(engine.set_process_state(104, ProcessState::Running).is_ok());
    }

    #[test]
    fn test_cgroup_assignment() {
        let mut engine = SovereignRing3UserModeEngine::new();
        engine.register_user_process(
            105,
            1,
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
            SecuritySandboxLevel::None,
        );

        assert!(engine.assign_cgroup(105, 42).is_ok());
        let process = engine.active_processes.iter().find(|p| p.pid == 105).unwrap();
        assert_eq!(process.cgroup_id, Some(42));
    }
}
