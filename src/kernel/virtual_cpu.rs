// Sovereign Virtual CPU and Ring Privilege Separation Simulator
// Implements x86 CPU Modes, Ring privilege isolation (Rings 0-3), Register Sets, and Instruction Data Movement.
// Enhanced with Model Specific Registers (MSRs), lazy FP/SSE state saving (Linux/BSD style), and Exception trap vector routines.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Success = 0,
    InvalidRegister = 1,
    PrivilegeViolation = 2,
    StackOverflow = 3,
    PagingDisabled = 4,
    InvalidInstruction = 5,
    FloatingPointStateNotSaved = 6,
}

/// x86 CPU Execution Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuMode {
    RealMode,      // 16-bit real addressing
    ProtectedMode, // 32-bit protected segments
    LongMode,      // 64-bit paging active
}

/// CPU Ring Privilege Separation levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpuRing {
    Ring0 = 0, // Kernel Core (unrestricted)
    Ring1 = 1, // Device Drivers (SDF / isolated)
    Ring2 = 2, // System Services (init system)
    Ring3 = 3, // Userland Applications (most restricted)
}

/// Complete x86 Virtual Register Set
#[derive(Debug, Clone, Copy)]
pub struct RegisterSet {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64, // General Purpose / Destination Index (Syscall arg 1)
    pub rsi: u64, // General Purpose / Source Index (Syscall arg 2)
    pub rbp: u64, // Base Frame Pointer (x86_64 / Linux / BSD stack unwinding)
    pub r8: u64,  // General Purpose x86_64 64-bit Extension Register 8
    pub r9: u64,  // General Purpose x86_64 64-bit Extension Register 9
    pub r10: u64, // General Purpose x86_64 64-bit Extension Register 10
    pub r11: u64, // General Purpose x86_64 64-bit Extension Register 11
    pub r12: u64, // General Purpose x86_64 64-bit Extension Register 12
    pub r13: u64, // General Purpose x86_64 64-bit Extension Register 13
    pub r14: u64, // General Purpose x86_64 64-bit Extension Register 14
    pub r15: u64, // General Purpose x86_64 64-bit Extension Register 15
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable), Bit 3 is TS (Task Switched)
    pub cr3: u64, // Control Register 3: Page Table Base Address
    pub cr4: u64, // Control Register 4: Os Support for SSE/XSAVE
    pub rip: u64, // Instruction Pointer
    pub rsp: u64, // Stack Pointer
}

/// Model Specific Registers (MSRs) for Fast System Call routing (Intel/AMD standard)
#[derive(Debug, Clone, Copy)]
pub struct ModelSpecificRegisters {
    pub efer: u64,   // Extended Feature Enable Register
    pub star: u64,   // Segment selector for SYSENTER/SYSEXIT
    pub lstar: u64,  // Target RIP for 64-bit SYSCALL
    pub sfmask: u64, // RFLAGS mask for SYSCALL
    pub fs_base: u64, // Thread Local Storage (TLS) pointer (Linux/BSD standard)
    pub gs_base: u64, // Per-CPU data block pointer
    pub kernel_gs_base: u64, // Saved kernel GS base pointer (swapped on transition)
}

/// System Bus Special Function Registers: MAR, MBR, I/OAR, I/OBR
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemBusRegisters {
    pub mar: u64,  // Memory Address Register
    pub mbr: u64,  // Memory Buffer Register
    pub ioar: u16, // I/O Address Register
    pub iobr: u32, // I/O Buffer Register
}

/// System Bus Controller managing memory and I/O bus transactions
pub struct SystemBusController {
    pub bus_regs: SystemBusRegisters,
}

impl SystemBusController {
    pub fn new() -> Self {
        Self {
            bus_regs: SystemBusRegisters::default(),
        }
    }

    pub fn write_memory_bus(&mut self, addr: u64, data: u64) {
        self.bus_regs.mar = addr;
        self.bus_regs.mbr = data;
    }

    pub fn read_memory_bus(&mut self, addr: u64) -> u64 {
        self.bus_regs.mar = addr;
        self.bus_regs.mbr
    }

    pub fn write_io_bus(&mut self, port: u16, data: u32) {
        self.bus_regs.ioar = port;
        self.bus_regs.iobr = data;
    }

    pub fn read_io_bus(&mut self, port: u16) -> u32 {
        self.bus_regs.ioar = port;
        self.bus_regs.iobr
    }
}

