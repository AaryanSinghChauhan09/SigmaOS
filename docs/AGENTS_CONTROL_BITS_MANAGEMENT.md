# AI Agent Guidelines: Control Bits Operation Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Control Bits Operation Management**, x86/x64 Control Registers (CR0, CR3, CR4), ARM System Control Registers (`sctlr_el1`), hardware bitfield masking, CPU feature flag management, and supervisor execution/access protection in SigmaOS.

SigmaOS manages control bits via zero-dependency `#![no_std]` bitwise abstractions to guarantee atomic processor mode transitions, hardware-enforced memory isolation, and safe kernel execution.

---

## 1. Control Bits & Register Subsystems

AI agents interacting with control registers in SigmaOS must interface with the following architectural modules:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Control Register Emulation (`write_cr0`, `write_cr4`)** | `src/kernel/gap_closing.rs` | Simulates writes to x86 CR0 (PE, WP) and CR4 (SMEP, SMAP, PGE) with compliance checking. |
| **Multi-Arch Architecture Context** | `src/kernel/architecture.rs` | Manages CPU control registers (`cr0`, `cr4`, `sctlr_el1`) across x86, ARM64, RISC-V, and LoongArch ports. |
| **Virtual CPU Execution (`VirtualCpu`)** | `src/kernel/virtual_cpu.rs` | Handles protected mode enable (`PE` bit) and paging enable (`PG` bit) state transitions. |
| **CR4 SMEP/SMAP Hardening** | `src/security/kernel_hardening.rs` | Simulates hardware Supervisor Mode Execution (`SMEP`) and Access (`SMAP`) Prevention via CR4 bits 20 & 21. |

---

## 2. Key x86/x64 Control Register Bit Definitions

```
CR0 Register Bit Map:
  Bit 0  (PE) : Protection Enable (1 = Protected/Long Mode active)
  Bit 16 (WP) : Write Protect (1 = Supervisor cannot write to read-only user pages)
  Bit 31 (PG) : Paging Enable (1 = Virtual memory paging active)

CR4 Register Bit Map:
  Bit 7  (PGE)  : Page Global Enable
  Bit 20 (SMEP) : Supervisor Mode Execution Prevention (1 = Faults if Ring 0 executes Ring 3 instructions)
  Bit 21 (SMAP) : Supervisor Mode Access Prevention (1 = Faults if Ring 0 accesses Ring 3 data without STAC)
```

---

## 3. Control Bit Manipulation Protocols

When updating or writing control bits to CPU or hardware registers:

### 1. Read-Modify-Write Rule
- **Rule:** Never perform blind writes to control registers. Read the current register value, apply explicit bitwise OR (`|`) or bitwise AND-NOT (`& !`), and write back.

```rust
// Modifying CR4 bits safely in SigmaOS gap closing module
pub fn write_cr4(&mut self, val: u64) {
    self.cr4_pge = (val & (1 << 7)) != 0;
    self.cr4_smep = (val & (1 << 20)) != 0;
    self.cr4_smap = (val & (1 << 21)) != 0;
}
```

### 2. Mandatory Protection Flags
- Operating in Ring 0 MUST keep `WP` (Write Protect = bit 16), `SMEP` (bit 20), and `SMAP` (bit 21) set to prevent kernel exploitation via user-space buffer injections.

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing changes to control register or CPU feature bit manipulation logic:

- [ ] Are reserved bits in control registers masked off during bitwise updates?
- [ ] Are SMEP (bit 20) and SMAP (bit 21) enabled by default in simulated CR4 writes?
- [ ] Do unit tests verify that bit toggling correctly updates individual feature flags without side effects?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
