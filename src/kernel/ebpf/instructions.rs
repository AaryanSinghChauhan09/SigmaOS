// eBPF Instruction Set Definition (Linux-compatible)
// Implements complete 64-bit eBPF instruction encoding/decoding

#![no_std]
extern crate alloc;

use alloc::string::String;

/// eBPF instruction class (bits 0-2 of opcode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EbpfOpClass {
    Ld = 0x00,
    Ldx = 0x01,
    St = 0x02,
    Stx = 0x03,
    Alu = 0x04,
    Jmp = 0x05,
    Jmp32 = 0x06,
    Alu64 = 0x07,
}

/// Source operand type (bit 3 of opcode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EbpfSrc {
    Imm = 0x00, // Immediate value
    Reg = 0x08, // Register
}

/// Memory access size (bits 3-4 for LD/ST)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EbpfSize {
    W = 0x00,  // 32-bit
    H = 0x08,  // 16-bit
    B = 0x10,  // 8-bit
    Dw = 0x18, // 64-bit
}

/// Raw eBPF instruction (Linux struct bpf_insn compatible)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BpfInsn {
    pub opcode: u8,
    pub dst_reg: u8, // bits 0-3: dst register, bits 4-7: src register
    pub src_reg: u8, // (packed as single byte in Linux, split here for clarity)
    pub off: i16,
    pub imm: i32,
}

/// eBPF opcodes enum for high-level representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfOpcode {
    // ALU64 operations (class 0x07)
    ADD64_IMM,
    ADD64_REG,
    SUB64_IMM,
    SUB64_REG,
    MUL64_IMM,
    MUL64_REG,
    DIV64_IMM,
    DIV64_REG,
    OR64_IMM,
    OR64_REG,
    AND64_IMM,
    AND64_REG,
    LSH64_IMM,
    LSH64_REG,
    RSH64_IMM,
    RSH64_REG,
    NEG64,
    MOD64_IMM,
    MOD64_REG,
    XOR64_IMM,
    XOR64_REG,
    MOV64_IMM,
    MOV64_REG,
    ARSH64_IMM,
    ARSH64_REG,

    // ALU32 operations (class 0x04)
    ADD32_IMM,
    ADD32_REG,
    SUB32_IMM,
    SUB32_REG,
    MOV32_IMM,
    MOV32_REG,

    // Load operations
    LD_DW_IMM,
    LDX_MEM_B,
    LDX_MEM_H,
    LDX_MEM_W,
    LDX_MEM_DW,

    // Store operations
    STX_MEM_B,
    STX_MEM_H,
    STX_MEM_W,
    STX_MEM_DW,
    ST_MEM_B,
    ST_MEM_H,
    ST_MEM_W,
    ST_MEM_DW,

    // Jump operations (class 0x05)
    JA,
    JEQ_IMM,
    JEQ_REG,
    JGT_IMM,
    JGT_REG,
    JGE_IMM,
    JGE_REG,
    JNE_IMM,
    JNE_REG,
    JLT_IMM,
    JLT_REG,
    JLE_IMM,
    JLE_REG,
    JSGT_IMM,
    JSGT_REG,
    JSLT_IMM,
    JSLT_REG,

    // Special operations
    CALL,
    EXIT,
}

// Opcode constants (Linux-compatible)
pub const BPF_CLASS_LD: u8 = 0x00;
pub const BPF_CLASS_LDX: u8 = 0x01;
pub const BPF_CLASS_ST: u8 = 0x02;
pub const BPF_CLASS_STX: u8 = 0x03;
pub const BPF_CLASS_ALU: u8 = 0x04;
pub const BPF_CLASS_JMP: u8 = 0x05;
pub const BPF_CLASS_JMP32: u8 = 0x06;
pub const BPF_CLASS_ALU64: u8 = 0x07;

pub const BPF_OP_ADD: u8 = 0x00;
pub const BPF_OP_SUB: u8 = 0x10;
pub const BPF_OP_MUL: u8 = 0x20;
pub const BPF_OP_DIV: u8 = 0x30;
pub const BPF_OP_OR: u8 = 0x40;
pub const BPF_OP_AND: u8 = 0x50;
pub const BPF_OP_LSH: u8 = 0x60;
pub const BPF_OP_RSH: u8 = 0x70;
pub const BPF_OP_NEG: u8 = 0x80;
pub const BPF_OP_MOD: u8 = 0x90;
pub const BPF_OP_XOR: u8 = 0xa0;
pub const BPF_OP_MOV: u8 = 0xb0;
pub const BPF_OP_ARSH: u8 = 0xc0;

