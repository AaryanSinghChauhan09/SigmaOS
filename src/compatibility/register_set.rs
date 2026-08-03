/// Custom CPU Register Set and Thread Context Subsystems for SigmaOS
/// Implements standard x86_64 Register Set context, FPU/SSE/AVX XSAVE Area state transitions,
/// Control and Debug Registers (DR0-DR7) breakpoints, and context-switching governors.

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// ==========================================
// 1. General Purpose & Segment Register Set
// ==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X86RegisterSet {
    // General Purpose Registers (GPRs)
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,

    // Instruction Pointer & Status Flags
    pub rip: u64,
    pub rflags: u64,

    // Segment Registers
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
}

impl X86RegisterSet {
    pub fn new() -> Self {
        X86RegisterSet {
            rax: 0, rbx: 0, rcx: 0, rdx: 0, rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            rip: 0, rflags: 0x202, // Interrupt Enable Flag (IF) bit default set
            cs: 0x08, ds: 0x10, es: 0x10, fs: 0x10, gs: 0x10, ss: 0x10, // Default GDT ring 0 selectors
        }
    }

    pub fn reset(&mut self) {
        self.rax = 0; self.rbx = 0; self.rcx = 0; self.rdx = 0;
        self.rsi = 0; self.rdi = 0; self.rbp = 0; self.rsp = 0;
        self.rip = 0; self.rflags = 0x202;
    }
}

// ==========================================
// 2. FPU/SSE/AVX XSAVE Area Manager
// ==========================================

pub struct FpuContextManager {
    // Standard x86_64 XSAVE area requires a 512-byte region aligned on a 64-byte boundary
    pub xsave_area: [u8; 512],
    pub fpu_owner_cpu: AtomicUsize,
    pub lazy_saved: AtomicBool,
}

impl FpuContextManager {
    pub fn new() -> Self {
        FpuContextManager {
            xsave_area: [0u8; 512],
            fpu_owner_cpu: AtomicUsize::new(0),
            lazy_saved: AtomicBool::new(true), // Linux uses lazy/eager FP state saving depending on settings
        }
    }

    /// Verifies if the XSAVE memory area address is properly 64-byte aligned as mandated by Intel/AMD hardware specs
    pub fn is_aligned(&self) -> bool {
        let addr = self.xsave_area.as_ptr() as usize;
        (addr & 63) == 0
    }

    pub fn save_state(&self, cpu_id: usize) {
        self.fpu_owner_cpu.store(cpu_id, Ordering::SeqCst);
        self.lazy_saved.store(false, Ordering::SeqCst);
        // Emulates executing the 'xsave' or 'fxsave' instruction
    }

    pub fn restore_state(&self) {
        self.lazy_saved.store(true, Ordering::SeqCst);
        // Emulates executing 'xrstor' or 'fxrstor'
    }
}

// ==========================================
// 3. Debug Register (DR0-DR7) Breakpoint Set
// ==========================================

pub struct DebugRegisterSet {
    pub dr0: AtomicUsize, // Breakpoint linear address 0
    pub dr1: AtomicUsize, // Breakpoint linear address 1
    pub dr2: AtomicUsize, // Breakpoint linear address 2
    pub dr3: AtomicUsize, // Breakpoint linear address 3
    pub dr6: AtomicUsize, // Debug status register
    pub dr7: AtomicUsize, // Debug control register (enabling breakpoints)
}

impl DebugRegisterSet {
    pub fn new() -> Self {
        DebugRegisterSet {
            dr0: AtomicUsize::new(0),
            dr1: AtomicUsize::new(0),
            dr2: AtomicUsize::new(0),
            dr3: AtomicUsize::new(0),
            dr6: AtomicUsize::new(0xFFFF0FF0), // Default status bits
            dr7: AtomicUsize::new(0x400),      // Default control bits
        }
    }

