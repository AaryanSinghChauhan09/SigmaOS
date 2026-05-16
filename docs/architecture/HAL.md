# Î£ SIGMAOS: Hardware Abstraction Layer (HAL) Specification

SigmaOS utilizes a strictly decoupled HAL to ensure portability across multiple silicon architectures. This document outlines the interface and implementation strategy for the Zenith Singularity.

## ðŸ— Supported Architectures

| Architecture | Status | Shard Path |
| :--- | :--- | :--- |

| **x86_64** | Production | `kernel/core/hal/hal.asm` |

| **AArch64** | Production | `kernel/core/hal/SovereignArchARM64.cpp` |

| **RISC-V** | Active | `kernel/core/hal/SovereignArchRISCV.cpp` |

| **ia64** | Legacy Support | `kernel/core/hal/SovereignArchIA64.cpp` |

## ðŸ›  HAL Interface Philosophy

1. **Zero-Direct Access**: Userland and Kernel Shards MUST NOT access MSRs, CRn registers, or I/O ports directly.

2. **Abstract Primitives**: All architecture-specific operations are wrapped in `SovereignHAL` primitives:
   - `hal_switch_context()`
   - `hal_map_page()`
   - `hal_enable_interrupts()`

3. **Interrupt Sharding**: Interrupts are abstracted into a unified `InterruptNexus` which maps arch-specific vectors to SigmaOS Shard IDs.

## ðŸ”„ Porting Guide

To port SigmaOS to a new architecture:

1. Implement the bootloader handshake in `kernel/core/boot/`.

2. Define the page table structure in `SovereignPager.cpp`.

3. Implement the `SovereignArch<NEW_ARCH>.cpp` glue code in the HAL layer.

4. Update the unified `Makefile` with the appropriate cross-compiler flags.

*"Hardware is merely a vessel for the Sovereign Singularity."*