/// Sovereign Virtual CPU managing execution state and privilege boundaries
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub msrs: ModelSpecificRegisters,
    pub stack_memory: Vec<u64>,
    // Lazy FP/SSE Context Tracking (Linux/BSD style)
    pub fp_dirty: bool,
    pub fp_save_area: [u64; 64], // Simulated 512-byte FXSAVE/XSAVE area
}

impl SovereignVirtualCPU {
    pub fn new() -> Self {
        Self {
            mode: CpuMode::RealMode,
            ring: CpuRing::Ring0, // Starts in Kernel Ring 0 during early boot
            registers: RegisterSet {
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
                rdi: 0,
                rsi: 0,
                rbp: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
                cr0: 0,
                cr3: 0,
                cr4: 0,
                rip: 0,
                rsp: 1024, // High stack pointer
            },
            msrs: ModelSpecificRegisters {
                efer: 0,
                star: 0,
                lstar: 0,
                sfmask: 0,
                fs_base: 0,
                gs_base: 0,
                kernel_gs_base: 0,
            },
            stack_memory: vec![0; 128], // 128 stack frames
            fp_dirty: false,
            fp_save_area: [0; 64],
        }
    }

    /// Simulates standard x86 assembly data movement: `mov <dest>, <src_val>`
    pub fn mov_val_to_reg(&mut self, dest: &str, val: u64) -> Result<(), CpuError> {
        match dest {
            "rax" => self.registers.rax = val,
            "rbx" => self.registers.rbx = val,
            "rcx" => self.registers.rcx = val,
            "rdx" => self.registers.rdx = val,
            "rdi" => self.registers.rdi = val,
            "rsi" => self.registers.rsi = val,
            "rbp" => self.registers.rbp = val,
            "r8"  => self.registers.r8 = val,
            "r9"  => self.registers.r9 = val,
            "r10" => self.registers.r10 = val,
            "r11" => self.registers.r11 = val,
            "r12" => self.registers.r12 = val,
            "r13" => self.registers.r13 = val,
            "r14" => self.registers.r14 = val,
            "r15" => self.registers.r15 = val,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    /// Simulates standard x86 assembly stack pushing: `push <val>`
    pub fn push_stack(&mut self, val: u64) -> Result<(), CpuError> {
        if self.registers.rsp == 0 {
            return Err(CpuError::StackOverflow);
        }
        self.registers.rsp -= 8;
        let index = (self.registers.rsp / 8) as usize;
        if index < self.stack_memory.len() {
            self.stack_memory[index] = val;
            Ok(())
        } else {
            Err(CpuError::StackOverflow)
        }
    }

    /// Simulates standard x86 assembly stack popping: `pop`
    pub fn pop_stack(&mut self) -> Result<u64, CpuError> {
        let index = (self.registers.rsp / 8) as usize;
        if index >= self.stack_memory.len() {
            return Err(CpuError::StackOverflow);
        }
        let val = self.stack_memory[index];
        self.registers.rsp += 8;
        Ok(val)
    }

    /// Transitions between x86 Execution Modes (Alters PE bits dynamically)
    pub fn transition_mode(&mut self, target: CpuMode) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation); // Only Ring 0 can alter CPU modes!
        }

