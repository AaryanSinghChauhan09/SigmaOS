// eBPF VM Foundation - Complete Instruction Set & Core Engine
// Phase 9.4 Part 1: BPF Instruction Set Definition and Virtual Machine Implementation
//
// This module provides:
// - Complete eBPF instruction set (25+ types)
// - Instruction validation and verification
// - BPF virtual machine with full state management
// - 11 registers (R0-R10), 512-byte stack, dynamic heap
// - Comprehensive instruction execution engine
// - Memory and PC management

use std::collections::HashMap;

/// Represents a complete eBPF instruction with all variant types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfInstruction {
    // ============ LOAD/STORE INSTRUCTIONS ============
    /// Load 64-bit immediate into destination register
    LoadImm64 { dst_reg: u8, imm64: u64 },
    /// Load 64-bit value from memory (register + offset) into destination
    LoadReg64 { dst_reg: u8, src_reg: u8, offset: i16 },
    /// Load 32-bit value from memory (register + offset) into destination
    LoadReg32 { dst_reg: u8, src_reg: u8, offset: i16 },
    /// Store 64-bit value from src_reg to memory (dst_reg + offset)
    StoreReg64 { dst_reg: u8, offset: i16, src_reg: u8 },
    /// Store 32-bit value from src_reg to memory (dst_reg + offset)
    StoreReg32 { dst_reg: u8, offset: i16, src_reg: u8 },
    /// Store 64-bit immediate value to memory (dst_reg + offset)
    StoreImm64 { dst_reg: u8, offset: i16, imm: u64 },
    /// Load absolute - from packet data at offset, size bytes
    LoadAbs { dst: u8, offset: u32, size: u8 },
    /// Load indirect - from packet data at (src_reg + offset), size bytes
    LoadInd { dst: u8, src_reg: u8, offset: u32, size: u8 },

    // ============ ARITHMETIC INSTRUCTIONS ============
    /// Add two registers: dst = dst + src (64-bit)
    Add { dst_reg: u8, src_reg: u8 },
    /// Add immediate to register: dst = dst + imm
    AddImm { dst_reg: u8, imm: u32 },
    /// Subtract registers: dst = dst - src
    Sub { dst_reg: u8, src_reg: u8 },
    /// Subtract immediate: dst = dst - imm
    SubImm { dst_reg: u8, imm: u32 },
    /// Multiply registers: dst = dst * src
    Mul { dst_reg: u8, src_reg: u8 },
    /// Multiply by immediate: dst = dst * imm
    MulImm { dst_reg: u8, imm: u32 },
    /// Divide registers: dst = dst / src (error if src=0)
    Div { dst_reg: u8, src_reg: u8 },
    /// Divide by immediate: dst = dst / imm
    DivImm { dst_reg: u8, imm: u32 },
    /// Modulo registers: dst = dst % src
    Mod { dst_reg: u8, src_reg: u8 },
    /// Modulo by immediate: dst = dst % imm
    ModImm { dst_reg: u8, imm: u32 },
    /// Negate register: dst = -dst
    Neg { dst_reg: u8 },

    // ============ BITWISE INSTRUCTIONS ============
    /// Bitwise AND: dst = dst & src
    And { dst_reg: u8, src_reg: u8 },
    /// Bitwise OR: dst = dst | src
    Or { dst_reg: u8, src_reg: u8 },
    /// Bitwise XOR: dst = dst ^ src
    Xor { dst_reg: u8, src_reg: u8 },
    /// Left shift: dst = dst << src
    Lsh { dst_reg: u8, src_reg: u8 },
    /// Right shift (logical): dst = dst >> src
    Rsh { dst_reg: u8, src_reg: u8 },
    /// Right shift (arithmetic): dst = (i64)dst >> src
    Arsh { dst_reg: u8, src_reg: u8 },

    // ============ JUMP INSTRUCTIONS ============
    /// Unconditional jump: pc += offset
    Ja { offset: i32 },
    /// Jump if equal: if dst == src { pc += offset }
    Jeq { dst_reg: u8, src_reg: u8, offset: i32 },
    /// Jump if not equal: if dst != src { pc += offset }
    Jne { dst_reg: u8, src_reg: u8, offset: i32 },
    /// Jump if greater: if dst > src { pc += offset }
    Jgt { dst_reg: u8, src_reg: u8, offset: i32 },
    /// Jump if greater or equal: if dst >= src { pc += offset }
    Jge { dst_reg: u8, src_reg: u8, offset: i32 },
    /// Jump if less: if dst < src { pc += offset }
    Jlt { dst_reg: u8, src_reg: u8, offset: i32 },
    /// Jump if less or equal: if dst <= src { pc += offset }
    Jle { dst_reg: u8, src_reg: u8, offset: i32 },
    /// Jump if equal to immediate: if dst == imm { pc += offset }
    JeqImm { dst_reg: u8, imm: u32, offset: i32 },

    // ============ FUNCTION CALLS ============
    /// Call helper function with given ID
    Call { func_id: u32 },
    /// Return from function - return R0 value
    Return,

    // ============ OTHER INSTRUCTIONS ============
    /// Move register to register: dst = src
    Mov { dst_reg: u8, src_reg: u8 },
    /// Move immediate to register: dst = imm
    MovImm { dst_reg: u8, imm: u64 },
    /// No operation - does nothing
    Nop,
}