pub const BPF_OP_JA: u8 = 0x00;
pub const BPF_OP_JEQ: u8 = 0x10;
pub const BPF_OP_JGT: u8 = 0x20;
pub const BPF_OP_JGE: u8 = 0x30;
pub const BPF_OP_JNE: u8 = 0x50;
pub const BPF_OP_JLT: u8 = 0xa0;
pub const BPF_OP_JLE: u8 = 0xb0;
pub const BPF_OP_JSGT: u8 = 0x60;
pub const BPF_OP_JSLT: u8 = 0xc0;
pub const BPF_OP_CALL: u8 = 0x80;
pub const BPF_OP_EXIT: u8 = 0x90;

pub const BPF_SRC_IMM: u8 = 0x00;
pub const BPF_SRC_REG: u8 = 0x08;

pub const BPF_SIZE_W: u8 = 0x00; // 32-bit
pub const BPF_SIZE_H: u8 = 0x08; // 16-bit
pub const BPF_SIZE_B: u8 = 0x10; // 8-bit
pub const BPF_SIZE_DW: u8 = 0x18; // 64-bit

pub const BPF_MODE_MEM: u8 = 0x60;
pub const BPF_MODE_IMM: u8 = 0x00;

// eBPF constants
pub const MAX_BPF_INSNS: usize = 4096;
pub const MAX_BPF_STACK: usize = 512;
pub const NUM_BPF_REGS: usize = 11;

impl BpfInsn {
    /// Create a new BPF instruction
    pub fn new(opcode: u8, dst_reg: u8, src_reg: u8, off: i16, imm: i32) -> Self {
        Self {
            opcode,
            dst_reg,
            src_reg,
            off,
            imm,
        }
    }

    /// Create an ALU64 immediate instruction
    pub fn alu64_imm(op: u8, dst: u8, imm: i32) -> Self {
        Self::new(BPF_CLASS_ALU64 | op | BPF_SRC_IMM, dst, 0, 0, imm)
    }

    /// Create an ALU64 register instruction
    pub fn alu64_reg(op: u8, dst: u8, src: u8) -> Self {
        Self::new(BPF_CLASS_ALU64 | op | BPF_SRC_REG, dst, src, 0, 0)
    }

    /// Create a jump immediate instruction
    pub fn jmp_imm(op: u8, dst: u8, imm: i32, off: i16) -> Self {
        Self::new(BPF_CLASS_JMP | op | BPF_SRC_IMM, dst, 0, off, imm)
    }

    /// Create a jump register instruction
    pub fn jmp_reg(op: u8, dst: u8, src: u8, off: i16) -> Self {
        Self::new(BPF_CLASS_JMP | op | BPF_SRC_REG, dst, src, off, 0)
    }

    /// Create a load from memory instruction
    pub fn ldx_mem(size: u8, dst: u8, src: u8, off: i16) -> Self {
        Self::new(BPF_CLASS_LDX | BPF_MODE_MEM | size, dst, src, off, 0)
    }

    /// Create a store to memory instruction
    pub fn stx_mem(size: u8, dst: u8, src: u8, off: i16) -> Self {
        Self::new(BPF_CLASS_STX | BPF_MODE_MEM | size, dst, src, off, 0)
    }

    /// Create a call instruction
    pub fn call(func_id: i32) -> Self {
        Self::new(BPF_CLASS_JMP | BPF_OP_CALL, 0, 0, 0, func_id)
    }

    /// Create an exit instruction
    pub fn exit_insn() -> Self {
        Self::new(BPF_CLASS_JMP | BPF_OP_EXIT, 0, 0, 0, 0)
    }

    /// Extract instruction class (bits 0-2)
    pub fn get_class(&self) -> u8 {
        self.opcode & 0x07
    }

    /// Extract operation code (bits 4-7)
    pub fn get_op(&self) -> u8 {
        self.opcode & 0xf0
    }

    /// Extract source type (bit 3)
    pub fn get_src(&self) -> u8 {
        self.opcode & 0x08
    }

    /// Check if this is an ALU64 instruction
    pub fn is_alu64(&self) -> bool {
        self.get_class() == BPF_CLASS_ALU64
    }

    /// Check if this is a jump instruction
    pub fn is_jump(&self) -> bool {
        let class = self.get_class();
        class == BPF_CLASS_JMP || class == BPF_CLASS_JMP32
    }

    /// Check if this is a call instruction
    pub fn is_call(&self) -> bool {
        self.get_class() == BPF_CLASS_JMP && self.get_op() == BPF_OP_CALL
    }

    /// Check if this is an exit instruction
    pub fn is_exit(&self) -> bool {
        self.get_class() == BPF_CLASS_JMP && self.get_op() == BPF_OP_EXIT
    }
}