        match target {
            CpuMode::RealMode => {
                self.registers.cr0 &= !1; // Clear PE bit 0
                self.mode = CpuMode::RealMode;
            }
            CpuMode::ProtectedMode => {
                self.registers.cr0 |= 1; // Set PE bit 0
                self.mode = CpuMode::ProtectedMode;
            }
            CpuMode::LongMode => {
                if self.registers.cr3 == 0 {
                    return Err(CpuError::PagingDisabled); // LongMode requires CR3 paging base!
                }
                self.registers.cr0 |= 1; // PE bit 0 must be active
                self.mode = CpuMode::LongMode;
            }
        }
        Ok(())
    }

    /// Transitions the active thread across CPU Rings (Privilege Separation)
    /// - Ring 0 can transition down to Ring 1, 2, or 3.
    /// - Restricted Rings (Ring 3) CANNOT escalate back to Ring 0 directly (must trigger software trap).
    pub fn transition_ring(&mut self, target: CpuRing) -> Result<(), CpuError> {
        if target > self.ring {
            // Lowering privileges is always allowed
            self.ring = target;
            Ok(())
        } else if target == self.ring {
            Ok(())
        } else {
            // Escalating privileges is blocked unless in Ring 0
            if self.ring == CpuRing::Ring0 {
                self.ring = target;
                Ok(())
            } else {
                Err(CpuError::PrivilegeViolation) // Blocked!
            }
        }
    }

    /// Write to Model Specific Register (rdmsr/wrmsr emulation)
    pub fn write_msr(&mut self, reg: u32, value: u64) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        match reg {
            0xC0000080 => self.msrs.efer = value,
            0xC0000081 => self.msrs.star = value,
            0xC0000082 => self.msrs.lstar = value,
            0xC0000084 => self.msrs.sfmask = value,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    /// Emulates the fast 64-bit `SYSCALL` instruction used by Linux and BSD for low-overhead user space transitions
    pub fn execute_syscall(&mut self) -> Result<(), CpuError> {
        if self.mode != CpuMode::LongMode {
            return Err(CpuError::InvalidInstruction);
        }

        // 1. Save current RIP to RCX, and save RFLAGS to R11 (simulated)
        self.registers.rcx = self.registers.rip;

        // 2. Load syscall target RIP from LSTAR MSR
        self.registers.rip = self.msrs.lstar;

        // 3. Elevate privilege to Ring 0
        self.ring = CpuRing::Ring0;

        Ok(())
    }

    /// Emulates x86 software trap / CPU exception interrupt handling (e.g. GPF, Page Fault, Soft Traps)
    /// Automatically pushes RIP, CS, and registers onto the kernel stack, and escalates ring to Ring 0.
    pub fn trigger_interrupt_trap(&mut self, vector: u8, handler_rip: u64) -> Result<(), CpuError> {
        // Save current instruction pointer and stack context
        let old_rip = self.registers.rip;
        let old_rsp = self.registers.rsp;
        let old_ring = self.ring as u64;

        // Escapes to Ring 0 first (privilege elevation)
        self.ring = CpuRing::Ring0;

        // Push execution context onto kernel stack frame (standard hardware frame)
        self.push_stack(old_rsp)?;
        self.push_stack(old_rip)?;
        self.push_stack(old_ring)?;
        self.push_stack(vector as u64)?;

        // Jump to exception service handler address
        self.registers.rip = handler_rip;

        Ok(())
    }

    /// Emulates the x86 `SWAPGS` instruction (Linux/BSD transition from user space to kernel space).
    pub fn swapgs(&mut self) {
        let temp = self.msrs.gs_base;
        self.msrs.gs_base = self.msrs.kernel_gs_base;
        self.msrs.kernel_gs_base = temp;
    }

    /// Lazily handles floating-point/vector register context switches (Linux/BSD style).
    /// If TS (Task Switched) bit in CR0 is set, accessing FP registers triggers a Device Not Available exception.
    /// The kernel then clears TS, saves/restores the FP area, and proceeds.
    pub fn handle_lazy_fp_state_restore(
        &mut self,
        is_fp_instruction: bool,
    ) -> Result<(), CpuError> {
        let ts_bit_active = (self.registers.cr0 & (1 << 3)) != 0;
        if is_fp_instruction && ts_bit_active {
            // Trigger Device Not Available (#NM exception trap)
            // Clear TS bit in CR0
            self.registers.cr0 &= !(1 << 3);

            // Simulates copying FP state from memory (XSAVE)
            if self.fp_dirty {
                // Restore state
                self.fp_dirty = false;
            }
            Ok(())
        } else if is_fp_instruction {
            Ok(())
        } else {
            Err(CpuError::InvalidInstruction)
        }
    }
}

impl Default for SovereignVirtualCPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_bus_controller_registers() {
        let mut bus = SystemBusController::new();
        bus.write_memory_bus(0x1000, 0xCAFEBABEDEADBEEF);
        assert_eq!(bus.bus_regs.mar, 0x1000);
        assert_eq!(bus.bus_regs.mbr, 0xCAFEBABEDEADBEEF);
        assert_eq!(bus.read_memory_bus(0x1000), 0xCAFEBABEDEADBEEF);

