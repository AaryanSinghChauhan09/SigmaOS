# eBPF CO-RE Subsystem Implementation Plan

## Overview
This plan implements a full eBPF (extended Berkeley Packet Filter) CO-RE subsystem for SigmaOS as a new `src/kernel/ebpf/` submodule directory, inspired by Linux kernel/bpf/ and arch/x86/net/bpf_jit_comp.c.

## Current State Analysis

### Existing Files (DO NOT DELETE - work alongside):
1. **src/kernel/ebpf.rs** - Basic eBPF engine with:
   - EbpfInstruction struct (10 opcodes)
   - EbpfVerifier with basic validation
   - EbpfEngine with registers[10], stack[512], HashMap map
   - EbpfXdpFilterEngine for packet filtering
   - BpfRingBufferEngine for lock-free kernel-to-userland events
   - Tests for verification, XDP hooks, and ring buffer

2. **src/kernel/ebpf_vm.rs** - Full VM implementation with:
   - BpfVm with 11 registers, 512-byte stack, heap
   - Complete BpfInstruction enum (25+ variants)
   - Helper function trait and registry (BPF_MAP_LOOKUP_ELEM, BPF_KTIME_GET_NS, etc.)
   - Instruction validation functions
   - Uses std:: (conditionally)

3. **src/kernel/ebpf_verification.rs** - Verification engine with:
   - VerificationError enum
   - BpfProgramVerifier with CFG analysis
   - Bounds checking, register validation, memory access validation
   - Loop detection and reachability analysis
   - Uses std:: collections

4. **src/kernel/ebpf_xdp.rs** - XDP implementation with:
   - EbpfInsn struct (C-compatible)
   - EbpfRegisters (11 registers)
   - EbpfProgType, XdpAction, EbpfMapType enums
   - EbpfMap with BTreeMap backing
   - EbpfProgram with verify() and jit_compile() methods
   - EbpfVm with interpreter
   - Uses #![no_std] + alloc

5. **src/kernel/missing_linux_kernel_components.rs** - Contains:
   - BpfRingBufferStreamEngine (already implemented, avoid duplication)

### Existing Module Structure:
- `src/kernel/mod.rs` declares `pub mod ebpf;` which currently resolves to `ebpf.rs`
- When `src/kernel/ebpf/mod.rs` exists, Rust will use the directory instead

## Migration Strategy: ebpf.rs → ebpf/ Directory

### Problem:
The current `src/kernel/mod.rs` has `pub mod ebpf;` which resolves to `ebpf.rs`. When we create `src/kernel/ebpf/` directory with `mod.rs`, Rust will automatically use the directory. We must preserve all public exports to avoid breaking existing code.

### Solution:
1. Create `src/kernel/ebpf/` directory with `mod.rs` as the root
2. Move **copies** of key types from ebpf.rs into new submodules (do not delete ebpf.rs yet)
3. Re-export everything from ebpf.rs in the new `ebpf/mod.rs`
4. Once verified working, optionally rename `ebpf.rs` to `ebpf_legacy.rs` (but keep it for compatibility)

## Implementation Plan

---

### ✅ STEP 1: Create ebpf/ directory structure and mod.rs

**What to do:**
Create the new `src/kernel/ebpf/` directory and the root module file `mod.rs`. This file will re-export all existing public types from the flat `ebpf.rs` file to maintain backward compatibility, then declare and re-export all new submodules.

**Files to create:**
- `src/kernel/ebpf/mod.rs`

**Verify:**
Run `cargo check --lib` and confirm 0 compilation errors. The module should compile but not change any behavior yet.

---

### ✅ STEP 2: Create instructions.rs - Complete eBPF instruction set

**What to do:**
Implement the complete 64-bit eBPF instruction set with all ALU, memory, control flow, and atomic operations. This module defines the bytecode format and provides parsing/encoding functions.

**API Design:**