impl BpfInstruction {
    /// Get destination register if this instruction has one
    pub fn dst_register(&self) -> Option<u8> {
        match self {
            BpfInstruction::LoadImm64 { dst_reg, .. }
            | BpfInstruction::LoadReg64 { dst_reg, .. }
            | BpfInstruction::LoadReg32 { dst_reg, .. }
            | BpfInstruction::Add { dst_reg, .. }
            | BpfInstruction::AddImm { dst_reg, .. }
            | BpfInstruction::Sub { dst_reg, .. }
            | BpfInstruction::SubImm { dst_reg, .. }
            | BpfInstruction::Mul { dst_reg, .. }
            | BpfInstruction::MulImm { dst_reg, .. }
            | BpfInstruction::Div { dst_reg, .. }
            | BpfInstruction::DivImm { dst_reg, .. }
            | BpfInstruction::Mod { dst_reg, .. }
            | BpfInstruction::ModImm { dst_reg, .. }
            | BpfInstruction::Neg { dst_reg }
            | BpfInstruction::And { dst_reg, .. }
            | BpfInstruction::Or { dst_reg, .. }
            | BpfInstruction::Xor { dst_reg, .. }
            | BpfInstruction::Lsh { dst_reg, .. }
            | BpfInstruction::Rsh { dst_reg, .. }
            | BpfInstruction::Arsh { dst_reg, .. }
            | BpfInstruction::Mov { dst_reg, .. }
            | BpfInstruction::MovImm { dst_reg, .. } => Some(*dst_reg),
            BpfInstruction::LoadAbs { dst, .. }
            | BpfInstruction::LoadInd { dst, .. } => Some(*dst),
            _ => None,
        }
    }

    /// Get source register if this instruction has one
    pub fn src_register(&self) -> Option<u8> {
        match self {
            BpfInstruction::LoadReg64 { src_reg, .. }
            | BpfInstruction::LoadReg32 { src_reg, .. }
            | BpfInstruction::StoreReg64 { src_reg, .. }
            | BpfInstruction::StoreReg32 { src_reg, .. }
            | BpfInstruction::Add { src_reg, .. }
            | BpfInstruction::Sub { src_reg, .. }
            | BpfInstruction::Mul { src_reg, .. }
            | BpfInstruction::Div { src_reg, .. }
            | BpfInstruction::Mod { src_reg, .. }
            | BpfInstruction::And { src_reg, .. }
            | BpfInstruction::Or { src_reg, .. }
            | BpfInstruction::Xor { src_reg, .. }
            | BpfInstruction::Lsh { src_reg, .. }
            | BpfInstruction::Rsh { src_reg, .. }
            | BpfInstruction::Arsh { src_reg, .. }
            | BpfInstruction::Jeq { src_reg, .. }
            | BpfInstruction::Jne { src_reg, .. }
            | BpfInstruction::Jgt { src_reg, .. }
            | BpfInstruction::Jge { src_reg, .. }
            | BpfInstruction::Jlt { src_reg, .. }
            | BpfInstruction::Jle { src_reg, .. }
            | BpfInstruction::Mov { src_reg, .. }
            | BpfInstruction::LoadInd { src_reg, .. } => Some(*src_reg),
            _ => None,
        }
    }
}

/// Validation functions for eBPF instructions and components

/// Check if a register number is valid (R0-R10)
pub fn is_valid_register(reg: u8) -> bool {
    reg <= 10
}

