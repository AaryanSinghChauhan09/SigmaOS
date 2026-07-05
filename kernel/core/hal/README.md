# SigmaOS Hardware Abstraction Layer (HAL)

The SigmaOS HAL provides a clean, zero-overhead interface isolating architecture-specific assembly stubs (`x86`, `ARM`, `RISC-V`) from the microkernel core.

## Mechanism

- `hal.h` / `sigma_hal.h`: Exposes generic hardware operations (`cpu_halt`, `timer_init`, `interrupt_init`, `mmu_map`, `read_io`, `write_io`).

- `hal_x86.S`, `hal_arm.S`, `hal_riscv.S`: Minimal assembly entry points executed early during bootstrap.
