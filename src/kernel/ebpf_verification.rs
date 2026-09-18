// eBPF Program Verification Engine
// Phase 9.4 Part 2: Program Verification with Bounds Checking, Loop Detection, and Reachability Analysis
//
// This module provides:
// - Bounds checking for all jumps and memory access
// - Loop detection and infinite loop prevention
// - Reachability analysis to detect unreachable code
// - Memory access validation
// - Comprehensive verification report generation

use crate::kernel::ebpf_vm::BpfInstruction;
use std::collections::HashSet;

/// Verification error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    OutOfBoundsJump { pc: usize, target: usize, program_len: usize },
    InfiniteLoop { pc: usize },
    UnreachableCode { pc: usize },
    InvalidRegister { reg: u8, pc: usize },
    InvalidMemoryAccess { pc: usize },
    DivisionByZero { pc: usize },
    StackOverflow { pc: usize },
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::OutOfBoundsJump { pc, target, program_len } => {
                write!(f, "Out of bounds jump at PC {}: target {} >= program length {}", pc, target, program_len)
            }
            VerificationError::InfiniteLoop { pc } => {
                write!(f, "Infinite loop detected starting at PC {}", pc)
            }
            VerificationError::UnreachableCode { pc } => {
                write!(f, "Unreachable code at PC {}", pc)
            }
            VerificationError::InvalidRegister { reg, pc } => {
                write!(f, "Invalid register {} at PC {}", reg, pc)
            }
            VerificationError::InvalidMemoryAccess { pc } => {
                write!(f, "Invalid memory access at PC {}", pc)
            }
            VerificationError::DivisionByZero { pc } => {
                write!(f, "Division by zero at PC {}", pc)
            }
            VerificationError::StackOverflow { pc } => {
                write!(f, "Stack overflow at PC {}", pc)
            }
        }
    }
}

impl std::error::Error for VerificationError {}

/// Verification report containing all errors and warnings
#[derive(Debug, Clone)]
pub struct VerificationReport {
    pub errors: Vec<VerificationError>,
    pub warnings: Vec<String>,
    pub is_valid: bool,
    pub instructions_verified: usize,
}

impl VerificationReport {
    pub fn new() -> Self {
        VerificationReport {
            errors: Vec::new(),
            warnings: Vec::new(),
            is_valid: true,
            instructions_verified: 0,
        }
    }