```rust
#![no_std]
extern crate alloc;

/// Raw eBPF instruction (Linux struct bpf_insn compatible)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EbpfInsn {
    pub opcode: u8,
    pub dst_reg: u8,   // bits 0-3
    pub src_reg: u8,   // bits 4-7
    pub off: i16,
    pub imm: i32,
}

/// eBPF instruction class (bits 0-2 of opcode)
pub const BPF_CLASS_LD: u8 = 0x00;
pub const BPF_CLASS_LDX: u8 = 0x01;
pub const BPF_CLASS_ST: u8 = 0x02;
pub const BPF_CLASS_STX: u8 = 0x03;
pub const BPF_CLASS_ALU: u8 = 0x04;
pub const BPF_CLASS_JMP: u8 = 0x05;
pub const BPF_CLASS_JMP32: u8 = 0x06;
pub const BPF_CLASS_ALU64: u8 = 0x07;

/// ALU/JMP operations (bits 4-7 of opcode)
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
pub const BPF_OP_END: u8 = 0xd0;
// Jump operations
pub const BPF_OP_JA: u8 = 0x00;
pub const BPF_OP_JEQ: u8 = 0x10;
pub const BPF_OP_JGT: u8 = 0x20;
pub const BPF_OP_JGE: u8 = 0x30;
pub const BPF_OP_JSET: u8 = 0x40;
pub const BPF_OP_JNE: u8 = 0x50;
pub const BPF_OP_JSGT: u8 = 0x60;
pub const BPF_OP_JSGE: u8 = 0x70;
pub const BPF_OP_CALL: u8 = 0x80;
pub const BPF_OP_EXIT: u8 = 0x90;
pub const BPF_OP_JLT: u8 = 0xa0;
pub const BPF_OP_JLE: u8 = 0xb0;
pub const BPF_OP_JSLT: u8 = 0xc0;
pub const BPF_OP_JSLE: u8 = 0xd0;

/// Source operand (bit 3 of opcode)
pub const BPF_SRC_IMM: u8 = 0x00;  // Immediate
pub const BPF_SRC_REG: u8 = 0x08;  // Register

/// Size field (bits 3-4 of opcode for LD/ST)
pub const BPF_SIZE_W: u8 = 0x00;   // 32-bit
pub const BPF_SIZE_H: u8 = 0x08;   // 16-bit
pub const BPF_SIZE_B: u8 = 0x10;   // 8-bit
pub const BPF_SIZE_DW: u8 = 0x18;  // 64-bit

/// Mode field (bits 5-7 of opcode for LD/ST)
pub const BPF_MODE_IMM: u8 = 0x00;
pub const BPF_MODE_ABS: u8 = 0x20;
pub const BPF_MODE_IND: u8 = 0x40;
pub const BPF_MODE_MEM: u8 = 0x60;
pub const BPF_MODE_ATOMIC: u8 = 0xc0;

impl EbpfInsn {
    pub fn new(opcode: u8, dst: u8, src: u8, off: i16, imm: i32) -> Self { ... }
    pub fn get_class(&self) -> u8 { self.opcode & 0x07 }
    pub fn get_op(&self) -> u8 { self.opcode & 0xf0 }
    pub fn is_alu64(&self) -> bool { self.get_class() == BPF_CLASS_ALU64 }
    pub fn is_jump(&self) -> bool { ... }
    pub fn is_call(&self) -> bool { ... }
    pub fn is_exit(&self) -> bool { ... }
}

pub fn disassemble(insn: &EbpfInsn) -> alloc::string::String { ... }
```

**Files to create:**
- `src/kernel/ebpf/instructions.rs`

**Verify:**
Run `cargo check --lib` and confirm 0 errors. Add unit tests for instruction encoding/decoding.

---

### ✅ STEP 3: Create verifier.rs - CFG-based bytecode verifier

**What to do:**
Implement a control-flow-graph-based bytecode verifier that ensures programs are safe before execution. This includes bounds checking, register state tracking, memory access validation, and loop detection.

**API Design:**