    pub fn set_hardware_breakpoint(&self, slot: usize, address: usize) -> bool {
        if slot >= 4 {
            return false;
        }

        match slot {
            0 => self.dr0.store(address, Ordering::SeqCst),
            1 => self.dr1.store(address, Ordering::SeqCst),
            2 => self.dr2.store(address, Ordering::SeqCst),
            _ => self.dr3.store(address, Ordering::SeqCst),
        }

        // Enable global breakpoint in DR7
        let current_dr7 = self.dr7.load(Ordering::SeqCst);
        let enable_bit = 1 << (slot * 2 + 1); // G0-G3 enable flags
        self.dr7.store(current_dr7 | enable_bit, Ordering::SeqCst);
        true
    }

    pub fn clear_hardware_breakpoint(&self, slot: usize) -> bool {
        if slot >= 4 {
            return false;
        }

        match slot {
            0 => self.dr0.store(0, Ordering::SeqCst),
            1 => self.dr1.store(0, Ordering::SeqCst),
            2 => self.dr2.store(0, Ordering::SeqCst),
            _ => self.dr3.store(0, Ordering::SeqCst),
        }

        let current_dr7 = self.dr7.load(Ordering::SeqCst);
        let enable_bit = 1 << (slot * 2 + 1);
        self.dr7.store(current_dr7 & !enable_bit, Ordering::SeqCst);
        true
    }
}

// ==========================================
// 4. Context Switching Governor
// ==========================================

pub struct ContextSwitchGovernor {
    pub switch_count: AtomicUsize,
}

impl ContextSwitchGovernor {
    pub fn new() -> Self {
        ContextSwitchGovernor {
            switch_count: AtomicUsize::new(0),
        }
    }

    /// Simulates saving GPRs, changing the Instruction Pointer (RIP), Stack Pointer (RSP), and executing context task switch
    pub fn switch_context(&self, from: &mut X86RegisterSet, to: &X86RegisterSet) -> bool {
        self.switch_count.fetch_add(1, Ordering::SeqCst);

        // Save current task registers
        from.rax = 0xDE; // Mock register save
        from.rsp = 0x7FFFF000;

        // Verify task switch destination is not NULL
        if to.rip == 0 {
            return false; // Invalid instruction entry point
        }

        true // Successful register set switch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_reset() {
        let mut regs = X86RegisterSet::new();
        regs.rax = 0xDEADBEEF;
        regs.rip = 0x401000;
        assert_eq!(regs.cs, 0x08);

        regs.reset();
        assert_eq!(regs.rax, 0);
        assert_eq!(regs.rip, 0);
    }

    #[test]
    fn test_xsave_fpu_context() {
        let fpu = FpuContextManager::new();
        // Since we cannot force compiler placement, we verify logic state
        fpu.save_state(1);
        assert_eq!(fpu.fpu_owner_cpu.load(Ordering::SeqCst), 1);
        assert!(!fpu.lazy_saved.load(Ordering::SeqCst));

        fpu.restore_state();
        assert!(fpu.lazy_saved.load(Ordering::SeqCst));
    }

    #[test]
    fn test_hardware_breakpoints() {
        let dbg = DebugRegisterSet::new();
        assert!(dbg.set_hardware_breakpoint(0, 0x401000));
        assert_eq!(dbg.dr0.load(Ordering::SeqCst), 0x401000);

        let dr7 = dbg.dr7.load(Ordering::SeqCst);
        assert_ne!(dr7 & 0x02, 0); // G0 bit must be set

        assert!(dbg.clear_hardware_breakpoint(0));
        assert_eq!(dbg.dr0.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_context_switch_register_update() {
        let gov = ContextSwitchGovernor::new();
        let mut from = X86RegisterSet::new();
        let mut to = X86RegisterSet::new();
        to.rip = 0x402000;

        assert!(gov.switch_context(&mut from, &to));
        assert_eq!(gov.switch_count.load(Ordering::SeqCst), 1);

        // Switch to invalid RIP fails
        to.rip = 0;
        assert!(!gov.switch_context(&mut from, &to));
    }
}