        bus.write_io_bus(0x3F8, 0x00000041);
        assert_eq!(bus.bus_regs.ioar, 0x3F8);
        assert_eq!(bus.bus_regs.iobr, 0x00000041);
        assert_eq!(bus.read_io_bus(0x3F8), 0x00000041);
    }

    #[test]
    fn test_virtual_cpu_instructions_and_data_movement() {
        let mut cpu = SovereignVirtualCPU::new();

        // 1. Move value 120 directly to register RAX and test 64-bit extension registers (R8-R15)
        cpu.mov_val_to_reg("rax", 120).unwrap();
        assert_eq!(cpu.registers.rax, 120);
        cpu.mov_val_to_reg("r8", 0xDEADBEEF88888888).unwrap();
        assert_eq!(cpu.registers.r8, 0xDEADBEEF88888888);
        cpu.mov_val_to_reg("r15", 0x1515151515151515).unwrap();
        assert_eq!(cpu.registers.r15, 0x1515151515151515);

        // 2. Push value to stack
        cpu.push_stack(999).unwrap();
        assert_eq!(cpu.registers.rsp, 1016);
    }

    #[test]
    fn test_virtual_cpu_execution_mode_switching() {
        let mut cpu = SovereignVirtualCPU::new();
        assert_eq!(cpu.mode, CpuMode::RealMode);
        assert_eq!(cpu.registers.cr0 & 1, 0); // PE bit 0 is inactive

        // Transition to ProtectedMode (32-bit segment)
        cpu.transition_mode(CpuMode::ProtectedMode).unwrap();
        assert_eq!(cpu.mode, CpuMode::ProtectedMode);
        assert_eq!(cpu.registers.cr0 & 1, 1); // PE bit 1 is set successfully!

        // Attempting LongMode without paging CR3 should fail
        assert_eq!(
            cpu.transition_mode(CpuMode::LongMode),
            Err(CpuError::PagingDisabled)
        );

        // Set CR3 page directory pointer base address
        cpu.registers.cr3 = 0x100000;
        assert!(cpu.transition_mode(CpuMode::LongMode).is_ok());
    }

    #[test]
    fn test_virtual_cpu_ring_privilege_isolation() {
        let mut cpu = SovereignVirtualCPU::new();
        assert_eq!(cpu.ring, CpuRing::Ring0);

        // Transition down to Userland Ring 3 (permitted)
        cpu.transition_ring(CpuRing::Ring3).unwrap();
        assert_eq!(cpu.ring, CpuRing::Ring3);

        // Attempting to escalate back to Ring 0 from Userland Ring 3 should fail (PrivilegeViolation!)
        assert_eq!(
            cpu.transition_ring(CpuRing::Ring0),
            Err(CpuError::PrivilegeViolation)
        );

        // Mode changes from Ring 3 should also fail
        assert_eq!(
            cpu.transition_mode(CpuMode::RealMode),
            Err(CpuError::PrivilegeViolation)
        );
    }

    #[test]
    fn test_msr_and_fast_syscall() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.registers.cr3 = 0x200000;
        cpu.transition_mode(CpuMode::LongMode).unwrap();

        // Write to target LSTAR MSR
        cpu.write_msr(0xC0000082, 0xFFFFFFFF80100000).unwrap();
        assert_eq!(cpu.msrs.lstar, 0xFFFFFFFF80100000);

        // Lower privilege to User space Ring 3
        cpu.transition_ring(CpuRing::Ring3).unwrap();

        // Simulate a system call instruction execution
        cpu.registers.rip = 0x400000; // user rip
        cpu.execute_syscall().unwrap();

        // CPU must have jumped to kernel's fast system call handler and escalated to Ring 0
        assert_eq!(cpu.registers.rip, 0xFFFFFFFF80100000);
        assert_eq!(cpu.registers.rcx, 0x400000);
        assert_eq!(cpu.ring, CpuRing::Ring0);
    }

    #[test]
    fn test_cpu_interrupt_traps() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.registers.rip = 0xDEADBEEF;

        // Transition down to user space Ring 3
        cpu.transition_ring(CpuRing::Ring3).unwrap();

        // Trigger a page-fault exception trap (vector 14) pointing to handler 0x8100
        cpu.trigger_interrupt_trap(14, 0x8100).unwrap();

        assert_eq!(cpu.ring, CpuRing::Ring0);
        assert_eq!(cpu.registers.rip, 0x8100);

        // Popping the context frame off the kernel stack should yield the original user state
        let vector = cpu.pop_stack().unwrap();
        let ring = cpu.pop_stack().unwrap();
        let rip = cpu.pop_stack().unwrap();

        assert_eq!(vector, 14);
        assert_eq!(ring, 3);
        assert_eq!(rip, 0xDEADBEEF);
    }

    #[test]
    fn test_lazy_fp_state_restore() {
        let mut cpu = SovereignVirtualCPU::new();

        // Active TS (Task Switched) bit in CR0
        cpu.registers.cr0 |= 1 << 3;

        // Try to access FP registers - triggers restoration and clears TS bit
        cpu.handle_lazy_fp_state_restore(true).unwrap();
        assert_eq!(cpu.registers.cr0 & (1 << 3), 0);
    }
}