```rust
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use super::instructions::EbpfInsn;

#[derive(Debug, Clone, Copy)]
pub struct RegisterState {
    pub value_type: ValueType,
    pub min_value: i64,
    pub max_value: i64,
    pub is_initialized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Unknown,
    ScalarValue,
    PtrToStack,
    PtrToPacket,
    PtrToMap,
}

pub struct VerifierState {
    pub regs: [RegisterState; 11],
    pub stack: [u8; 512],
    pub pc: usize,
}

pub struct Verifier {
    program: Vec<EbpfInsn>,
    states: BTreeMap<usize, VerifierState>,
    max_insns: usize,
}

impl Verifier {
    pub fn new(program: Vec<EbpfInsn>) -> Self { ... }
    
    /// Run full verification pass
    pub fn verify(&mut self) -> Result<(), VerifierError> {
        self.check_program_length()?;
        self.check_cfg()?;
        self.check_register_bounds()?;
        self.check_memory_access()?;
        self.check_stack_depth()?;
        self.check_loops()?;
        Ok(())
    }
    
    fn check_program_length(&self) -> Result<(), VerifierError> { ... }
    fn check_cfg(&mut self) -> Result<(), VerifierError> { ... }
    fn check_register_bounds(&self) -> Result<(), VerifierError> { ... }
    fn check_memory_access(&self) -> Result<(), VerifierError> { ... }
    fn check_stack_depth(&self) -> Result<(), VerifierError> { ... }
    fn check_loops(&self) -> Result<(), VerifierError> { ... }
}

#[derive(Debug, Clone)]
pub enum VerifierError {
    ProgramTooLarge,
    InvalidJumpTarget,
    InvalidRegister,
    UninitializedRegister,
    StackOverflow,
    InvalidMemoryAccess,
    InfiniteLoop,
    DivisionByZero,
}
```

**Files to create:**
- `src/kernel/ebpf/verifier.rs`

**Verify:**
Run `cargo check --lib` and confirm 0 errors. Add tests for valid/invalid programs.

---

### ✅ STEP 4: Create jit.rs - x86_64 JIT compiler

**What to do:**
Implement a JIT compiler that translates verified eBPF bytecode to native x86_64 machine code. This provides near-native performance for eBPF programs.

**API Design:**

```rust
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use super::instructions::EbpfInsn;

/// x86_64 register mapping for eBPF registers
/// eBPF R0-R10 -> x86_64 registers
pub const X86_REG_MAP: [X86Register; 11] = [
    X86Register::Rax,  // R0 (return value)
    X86Register::Rdi,  // R1 (arg1)
    X86Register::Rsi,  // R2 (arg2)
    X86Register::Rdx,  // R3 (arg3)
    X86Register::Rcx,  // R4 (arg4)
    X86Register::R8,   // R5 (arg5)
    X86Register::Rbx,  // R6 (callee-saved)
    X86Register::R13,  // R7 (callee-saved)
    X86Register::R14,  // R8 (callee-saved)
    X86Register::R15,  // R9 (callee-saved)
    X86Register::Rbp,  // R10 (frame pointer)
];

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum X86Register {
    Rax = 0, Rcx = 1, Rdx = 2, Rbx = 3,
    Rsp = 4, Rbp = 5, Rsi = 6, Rdi = 7,
    R8 = 8, R9 = 9, R10 = 10, R11 = 11,
    R12 = 12, R13 = 13, R14 = 14, R15 = 15,
}

pub struct JitCompiler {
    program: Vec<EbpfInsn>,
    code_buffer: Vec<u8>,
    pc_map: Vec<usize>,  // Maps eBPF PC to x86 offset
}

impl JitCompiler {
    pub fn new(program: Vec<EbpfInsn>) -> Self { ... }
    
    pub fn compile(&mut self) -> Result<Vec<u8>, JitError> {
        self.emit_prologue()?;
        for (pc, insn) in self.program.iter().enumerate() {
            self.pc_map[pc] = self.code_buffer.len();
            self.compile_instruction(insn)?;
        }
        self.emit_epilogue()?;
        Ok(self.code_buffer.clone())
    }
    
    fn emit_prologue(&mut self) -> Result<(), JitError> {
        // push rbp; mov rbp, rsp; sub rsp, 512 (stack)
        self.emit_push(X86Register::Rbp);
        self.emit_mov_reg_reg(X86Register::Rbp, X86Register::Rsp);
        self.emit_sub_imm(X86Register::Rsp, 512);
        Ok(())
    }
    
    fn emit_epilogue(&mut self) -> Result<(), JitError> {
        // mov rsp, rbp; pop rbp; ret
        self.emit_mov_reg_reg(X86Register::Rsp, X86Register::Rbp);
        self.emit_pop(X86Register::Rbp);
        self.emit_ret();
        Ok(())
    }
    
    fn compile_instruction(&mut self, insn: &EbpfInsn) -> Result<(), JitError> { ... }
    
    // x86_64 instruction emitters (raw byte emission)
    fn emit_push(&mut self, reg: X86Register) { ... }
    fn emit_pop(&mut self, reg: X86Register) { ... }
    fn emit_mov_reg_reg(&mut self, dst: X86Register, src: X86Register) { ... }
    fn emit_mov_reg_imm(&mut self, dst: X86Register, imm: i64) { ... }
    fn emit_add_reg_reg(&mut self, dst: X86Register, src: X86Register) { ... }
    fn emit_sub_imm(&mut self, reg: X86Register, imm: i32) { ... }
    fn emit_jmp(&mut self, offset: i32) { ... }
    fn emit_ret(&mut self) { ... }
}

#[derive(Debug)]
pub enum JitError {
    UnsupportedInstruction,
    BufferOverflow,
}
```

