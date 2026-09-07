# AI Agents Buffer Overflow Management Specification for SigmaOS

## Abstract
This specification defines the buffer overflow protection and memory safety architecture for AI agents developing, compiling, or executing code within SigmaOS. SigmaOS mitigates spatial and temporal memory corruption risks (including stack smashing, heap overflows, use-after-free, and return-oriented programming) through zero-dependency Rust `#![no_std]` bounds checking, W^X (Write XOR Execute) memory pages, stack guard pages, KASLR address space randomization, and OpenBSD-inspired Retguard protection.

---

## 1. Memory Safety Architecture & Mitigation Layers

```
[ AI Agent Code Execution ]
           │
           ▼ (Rust Safe Slices & Bounds Checking)
[ Safe Buffer Access Layer ]
           │
           ▼ (Stack Guard Pages & Retguard Canary)
[ Stack Protection Layer ]
           │
           ▼ (W^X / DEP Page Permissions & KASLR)
[ Sovereign Kernel MMU Enforcement ]
```

---

## 2. Mitigation Mechanisms in SigmaOS

### 2.1 Safe Slice Abstractions & Strict Bounds Checking
- **No Unbounded Pointer Arithmetic**: Raw pointer arithmetic is forbidden in safe code. All buffer operations must use safe Rust slice abstractions (`&[u8]`, `&mut [u8]`, `alloc::vec::Vec`).
- **Checked Subslice Indexing**: Array and slice indexing is checked at compile-time or runtime. Out-of-bounds accesses trigger an immediate panic and process termination, preventing memory corruption propagation.

### 2.2 W^X (Write XOR Execute) & DEP Memory Protection
- **Page Permission Enforcement**: Memory pages marked writable (`PROT_WRITE`) cannot simultaneously be marked executable (`PROT_EXEC`) via `SovereignKaslrWxAllocator`.
- **Non-Executable Stack & Heap**: Stack and heap memory regions are configured as non-executable (`NX` / `XN` bit) at the Page Table Entry (PTE) level.

### 2.3 Stack Guard Pages & Shadow Stack Canaries
- **Stack Guard Pages**: Unmapped guard pages (non-present MMU pages) are placed at the top and bottom of kernel and userland thread stacks. Any stack overflow immediately triggers a Page Fault (`0x0E`) rather than overwriting adjacent memory.
- **OpenBSD Retguard Parity (`OpenBsdRetguardEngine`)**:
  - Function return addresses on the stack are protected with per-thread randomized XOR canaries.
  - Prior to executing `RET`, the canary is validated. Mismatches trigger an instant kernel panic (`RETGUARD_TRAP`).

### 2.4 Address Space Layout Randomization (KASLR)
- **Kernel & Userland ASLR**:
  - Base addresses for executable text, stack, heap, and mmap memory regions are randomized at process spawn and boot time.
  - Randomization entropy is sourced from `SovereignCsprng` (ChaCha20-inspired CSPRNG).

---

## 3. Directives for AI Agents Developing Driver & C/Assembly Code

1. **Unsafe Block Auditing**:
   - `unsafe` blocks in C/C++ FFI or assembly must include explicit invariants validating buffer length and alignment.
2. **Buffer Copy Functions**:
   - C-style `strcpy` or `sprintf` functions are prohibited. Native implementations must use bounded string copy functions (`strncpy_safe`) with explicit maximum capacity parameters.
3. **Automated Fuzzing & Static Analysis**:
   - Code changes undergo automated fuzz testing (`tests/test_stress_fuzz_bench.py`) with malformed payloads to verify memory robustness.

---

## 4. Wiki Synchronization

This document is synchronized across all documentation hubs via `./scripts/sync_wiki.sh`:
- `WIKI/AI_AGENTS_BUFFER_OVERFLOW_SPEC.md`
- `wiki/AI_AGENTS_BUFFER_OVERFLOW_SPEC.md`
- `wiki_repo/AI_AGENTS_BUFFER_OVERFLOW_SPEC.md`

---

*Specification Version: 1.0.0 — SigmaOS Memory Safety Architecture*
