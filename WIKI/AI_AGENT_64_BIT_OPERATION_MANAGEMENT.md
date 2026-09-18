# 💻 AI Agent 64-Bit Operation Management in SigmaOS

## Executive Summary
SigmaOS is designed as a native 64-bit operating system architecture spanning x86_64 (AMD64 / Intel 64), AArch64 (ARM 64-bit), and RISC-V 64-bit (`rv64gc`). Autonomous AI Agents (**Bolt ⚡**, **Palette 🎨**, and **Sentinel 🛡️**) managing kernel HAL abstractions (`src/kernel/architecture.rs`), virtual memory paging, CPU register state, 64-bit arithmetic operations, and legacy 32-bit binary compatibility execution layers must enforce strict 64-bit alignment and memory safety invariants.

---

## 1. Supported 64-Bit CPU Architectures

```
+-------------------------------------------------------------+
|                SigmaOS 64-Bit Hardware HAL                  |
+-------------------------------------------------------------+
                              │
     ┌────────────────────────┼────────────────────────┐
     ▼                        ▼                        ▼
 x86_64 Long Mode         AArch64 (ARMv8/v9)        RISC-V 64-Bit
 (4-Level / 5-Level)      (TTBR0 / TTBR1 VMM)     (Sv39 / Sv48 Paging)
```

1. **x86_64 Long Mode**: Enforces 64-bit flat address space, 16 general-purpose 64-bit registers (`RAX`..`R15`), 4-level (PML4) or 5-level (PML5) paging, AVX-512 vector registers, and `SYSCALL`/`SYSRET` fast system call instructions.
2. **AArch64**: Utilizes 31 64-bit registers (`X0`..`X30`), 64-bit program counter (`PC`), dual translation tables (`TTBR0_EL1` user, `TTBR1_EL1` kernel), NEON/SVE vector SIMD, and `SVC` instruction traps.
3. **RISC-V 64-Bit (`rv64gc`)**: Provides 32 64-bit registers (`x0`..`x31`), Sv39 (39-bit virtual address) or Sv48 paging via `satp` register, vector extensions, and `ECALL` supervisor system call instructions.

---

## 2. 64-Bit Virtual Memory Layout & Canonical Addressing

To maintain hardware memory management unit (MMU) invariants:
- **Canonical Address Boundary**: In 48-bit virtual addressing (x86_64 PML4), bits 47 through 63 MUST be sign-extended copies of bit 47:
  - **User Space**: `0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF` (Lower Canonical Half).
  - **Kernel Space**: `0xFFFF_8000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF` (Higher Canonical Half).
- **Non-Canonical Faults**: Accessing a non-canonical address triggers a General Protection Fault (`#GP`) or Data Abort fault handled by MMU recovery routines.

---

## 3. 64-Bit Data Types & Struct Alignment

In the 64-bit LP64 data model (`long` and pointers are 64 bits):
- `u64` / `usize`: 8 bytes, alignment = 8 bytes.
- `u128`: 16 bytes, alignment = 16 bytes (used for 128-bit atomic CAS operations and post-quantum cryptographic primitives).
- **Structure Padding**: Struct fields MUST be ordered by descending alignment size to eliminate implicit memory padding overhead.

---

## 4. AI Agent Operational Guidelines

1. **Bolt ⚡ (Performance Optimization)**:
   - Leverage native 64-bit word operations and 64-bit atomic instructions (`AtomicU64`) to perform fast lock-free atomic updates without spinlock overhead.
   - Utilize 128-bit / 256-bit SIMD registers (`YMM`/`ZMM`/`NEON`) for bulk `copy_from_slice` memory transfers.

2. **Palette 🎨 (UX & Platform)**:
   - Ensure system monitoring utilities correctly handle 64-bit integer values when rendering memory sizes (`> 4 GB`) and uptime metrics without integer overflow truncation.

3. **Sentinel 🛡️ (Security & Hardening)**:
   - Enforce 64-bit Address Space Layout Randomization (ASLR) with 36+ bits of entropy to protect against buffer overflow exploits.
   - Enforce NX (No-Execute) / XD (Execute-Disable) page protection flags across all 64-bit userland data pages.