**Files to create:**
- `src/kernel/ebpf/jit.rs`

**Verify:**
Run `cargo check --lib` and confirm 0 errors. Add tests for simple programs (load immediate, return).

---

### ✅ STEP 5: Create ringbuf.rs - Lock-free ring buffer

**What to do:**
Implement a lock-free ring buffer for streaming events from kernel eBPF programs to userland without locks or blocking. Uses atomic head/tail pointers.

**API Design:**

```rust
#![no_std]
extern crate alloc;
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;

pub struct RingBuffer {
    data: Vec<u8>,
    capacity: usize,
    head: AtomicUsize,  // Producer position
    tail: AtomicUsize,  // Consumer position
    mask: usize,        // capacity - 1 (for fast modulo)
}

impl RingBuffer {
    pub fn new(capacity_bytes: usize) -> Self {
        let cap = capacity_bytes.next_power_of_two().max(4096);
        Self {
            data: alloc::vec![0u8; cap],
            capacity: cap,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            mask: cap - 1,
        }
    }
    
    /// Reserve space for a sample (lock-free)
    pub fn reserve(&self, len: usize) -> Result<usize, RingBufError> {
        let padded_len = (len + 7) & !7;  // 8-byte align
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        let available = self.capacity - (head - tail);
        
        if padded_len + 8 > available {
            return Err(RingBufError::NoSpace);
        }
        
        // Atomic CAS to reserve space
        let new_head = head + padded_len + 8;
        match self.head.compare_exchange(head, new_head, Ordering::Release, Ordering::Relaxed) {
            Ok(_) => Ok(head),
            Err(_) => Err(RingBufError::Contention),
        }
    }
    
    /// Submit data to reserved space
    pub fn submit(&mut self, offset: usize, data: &[u8]) -> Result<(), RingBufError> { ... }
    
    /// Discard reserved space
    pub fn discard(&mut self, offset: usize) -> Result<(), RingBufError> { ... }
    
    /// Consume next sample (userland reader)
    pub fn consume(&self) -> Option<Vec<u8>> { ... }
}

#[derive(Debug)]
pub enum RingBufError {
    NoSpace,
    Contention,
    InvalidOffset,
}
```

**Files to create:**
- `src/kernel/ebpf/ringbuf.rs`

**Verify:**
Run `cargo check --lib` and confirm 0 errors. Add tests for concurrent reserve/submit/consume.

