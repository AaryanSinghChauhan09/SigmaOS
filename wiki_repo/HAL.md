# SigmaOS Hardware Abstraction Layer (HAL)

The SigmaOS HAL establishes **Zero-Dependency Architecture Portability**, enabling the OS to run identically on x86_64, ARM, and RISC-V.

## Design
*   `hal.hpp`: The abstract C++ interface.
*   `hal_x86.cpp`: Direct x86 assembly implementations (`outb`, `inb`).
*   `hal_arm.cpp`: ARM memory-mapped I/O.
*   `hal_riscv.cpp`: RISC-V specific CSR reads.

The `SovereignRegistry` handles declarative binding, allowing the kernel to boot universally without hard-coded CPU logic.
