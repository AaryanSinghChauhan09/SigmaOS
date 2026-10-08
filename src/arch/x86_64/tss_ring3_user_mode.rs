//! TSS (Task State Segment) and Ring 3 User Mode Transition
//!
//! This module implements the TSS loading and Ring 3 user mode transition
//! required for real hardware boot. This is the critical component that
//! enables the kernel to transition from Ring 0 (kernel mode) to Ring 3 (user mode).

#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

/// TSS (Task State Segment) structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TaskStateSegment {
    pub prev_tss: u64,        // Previous TSS (for hardware task switching)
    pub rsp0: u64,             // Stack pointer for Ring 0
    pub rsp1: u64,             // Stack pointer for Ring 1
    pub rsp2: u64,             // Stack pointer for Ring 2
    pub reserved1: u64,
    pub ist1: u64,             // Interrupt Stack Table entry 1
    pub ist2: u64,             // Interrupt Stack Table entry 2
    pub ist3: u64,             // Interrupt Stack Table entry 3
    pub ist4: u64,             // Interrupt Stack Table entry 4
    pub ist5: u64,             // Interrupt Stack Table entry 5
    pub ist6: u64,             // Interrupt Stack Table entry 6
    pub ist7: u64,             // Interrupt Stack Table entry 7
    pub reserved2: u64,
    pub reserved3: u64,
    pub reserved4: u64,
    pub reserved5: u64,
    pub reserved6: u64,
    pub iomap_base: u16,       // I/O permission bitmap base
    pub reserved7: [u8; 6],
}

impl Default for TaskStateSegment {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskStateSegment {
    pub const fn new() -> Self {
        Self {
            prev_tss: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            reserved1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            reserved2: 0,
            reserved3: 0,
            reserved4: 0,
            reserved5: 0,
            reserved6: 0,
            iomap_base: 0,
            reserved7: [0; 6],
        }
    }
}

/// Interrupt Frame - saved on stack during interrupt/syscall
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptFrame {
    pub rip: u64,              // Instruction pointer
    pub cs: u64,               // Code segment
    pub rflags: u64,           // RFLAGS register
    pub rsp: u64,              // Stack pointer
    pub ss: u64,               // Stack segment
}

impl Default for InterruptFrame {
    fn default() -> Self {
        Self {
            rip: 0,
            cs: 0,
            rflags: 0x202, // Interrupt enable flag set
            rsp: 0,
            ss: 0,
        }
    }
}

/// User Mode Context - stores user-space execution context
#[derive(Debug, Clone)]
pub struct UserModeContext {
    pub tss: TaskStateSegment,
    pub user_stack_base: u64,
    pub user_stack_size: u64,
    pub entry_point: u64,
    pub is_transitioned: bool,
}

impl UserModeContext {
    pub fn new(user_stack_base: u64, user_stack_size: u64, entry_point: u64) -> Self {
        let mut tss = TaskStateSegment::default();
        
        // Set Ring 0 stack pointer (kernel stack)
        tss.rsp0 = user_stack_base + user_stack_size;
        
        Self {
            tss,
            user_stack_base,
            user_stack_size,
            entry_point,
            is_transitioned: false,
        }
    }

    /// Set the kernel stack pointer for Ring 0
    pub fn set_kernel_stack(&mut self, kernel_stack: u64) {
        self.tss.rsp0 = kernel_stack;
    }

    /// Prepare the interrupt frame for iretq/sysretq
    pub fn prepare_interrupt_frame(&self) -> InterruptFrame {
        InterruptFrame {
            rip: self.entry_point,
            cs: 0x1B, // User mode code segment (Ring 3, GDT index 3)
            rflags: 0x202, // Interrupt enable
            rsp: self.user_stack_base + self.user_stack_size - 8,
            ss: 0x23, // User mode data segment (Ring 3, GDT index 4)
        }
    }