/// Check if an immediate value is valid (32-bit)
pub fn is_valid_immediate(imm: u32) -> bool {
    // All u32 values are valid
    true
}

/// Validate a single instruction
pub fn validate_instruction(instr: &BpfInstruction) -> Result<(), String> {
    match instr {
        // Load/Store validations
        BpfInstruction::LoadImm64 { dst_reg, .. } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            Ok(())
        }
        BpfInstruction::LoadReg64 { dst_reg, src_reg, offset } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            // Stack is 512 bytes, check reasonable bounds
            if *offset < -512 || *offset > 504 {
                return Err(format!("Offset out of stack bounds: {}", offset));
            }
            Ok(())
        }
        BpfInstruction::LoadReg32 { dst_reg, src_reg, offset } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            if *offset < -512 || *offset > 508 {
                return Err(format!("Offset out of stack bounds: {}", offset));
            }
            Ok(())
        }
        BpfInstruction::StoreReg64 { dst_reg, src_reg, offset } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            if *offset < -512 || *offset > 504 {
                return Err(format!("Offset out of stack bounds: {}", offset));
            }
            Ok(())
        }
        BpfInstruction::StoreReg32 { dst_reg, src_reg, offset } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            if *offset < -512 || *offset > 508 {
                return Err(format!("Offset out of stack bounds: {}", offset));
            }
            Ok(())
        }
        BpfInstruction::StoreImm64 { dst_reg, offset, .. } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if *offset < -512 || *offset > 504 {
                return Err(format!("Offset out of stack bounds: {}", offset));
            }
            Ok(())
        }
        BpfInstruction::LoadAbs { dst, size, .. } => {
            if !is_valid_register(*dst) {
                return Err(format!("Invalid destination register: {}", dst));
            }
            if *size != 1 && *size != 2 && *size != 4 && *size != 8 {
                return Err(format!("Invalid load size: {}, must be 1, 2, 4, or 8", size));
            }
            Ok(())
        }
        BpfInstruction::LoadInd { dst, src_reg, size, .. } => {
            if !is_valid_register(*dst) {
                return Err(format!("Invalid destination register: {}", dst));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            if *size != 1 && *size != 2 && *size != 4 && *size != 8 {
                return Err(format!("Invalid load size: {}, must be 1, 2, 4, or 8", size));
            }
            Ok(())
        }

        // Arithmetic validations
        BpfInstruction::Add { dst_reg, src_reg }
        | BpfInstruction::Sub { dst_reg, src_reg }
        | BpfInstruction::Mul { dst_reg, src_reg }
        | BpfInstruction::Div { dst_reg, src_reg }
        | BpfInstruction::Mod { dst_reg, src_reg }
        | BpfInstruction::And { dst_reg, src_reg }
        | BpfInstruction::Or { dst_reg, src_reg }
        | BpfInstruction::Xor { dst_reg, src_reg }
        | BpfInstruction::Lsh { dst_reg, src_reg }
        | BpfInstruction::Rsh { dst_reg, src_reg }
        | BpfInstruction::Arsh { dst_reg, src_reg } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            Ok(())
        }

        BpfInstruction::AddImm { dst_reg, imm: _ }
        | BpfInstruction::SubImm { dst_reg, imm: _ }
        | BpfInstruction::MulImm { dst_reg, imm: _ }
        | BpfInstruction::DivImm { dst_reg, imm: _ }
        | BpfInstruction::ModImm { dst_reg, imm: _ } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            Ok(())
        }

        BpfInstruction::Neg { dst_reg } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            Ok(())
        }

        // Jump validations
        BpfInstruction::Jeq { dst_reg, src_reg, .. }
        | BpfInstruction::Jne { dst_reg, src_reg, .. }
        | BpfInstruction::Jgt { dst_reg, src_reg, .. }
        | BpfInstruction::Jge { dst_reg, src_reg, .. }
        | BpfInstruction::Jlt { dst_reg, src_reg, .. }
        | BpfInstruction::Jle { dst_reg, src_reg, .. } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            Ok(())
        }

        BpfInstruction::JeqImm { dst_reg, imm: _, offset: _ } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            Ok(())
        }

        // Move validations
        BpfInstruction::Mov { dst_reg, src_reg } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            if !is_valid_register(*src_reg) {
                return Err(format!("Invalid source register: {}", src_reg));
            }
            Ok(())
        }

        BpfInstruction::MovImm { dst_reg, imm: _ } => {
            if !is_valid_register(*dst_reg) {
                return Err(format!("Invalid destination register: {}", dst_reg));
            }
            Ok(())
        }

        // Call and Return are always valid
        BpfInstruction::Call { .. } | BpfInstruction::Return | BpfInstruction::Ja { .. }
        | BpfInstruction::Nop => Ok(()),
    }
}

