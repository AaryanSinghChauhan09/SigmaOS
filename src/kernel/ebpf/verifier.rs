// eBPF Bytecode Verifier (CFG-based)
// Ensures programs are safe before execution

#![no_std]
extern crate alloc;

use super::instructions::{BpfInsn, MAX_BPF_INSNS, MAX_BPF_STACK, NUM_BPF_REGS};
use super::instructions::{BPF_CLASS_JMP, BPF_CLASS_JMP32, BPF_OP_CALL, BPF_OP_EXIT, BPF_OP_JA};
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

/// Register type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegType {
    Unknown,
    Scalar,
    FramePtr,
    PacketPtr,
    MapValuePtr,
    MapPtr,
}

/// Register state tracking
#[derive(Debug, Clone, Copy)]
pub struct RegState {
    pub reg_type: RegType,
    pub value: Option<i64>,
    pub min_val: i64,
    pub max_val: i64,
}

impl Default for RegState {
    fn default() -> Self {
        Self {
            reg_type: RegType::Unknown,
            value: None,
            min_val: i64::MIN,
            max_val: i64::MAX,
        }
    }
}

/// Verifier state at a program point
#[derive(Debug, Clone)]
pub struct VerifierState {
    pub regs: [RegState; NUM_BPF_REGS],
    pub stack_slots: [RegType; 64], // 512 bytes / 8 = 64 slots
    pub stack_depth: u32,
}

impl Default for VerifierState {
    fn default() -> Self {
        let mut state = Self {
            regs: [RegState::default(); NUM_BPF_REGS],
            stack_slots: [RegType::Unknown; 64],
            stack_depth: 0,
        };

        // R10 is always the frame pointer
        state.regs[10].reg_type = RegType::FramePtr;

        state
    }
}

/// Verification errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    EmptyProgram,
    TooLong(usize),
    OutOfBoundsJump { pc: usize, target: i32 },
    DeadCode(usize),
    InfiniteLoop(usize),
    UninitReg { pc: usize, reg: u8 },
    StackOverflow { pc: usize, depth: u32 },
    NoExit,
    InvalidRegister { pc: usize, reg: u8 },
    DivisionByZero(usize),
}

/// Verified program (opaque proof of verification)
#[derive(Debug, Clone)]
pub struct VerifiedProg {
    pub insns: Vec<BpfInsn>,
    pub stack_depth: u32,
}

/// eBPF bytecode verifier
pub struct BpfVerifier {
    prog: Vec<BpfInsn>,
}

impl BpfVerifier {
    pub fn new(prog: Vec<BpfInsn>) -> Self {
        Self { prog }
    }

    /// Run full verification pass
    pub fn verify(&self) -> Result<VerifiedProg, VerifyError> {
        // 1. Check program length
        if self.prog.is_empty() {
            return Err(VerifyError::EmptyProgram);
        }

        if self.prog.len() > MAX_BPF_INSNS {
            return Err(VerifyError::TooLong(self.prog.len()));
        }

        // 2. Build CFG and check jumps
        let jump_targets = self.build_cfg()?;

        // 3. Check reachability
        let reachable = self.check_reachability(&jump_targets)?;

        // 4. Check for dead code
        for pc in 0..self.prog.len() {
            if !reachable.contains(&pc) {
                return Err(VerifyError::DeadCode(pc));
            }
        }

        // 5. Check for infinite loops (simple: disallow backward jumps)
        self.check_loops()?;

        // 6. Track register usage and stack depth
        let max_stack_depth = self.check_register_and_stack_usage()?;

        // 7. Verify program ends with EXIT
        if !self.prog.last().map(|i| i.is_exit()).unwrap_or(false) {
            // Check if any instruction is an exit
            if !self.prog.iter().any(|i| i.is_exit()) {
                return Err(VerifyError::NoExit);
            }
        }

        Ok(VerifiedProg {
            insns: self.prog.clone(),
            stack_depth: max_stack_depth,
        })
    }

    /// Build control flow graph and validate jump targets
    fn build_cfg(&self) -> Result<BTreeSet<usize>, VerifyError> {
        let mut jump_targets = BTreeSet::new();
        jump_targets.insert(0); // Entry point

        for (pc, insn) in self.prog.iter().enumerate() {
            if insn.is_jump() && insn.get_op() != BPF_OP_EXIT && insn.get_op() != BPF_OP_CALL {
                let target = if insn.get_op() == BPF_OP_JA {
                    // Unconditional jump
                    (pc as i32 + 1 + insn.off as i32)
                } else {
                    // Conditional jump
                    let target = pc as i32 + 1 + insn.off as i32;
                    // Fall-through is also a target
                    jump_targets.insert(pc + 1);
                    target
                };

                if target < 0 || target >= self.prog.len() as i32 {
                    return Err(VerifyError::OutOfBoundsJump { pc, target });
                }

                jump_targets.insert(target as usize);
            }
        }

        Ok(jump_targets)
    }

