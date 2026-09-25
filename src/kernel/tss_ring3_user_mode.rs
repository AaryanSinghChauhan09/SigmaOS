// SPDX-License-Identifier: MIT
// SigmaOS x86_64 User Mode (Ring 3) & Task State Segment (TSS) Protection Engine
// Inspired by Linux (arch/x86/kernel/process_64.c, entry_64.S) and FreeBSD (sys/amd64/amd64/machdep.c)

use std::vec::Vec;
use std::string::String;

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

#[derive(Debug, Clone)]
pub struct UserModeProcessContext {
    pub pid: u64,
    pub user_cr3_page_table: u64, // KPTI Shadow Page Table for User Space
    pub kernel_cr3_page_table: u64, // Full Kernel Page Table
    pub kernel_stack_top: u64,
    pub user_stack_top: u64,
    pub entry_point: u64,
    pub is_ring3_active: bool,
    pub allowed_io_ports: Vec<u16>,
}

pub struct SovereignRing3UserModeEngine {
    pub tss: SovereignTaskStateSegment64,
    pub active_processes: Vec<UserModeProcessContext>,
    pub current_pid: Option<u64>,
}

impl SovereignRing3UserModeEngine {
    pub fn new() -> Self {
        Self {
            tss: SovereignTaskStateSegment64::new(),
            active_processes: Vec::new(),
            current_pid: None,
        }
    }

    pub fn register_user_process(
        &mut self,
        pid: u64,
        user_cr3: u64,
        kernel_cr3: u64,
        kernel_stack: u64,
        user_stack: u64,
        entry: u64,
    ) {
        let ctx = UserModeProcessContext {
            pid,
            user_cr3_page_table: user_cr3,
            kernel_cr3_page_table: kernel_cr3,
            kernel_stack_top: kernel_stack,
            user_stack_top: user_stack,
            entry_point: entry,
            is_ring3_active: false,
            allowed_io_ports: Vec::new(),
        };
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
            0x1000,
            0x2000,
            0xFFFF_8000_8000_0000,
            0x0000_7FFF_8000_0000,
            0x0000_7FFF_0000_0000,
        );

        let frame = engine.prepare_ring3_transition(101).unwrap();
        assert_eq!(frame.rip, 0x0000_7FFF_0000_0000);
        assert_eq!(engine.tss.privilege_stack_table[0], 0xFFFF_8000_8000_0000);

        assert!(engine.authorize_io_port(101, 0x3F8).is_ok()); // COM1 serial port
    }
}