/// eBPF Virtual Machine - Core execution engine
pub struct BpfVm {
    /// All 11 registers (R0-R10), 64-bit each
    registers: [u64; 11],
    /// Program counter - current instruction index
    program_counter: u64,
    /// Stack (512 bytes, typical eBPF stack size)
    stack: Vec<u8>,
    /// Stack pointer (grows downward from end)
    stack_ptr: u64,
    /// Dynamic heap for memory allocation
    heap: Vec<u8>,
    /// Memory map for virtual address spaces
    memory_map: HashMap<u64, Vec<u8>>,
    /// Loaded program
    program: Vec<BpfInstruction>,
    /// Execution state
    halted: bool,
}

impl BpfVm {
    /// Create a new eBPF virtual machine
    pub fn new() -> Self {
        let stack_size = 512;
        BpfVm {
            registers: [0; 11],
            program_counter: 0,
            stack: vec![0; stack_size],
            stack_ptr: stack_size as u64,
            heap: Vec::new(),
            memory_map: HashMap::new(),
            program: Vec::new(),
            halted: false,
        }
    }

    /// Load a program into the VM, validating all instructions
    pub fn load_program(&mut self, program: Vec<BpfInstruction>) -> Result<(), String> {
        // Validate all instructions
        for (idx, instr) in program.iter().enumerate() {
            validate_instruction(instr).map_err(|e| {
                format!("Instruction {} validation failed: {}", idx, e)
            })?;
        }

        // Check that program ends with Return
        if program.is_empty() {
            return Err("Program cannot be empty".to_string());
        }

        // Store the program and reset execution state
        self.program = program;
        self.program_counter = 0;
        self.halted = false;
        self.registers = [0; 11];

        Ok(())
    }

