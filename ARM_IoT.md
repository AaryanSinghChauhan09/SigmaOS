# ARM & IoT Architecture Specification

This document details the hardware abstraction policies, memory layouts, and SoC boot protocols implemented to achieve first-class support for ARM64 (AArch64) and RISC-V embedded architectures.

---

## 📱 Hardware Support Matrix

| Platform | SoC | Architecture | Memory Map Base | Primary Interrupt Controller |
| :--- | :--- | :--- | :--- | :--- |
| **Raspberry Pi 3** | Broadcom BCM2837 | ARMv8-A (64-bit) | `0x3F000000` | BCM2837 L1 / L2 Controller |
| **Raspberry Pi 4** | Broadcom BCM2711 | ARMv8.2-A | `0xFE000000` | ARM GIC-400 (Generic Interrupt Controller) |
| **Embedded QEMU** | Virt Machine | RISC-V RV64GC | `0x80000000` | PLIC (Platform-Level Interrupt Controller) |

---

## 🚀 Boot Protocols & Init Sequence

Unlike x86_64 MultiBoot2 structures, ARM targets boot via device trees (`DTB`) and direct kernel loading:

```
[Broadcom BootROM / GPU Firmware]
             │
             ▼
[config.txt -> Load kernel8.img]
             │
             ▼
[SovereignInit ARM entry point (setup stack, EL2 to EL1)]
             │
             ▼
[Initialize MMU, Translation Tables (TTBR0/TTBR1)]
             │
             ▼
[Register BSP Core, spin up Secondary Cores (PSCI)]
             │
             ▼
[SigmaOS Scheduler launch (EEVDF scheduler active)]
```

### Exception Levels on ARM64

SigmaOS utilizes ARM64 security exception levels as follows:
- **EL3**: Secure Monitor (reserved for firmware/TrustZone secure enclave)
- **EL2**: Hypervisor (unused, or hosting `SovereignKVM` virtualization)
- **EL1**: Sovereign Kernel (Scheduler, Memory management, VFS)
- **EL0**: Userland / Applications (`sigpkg`, `sigma-shell`, `zenith-desktop`)

---

## 🛠️ Peripheral Interconnection Core

Drivers for serial interfaces (PL011 UART) and system timers are written in modular Rust packages:

```rust
// kernel/src/drivers/uart/pl011.rs
pub struct Pl011Uart {
    base_address: usize,
}

impl Pl011Uart {
    pub const fn new(base: usize) -> Self {
        Pl011Uart { base_address: base }
    }

    pub unsafe fn write_char(&self, c: char) {
        let dr = self.base_address as *mut u32;
        let fr = (self.base_address + 0x18) as *const u32;
        
        // Wait until transmit FIFO is not full
        while (*fr & (1 << 5)) != 0 {}
        *dr = c as u32;
    }
}
```

---

## 💾 Declarative Cross-Compilation Profile

Building for ARM64 targets requires applying the `--target aarch64-unknown-none` configuration.

```toml
# sigma.toml (ARM profile snippet)
[profile.arm64]
target = "aarch64-unknown-none"
toolchain = "nightly"
optimization_level = 3
lto = "fat"
features = ["bcm2711-gpio", "gic-400"]
```
