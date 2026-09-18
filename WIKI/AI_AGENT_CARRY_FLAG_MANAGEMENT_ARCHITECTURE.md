# AI Agent Carry Flag Management Architecture in SigmaOS

## 1. Executive Summary & Architectural Overview
In high-performance microkernel operating systems, hardware status flags—specifically the **Carry Flag (CF)**—play a fundamental role in low-level CPU arithmetic, multi-precision bignum operations (e.g. 128-bit, 256-bit, and 512-bit post-quantum cryptography), virtual machine ALU emulation, kernel timer nanos/seconds carry propagation, and eBPF static verification.

SigmaOS defines a unified, zero-dependency `#![no_std]` architecture for Carry Flag Management. This document specifies the architectural model linking low-level ISA primitives (`RFLAGS.CF`, `PSTATE.C`), kernel bignum arithmetic in `klib`, hardware ALU emulation in `src/drivers/dde.rs` and `src/compatibility/fedora.rs`, and agentic Carry Flag inspection for AI coding agents.

```
+-----------------------------------------------------------------------------------+
|                            SIGMAOS AI AGENT RUNTIME                               |
+-----------------------------------------------------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|                     CARRY FLAG (CF) MANAGEMENT ARCHITECTURE                       |
+-----------------------------------------------------------------------------------+
|  1. ISA Status Registers       | 2. Multi-Word Arithmetic    | 3. Kernel Services |
|     - x86_64: RFLAGS.CF (bit 0)|    - u128 / u256 / u512      |    - Time Nanos Carry|
|     - AArch64: PSTATE.C (bit 29)|   - overflowing_add / sub  |    - PQC Dilithium/Kyber|
|     - RISC-V 64: Branch/Carry  |    - carrying_add / borrow   |    - eBPF ALU Verify|
+-----------------------------------------------------------------------------------+
                                          |
                                          v
+-----------------------------------------------------------------------------------+
|                 ALU & HARDWARE EMULATION ENGINE (dde.rs / fedora.rs)               |
+-----------------------------------------------------------------------------------+
```

---

## 2. Hardware ISA Carry Flag Representation

### 2.1 x86_64 Architecture (`RFLAGS.CF`)
- **Bit Position**: Bit 0 of the `EFLAGS`/`RFLAGS` register.
- **Semantics**: Set to `1` if an unsigned arithmetic operation (ADD, SUB, ADC, SBB) generates a carry out or borrow into the most significant bit; cleared to `0` otherwise.
- **Assembly Primitives**:
  - `ADC dst, src`: Add with carry (`dst = dst + src + CF`).
  - `SBB dst, src`: Subtract with borrow (`dst = dst - (src + CF)`).
  - `CLC` / `STC` / `CMC`: Clear, Set, and Complement Carry Flag.
  - `ROL` / `ROR` / `RCL` / `RCR`: Bitwise rotations through carry.

### 2.2 AArch64 Architecture (`PSTATE.C`)
- **Bit Position**: Bit 29 of Condition Flags (`NZCV`) in `PSTATE` / `CPSR`.
- **Semantics**: Set to `1` when an addition results in an unsigned overflow (carry-out). For subtraction, `C = 1` indicates **no borrow** (inverted borrow semantics standard in ARM ISA).
- **Assembly Primitives**:
  - `ADCS Xd, Xn, Xm`: Add with carry and update condition flags.
  - `SBCS Xd, Xn, Xm`: Subtract with carry and update condition flags.

### 2.3 RISC-V 64 Architecture
- RISC-V does not possess a hardware status register with a dedicated Carry Flag.
- Carry generation is computed explicitly via comparison instructions (`SLTU rd, rs1, rs2`), enabling branchless carry propagation in multi-precision arithmetic chains.

---

## 3. Kernel Carry Propagation Subsystems in SigmaOS

### 3.1 Multi-Word Bignum & PQC Cryptography (`klib`)
Post-Quantum Cryptographic routines (Dilithium-5, Kyber-1024, Falcon-1024) rely on 256-bit and 512-bit modular arithmetic. `klib` implements zero-allocation multi-word addition and subtraction using Rust's `overflowing_add` and `overflowing_sub`:

```rust
pub fn add_256_with_carry(a: &[u64; 4], b: &[u64; 4], result: &mut [u64; 4]) -> bool {
    let mut carry = false;
    for i in 0..4 {
        let (sum1, c1) = a[i].overflowing_add(b[i]);
        let (sum2, c2) = sum1.overflowing_add(carry as u64);
        result[i] = sum2;
        carry = c1 || c2;
    }
    carry // Returns final carry-out
}
```

### 3.2 Hardware ALU Emulation (`src/drivers/dde.rs` & `src/compatibility/fedora.rs`)
In driver translation and binary compatibility layers, virtual CPU state tracks the carry flag (`cf` or `flags.carry`):

- **Addition**:
  ```rust
  let (res, carry) = op1.overflowing_add(op2);
  self.flags.carry = carry;
  ```
- **Subtraction / Comparison**:
  ```rust
  let (res, carry) = op1.overflowing_sub(op2);
  self.flags.carry = carry; // Indicates unsigned borrow
  ```
- **Shift & Rotate Operations**:
  For logical shift right (`SHR`) or shift left (`SHL`), the bit shifted out of the operand is captured and stored into `flags.carry`.

### 3.3 Kernel High-Precision Timekeeping (`src/klib/time_impl.rs`)
Timekeeping primitives (`SigmaTime`, `SigmaDuration`) normalize nanosecond accumulations into seconds via carry calculation:

```rust
let carry = if new_nanos >= 1_000_000_000 { 1 } else { 0 };
SigmaTime {
    seconds: new_seconds + carry as u64,
    nanos: new_nanos % 1_000_000_000,
}
```

---

## 4. AI Agent Architectural Guidelines for Carry Flags

1. **Explicit Carry Tracking**: AI agents generating or modifying kernel assembly, emulator ALUs, or bignum routines must explicitly handle carry flags without relying on implicit compiler behavior.
2. **Zero-Allocation Enforcement**: All carry propagation functions in `klib` and kernel subsystems must operate with `$O(1)$` stack space and zero dynamic heap allocation (`#![no_std]`).
3. **ARM vs. x86 Carry Semantics Awareness**: AI agents writing cross-architecture translation layers must account for ARM's inverted borrow flag convention (`C=1` means no borrow in `SBCS`) versus x86_64 (`CF=1` means borrow in `SBB`).
4. **Static eBPF Verification**: When generating eBPF bytecode instructions (`EbpfInstruction`), agents must verify that arithmetic operations involving carry do not cause unverified 64-bit integer wraparound.

---
*Maintained by the SigmaOS Kernel & Core Architecture Steering Committee.*