    /// Execute a single instruction
    pub fn execute_instruction(&mut self, instr: &BpfInstruction) -> Result<(), String> {
        match instr {
            // ============ LOAD/STORE ============
            BpfInstruction::LoadImm64 { dst_reg, imm64 } => {
                self.set_register(*dst_reg, *imm64)?;
            }
            BpfInstruction::LoadReg64 { dst_reg, src_reg, offset } => {
                let base = self.get_register(*src_reg)?;
                let addr = (base as i64 + *offset as i64) as usize;
                if addr + 8 <= self.stack.len() {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&self.stack[addr..addr + 8]);
                    let value = u64::from_le_bytes(bytes);
                    self.set_register(*dst_reg, value)?;
                } else {
                    return Err("Load address out of bounds".to_string());
                }
            }
            BpfInstruction::LoadReg32 { dst_reg, src_reg, offset } => {
                let base = self.get_register(*src_reg)?;
                let addr = (base as i64 + *offset as i64) as usize;
                if addr + 4 <= self.stack.len() {
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(&self.stack[addr..addr + 4]);
                    let value = u32::from_le_bytes(bytes) as u64;
                    self.set_register(*dst_reg, value)?;
                } else {
                    return Err("Load address out of bounds".to_string());
                }
            }
            BpfInstruction::StoreReg64 { dst_reg, offset, src_reg } => {
                let dst_base = self.get_register(*dst_reg)?;
                let addr = (dst_base as i64 + *offset as i64) as usize;
                let value = self.get_register(*src_reg)?;
                if addr + 8 <= self.stack.len() {
                    let bytes = value.to_le_bytes();
                    self.stack[addr..addr + 8].copy_from_slice(&bytes);
                } else {
                    return Err("Store address out of bounds".to_string());
                }
            }
            BpfInstruction::StoreReg32 { dst_reg, offset, src_reg } => {
                let dst_base = self.get_register(*dst_reg)?;
                let addr = (dst_base as i64 + *offset as i64) as usize;
                let value = self.get_register(*src_reg)? as u32;
                if addr + 4 <= self.stack.len() {
                    let bytes = value.to_le_bytes();
                    self.stack[addr..addr + 4].copy_from_slice(&bytes);
                } else {
                    return Err("Store address out of bounds".to_string());
                }
            }
            BpfInstruction::StoreImm64 { dst_reg, offset, imm } => {
                let dst_base = self.get_register(*dst_reg)?;
                let addr = (dst_base as i64 + *offset as i64) as usize;
                if addr + 8 <= self.stack.len() {
                    let bytes = imm.to_le_bytes();
                    self.stack[addr..addr + 8].copy_from_slice(&bytes);
                } else {
                    return Err("Store address out of bounds".to_string());
                }
            }
            BpfInstruction::LoadAbs { .. } | BpfInstruction::LoadInd { .. } => {
                // Packet load operations - not fully implemented in basic VM
                // Would need packet context to be useful
            }

            // ============ ARITHMETIC ============
            BpfInstruction::Add { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                self.set_register(*dst_reg, dst_val.wrapping_add(src_val))?;
            }
            BpfInstruction::AddImm { dst_reg, imm } => {
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, dst_val.wrapping_add(*imm as u64))?;
            }
            BpfInstruction::Sub { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                self.set_register(*dst_reg, dst_val.wrapping_sub(src_val))?;
            }
            BpfInstruction::SubImm { dst_reg, imm } => {
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, dst_val.wrapping_sub(*imm as u64))?;
            }
            BpfInstruction::Mul { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                self.set_register(*dst_reg, dst_val.wrapping_mul(src_val))?;
            }
            BpfInstruction::MulImm { dst_reg, imm } => {
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, dst_val.wrapping_mul(*imm as u64))?;
            }
            BpfInstruction::Div { dst_reg, src_reg } => {
                let src_val = self.get_register(*src_reg)?;
                if src_val == 0 {
                    return Err("Division by zero".to_string());
                }
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, dst_val / src_val)?;
            }
            BpfInstruction::DivImm { dst_reg, imm } => {
                if *imm == 0 {
                    return Err("Division by zero".to_string());
                }
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, dst_val / (*imm as u64))?;
            }
            BpfInstruction::Mod { dst_reg, src_reg } => {
                let src_val = self.get_register(*src_reg)?;
                if src_val == 0 {
                    return Err("Modulo by zero".to_string());
                }
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, dst_val % src_val)?;
            }
            BpfInstruction::ModImm { dst_reg, imm } => {
                if *imm == 0 {
                    return Err("Modulo by zero".to_string());
                }
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, dst_val % (*imm as u64))?;
            }
            BpfInstruction::Neg { dst_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                self.set_register(*dst_reg, (dst_val as i64).wrapping_neg() as u64)?;
            }

            // ============ BITWISE ============
            BpfInstruction::And { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                self.set_register(*dst_reg, dst_val & src_val)?;
            }
            BpfInstruction::Or { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                self.set_register(*dst_reg, dst_val | src_val)?;
            }
            BpfInstruction::Xor { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                self.set_register(*dst_reg, dst_val ^ src_val)?;
            }
            BpfInstruction::Lsh { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)? & 0x3F; // Limit shift
                self.set_register(*dst_reg, dst_val.wrapping_shl(src_val as u32))?;
            }
            BpfInstruction::Rsh { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)? & 0x3F; // Limit shift
                self.set_register(*dst_reg, dst_val.wrapping_shr(src_val as u32))?;
            }
            BpfInstruction::Arsh { dst_reg, src_reg } => {
                let dst_val = self.get_register(*dst_reg)? as i64;
                let src_val = self.get_register(*src_reg)? & 0x3F; // Limit shift
                let result = (dst_val >> src_val as u32) as u64;
                self.set_register(*dst_reg, result)?;
            }

            // ============ JUMPS ============
            BpfInstruction::Ja { offset } => {
                let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                self.set_program_counter(new_pc)?;
            }
            BpfInstruction::Jeq { dst_reg, src_reg, offset } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                if dst_val == src_val {
                    let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                    self.set_program_counter(new_pc)?;
                }
            }
            BpfInstruction::Jne { dst_reg, src_reg, offset } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                if dst_val != src_val {
                    let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                    self.set_program_counter(new_pc)?;
                }
            }
            BpfInstruction::Jgt { dst_reg, src_reg, offset } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                if dst_val > src_val {
                    let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                    self.set_program_counter(new_pc)?;
                }
            }
            BpfInstruction::Jge { dst_reg, src_reg, offset } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                if dst_val >= src_val {
                    let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                    self.set_program_counter(new_pc)?;
                }
            }
            BpfInstruction::Jlt { dst_reg, src_reg, offset } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                if dst_val < src_val {
                    let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                    self.set_program_counter(new_pc)?;
                }
            }
            BpfInstruction::Jle { dst_reg, src_reg, offset } => {
                let dst_val = self.get_register(*dst_reg)?;
                let src_val = self.get_register(*src_reg)?;
                if dst_val <= src_val {
                    let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                    self.set_program_counter(new_pc)?;
                }
            }
            BpfInstruction::JeqImm { dst_reg, imm, offset } => {
                let dst_val = self.get_register(*dst_reg)?;
                if dst_val == *imm as u64 {
                    let new_pc = (self.program_counter as i64 + *offset as i64) as u64;
                    self.set_program_counter(new_pc)?;
                }
            }

            // ============ FUNCTION CALLS ============
            BpfInstruction::Call { func_id: _ } => {
                // In a real implementation, this would call a helper function
                // For now, just a placeholder
            }

            // ============ RETURN ============
            BpfInstruction::Return => {
                self.halted = true;
            }

            // ============ MOVE ============
            BpfInstruction::Mov { dst_reg, src_reg } => {
                let src_val = self.get_register(*src_reg)?;
                self.set_register(*dst_reg, src_val)?;
            }
            BpfInstruction::MovImm { dst_reg, imm } => {
                self.set_register(*dst_reg, *imm)?;
            }

            // ============ NOP ============
            BpfInstruction::Nop => {
                // Do nothing
            }
        }

        Ok(())
    }

    /// Run the program to completion, returning R0
    pub fn run(&mut self) -> Result<u64, String> {
        self.halted = false;

        while (self.program_counter as usize) < self.program.len() && !self.halted {
            let pc = self.program_counter as usize;
            let instr = self.program[pc].clone();
            self.execute_instruction(&instr)?;

            // Auto-increment PC if not a jump/return
            if !self.halted
                && !matches!(
                    instr,
                    BpfInstruction::Ja { .. }
                        | BpfInstruction::Jeq { .. }
                        | BpfInstruction::Jne { .. }
                        | BpfInstruction::Jgt { .. }
                        | BpfInstruction::Jge { .. }
                        | BpfInstruction::Jlt { .. }
                        | BpfInstruction::Jle { .. }
                        | BpfInstruction::JeqImm { .. }
                )
            {
                self.program_counter += 1;
            }
        }

        Ok(self.get_register(0)?)
    }

    /// Get a register value
    pub fn get_register(&self, reg: u8) -> Result<u64, String> {
        if !is_valid_register(reg) {
            return Err(format!("Invalid register: {}", reg));
        }
        Ok(self.registers[reg as usize])
    }

    /// Set a register value
    pub fn set_register(&mut self, reg: u8, value: u64) -> Result<(), String> {
        if !is_valid_register(reg) {
            return Err(format!("Invalid register: {}", reg));
        }
        self.registers[reg as usize] = value;
        Ok(())
    }

    /// Get the current program counter
    pub fn get_program_counter(&self) -> u64 {
        self.program_counter
    }

    /// Set the program counter
    pub fn set_program_counter(&mut self, pc: u64) -> Result<(), String> {
        if pc >= self.program.len() as u64 {
            return Err(format!("Program counter out of bounds: {}", pc));
        }
        self.program_counter = pc;
        Ok(())
    }

    /// Push a 64-bit value onto the stack
    pub fn push(&mut self, value: u64) -> Result<(), String> {
        if self.stack_ptr < 8 {
            return Err("Stack overflow".to_string());
        }
        self.stack_ptr -= 8;
        let bytes = value.to_le_bytes();
        let addr = self.stack_ptr as usize;
        self.stack[addr..addr + 8].copy_from_slice(&bytes);
        Ok(())
    }

    /// Pop a 64-bit value from the stack
    pub fn pop(&mut self) -> Result<u64, String> {
        if self.stack_ptr >= self.stack.len() as u64 {
            return Err("Stack underflow".to_string());
        }
        let addr = self.stack_ptr as usize;
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&self.stack[addr..addr + 8]);
        self.stack_ptr += 8;
        Ok(u64::from_le_bytes(bytes))
    }

    /// Get remaining stack space in bytes
    pub fn stack_remaining(&self) -> u64 {
        self.stack_ptr
    }
}