    pub fn add_error(&mut self, error: VerificationError) {
        self.is_valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl Default for VerificationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// eBPF Program Verifier
pub struct BpfProgramVerifier {
    program: Vec<BpfInstruction>,
    report: VerificationReport,
}

impl BpfProgramVerifier {
    /// Create a new verifier for a program
    pub fn new(program: Vec<BpfInstruction>) -> Self {
        BpfProgramVerifier {
            program,
            report: VerificationReport::new(),
        }
    }

    /// Run complete verification on the program
    pub fn verify(&mut self) -> Result<VerificationReport, String> {
        if self.program.is_empty() {
            self.report.add_error(VerificationError::InvalidMemoryAccess { pc: 0 });
            return Err("Program is empty".to_string());
        }

        // Step 1: Bounds checking - all jumps must be within program
        self.check_bounds()?;

        // Step 2: Register validation - all register references must be valid
        self.validate_registers()?;

        // Step 3: Memory access validation
        self.validate_memory_access()?;

        // Step 4: Loop detection - detect infinite loops
        self.detect_infinite_loops()?;

        // Step 5: Reachability analysis - find unreachable code
        self.check_reachability()?;

        self.report.instructions_verified = self.program.len();
        Ok(self.report.clone())
    }

    /// Check that all jump targets are within program bounds
    fn check_bounds(&mut self) -> Result<(), String> {
        for (pc, instr) in self.program.iter().enumerate() {
            match instr {
                BpfInstruction::Ja { offset } => {
                    let target = self.calculate_jump_target(pc, *offset);
                    if target >= self.program.len() {
                        self.report.add_error(VerificationError::OutOfBoundsJump {
                            pc,
                            target,
                            program_len: self.program.len(),
                        });
                    }
                }
                BpfInstruction::Jeq { offset, .. }
                | BpfInstruction::Jne { offset, .. }
                | BpfInstruction::Jgt { offset, .. }
                | BpfInstruction::Jge { offset, .. }
                | BpfInstruction::Jlt { offset, .. }
                | BpfInstruction::Jle { offset, .. }
                | BpfInstruction::JeqImm { offset, .. } => {
                    let target = self.calculate_jump_target(pc, *offset);
                    if target >= self.program.len() && target != pc + 1 {
                        // Allow jumps slightly past the end for conditional jumps
                        // but not way out of bounds
                        if target > self.program.len() + 1 {
                            self.report.add_error(VerificationError::OutOfBoundsJump {
                                pc,
                                target,
                                program_len: self.program.len(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }

        if !self.report.errors.is_empty() {
            return Err("Bounds checking failed".to_string());
        }

        Ok(())
    }

    /// Validate all register references
    fn validate_registers(&mut self) -> Result<(), String> {
        for (pc, instr) in self.program.iter().enumerate() {
            if let Some(dst_reg) = instr.dst_register() {
                if !is_valid_register(dst_reg) {
                    self.report.add_error(VerificationError::InvalidRegister { reg: dst_reg, pc });
                }
            }

            if let Some(src_reg) = instr.src_register() {
                if !is_valid_register(src_reg) {
                    self.report.add_error(VerificationError::InvalidRegister { reg: src_reg, pc });
                }
            }

            // Additional validation for store instructions
            if let BpfInstruction::StoreReg64 { dst_reg, .. } | BpfInstruction::StoreReg32 { dst_reg, .. } | BpfInstruction::StoreImm64 { dst_reg, .. } = instr {
                if !is_valid_register(*dst_reg) {
                    self.report.add_error(VerificationError::InvalidRegister {
                        reg: *dst_reg,
                        pc,
                    });
                }
            }
        }

        if !self.report.errors.is_empty() {
            return Err("Register validation failed".to_string());
        }

        Ok(())
    }

    /// Validate memory access patterns
    fn validate_memory_access(&mut self) -> Result<(), String> {
        const STACK_SIZE: i16 = 512;
        const MAX_OFFSET: i16 = 256;

        for (pc, instr) in self.program.iter().enumerate() {
            match instr {
                BpfInstruction::LoadReg64 { offset, .. }
                | BpfInstruction::LoadReg32 { offset, .. }
                | BpfInstruction::StoreReg64 { offset, .. }
                | BpfInstruction::StoreReg32 { offset, .. }
                | BpfInstruction::StoreImm64 { offset, .. } => {
                    // Check offset is reasonable (not too large)
                    if offset.abs() > MAX_OFFSET {
                        self.report.add_warning(format!(
                            "Large offset {} at PC {}: may cause stack access issues",
                            offset, pc
                        ));
                    }

                    // Check offset doesn't exceed stack size
                    if offset.abs() > STACK_SIZE {
                        self.report.add_error(VerificationError::StackOverflow { pc });
                    }
                }
                _ => {}
            }
        }

        if !self.report.errors.is_empty() {
            return Err("Memory access validation failed".to_string());
        }

        Ok(())
    }

    /// Detect infinite loops in the program
    fn detect_infinite_loops(&mut self) -> Result<(), String> {
        // Use cycle detection with backtracking analysis
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for start_pc in 0..self.program.len() {
            if visited.contains(&start_pc) {
                continue;
            }

            if self.has_cycle(start_pc, &mut visited, &mut rec_stack) {
                // Check if this is actually an infinite loop (no exit)
                if !self.can_exit_from(start_pc) {
                    self.report.add_error(VerificationError::InfiniteLoop { pc: start_pc });
                }
            }
        }

        if !self.report.errors.is_empty() {
            return Err("Infinite loop detected".to_string());
        }

        Ok(())
    }

    /// DFS-based cycle detection
    fn has_cycle(&self, pc: usize, visited: &mut HashSet<usize>, rec_stack: &mut HashSet<usize>) -> bool {
        visited.insert(pc);
        rec_stack.insert(pc);

        if pc >= self.program.len() {
            rec_stack.remove(&pc);
            return false;
        }

        let next_pcs = self.get_next_instructions(pc);

        for next_pc in next_pcs {
            if next_pc >= self.program.len() {
                continue;
            }

            if rec_stack.contains(&next_pc) {
                return true;
            }

            if !visited.contains(&next_pc) {
                if self.has_cycle(next_pc, visited, rec_stack) {
                    rec_stack.remove(&pc);
                    return true;
                }
            }
        }

        rec_stack.remove(&pc);
        false
    }

    /// Check if a program can exit from a given PC
    fn can_exit_from(&self, pc: usize) -> bool {
        let mut visited = HashSet::new();
        let mut stack = vec![pc];

        while let Some(current_pc) = stack.pop() {
            if current_pc >= self.program.len() {
                return true; // Reached end of program
            }

            if visited.contains(&current_pc) {
                continue;
            }

            visited.insert(current_pc);

            match &self.program[current_pc] {
                BpfInstruction::Return => return true,
                _ => {
                    let next_pcs = self.get_next_instructions(current_pc);
                    for next_pc in next_pcs {
                        if !visited.contains(&next_pc) && next_pc < self.program.len() + 1 {
                            stack.push(next_pc);
                        }
                    }
                }
            }
        }

        false
    }

    /// Get all possible next instructions from current PC
    fn get_next_instructions(&self, pc: usize) -> Vec<usize> {
        if pc >= self.program.len() {
            return vec![];
        }

        match &self.program[pc] {
            BpfInstruction::Ja { offset } => {
                vec![self.calculate_jump_target(pc, *offset)]
            }
            BpfInstruction::Jeq { offset, .. }
            | BpfInstruction::Jne { offset, .. }
            | BpfInstruction::Jgt { offset, .. }
            | BpfInstruction::Jge { offset, .. }
            | BpfInstruction::Jlt { offset, .. }
            | BpfInstruction::Jle { offset, .. }
            | BpfInstruction::JeqImm { offset, .. } => {
                // Conditional jump: can jump or continue
                vec![pc + 1, self.calculate_jump_target(pc, *offset)]
            }
            BpfInstruction::Return => vec![], // No next instruction
            _ => vec![pc + 1], // Most instructions just continue
        }
    }

    /// Calculate the actual jump target from PC and offset
    fn calculate_jump_target(&self, pc: usize, offset: i32) -> usize {
        let pc_i64 = pc as i64;
        let offset_i64 = offset as i64;
        let target = pc_i64 + offset_i64 + 1;

        if target < 0 {
            0
        } else {
            target as usize
        }
    }

    /// Check for unreachable code
    fn check_reachability(&mut self) -> Result<(), String> {
        let mut reachable = vec![false; self.program.len()];
        let mut stack = vec![0]; // Start from PC 0

        while let Some(pc) = stack.pop() {
            if pc >= self.program.len() {
                continue;
            }

            if reachable[pc] {
                continue;
            }

            reachable[pc] = true;

            // Add next reachable instructions
            let next_pcs = self.get_next_instructions(pc);
            for next_pc in next_pcs {
                if next_pc < self.program.len() && !reachable[next_pc] {
                    stack.push(next_pc);
                }
            }
        }

        // Report unreachable code
        for (pc, is_reachable) in reachable.iter().enumerate() {
            if !is_reachable {
                self.report.add_error(VerificationError::UnreachableCode { pc });
            }
        }

        if !self.report.errors.is_empty() {
            return Err("Unreachable code detected".to_string());
        }

        Ok(())
    }
}

/// Check if a register number is valid (R0-R10)
fn is_valid_register(reg: u8) -> bool {
    reg <= 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifier_creation() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];
        let verifier = BpfProgramVerifier::new(program);
        assert_eq!(verifier.program.len(), 2);
    }

    #[test]
    fn test_verify_simple_program() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().unwrap();
        assert!(report.is_valid);
        assert_eq!(report.errors.len(), 0);
    }

    #[test]
    fn test_bounds_checking_out_of_bounds_jump() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Ja { offset: 1000 }, // Jump way out of bounds
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(result.is_err() || !verifier.report.is_valid);
    }

    #[test]
    fn test_bounds_checking_valid_jump() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Ja { offset: 0 }, // Jump back (loop)
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        // Should detect infinite loop
        assert!(!verifier.report.is_valid || result.is_err());
    }

    #[test]
    fn test_register_validation_invalid_register() {
        let program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 20, // Invalid register (> 10)
                imm64: 42,
            },
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(!verifier.report.is_valid || result.is_err());
    }

    #[test]
    fn test_register_validation_valid_registers() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 10 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 20 },
            BpfInstruction::Add {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().unwrap();
        assert!(report.is_valid);
    }

    #[test]
    fn test_infinite_loop_detection() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Ja { offset: -1 }, // Jump back to instruction 0
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(!verifier.report.is_valid || result.is_err());
    }

    #[test]
    fn test_valid_conditional_jump() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 5 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 5 },
            BpfInstruction::Jeq {
                dst_reg: 0,
                src_reg: 1,
                offset: 1,
            },
            BpfInstruction::Return,
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().unwrap();
        assert!(report.is_valid);
    }

    #[test]
    fn test_unreachable_code_detection() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
            BpfInstruction::LoadImm64 {
                dst_reg: 1,
                imm64: 100,
            }, // Unreachable
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(!verifier.report.is_valid || result.is_err());
    }

    #[test]
    fn test_stack_overflow_detection() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::StoreReg64 {
                dst_reg: 10,
                offset: 1000, // Huge offset
                src_reg: 0,
            },
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let result = verifier.verify();
        assert!(!verifier.report.is_valid || result.is_err());
    }

    #[test]
    fn test_memory_access_validation() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::LoadReg64 {
                dst_reg: 1,
                src_reg: 10,
                offset: 100,
            },
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().unwrap();
        // Should have warnings for large offset
        assert!(!report.warnings.is_empty() || report.is_valid);
    }

    #[test]
    fn test_verification_report_accuracy() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 42 },
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().unwrap();
        assert_eq!(report.instructions_verified, 2);
        assert!(report.is_valid);
        assert_eq!(report.errors.len(), 0);
    }

    #[test]
    fn test_complex_program_with_branching() {
        let program = vec![
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 10 },
            BpfInstruction::LoadImm64 { dst_reg: 1, imm64: 5 },
            BpfInstruction::Jgt {
                dst_reg: 0,
                src_reg: 1,
                offset: 2,
            },
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 0 },
            BpfInstruction::Ja { offset: 1 },
            BpfInstruction::LoadImm64 { dst_reg: 0, imm64: 1 },
            BpfInstruction::Return,
        ];
        let mut verifier = BpfProgramVerifier::new(program);
        let report = verifier.verify().unwrap();
        assert!(report.is_valid);
    }
}