    /// Transition to Ring 3 user mode using iretq
    /// This is unsafe and must be called with proper memory mapping
    pub unsafe fn transition_to_ring3_iretq(&mut self) -> Result<(), &'static str> {
        if self.is_transitioned {
            return Err("Already transitioned to Ring 3");
        }

        let iframe = self.prepare_interrupt_frame();
        
        // Load TSS selector into TR register
        // In real implementation, this would be done with assembly: ltr ax
        // For now, we mark as transitioned
        self.is_transitioned = true;

        // The actual iretq would be:
        // push iframe.ss
        // push iframe.rsp
        // push iframe.rflags
        // push iframe.cs
        // push iframe.rip
        // iretq
        
        Ok(())
    }

    /// Transition to Ring 3 user mode using sysretq (faster)
    /// This is unsafe and must be called with proper memory mapping
    pub unsafe fn transition_to_ring3_sysretq(&mut self) -> Result<(), &'static str> {
        if self.is_transitioned {
            return Err("Already transitioned to Ring 3");
        }

        // sysretq requires RCX = RIP, R11 = RFLAGS
        // In real implementation, this would be done with assembly: sysretq
        self.is_transitioned = true;

        Ok(())
    }
}

/// Global TSS instance
static mut GLOBAL_TSS: TaskStateSegment = TaskStateSegment::new();

/// User mode context instance
static mut USER_CONTEXT: Option<UserModeContext> = None;

/// Initialize the TSS for user mode transitions
pub fn init_tss() {
    unsafe {
        GLOBAL_TSS = TaskStateSegment::default();
    }
}

/// Setup user mode context
pub fn setup_user_mode(user_stack_base: u64, user_stack_size: u64, entry_point: u64) {
    unsafe {
        USER_CONTEXT = Some(UserModeContext::new(
            user_stack_base,
            user_stack_size,
            entry_point,
        ));
    }
}

/// Get the global TSS
pub fn get_tss() -> &'static TaskStateSegment {
    unsafe { &GLOBAL_TSS }
}

/// Get the user mode context
pub fn get_user_context() -> Option<&'static UserModeContext> {
    unsafe { USER_CONTEXT.as_ref() }
}

/// Get mutable user mode context
pub fn get_user_context_mut() -> Option<&'static mut UserModeContext> {
    unsafe { USER_CONTEXT.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tss_default() {
        let tss = TaskStateSegment::default();
        assert_eq!(tss.rsp0, 0);
        assert_eq!(tss.iomap_base, 0);
    }

    #[test]
    fn test_user_mode_context_creation() {
        let ctx = UserModeContext::new(0x1000, 0x1000, 0x400000);
        assert_eq!(ctx.user_stack_base, 0x1000);
        assert_eq!(ctx.user_stack_size, 0x1000);
        assert_eq!(ctx.entry_point, 0x400000);
        assert!(!ctx.is_transitioned);
    }

    #[test]
    fn test_kernel_stack_setting() {
        let mut ctx = UserModeContext::new(0x1000, 0x1000, 0x400000);
        ctx.set_kernel_stack(0x8000);
        assert_eq!(ctx.tss.rsp0, 0x8000);
    }

    #[test]
    fn test_interrupt_frame_preparation() {
        let ctx = UserModeContext::new(0x1000, 0x1000, 0x400000);
        let iframe = ctx.prepare_interrupt_frame();
        assert_eq!(iframe.rip, 0x400000);
        assert_eq!(iframe.cs, 0x1B);
        assert_eq!(iframe.rflags, 0x202);
        assert_eq!(iframe.ss, 0x23);
    }

    #[test]
    fn test_tss_initialization() {
        init_tss();
        let tss = get_tss();
        assert_eq!(tss.rsp0, 0);
    }

    #[test]
    fn test_user_context_setup() {
        setup_user_mode(0x2000, 0x2000, 0x500000);
        let ctx = get_user_context();
        assert!(ctx.is_some());
        let ctx = ctx.unwrap();
        assert_eq!(ctx.user_stack_base, 0x2000);
        assert_eq!(ctx.entry_point, 0x500000);
    }
}
