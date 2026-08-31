# Σ ext/hal — Hardware Abstraction Layer (SovereignHAL)

Provides a **single, architecture-agnostic interface** that the kernel uses to
communicate with hardware. Porting SigmaOS to a new CPU means implementing one
HAL backend — nothing else needs to change.

## Source Files

| File | Description |
|---|---|
| `hal.rs` | Core HAL trait definitions and dispatch table |
| `hw_detect.rs` | Runtime CPU / ACPI / DTB hardware discovery |
| `accel_hal.rs` | Hardware accelerator HAL (GPU compute / NPU / DSP) |

## Supported Targets

| Architecture | Status |
|---|---|
| x86_64 | ✅ Active |
| AArch64 (ARM64) | 🔧 In-progress |
| RISC-V RV64GC | 📋 Planned |

## API Interface

```c
// Initialise the HAL for the detected platform
void hal_init(void);

// Bind an interrupt vector to a kernel handler
void hal_set_irq_handler(uint32_t vec, void (*fn)(void));

// Flush the CPU TLB (all cores)
void hal_flush_tlb(void);

// Read a nanosecond-precision hardware timestamp
uint64_t hal_get_timestamp_ns(void);

// Map a physical address range into kernel virtual space
void *hal_map_phys(uint64_t phys, size_t size, uint32_t flags);

// Detect platform (returns SIGMA_ARCH_X86_64 / AARCH64 / RISCV64)
sigma_arch_t hal_detect_arch(void);
```

## Hardware Discovery

`hw_detect.rs` interrogates ACPI RSDP / MADT / SRAT on x86_64 and the
Flattened Device Tree on ARM/RISC-V to build a unified topology map:

```
ACPI RSDP → XSDT → MADT   (interrupt routing)
                  → SRAT   (NUMA node topology)
                  → MCFG   (PCIe ECAM base)
```

## Roadmap

- [x] x86_64 HAL backend (TSC, LAPIC, IOAPIC)

- [x] Hardware discovery (`hw_detect.rs`)

- [x] Accelerator HAL stub (`accel_hal.rs`)

- [ ] AArch64 GIC-v3 interrupt controller

- [ ] RISC-V PLIC / CLINT integration

- [ ] ACPI Power Management (S3/S4 sleep states)

- [ ] Secure Enclave HAL (SGX / TrustZone)

## Related Modules

- [`modules/core/drivers`](../../core/drivers/README.md) — Drivers that use HAL

- [`modules/core/kernel`](../../core/kernel/README.md) — Kernel that drives HAL