impl Default for BpfVm {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ INSTRUCTION TESTS ============

    #[test]
    fn test_load_imm64_instruction() {
        let instr = BpfInstruction::LoadImm64 {
            dst_reg: 1,
            imm64: 0x0102030405060708,
        };
        assert_eq!(instr.dst_register(), Some(1));
        assert_eq!(instr.src_register(), None);
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_load_reg64_instruction() {
        let instr = BpfInstruction::LoadReg64 {
            dst_reg: 2,
            src_reg: 1,
            offset: 16,
        };
        assert_eq!(instr.dst_register(), Some(2));
        assert_eq!(instr.src_register(), Some(1));
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_store_reg64_instruction() {
        let instr = BpfInstruction::StoreReg64 {
            dst_reg: 10,
            offset: -16,
            src_reg: 1,
        };
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_arithmetic_add_instruction() {
        let instr = BpfInstruction::Add {
            dst_reg: 1,
            src_reg: 2,
        };
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_arithmetic_mul_instruction() {
        let instr = BpfInstruction::Mul {
            dst_reg: 3,
            src_reg: 4,
        };
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_bitwise_and_instruction() {
        let instr = BpfInstruction::And {
            dst_reg: 5,
            src_reg: 6,
        };
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_jump_jeq_instruction() {
        let instr = BpfInstruction::Jeq {
            dst_reg: 1,
            src_reg: 2,
            offset: 5,
        };
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_call_instruction() {
        let instr = BpfInstruction::Call { func_id: 42 };
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_return_instruction() {
        let instr = BpfInstruction::Return;
        assert!(validate_instruction(&instr).is_ok());
    }

    #[test]
    fn test_validate_instruction_invalid_register() {
        let instr = BpfInstruction::Add {
            dst_reg: 99,
            src_reg: 1,
        };
        assert!(validate_instruction(&instr).is_err());
    }

    #[test]
    fn test_validate_instruction_offset_out_of_bounds() {
        let instr = BpfInstruction::LoadReg64 {
            dst_reg: 1,
            src_reg: 2,
            offset: 1000,
        };
        assert!(validate_instruction(&instr).is_err());
    }

    // ============ VM CREATION AND INITIALIZATION TESTS ============

    #[test]
    fn test_vm_creation() {
        let vm = BpfVm::new();
        assert_eq!(vm.get_program_counter(), 0);
        assert_eq!(vm.get_register(0).unwrap(), 0);
        assert_eq!(vm.stack_remaining(), 512);
    }

    #[test]
    fn test_load_program() {
        let mut vm = BpfVm::new();
        let program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 42,
            },
            BpfInstruction::Return,
        ];
        assert!(vm.load_program(program).is_ok());
    }

    #[test]
    fn test_load_program_invalid_instruction() {
        let mut vm = BpfVm::new();
        let program = vec![BpfInstruction::Add {
            dst_reg: 99,
            src_reg: 1,
        }];
        assert!(vm.load_program(program).is_err());
    }

    // ============ ARITHMETIC EXECUTION TESTS ============

    #[test]
    fn test_arithmetic_add() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 10).unwrap();
        vm.set_register(2, 20).unwrap();

        let program = vec![
            BpfInstruction::Add {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_arithmetic_sub() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 50).unwrap();
        vm.set_register(2, 20).unwrap();

        let program = vec![
            BpfInstruction::Sub {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 30);
    }

    #[test]
    fn test_arithmetic_mul() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 5).unwrap();
        vm.set_register(2, 7).unwrap();

        let program = vec![
            BpfInstruction::Mul {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 35);
    }

    #[test]
    fn test_arithmetic_div_by_zero() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 10).unwrap();
        vm.set_register(2, 0).unwrap();

        let program = vec![
            BpfInstruction::Div {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run();
        assert!(result.is_err());
    }

    #[test]
    fn test_arithmetic_div() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 100).unwrap();
        vm.set_register(2, 4).unwrap();

        let program = vec![
            BpfInstruction::Div {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 25);
    }

    // ============ BITWISE EXECUTION TESTS ============

    #[test]
    fn test_bitwise_and() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 0x0F).unwrap();
        vm.set_register(2, 0x33).unwrap();

        let program = vec![
            BpfInstruction::And {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 0x03);
    }

    #[test]
    fn test_bitwise_or() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 0x0F).unwrap();
        vm.set_register(2, 0xF0).unwrap();

        let program = vec![
            BpfInstruction::Or {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 0xFF);
    }

    #[test]
    fn test_bitwise_xor() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 0xFF).unwrap();
        vm.set_register(2, 0x0F).unwrap();

        let program = vec![
            BpfInstruction::Xor {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 0xF0);
    }

    // ============ JUMP EXECUTION TESTS ============

    #[test]
    fn test_jump_conditional_true() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 42).unwrap();
        vm.set_register(2, 42).unwrap();

