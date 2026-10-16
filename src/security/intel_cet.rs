// SigmaOS Intel Control-Flow Enforcement Technology (CET) Engine
// Implements Shadow Stack (SHSTK) and Indirect Branch Tracking (IBT) defenses against ROP/COP attacks.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// ENDBR64 instruction opcode prefix bytes (F3 0F 1E FA)
pub const ENDBR64_OPCODE: [u8; 4] = [0xF3, 0x0F, 0x1E, 0xFA];

/// Shadow Stack Entry (64-bit return address)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShadowStackFrame {
    pub return_addr: u64,
}

/// Intel CET Hardware State Configuration
#[derive(Debug, Clone)]
pub struct IntelCetEngine {
    pub shadow_stack_enabled: bool,
    pub ibt_enabled: bool,
    pub shadow_stack: Vec<ShadowStackFrame>,
    pub shadow_stack_pointer: usize,
    pub control_protection_faults: u64,
}

impl IntelCetEngine {
    pub fn new(shadow_stack_enabled: bool, ibt_enabled: bool) -> Self {
        Self {
            shadow_stack_enabled,
            ibt_enabled,
            shadow_stack: Vec::with_capacity(1024),
            shadow_stack_pointer: 0,
            control_protection_faults: 0,
        }
    }

    /// Emulate CALL instruction: push return address onto both normal stack and shadow stack
    pub fn on_call_instruction(&mut self, ret_addr: u64) {
        if self.shadow_stack_enabled {
            self.shadow_stack.push(ShadowStackFrame {
                return_addr: ret_addr,
            });
            self.shadow_stack_pointer = self.shadow_stack.len();
        }
    }

    /// Emulate RET instruction: pop return address from shadow stack and verify against regular stack
    pub fn on_ret_instruction(&mut self, regular_ret_addr: u64) -> Result<(), &'static str> {
        if !self.shadow_stack_enabled {
            return Ok(());
        }

        if let Some(frame) = self.shadow_stack.pop() {
            self.shadow_stack_pointer = self.shadow_stack.len();
            if frame.return_addr == regular_ret_addr {
                Ok(())
            } else {
                self.control_protection_faults += 1;
                Err("#CP Fault: Control Protection exception (Shadow stack return address mismatch / ROP attack)")
            }
        } else {
            self.control_protection_faults += 1;
            Err("#CP Fault: Shadow stack underflow")
        }
    }

    /// Emulate indirect jump/call target check: must land on an ENDBR64 instruction
    pub fn verify_indirect_branch_target(
        &mut self,
        target_code: &[u8],
    ) -> Result<(), &'static str> {
        if !self.ibt_enabled {
            return Ok(());
        }

        if target_code.len() >= 4 && &target_code[0..4] == ENDBR64_OPCODE {
            Ok(())
        } else {
            self.control_protection_faults += 1;
            Err("#CP Fault: Missing ENDBR64 instruction at indirect branch target")
        }
    }
}

impl Default for IntelCetEngine {
    fn default() -> Self {
        Self::new(true, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_cet_shadow_stack_and_ibt() {
        let mut cet = IntelCetEngine::new(true, true);

        // Valid call and return sequence
        cet.on_call_instruction(0x401050);
        assert!(cet.on_ret_instruction(0x401050).is_ok());

        // ROP stack-smashing attack simulation
        cet.on_call_instruction(0x401050);
        // Attacker overwrote stack return address with gadget 0x409999
        assert!(cet.on_ret_instruction(0x409999).is_err());
        assert_eq!(cet.control_protection_faults, 1);

        // IBT verification
        let valid_target = [0xF3, 0x0F, 0x1E, 0xFA, 0x90, 0xC3]; // ENDBR64, NOP, RET
        assert!(cet.verify_indirect_branch_target(&valid_target).is_ok());

        let invalid_target = [0x55, 0x48, 0x89, 0xE5]; // PUSH RBP; MOV RBP, RSP without ENDBR64
        assert!(cet.verify_indirect_branch_target(&invalid_target).is_err());
        assert_eq!(cet.control_protection_faults, 2);
    }
}