---

### ✅ STEP 6: Create maps.rs - BPF hash/array maps

**What to do:**
Implement BPF map types (array and hash) for sharing data between eBPF programs and userland. These are key-value stores accessible via helper functions.

**API Design:**

```rust
#![no_std]
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfMapType {
    Array = 2,
    Hash = 1,
    ProgArray = 3,
    PercpuHash = 5,
    PercpuArray = 6,
    LruHash = 9,
}

pub trait BpfMap {
    fn map_type(&self) -> BpfMapType;
    fn key_size(&self) -> u32;
    fn value_size(&self) -> u32;
    fn max_entries(&self) -> u32;
    
    fn lookup(&self, key: &[u8]) -> Option<&[u8]>;
    fn update(&mut self, key: &[u8], value: &[u8], flags: u64) -> Result<(), MapError>;
    fn delete(&mut self, key: &[u8]) -> Result<(), MapError>;
}

pub struct ArrayMap {
    value_size: u32,
    max_entries: u32,
    values: Vec<Vec<u8>>,
}

impl ArrayMap {
    pub fn new(value_size: u32, max_entries: u32) -> Self {
        let mut values = Vec::with_capacity(max_entries as usize);
        for _ in 0..max_entries {
            values.push(alloc::vec![0u8; value_size as usize]);
        }
        Self { value_size, max_entries, values }
    }
}

impl BpfMap for ArrayMap {
    fn map_type(&self) -> BpfMapType { BpfMapType::Array }
    fn key_size(&self) -> u32 { 4 }
    fn value_size(&self) -> u32 { self.value_size }
    fn max_entries(&self) -> u32 { self.max_entries }
    
    fn lookup(&self, key: &[u8]) -> Option<&[u8]> {
        if key.len() != 4 { return None; }
        let idx = u32::from_ne_bytes([key[0], key[1], key[2], key[3]]) as usize;
        self.values.get(idx).map(|v| v.as_slice())
    }
    
    fn update(&mut self, key: &[u8], value: &[u8], _flags: u64) -> Result<(), MapError> { ... }
    fn delete(&mut self, _key: &[u8]) -> Result<(), MapError> { Err(MapError::NotSupported) }
}

pub struct HashMap {
    key_size: u32,
    value_size: u32,
    max_entries: u32,
    map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl HashMap {
    pub fn new(key_size: u32, value_size: u32, max_entries: u32) -> Self { ... }
}

impl BpfMap for HashMap {
    fn map_type(&self) -> BpfMapType { BpfMapType::Hash }
    fn key_size(&self) -> u32 { self.key_size }
    fn value_size(&self) -> u32 { self.value_size }
    fn max_entries(&self) -> u32 { self.max_entries }
    
    fn lookup(&self, key: &[u8]) -> Option<&[u8]> { ... }
    fn update(&mut self, key: &[u8], value: &[u8], flags: u64) -> Result<(), MapError> { ... }
    fn delete(&mut self, key: &[u8]) -> Result<(), MapError> { ... }
}

#[derive(Debug)]
pub enum MapError {
    KeyNotFound,
    MapFull,
    InvalidKey,
    InvalidValue,
    NotSupported,
}
```

**Files to create:**
- `src/kernel/ebpf/maps.rs`

**Verify:**
Run `cargo check --lib` and confirm 0 errors. Add tests for array/hash operations.

---

### ✅ STEP 7: Update ebpf/mod.rs to re-export all modules

**What to do:**
Update the root `src/kernel/ebpf/mod.rs` to declare all new submodules and re-export the old ebpf.rs types for backward compatibility.

**Re-export strategy:**

