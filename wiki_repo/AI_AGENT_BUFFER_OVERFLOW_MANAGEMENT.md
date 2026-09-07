# AI Agent Basic Buffer Overflow Management in SigmaOS

## Overview
SigmaOS incorporates a multi-tier Buffer Overflow Defense Subsystem governed by autonomous AI Agents (**Sentinel** 🛡️, **Bolt** ⚡, **Palette** 🎨). This document defines operational protocols, memory safety invariants, mitigation techniques, and inspection tools for AI agents managing buffer overflow risks across bare-metal kernel primitives, unsafe code blocks, dynamic C/C++ ABI wrappers, and userland binaries.

AI agents interact directly with `src/security/input_validation.rs`, `src/security/binary_protection.rs` (`BinaryProtectionManager`), `src/memory/pmm_vmm.rs`, and `src/klib/ring_buffer.rs`.

---

## 1. Buffer Overflow Defense Frameworks & Architecture

### 1.1 Compile-Time & Language Level Memory Safety
SigmaOS core modules are written in bare-metal Rust, providing compile-time ownership, lifetime bounds, and slice bounds checking. Unsafe Rust blocks (`unsafe { ... }`) are subject to mandatory AI Agent security audits:
* **Slice Index Verification**: Raw pointer conversions (`*const u8`, `*mut u8`) must verify offset bounds (`ptr.add(idx) < ptr.add(cap)`) prior to dereferencing.
* **Bounded String Conversions**: C-string functions (`from_utf8`, `from_raw_parts`) must enforce explicit max-length bounds (`MAX_INPUT_LEN`).

### 1.2 Binary Protection & Hardening Techniques
Implemented in `src/security/binary_protection.rs` (`BinaryProtectionManager`):
* **Stack Smashing Protection (SSP / Stack Canaries)**: Injects randomized 64-bit stack canary values (`__stack_chk_guard`) prior to function frame execution. Stack canary corruptions immediately trigger `SIGSEGV` / kernel panic recovery.
* **Write XOR Execute (W^X / NX-Bit)**: Physical memory pages mapped via `BitmapFrameAllocator` (`src/memory/pmm_vmm.rs`) enforce non-executable stack and heap pages (`NO_EXECUTE` bit).
* **Address Space Layout Randomization (ASLR)**: Randomizes stack, heap, and memory-mapped segment base addresses with unmapped ASLR guard pages on both boundaries.
* **Position-Independent Executables (PIE)**: Enforces relocatable code layout for all userland binaries and dynamic modules.

### 1.3 Input Validation & Length Limits
Implemented in `src/security/input_validation.rs`. Enforces strict maximum length limits on network payloads, IPC messages, file paths, and environment variables, rejecting oversized inputs prior to memory copying.

---

## 2. AI Agent Operational Directives & Audit Workflows

### 2.1 Unsafe Code Block Audit Protocol
1. **Pointer Bounds Check Inspection**:
   When **Sentinel** 🛡️ audits `unsafe` blocks, it verifies that raw pointer arithmetic is bounded by explicit slice length checks (`offset < length`).
2. **Buffer Copy Primitive Enforcement**:
   Agents enforce the use of `copy_from_slice()` or `copy_nonoverlapping()` with explicit, pre-calculated length bounds over manual byte-by-byte loop writes.
3. **Array Index Wrapping Arithmetic**:
   In circular queues (`RingBuffer<T, CAP>`), indices must wrap using bitwise AND (`w & (CAP - 1)`) with power-of-two capacities, preventing out-of-bounds array access.

### 2.2 Binary Protection Verification
Before executing foreign or userland ELF binaries, security agents invoke `BinaryProtectionManager::inspect_elf()`:
- Check 1: Stack canary enabled (`SSP`).
- Check 2: Non-executable stack enabled (`NX`).
- Check 3: Position-Independent Executable (`PIE`).
- Check 4: Read-Only Relocations (`RELRO / Full RELRO`).

If any protection is missing, the agent constructs an isolated Landlock/Pledge sandbox before execution.

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Audit binary protection features (SSP, NX, PIE, RELRO) on binary
sigma-overflow audit-binary --file /usr/bin/sigpkg

# Scan Rust unsafe blocks for pointer bounds verification
sigma-overflow audit-unsafe --dir src/klib/

# Benchmark stack canary overhead and memory guard page alignment
sigma-overflow bench-canary
```