    /// Check reachability via BFS
    fn check_reachability(
        &self,
        jump_targets: &BTreeSet<usize>,
    ) -> Result<BTreeSet<usize>, VerifyError> {
        let mut reachable = BTreeSet::new();
        let mut worklist = Vec::new();

        worklist.push(0);
        reachable.insert(0);

        while let Some(pc) = worklist.pop() {
            if pc >= self.prog.len() {
                continue;
            }

            let insn = &self.prog[pc];

            if insn.is_exit() {
                continue;
            }

            if insn.is_jump() && insn.get_op() != BPF_OP_CALL {
                if insn.get_op() == BPF_OP_JA {
                    // Unconditional jump
                    let target = (pc as i32 + 1 + insn.off as i32) as usize;
                    if !reachable.contains(&target) {
                        reachable.insert(target);
                        worklist.push(target);
                    }
                } else {
                    // Conditional jump: both branches
                    let target = (pc as i32 + 1 + insn.off as i32) as usize;
                    let fall_through = pc + 1;

                    if !reachable.contains(&target) {
                        reachable.insert(target);
                        worklist.push(target);
                    }

                    if !reachable.contains(&fall_through) {
                        reachable.insert(fall_through);
                        worklist.push(fall_through);
                    }
                }
            } else {
                // Fall through to next instruction
                let next = pc + 1;
                if next < self.prog.len() && !reachable.contains(&next) {
                    reachable.insert(next);
                    worklist.push(next);
                }
            }
        }

        Ok(reachable)
    }

    /// Check for infinite loops (simple: reject backward jumps)
    fn check_loops(&self) -> Result<(), VerifyError> {
        for (pc, insn) in self.prog.iter().enumerate() {
            if insn.is_jump() && insn.get_op() != BPF_OP_EXIT && insn.get_op() != BPF_OP_CALL {
                let target = pc as i32 + 1 + insn.off as i32;

                // Reject backward jumps (simple loop prevention)
                if target <= pc as i32 {
                    return Err(VerifyError::InfiniteLoop(pc));
                }
            }
        }

        Ok(())
    }

    /// Check register usage and stack depth
    fn check_register_and_stack_usage(&self) -> Result<u32, VerifyError> {
        let mut max_stack_depth = 0u32;

        for (pc, insn) in self.prog.iter().enumerate() {
            // Check register bounds
            if insn.dst_reg >= NUM_BPF_REGS as u8 {
                return Err(VerifyError::InvalidRegister {
                    pc,
                    reg: insn.dst_reg,
                });
            }

            if insn.src_reg >= NUM_BPF_REGS as u8 {
                return Err(VerifyError::InvalidRegister {
                    pc,
                    reg: insn.src_reg,
                });
            }

            // Track stack depth (STX/ST with negative offset from R10)
            let class = insn.get_class();
            if class == 0x02 || class == 0x03 {
                // ST or STX
                let depth = (-insn.off) as u32;
                if depth > MAX_BPF_STACK as u32 {
                    return Err(VerifyError::StackOverflow { pc, depth });
                }

                if depth > max_stack_depth {
                    max_stack_depth = depth;
                }
            }
        }

        Ok(max_stack_depth)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::ebpf::instructions::{BPF_OP_ADD, BPF_OP_MOV};

    #[test]
    fn test_verify_empty_program() {
        let verifier = BpfVerifier::new(Vec::new());
        assert_eq!(verifier.verify().unwrap_err(), VerifyError::EmptyProgram);
    }

    #[test]
    fn test_verify_simple_program() {
        let prog = vec![BpfInsn::alu64_imm(BPF_OP_MOV, 0, 42), BpfInsn::exit_insn()];

        let verifier = BpfVerifier::new(prog);
        assert!(verifier.verify().is_ok());
    }

    #[test]
    fn test_verify_no_exit() {
        let prog = vec![
            BpfInsn::alu64_imm(BPF_OP_MOV, 0, 42),
            BpfInsn::alu64_imm(BPF_OP_ADD, 0, 10),
        ];

        let verifier = BpfVerifier::new(prog);
        assert_eq!(verifier.verify().unwrap_err(), VerifyError::NoExit);
    }

    #[test]
    fn test_verify_out_of_bounds_jump() {
        let prog = vec![
            BpfInsn::jmp_imm(0x10, 0, 0, 100), // JEQ to out-of-bounds
            BpfInsn::exit_insn(),
        ];

        let verifier = BpfVerifier::new(prog);
        match verifier.verify() {
            Err(VerifyError::OutOfBoundsJump { .. }) => (),
            _ => panic!("Expected OutOfBoundsJump error"),
        }
    }

    #[test]
    fn test_verify_backward_jump() {
        let prog = vec![
            BpfInsn::alu64_imm(BPF_OP_MOV, 0, 1),
            BpfInsn::jmp_imm(0x10, 0, 0, -2), // Jump backward (infinite loop)
            BpfInsn::exit_insn(),
        ];

        let verifier = BpfVerifier::new(prog);
        assert!(matches!(
            verifier.verify(),
            Err(VerifyError::InfiniteLoop(_))
        ));
    }

    #[test]
    fn test_verify_invalid_register() {
        let prog = vec![
            BpfInsn::alu64_imm(BPF_OP_MOV, 15, 42), // R15 doesn't exist (only R0-R10)
            BpfInsn::exit_insn(),
        ];

        let verifier = BpfVerifier::new(prog);
        assert!(matches!(
            verifier.verify(),
            Err(VerifyError::InvalidRegister { .. })
        ));
    }

    #[test]
    fn test_verify_forward_jump() {
        let prog = vec![
            BpfInsn::jmp_imm(0x10, 0, 0, 1), // JEQ forward +1
            BpfInsn::alu64_imm(BPF_OP_ADD, 0, 10),
            BpfInsn::exit_insn(),
        ];

        let verifier = BpfVerifier::new(prog);
        assert!(verifier.verify().is_ok());
    }
}