```rust
// Re-export legacy types from parent ebpf.rs for backward compatibility
pub use super::ebpf::{
    EbpfInstruction, EbpfVerifier, EbpfEngine,
    EbpfXdpFilterEngine, XdpHookType, XdpAction, XdpPacketContext,
    BpfRingBufferEngine, BpfRingBufferHeader, BpfRingBufferSample,
    EBPF_OP_ADD, EBPF_OP_ADDI, EBPF_OP_SUB, EBPF_OP_LD, EBPF_OP_ST,
    EBPF_OP_JEQ, EBPF_OP_JNE, EBPF_OP_MAP_LOOKUP, EBPF_OP_EXIT, EBPF_OP_DIV,
    BPF_RINGBUF_BUSY_BIT, BPF_RINGBUF_DISCARD_BIT,
};

// Declare new submodules
pub mod instructions;
pub mod verifier;
pub mod jit;
pub mod ringbuf;
pub mod maps;

// Re-export new types
pub use instructions::{EbpfInsn, BPF_CLASS_*, BPF_OP_*, disassemble};
pub use verifier::{Verifier, VerifierState, RegisterState, ValueType, VerifierError};
pub use jit::{JitCompiler, X86Register, X86_REG_MAP, JitError};
pub use ringbuf::{RingBuffer, RingBufError};
pub use maps::{BpfMap, BpfMapType, ArrayMap, HashMap as BpfHashMap, MapError};
```

**Files to modify:**
- `src/kernel/ebpf/mod.rs`

**Verify:**
Run `cargo check --lib` and confirm 0 errors. All existing imports should still work.

---

### ✅ STEP 8: Add comprehensive tests to each module

**What to do:**
Add `#[cfg(test)]` modules to each new file with comprehensive unit tests covering normal and edge cases.

**Test coverage required:**

1. **instructions.rs tests:**
   - Instruction encoding/decoding
   - Opcode extraction (get_class, get_op)
   - Disassembly output
   - Edge cases (invalid opcodes, register overflow)

2. **verifier.rs tests:**
   - Valid simple programs (load, add, return)
   - Invalid programs (out-of-bounds jump, uninitialized register)
   - CFG construction and traversal
   - Loop detection
   - Stack overflow detection

3. **jit.rs tests:**
   - Compile simple programs (mov, add, return)
   - Register mapping correctness
   - Prologue/epilogue generation
   - Jump offset calculation

4. **ringbuf.rs tests:**
   - Reserve/submit/consume cycle
   - Concurrent reserve (atomic semantics)
   - Buffer full condition
   - Discard handling

5. **maps.rs tests:**
   - Array map: lookup, update by index
   - Hash map: insert, lookup, delete
   - Max entries enforcement
   - Key/value size validation

**Files to modify:**
- All 5 new module files (add `#[cfg(test)] mod tests { ... }` at the end)

**Verify:**
Run `./run_sigma_tests.sh` and confirm all tests pass (existing + new). If test script doesn't automatically discover new tests, run `cargo test --lib` to verify.

---

### ✅ STEP 9: Integration test for full eBPF pipeline

**What to do:**
Add an integration test that exercises the full eBPF pipeline: write bytecode → verify → JIT compile → execute. This ensures all modules work together correctly.

