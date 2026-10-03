// eBPF CO-RE Subsystem Root Module
// Comprehensive eBPF implementation with instruction set, verifier, JIT, maps, and ring buffers

#![no_std]
extern crate alloc;

// Declare submodules
pub mod instructions;
pub mod verifier;
pub mod jit;
pub mod ringbuf;
pub mod maps;

// Re-export top-level types for ergonomics
pub use instructions::{
    BpfInsn, BpfOpcode, MAX_BPF_INSNS, MAX_BPF_STACK, NUM_BPF_REGS,
    EbpfOpClass, EbpfSrc, EbpfSize, disassemble,
    BPF_CLASS_LD, BPF_CLASS_LDX, BPF_CLASS_ST, BPF_CLASS_STX,
    BPF_CLASS_ALU, BPF_CLASS_JMP, BPF_CLASS_JMP32, BPF_CLASS_ALU64,
    BPF_OP_ADD, BPF_OP_SUB, BPF_OP_MUL, BPF_OP_DIV, BPF_OP_OR, BPF_OP_AND,
    BPF_OP_LSH, BPF_OP_RSH, BPF_OP_NEG, BPF_OP_MOD, BPF_OP_XOR, BPF_OP_MOV,
    BPF_OP_ARSH, BPF_OP_JA, BPF_OP_JEQ, BPF_OP_JGT, BPF_OP_JGE, BPF_OP_JNE,
    BPF_OP_JLT, BPF_OP_JLE, BPF_OP_JSGT, BPF_OP_JSLT, BPF_OP_CALL, BPF_OP_EXIT,
    BPF_SRC_IMM, BPF_SRC_REG,
};

pub use verifier::{
    BpfVerifier, VerifiedProg, VerifyError, RegType, RegState, VerifierState,
};

pub use jit::{
    JitCompiler, JitCode, JitError, X86Register, X86_REG_MAP,
};

pub use ringbuf::{
    BpfRingBuf, RingBufError,
};

pub use maps::{
    BpfArrayMap, BpfHashMap, BpfMap, BpfMapTable, BpfMapType, MapError,
    next_map_id, GLOBAL_MAP_ID,
};

// Backward-compat re-exports matching old ebpf.rs public surface
pub use instructions::BpfInsn as EbpfInstruction;

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_ebpf_pipeline() {
        // 1. Create a simple eBPF program (mov r0, 42; exit)
        let program = alloc::vec![
            BpfInsn::alu64_imm(BPF_OP_MOV, 0, 42),
            BpfInsn::exit_insn(),
        ];
        
        // 2. Verify the program
        let verifier = BpfVerifier::new(program.clone());
        let verified = verifier.verify();
        assert!(verified.is_ok());
        
        // 3. JIT compile the program
        let mut compiler = JitCompiler::new();
        let native_code = compiler.compile(&verified.unwrap());
        assert!(native_code.is_ok());
        
        let code = native_code.unwrap();
        assert!(!code.is_empty());
    }
    
    #[test]
    fn test_ebpf_map_access() {
        // Test BPF array map
        let mut map = BpfArrayMap::new(16);
        map.update(0, 12345).unwrap();
        
        let value = map.lookup(0).unwrap();
        assert_eq!(value, 12345);
        
        // Test BPF hash map
        let mut hmap = BpfHashMap::new(100, 4, 8);
        let key = alloc::vec![1, 2, 3, 4];
        let value = alloc::vec![10, 20, 30, 40, 50, 60, 70, 80];
        
        hmap.update(key.clone(), value.clone()).unwrap();
        let result = hmap.lookup(&key).unwrap();
        assert_eq!(result, value.as_slice());
    }
    
    #[test]
    fn test_ebpf_ringbuf() {
        let mut rb = BpfRingBuf::new(4096).unwrap();
        
        // Reserve and submit
        let offset = rb.reserve(64).unwrap();
        let data = b"Test ring buffer event";
        rb.submit(offset, data);
        
        // Consume
        let consumed = rb.consume().unwrap();
        assert_eq!(&consumed[..data.len()], data);
    }
    
    #[test]
    fn test_verifier_rejects_invalid() {
        // Invalid: out-of-bounds jump
        let program = alloc::vec![
            BpfInsn::jmp_imm(BPF_OP_JEQ, 0, 0, 1000),
            BpfInsn::exit_insn(),
        ];
        
        let verifier = BpfVerifier::new(program);
        assert!(matches!(verifier.verify(), Err(VerifyError::OutOfBoundsJump { .. })));
    }
}
