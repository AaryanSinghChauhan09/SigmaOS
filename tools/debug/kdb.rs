// SPDX-License-Identifier: GPL-3.0-or-later
// In-Kernel Debugger (KDB) for SigmaOS
// Location: tools/debug/kdb.rs

#![no_std]
extern crate alloc;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KernelBreakpoint {
    pub id: u32,
    pub address: usize,
    pub enabled: bool,
    pub hit_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuRegisterState {
    pub rip: usize,
    pub rsp: usize,
    pub rbp: usize,
    pub rax: usize,
    pub rbx: usize,
    pub rcx: usize,
    pub rdx: usize,
}

pub struct KernelDebugger {
    pub breakpoints: Vec<KernelBreakpoint>,
    pub registers: CpuRegisterState,
}

impl KernelDebugger {
    pub fn new() -> Self {
        KernelDebugger {
            breakpoints: Vec::new(),
            registers: CpuRegisterState {
                rip: 0,
                rsp: 0,
                rbp: 0,
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
            },
        }
    }

    pub fn set_breakpoint(&mut self, id: u32, address: usize) {
        self.breakpoints.push(KernelBreakpoint {
            id,
            address,
            enabled: true,
            hit_count: 0,
        });
    }

    pub fn check_breakpoint_hit(&mut self, address: usize) -> Option<u32> {
        for bp in self.breakpoints.iter_mut() {
            if bp.enabled && bp.address == address {
                bp.hit_count += 1;
                return Some(bp.id);
            }
        }
        None
    }

    pub fn update_registers(&mut self, regs: CpuRegisterState) {
        self.registers = regs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_debugger_breakpoint() {
        let mut kdb = KernelDebugger::new();
        kdb.set_breakpoint(1, 0xFFFFFFFF80001000);

        let hit_id = kdb.check_breakpoint_hit(0xFFFFFFFF80001000);
        assert_eq!(hit_id, Some(1));
        assert_eq!(kdb.breakpoints[0].hit_count, 1);

        assert_eq!(kdb.check_breakpoint_hit(0xFFFFFFFF80002000), None);
    }
}