**Test scenario:**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_full_ebpf_pipeline() {
        // 1. Create a simple eBPF program (add two numbers and return)
        let program = vec![
            EbpfInsn::new(BPF_CLASS_ALU64 | BPF_OP_MOV | BPF_SRC_IMM, 0, 0, 0, 10),  // r0 = 10
            EbpfInsn::new(BPF_CLASS_ALU64 | BPF_OP_ADD | BPF_SRC_IMM, 0, 0, 0, 32),  // r0 += 32
            EbpfInsn::new(BPF_CLASS_JMP | BPF_OP_EXIT, 0, 0, 0, 0),                   // exit
        ];
        
        // 2. Verify the program
        let mut verifier = Verifier::new(program.clone());
        assert!(verifier.verify().is_ok());
        
        // 3. JIT compile the program
        let mut compiler = JitCompiler::new(program);
        let native_code = compiler.compile().unwrap();
        assert!(!native_code.is_empty());
        
        // 4. (Optional) Execute if we had an interpreter
        // let result = execute(&native_code);
        // assert_eq!(result, 42);
    }
    
    #[test]
    fn test_ebpf_map_access() {
        // Test BPF map lookup from eBPF program
        let mut map = ArrayMap::new(8, 16);
        map.update(&[0, 0, 0, 0], &[1, 2, 3, 4, 5, 6, 7, 8], 0).unwrap();
        
        let value = map.lookup(&[0, 0, 0, 0]).unwrap();
        assert_eq!(value, &[1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
```

**Files to modify:**
- `src/kernel/ebpf/mod.rs` (add integration tests at the end)

**Verify:**
Run `cargo test --lib` and confirm the integration test passes.

---

## Register Mapping Table (x86_64 JIT)

| eBPF | x86_64 | Purpose | Preserved |
|------|--------|---------|-----------|
| R0   | rax    | Return value | No |
| R1   | rdi    | Arg1 / Ctx pointer | No |
| R2   | rsi    | Arg2 | No |
| R3   | rdx    | Arg3 | No |
| R4   | rcx    | Arg4 | No |
| R5   | r8     | Arg5 | No |
| R6   | rbx    | Callee-saved | Yes |
| R7   | r13    | Callee-saved | Yes |
| R8   | r14    | Callee-saved | Yes |
| R9   | r15    | Callee-saved | Yes |
| R10  | rbp    | Frame pointer (read-only) | Yes |

**Rationale:**
- Matches Linux kernel x86_64 BPF JIT mapping
- Uses System V AMD64 ABI calling convention
- Preserves callee-saved registers across calls
- R10 is read-only frame pointer (maps to rbp)

## CFG Verifier Data Structures

```rust
struct ControlFlowGraph {
    basic_blocks: BTreeMap<usize, BasicBlock>,
    edges: Vec<(usize, usize)>,  // (from_pc, to_pc)
}

struct BasicBlock {
    start_pc: usize,
    end_pc: usize,
    instructions: Vec<EbpfInsn>,
    successors: Vec<usize>,
    predecessors: Vec<usize>,
}
```

**Algorithm:**
1. Scan program and identify basic block boundaries (jump targets, after jumps, after calls)
2. Build CFG by connecting blocks via jump/fall-through edges
3. Perform DFS to detect unreachable blocks
4. Use Tarjan's algorithm to detect strongly connected components (loops)
5. Track register states per basic block entry/exit
6. Propagate states through successors until fixed point

## Lock-Free Ring Buffer Algorithm

**Data structure:**
```
[head (AtomicUsize)] [tail (AtomicUsize)] [data: Vec<u8>]
 ^                     ^
 producer             consumer
```

**Reserve algorithm (producer):**
1. Load `head` and `tail` atomically
2. Calculate available space: `capacity - (head - tail)`
3. If insufficient space, return `NoSpace` error
4. Atomic CAS: `compare_exchange(head, head + len)`
5. If CAS succeeds, return offset; else retry (contention)

**Submit algorithm (producer):**
1. Write data to `data[offset & mask]`
2. Write header: `len | 0` (not busy, not discarded)

**Consume algorithm (consumer):**
1. Load `tail`
2. Read header at `data[tail & mask]`
3. If busy bit set, spin-wait (producer still writing)
4. If discard bit set, skip sample and advance tail
5. Copy sample data to output buffer
6. Atomic store: `tail += len + header_size`

**Correctness:**
- Single producer, single consumer (SPSC) for simplicity
- Power-of-2 size ensures `(offset & mask)` wraps correctly
- Atomic operations ensure memory ordering

## BPF Map Types

| Type | Key | Value | Max Entries | Use Case |
|------|-----|-------|-------------|----------|
| Array | u32 index | Fixed-size | 64K | Fast per-CPU counters |
| Hash | Variable | Variable | 1M | General key-value lookups |
| ProgArray | u32 | Program FD | 64K | Tail call dispatch |
| PercpuHash | Variable | Variable | 1M | Per-CPU statistics |
| LruHash | Variable | Variable | 1M | Bounded-size cache |

**Implementation notes:**
- Array: Pre-allocated Vec, O(1) lookup by index
- Hash: BTreeMap (no_std compatible), O(log n) lookup
- PercpuHash: Array of Hash maps (one per CPU core)
- LruHash: Hash + doubly-linked list for LRU eviction

## Test Plan

### Unit Tests (per module)

**instructions.rs:**
- ✅ Instruction creation and field access
- ✅ Opcode extraction (class, op, src)
- ✅ Instruction type predicates (is_alu64, is_jump, is_call, is_exit)
- ✅ Disassembly output format
- ✅ Edge cases (invalid opcodes, register > 10)

**verifier.rs:**
- ✅ Accept valid programs (load, add, return)
- ✅ Reject programs with out-of-bounds jumps
- ✅ Reject programs with uninitialized register use
- ✅ Reject programs with stack overflow
- ✅ Detect infinite loops
- ✅ Build CFG correctly
- ✅ Track register states through branches

**jit.rs:**
- ✅ Emit prologue/epilogue correctly
- ✅ Compile ALU operations (add, sub, mul, div, mod)
- ✅ Compile jumps with correct offset calculation
- ✅ Compile calls (emit call instruction)
- ✅ Register mapping correctness
- ✅ Edge cases (division by zero, invalid instruction)

**ringbuf.rs:**
- ✅ Create buffer with power-of-2 size
- ✅ Reserve → submit → consume cycle
- ✅ Handle buffer full condition
- ✅ Discard reserved space
- ✅ Concurrent reserve operations (atomic CAS)
- ✅ Wrap-around behavior (head/tail overflow)

**maps.rs:**
- ✅ Array map: create, lookup, update
- ✅ Hash map: create, insert, lookup, delete
- ✅ Enforce max_entries limit
- ✅ Validate key/value sizes
- ✅ Edge cases (key not found, map full)

### Integration Tests

**test_full_ebpf_pipeline:**
- ✅ Write bytecode → verify → JIT compile
- ✅ Execute simple program (if interpreter available)
- ✅ Map access from eBPF program
- ✅ Ring buffer event submission

**test_xdp_packet_filter:**
- ✅ Attach XDP program to virtual interface
- ✅ Process packet and return action (PASS, DROP)
- ✅ Access packet data (packet->data, packet->data_end)

## Verification Strategy

After each step:
1. **Build check:** `cargo check --lib` → 0 errors
2. **Module test:** `cargo test --lib <module_name>` → all tests pass
3. **Full test:** `./run_sigma_tests.sh` → all tests pass
4. **Diff check:** Ensure `src/lib.rs` exports are updated if needed

## No_std Compliance Checklist

- [ ] All new files have `#![no_std]` at the top
- [ ] All new files have `extern crate alloc;`
- [ ] Use `alloc::vec::Vec`, not `std::vec::Vec`
- [ ] Use `alloc::collections::BTreeMap`, not `std::collections::HashMap`
- [ ] Use `alloc::string::String`, not `std::string::String`
- [ ] Use `alloc::boxed::Box`, not `std::boxed::Box`
- [ ] Use `core::sync::atomic::*`, not `std::sync::atomic::*`
- [ ] No `std::*` imports anywhere in new ebpf/ directory

## Success Criteria

1. ✅ `cargo check --lib` returns 0 errors
2. ✅ All existing tests continue to pass
3. ✅ New module tests all pass
4. ✅ Integration test passes
5. ✅ All files are `#![no_std]` compliant
6. ✅ Backward compatibility maintained (all ebpf.rs exports still work)
7. ✅ Documentation comments on all public types/functions

## Future Enhancements (out of scope)

- Multi-architecture JIT (ARM64, RISC-V)
- More map types (LPM trie, stack, queue)
- BTF (BPF Type Format) support
- CO-RE relocation
- Tail calls (bpf_tail_call)
- Helper function sandbox
- Per-CPU data structures
- BPF-to-BPF calls
- Function verification (bounded loops, subprograms)

---

**End of Implementation Plan**
