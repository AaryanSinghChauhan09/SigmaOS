# Cross-Platform Kernel Hardware Abstraction Layer (HAL)

## Overview

SigmaOS implements a modular **Hardware Abstraction Layer (HAL)** in [`src/arch/hal.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/arch/hal.rs) and [`src/compatibility/cross_platform_kernel.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/compatibility/cross_platform_kernel.rs), isolating architecture-specific machine details from the generic OS kernel.

***

## Target Architectures Supported

*   **x86\_64 / AMD64**: Long mode, 4-level/5-level paging, APIC/IOAPIC, AVX-512, CET.
*   **AArch64 / ARM64**: Exception Levels (EL0-EL2), GICv3/v4, NEON/SVE, Pointer Authentication (PAC).
*   **RISC-V (RV64GC)**: Sv39/Sv48 MMU, PLIC/AIA, Vector extension.

***

## Core HAL Interfaces

```rust
pub trait ArchitectureHal {
    /// Initialize MMU and switch to new page table root
    unsafe fn switch_address_space(&mut self, root_page_table: u64);

    /// Enable / disable hardware interrupts atomically
    fn set_interrupt_state(&self, enabled: bool) -> bool;

    /// Read monotonic hardware cycle counter
    fn read_monotonic_cycles(&self) -> u64;

    /// Invalidate TLB entry for specific virtual address
    unsafe fn invalidate_tlb_page(&self, vaddr: usize);
}
```

***

## Architectural Isolation Flow

    Generic Kernel Core (VFS, Scheduler, Networking, Sockets)
                               │
                               ▼
              [Architecture HAL Abstraction Boundary]
              ├── x86_64 HAL (APIC, PML4, CR3, CPUID)
              ├── AArch64 HAL (GICv3, TTBR0_EL1, PAC)
              └── RISC-V HAL (PLIC, satp, sstatus)