/// Disassemble a BPF instruction to human-readable string
pub fn disassemble(insn: &BpfInsn) -> String {
    use alloc::format;

    let class = insn.get_class();
    let op = insn.get_op();
    let src = insn.get_src();

    match class {
        BPF_CLASS_ALU64 | BPF_CLASS_ALU => {
            let is_64 = class == BPF_CLASS_ALU64;
            let suffix = if is_64 { "64" } else { "32" };
            let op_name = match op {
                BPF_OP_ADD => "add",
                BPF_OP_SUB => "sub",
                BPF_OP_MUL => "mul",
                BPF_OP_DIV => "div",
                BPF_OP_OR => "or",
                BPF_OP_AND => "and",
                BPF_OP_LSH => "lsh",
                BPF_OP_RSH => "rsh",
                BPF_OP_NEG => "neg",
                BPF_OP_MOD => "mod",
                BPF_OP_XOR => "xor",
                BPF_OP_MOV => "mov",
                BPF_OP_ARSH => "arsh",
                _ => "unknown",
            };

            if src == BPF_SRC_IMM {
                format!("{}{} r{}, {}", op_name, suffix, insn.dst_reg, insn.imm)
            } else {
                format!("{}{} r{}, r{}", op_name, suffix, insn.dst_reg, insn.src_reg)
            }
        }
        BPF_CLASS_JMP | BPF_CLASS_JMP32 => match op {
            BPF_OP_JA => format!("ja {}", insn.off),
            BPF_OP_CALL => format!("call {}", insn.imm),
            BPF_OP_EXIT => format!("exit"),
            BPF_OP_JEQ => format!(
                "jeq r{}, {}, {}",
                insn.dst_reg,
                if src == BPF_SRC_IMM {
                    format!("{}", insn.imm)
                } else {
                    format!("r{}", insn.src_reg)
                },
                insn.off
            ),
            BPF_OP_JNE => format!(
                "jne r{}, {}, {}",
                insn.dst_reg,
                if src == BPF_SRC_IMM {
                    format!("{}", insn.imm)
                } else {
                    format!("r{}", insn.src_reg)
                },
                insn.off
            ),
            BPF_OP_JGT => format!(
                "jgt r{}, {}, {}",
                insn.dst_reg,
                if src == BPF_SRC_IMM {
                    format!("{}", insn.imm)
                } else {
                    format!("r{}", insn.src_reg)
                },
                insn.off
            ),
            _ => format!("jmp_unknown 0x{:02x}", op),
        },
        BPF_CLASS_LDX => {
            let size = match insn.opcode & 0x18 {
                BPF_SIZE_B => "b",
                BPF_SIZE_H => "h",
                BPF_SIZE_W => "w",
                BPF_SIZE_DW => "dw",
                _ => "?",
            };
            format!(
                "ldx{} r{}, [r{}+{}]",
                size, insn.dst_reg, insn.src_reg, insn.off
            )
        }
        BPF_CLASS_STX => {
            let size = match insn.opcode & 0x18 {
                BPF_SIZE_B => "b",
                BPF_SIZE_H => "h",
                BPF_SIZE_W => "w",
                BPF_SIZE_DW => "dw",
                _ => "?",
            };
            format!(
                "stx{} [r{}+{}], r{}",
                size, insn.dst_reg, insn.off, insn.src_reg
            )
        }
        _ => format!("unknown 0x{:02x}", insn.opcode),
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_creation() {
        let insn = BpfInsn::new(0x07, 0, 1, 0, 42);
        assert_eq!(insn.opcode, 0x07);
        assert_eq!(insn.dst_reg, 0);
        assert_eq!(insn.src_reg, 1);
        assert_eq!(insn.imm, 42);
    }

    #[test]
    fn test_alu64_imm() {
        let insn = BpfInsn::alu64_imm(BPF_OP_ADD, 1, 100);
        assert_eq!(insn.get_class(), BPF_CLASS_ALU64);
        assert_eq!(insn.get_op(), BPF_OP_ADD);
        assert_eq!(insn.dst_reg, 1);
        assert_eq!(insn.imm, 100);
    }

    #[test]
    fn test_exit_instruction() {
        let insn = BpfInsn::exit_insn();
        assert!(insn.is_exit());
        assert!(insn.is_jump());
    }

    #[test]
    fn test_call_instruction() {
        let insn = BpfInsn::call(5);
        assert!(insn.is_call());
        assert_eq!(insn.imm, 5);
    }

    #[test]
    fn test_disassemble_alu() {
        let insn = BpfInsn::alu64_imm(BPF_OP_ADD, 0, 42);
        let dis = disassemble(&insn);
        assert!(dis.contains("add64"));
        assert!(dis.contains("r0"));
        assert!(dis.contains("42"));
    }

    #[test]
    fn test_disassemble_exit() {
        let insn = BpfInsn::exit_insn();
        let dis = disassemble(&insn);
        assert_eq!(dis, "exit");
    }

    #[test]
    fn test_instruction_predicates() {
        let mov = BpfInsn::alu64_imm(BPF_OP_MOV, 0, 10);
        assert!(mov.is_alu64());
        assert!(!mov.is_jump());

        let jmp = BpfInsn::jmp_imm(BPF_OP_JEQ, 0, 0, 5);
        assert!(jmp.is_jump());
        assert!(!jmp.is_alu64());
    }
}
