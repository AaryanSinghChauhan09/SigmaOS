# AI Agent Carry Flag Management Guide

## Overview
This wiki guide details the Carry Flag (CF) management protocols for AI coding agents operating on SigmaOS. It covers hardware CPU status registers, multi-word bignum arithmetic, ALU instruction emulation in compatibility layers, and zero-allocation carry propagation patterns.

## Hardware Carry Flag Conventions
- **x86_64 (`RFLAGS.CF`)**: Bit 0 of `RFLAGS`. Set on unsigned addition overflow or subtraction borrow.
- **AArch64 (`PSTATE.C`)**: Bit 29 of `NZCV` in `PSTATE`. Set on unsigned addition overflow; set on subtraction when NO borrow occurs (inverted borrow convention).
- **RISC-V 64**: Computed explicitly via set-less-than instructions (`sltu`).

## Carry Propagation Code Patterns in SigmaOS
```rust
// Bignum 256-bit addition with carry chain
pub fn add_256_with_carry(a: &[u64; 4], b: &[u64; 4], result: &mut [u64; 4]) -> bool {
    let mut carry = false;
    for i in 0..4 {
        let (sum1, c1) = a[i].overflowing_add(b[i]);
        let (sum2, c2) = sum1.overflowing_add(carry as u64);
        result[i] = sum2;
        carry = c1 || c2;
    }
    carry
}
```

## ALU Emulation Flag Updates (`src/drivers/dde.rs` / `src/compatibility/fedora.rs`)
```rust
let (res, carry) = op1.overflowing_add(op2);
self.flags.carry = carry;
```

## Related Documents
- `docs/AI_AGENT_CARRY_FLAG_MANAGEMENT_ARCHITECTURE.md`
- `docs/AI_AGENT_CARRY_FLAG_MANAGEMENT_GUIDELINES.md`
- `wiki/AI_AGENTS_GUIDE.md`
