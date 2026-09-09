# SigmaOS Sovereign Multi-Architecture HAL

## Overview

The `src/arch/sovereign_multiarch_hal.rs` module provides a unified Hardware Abstraction Layer
across all major CPU architectures, inspired by Linux's `arch/` subsystem and BSD's `sys/machine/` tree.

## Supported Architectures

| Architecture | ISP | Word Size | Endianness | Status |
|---|---|---|---|---|
| x86_64 (AMD64/Intel 64) | CISC | 64-bit | Little | ✅ Primary Target |
| x86_32 (IA-32) | CISC | 32-bit | Little | ✅ Supported |
| x86_16 | CISC | 16-bit | Little | 🔧 Planned |
| AArch64 (ARM64) | RISC | 64-bit | Little/Big | ✅ Supported |
| ARM32 (ARMv7) | RISC | 32-bit | Bi-endian | ✅ Supported |
| RISC-V 64 (RV64G) | RISC | 64-bit | Little | ✅ Supported |
| RISC-V 32 (RV32I) | RISC | 32-bit | Little | 🔧 Planned |
| MIPS 64 | RISC | 64-bit | Bi-endian | 🔧 Planned |
| PowerPC 64 | RISC | 64-bit | Bi-endian | 🔧 Planned |
| LoongArch64 | RISC | 64-bit | Little | 🔧 Planned |

## Key Components

### CPU Feature Flags (`CpuFeatureFlags`)
Captures per-CPU capabilities including FPU, SIMD/NEON/AVX, hardware virtualization (VT-x/AMD-V/ARM VHE),
memory encryption (AMD SME/SEV, Intel TME), TEE support, Spectre mitigations (IBRS, SSBD),
CET shadow stack, ARM PAC/BTI, and cache topology.

### Architecture-Specific Register Contexts
- `X86_64Regs` — Matches Linux `struct pt_regs` layout (System V ABI compliant)
- `Arm64Regs` — AArch64 general registers x0-x30, SP, PC, PSTATE
- `RiscV64Regs` — RISC-V RV64G register file (ra, sp, gp, tp, s0-s11, a0-a7, t0-t6)

### CPU Topology (`CpuTopology`)
Models NUMA nodes, physical cores, SMT threads, and big.LITTLE/DynamIQ cluster configurations.
Supports `Performance`, `Efficiency`, `Balanced`, and `RealTime` core types.

### Memory Map (`MemRegion`, `MemRegionType`)
Inspired by UEFI `MemoryType` and Linux `memblock`. Covers Usable RAM, Reserved, ACPI,
MMIO, Persistent Memory (NVDIMM), and Kernel code/data regions.

### Boot Parameters (`SovereignBootParams`)
Unified boot parameter structure covering:
- x86: `boot_params` / ACPI RSDP
- ARM64: Device Tree Blob (DTB) address
- RISC-V: SBI firmware handoff
- All: UEFI System Table, KASLR offset, initrd range

## CISC vs RISC Philosophy

SigmaOS follows the principle that the OS should be architecture-agnostic at higher layers
while maximizing native instruction efficiency at the hardware boundary:

- **CISC (x86)**: Exploit complex addressing modes for in-kernel tight loops, leverage AVX-512 for bulk memory operations
- **RISC (ARM64/RISC-V)**: Use load-store architecture naturally; no hidden microcode = deterministic latency for RT tasks
- **All**: Zero-dependency `#![no_std]` Rust ensures identical kernel behavior regardless of ISA

## Linux/BSD Inspiration

| SigmaOS Component | Linux Equivalent | BSD Equivalent |
|---|---|---|
| `sovereign_multiarch_hal.rs` | `arch/x86/`, `arch/arm64/` | `sys/amd64/`, `sys/arm64/` |
| `X86_64Regs` | `struct pt_regs` | `struct trapframe` |
| `MemRegion` | `struct memblock_region` | `struct phys_avail[]` |
| `CpuTopology` | `cpu_topology` / ACPI PPTT | `struct cpu_info` |
| `AbstractPte` | `pte_t` / `pgprot_t` | `pt_entry_t` |
| `SovereignBootParams` | `struct boot_params` | `struct preloaded_metadata` |

## See Also
- [Architecture Design](ARCHITECTURE.md)
- [Kernel Design](Kernel-Architecture.md)
- [Memory Management](memory-management.md)
