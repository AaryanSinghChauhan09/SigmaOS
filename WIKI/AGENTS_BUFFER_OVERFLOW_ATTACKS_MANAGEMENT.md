# AI Agent Guidelines: Buffer Overflow Attacks Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Buffer Overflow Attacks Management**, memory corruption mitigations, stack canary protections, DEP/W^X (Data Execution Prevention / Write XOR Execute) enforcement, ASLR (Address Space Layout Randomization), bounds-checked raw byte slice operations, safe arithmetic overflow prevention, and corrupted redzone detection in SigmaOS.

SigmaOS leverages Rust's memory safety guarantees alongside zero-dependency `#![no_std]` runtime guards to prevent stack smashing, heap overflows, out-of-bounds pointer writes, and integer-overflow-induced memory corruptions.

---

## 1. Buffer Overflow Mitigation Subsystems

AI agents interacting with security and memory safety in SigmaOS must interface with the following core subsystems:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Input Validation Module** | `src/security/input_validation.rs` | Prevents input-driven buffer overflows via strict slice length bounds (`MAX_PATH_LEN`, `MAX_COMMAND_LEN`) and checked arithmetic (`safe_add`, `safe_sub`, `safe_mul`). |
| **Kernel Hardening & Guard Pages** | `src/security/kernel_hardening.rs` | Hardened guard page allocations (`MemoryAccessError::BufferOverflow`) protecting kernel stacks and heap boundaries. |
| **Vulnerability & Canary Protection** | `src/security/vulnerability.rs` | Stack canary verification and vulnerability classification (`VulnerabilityClass::BufferOverflow`). |
| **Redzone Allocation Guard** | `src/system/memory.rs` | Verifies canary-protected redzones surrounding dynamic allocations (`corrupted_redzone`). |
| **Custom ASLR & Guard Allocator** | `src/klib/custom_allocator.rs` | ASLR guard pages and randomized stack frame offset allocations. |

---

## 2. Defence-in-Depth Mitigation Standards

AI agents must enforce the following 5-layer defence-in-depth security model when writing or reviewing code:

```
+-------------------------------------------------------------------+
| Layer 1: Compile-Time Bounds Checking (Rust Primitive Slices)    |
+-------------------------------------------------------------------+
                                 |
+-------------------------------------------------------------------+
| Layer 2: Safe Checked Arithmetic (safe_add, safe_mul)             |
+-------------------------------------------------------------------+
                                 |
+-------------------------------------------------------------------+
| Layer 3: Stack Guard Canaries & Redzones                          |
+-------------------------------------------------------------------+
                                 |
+-------------------------------------------------------------------+
| Layer 4: Hardened Non-Executable Stack (DEP/W^X)                  |
+-------------------------------------------------------------------+
                                 |
+-------------------------------------------------------------------+
| Layer 5: Kernel Memory Isolation & Guard Pages (ASLR)             |
+-------------------------------------------------------------------+
```

### 1. Bounds-Checked Slice Operations
- **Rule:** Never use unchecked raw pointer offsets or indexing without explicit length checks.
- Always validate slice lengths before copy operations (`input.len() <= target.len()`).

```rust
// Standard input bounds validation in SigmaOS
pub fn validate_path(path: &[u8]) -> Result<(), ValidationError> {
    if path.is_empty() {
        return Err(ValidationError::EmptyInput);
    }
    if path.len() > MAX_PATH_LEN {
        return Err(ValidationError::TooLong);
    }
    // ...
    Ok(())
}
```

### 2. Checked Integer Arithmetic
- Prevent buffer overflow vulnerabilities caused by integer wraparound in allocation size calculations.
- Use `safe_add`, `safe_sub`, `safe_mul`, or Rust's built-in `checked_add`, `checked_mul`, and `saturating_add`.

```rust
// Checked size calculation preventing allocation wrap
let total_bytes = safe_mul(item_count, item_size)
    .ok_or(ValidationError::Overflow)?;
```

### 3. Redzones & Canary Verification
- Dynamic allocation blocks MUST include corrupted redzone detection (`corrupted_redzone()`).
- On buffer overflow detection, trigger an immediate kernel security fault rather than allowing execution to continue.

---

## 3. Vulnerability Remediation Protocols

When responding to a reported buffer overflow issue:
1. **Source Root Cause Analysis:** Locate the un-bounded buffer copy or integer overflow in allocation sizing.
2. **Apply Checked Bounds:** Replace vulnerable index operations with checked methods (`get()`, `checked_add()`).
3. **Verify Canary Integrity:** Ensure stack guard canary integrity remains intact.
4. **Never Modify Binaries:** Modify source files under `src/`, then re-compile and test.

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes touching security, memory allocation, or raw input validation:

- [ ] Are all buffer copy operations bounded by slice capacity checks?
- [ ] Are allocation size calculations protected against integer overflow?
- [ ] Are stack canaries and non-executable stack flags (DEP/W^X) active?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
