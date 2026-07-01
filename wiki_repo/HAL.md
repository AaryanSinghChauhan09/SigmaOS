# SigmaOS Hardware Abstraction Layer (HAL)

The SovereignHAL abstracts all architecture-specific hardware operations into a uniform interface. The kernel never calls hardware directly — it always goes through the HAL.

---

## HAL Architecture

```
Kernel subsystems (scheduler, net, fs, drivers...)
    │  hal_xxx() calls
    ▼
SovereignHAL (hal/SovereignHAL.cpp)
    │
    ├── x86_64 backend  (hal/x86/)
    ├── ARM64 backend   (arch/arm64/)     ← Phase G
    └── RISC-V backend  (arch/riscv64/)   ← Phase H
```

---

## HAL Interfaces

### PCI/PCIe (`hal/sigma_pci.cpp`)
```cpp
// Enumerate PCI devices
sigma_pci_enumerate(pci_probe_callback);

// Read/write config space
uint32_t val = sigma_pci_read32(bus, dev, func, reg);
sigma_pci_write32(bus, dev, func, reg, val);

// Map a BAR
void* bar = sigma_pci_map_bar(device, bar_index);

// Enable MSI-X
sigma_pci_enable_msix(device, vectors, handlers);
```

### Interrupts
```cpp
// Request IRQ
sigma_irq_request(irq_num, handler, ctx);
sigma_irq_free(irq_num);

// Mask/unmask
sigma_irq_mask(irq_num);
sigma_irq_unmask(irq_num);

// APIC EOI
sigma_apic_eoi();
```

### Timers
```cpp
// Get monotonic timestamp (nanoseconds)
uint64_t now = sigma_time_ns();

// Set one-shot timer
sigma_timer_set_oneshot(delay_ns, callback, ctx);

// Set periodic timer (jiffies)
sigma_timer_set_periodic(interval_ns, callback, ctx);
```

### Memory-Mapped I/O
```cpp
// Read/write MMIO
uint32_t val = sigma_mmio_read32(addr);
sigma_mmio_write32(addr, val);

// Memory barrier
sigma_mb();   // full barrier
sigma_rmb();  // read barrier
sigma_wmb();  // write barrier
```

### Power Management
```cpp
// Set CPU frequency state
sigma_cpufreq_set(cpu_id, freq_hz);

// Enter low-power state
sigma_cpu_idle(idle_state);

// ACPI power state transition
sigma_acpi_enter_state(ACPI_S3);  // suspend
```

---

## Supported Architectures

### x86_64 (Primary — `arch/x86_64/`)
- PML4 4-level paging (`arch/x86_64/paging.asm`, `paging.c`)
- Context switch (`arch/x86_64/switch.asm`)
- Fast VMM paths (`arch/x86_64/vmm_fast.asm`)
- APIC + HPET timer
- ACPI MADT/SRAT/DSDT parsing
- MSR access (LSTAR, EFER, FS/GSBASE)

### ARM64 (`arch/arm64/`) — Phase G
- GIC (Generic Interrupt Controller)
- ARM MMU page table walker
- PSCI for power management
- BCM2711 BSP (Raspberry Pi 4)
- BCM2712 BSP (Raspberry Pi 5)
- NEON/SVE SIMD support

### RISC-V RV64GC (`arch/riscv64/`) — Phase H
- PLIC (Platform-Level Interrupt Controller)
- SBI (Supervisor Binary Interface) calls
- MMU Sv39/Sv48 page tables

---

## Boot Path (`arch/boot/`)

```asm
; arch/boot/sovereign_boot.asm
; Sets up:
;   - GDT (64-bit code/data segments)
;   - Initial page tables (identity map first 4 GB)
;   - Stack (8 KB per CPU)
;   - Jumps to sigma_kernel_main() in C++

; arch/boot/multiboot_header.asm
; Multiboot2 magic header for GRUB/limine bootloader
```

---

## HAL Status

| Component | Status |
|-----------|--------|
| x86_64 paging | ✅ Implemented |
| PCI enumeration | ✅ Implemented |
| PCIe MSI-X | ✅ Implemented |
| APIC init | ⬜ Phase G |
| HPET timer | ⬜ Phase G |
| ACPI parsing | 🔄 Partial |
| ARM64 GIC | ⬜ Phase G |
| ARM64 MMU | ⬜ Phase G |
| RISC-V PLIC | ⬜ Phase H |
| Power governor | ✅ Implemented |

---

*See also: [Architecture-Overview](Architecture-Overview) · [Kernel](Kernel) · [Driver-Development](Driver-Development)*