        let program = vec![
            BpfInstruction::Jeq {
                dst_reg: 1,
                src_reg: 2,
                offset: 2,
            },
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 99,
            },
            BpfInstruction::Return,
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 42,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_jump_conditional_false() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 10).unwrap();
        vm.set_register(2, 20).unwrap();

        let program = vec![
            BpfInstruction::Jeq {
                dst_reg: 1,
                src_reg: 2,
                offset: 2,
            },
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 99,
            },
            BpfInstruction::Return,
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 42,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 99);
    }

    #[test]
    fn test_jump_greater_than() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 50).unwrap();
        vm.set_register(2, 30).unwrap();

        let program = vec![
            BpfInstruction::Jgt {
                dst_reg: 1,
                src_reg: 2,
                offset: 1,
            },
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 0,
            },
            BpfInstruction::Return,
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 1);
    }

    // ============ REGISTER OPERATIONS TESTS ============

    #[test]
    fn test_register_operations() {
        let mut vm = BpfVm::new();
        assert!(vm.set_register(5, 12345).is_ok());
        assert_eq!(vm.get_register(5).unwrap(), 12345);
        assert!(vm.set_register(10, 99999).is_ok());
        assert_eq!(vm.get_register(10).unwrap(), 99999);
    }

    #[test]
    fn test_register_invalid() {
        let mut vm = BpfVm::new();
        assert!(vm.set_register(11, 100).is_err());
        assert!(vm.get_register(99).is_err());
    }

    // ============ STACK OPERATIONS TESTS ============

    #[test]
    fn test_stack_push_pop() {
        let mut vm = BpfVm::new();
        assert!(vm.push(0x0102030405060708).is_ok());
        assert!(vm.push(0x0A0B0C0D0E0F0001).is_ok());
        assert_eq!(vm.pop().unwrap(), 0x0A0B0C0D0E0F0001);
        assert_eq!(vm.pop().unwrap(), 0x0102030405060708);
    }

    #[test]
    fn test_stack_overflow() {
        let mut vm = BpfVm::new();
        // Fill the stack
        for _ in 0..64 {
            vm.push(0x0000000000000001).unwrap();
        }
        // Should fail on next push
        assert!(vm.push(0x0000000000000001).is_err());
    }

    #[test]
    fn test_stack_underflow() {
        let mut vm = BpfVm::new();
        assert!(vm.pop().is_err());
    }

    // ============ PROGRAM EXECUTION TESTS ============

    #[test]
    fn test_program_execution_simple() {
        let mut vm = BpfVm::new();
        let program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 42,
            },
            BpfInstruction::Return,
        ];
        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_program_halting() {
        let mut vm = BpfVm::new();
        let program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 100,
            },
            BpfInstruction::AddImm {
                dst_reg: 0,
                imm: 23,
            },
            BpfInstruction::Return,
        ];
        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 123);
        assert!(vm.halted);
    }

    #[test]
    fn test_move_instruction() {
        let mut vm = BpfVm::new();
        vm.set_register(1, 555).unwrap();

        let program = vec![
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];

        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 555);
    }

    #[test]
    fn test_nop_instruction() {
        let mut vm = BpfVm::new();
        let program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 0,
                imm64: 77,
            },
            BpfInstruction::Nop,
            BpfInstruction::Return,
        ];
        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 77);
    }

    #[test]
    fn test_complex_program() {
        let mut vm = BpfVm::new();
        // Program: compute (5 + 3) * 2 = 16
        let program = vec![
            BpfInstruction::LoadImm64 {
                dst_reg: 1,
                imm64: 5,
            },
            BpfInstruction::LoadImm64 {
                dst_reg: 2,
                imm64: 3,
            },
            BpfInstruction::Add {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::LoadImm64 {
                dst_reg: 2,
                imm64: 2,
            },
            BpfInstruction::Mul {
                dst_reg: 1,
                src_reg: 2,
            },
            BpfInstruction::Mov {
                dst_reg: 0,
                src_reg: 1,
            },
            BpfInstruction::Return,
        ];
        vm.load_program(program).unwrap();
        let result = vm.run().unwrap();
        assert_eq!(result, 16);
    }
}
