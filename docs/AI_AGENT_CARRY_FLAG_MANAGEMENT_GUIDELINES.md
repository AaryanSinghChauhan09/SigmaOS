# SigmaOS AI Agent Carry Flag Management Guidelines

## 1. Overview
This document provides developer guidelines and operational rules for AI coding agents (such as Jules) working with Carry Flag (CF) management, multi-word bignum arithmetic, ALU hardware emulation, and kernel timekeeping in SigmaOS.

## 2. Core Guidelines for AI Coding Agents

### 2.1 Bignum & Multi-Word Arithmetic
- **Use `overflowing_add` / `overflowing_sub`**: When chaining multi-word additions (e.g. 128-bit, 256-bit, 512-bit integers), AI agents must use Rust's `overflowing_add` and `overflowing_sub` methods to retrieve both the result and boolean carry output.
- **Carry Chain Order**: Always process multi-word integers from least-significant word (LSB) to most-significant word (MSB) in ascending index order.
- **No Heap Allocations**: Bignum carry routines must accept stack-allocated array references (`&[u64; N]`) to comply with `#![no_std]` zero-dependency rules.

### 2.2 Emulator & Compatibility ALU Flag Updates
- **DDE and Fedora Compatibility Modules**: When implementing CPU emulator instructions in `src/drivers/dde.rs` or `src/compatibility/fedora.rs`, AI agents must update `flags.carry` (or `self.cf`) on every ALU instruction (ADD, SUB, ADC, SBB, MUL, SHL, SHR, ROR, ROL).
- **Shift/Rotate Carry Extraction**: For shift/rotate operations, extract the bit shifted out of the boundary and assign it to `carry`:
  ```rust
  let carry = if shift > 0 { (val >> (shift - 1)) & 1 != 0 } else { false };
  self.update_flags(res, false, carry);
  ```

### 2.3 Subsystem Interoperability
- **Time Nanos Carry**: When updating nanosecond counters in timekeeping primitives (`src/klib/time_impl.rs`), ensure nanoseconds exceeding `1_000_000_000` carry over to seconds (`seconds + (nanos / 1_000_000_000)`).
- **PQC Cryptographic Primitives**: Multi-precision modular reduction in Post-Quantum Cryptography requires carry propagation. Verify that intermediate carry bits are cleared before returning public key or signature results.

### 2.4 Diagnostic Inspection & Testing
- **Unit Verification**: Any new arithmetic function or ALU instruction modification must include unit tests verifying carry flag set and clear conditions (`assert!(flags.carry)` and `assert!(!flags.carry)`).
- **Test Suite Execution**: All changes must be verified using `./run_sigma_tests.sh`.

---
*Maintained by the SigmaOS Kernel & AI Agent Governance Steering Committee.*
