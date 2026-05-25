# Hardware Abstraction Layer (HAL)

The **SovereignHAL** provides a single, architecture-agnostic interface that
the kernel uses to talk to hardware. Porting SigmaOS to a new CPU means
implementing one HAL backend — nothing else needs to change.

## Supported Targets
| Architecture | Status |
|---|---|
| x86_64 | ✅ Active |
| AArch64 (ARM64) | 🔧 In-progress |
| RISC-V RV64GC | 📋 Planned |

## Core Abstractions
```c
void hal_init(void);
void hal_set_irq_handler(uint32_t vec, void (*fn)(void));
void hal_flush_tlb(void);
uint64_t hal_get_timestamp_ns(void);
```

## Roadmap
- [ ] AArch64 MMU backend
- [ ] RISC-V SBI wrapper
- [ ] ACPI/DTB discovery
