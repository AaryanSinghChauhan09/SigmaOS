// Sovereign Virtual CPU and Ring Privilege Separation Simulator
// Implements x86 CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.

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
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable)
    pub cr3: u64, // Control Register 3: Page Table Base Address
    pub rip: u64, // Instruction Pointer
    pub rsp: u64, // Stack Pointer
}

/// Sovereign Virtual CPU managing execution state and privilege boundaries
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub stack_memory: Vec<u64>,
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
                cr0: 0,
                cr3: 0,
                rip: 0,
                rsp: 1024, // High stack pointer
            },
            stack_memory: vec![0; 128], // 128 stack frames
        }
    }

    /// Simulates standard x86 assembly data movement: `mov <dest>, <src_val>`
    pub fn mov_val_to_reg(&mut self, dest: &str, val: u64) -> Result<(), CpuError> {
        match dest {
            "rax" => self.registers.rax = val,
            "rbx" => self.registers.rbx = val,
            "rcx" => self.registers.rcx = val,
            "rdx" => self.registers.rdx = val,
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
    fn test_virtual_cpu_instructions_and_data_movement() {
        let mut cpu = SovereignVirtualCPU::new();

        // 1. Move value 120 directly to register RAX
        cpu.mov_val_to_reg("rax", 120).unwrap();
        assert_eq!(cpu.registers.rax, 120);

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
}
