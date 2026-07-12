# Linux Driver Compatibility Layer

SigmaOS ships a multi-layer Linux driver compatibility system so that existing
Linux hardware support immediately benefits SigmaOS users — without copying any
GPL kernel code (cleanroom implementation).

---

## Three-Layer Approach

```
┌──────────────────────────────────────────────────────────────────┐
│ Layer 3 — AI Porter (sigma-driver-porter)                        │
│   Study Linux driver structure → generate SigmaOS SDF skeleton   │
│   Best for: new driver development, long-term maintenance        │
├──────────────────────────────────────────────────────────────────┤
│ Layer 2 — Distro Compat Shim (drivers/linux_distros/compat.rs)  │
│   Export Linux kernel symbols (printk, kmalloc, request_irq…)   │
│   Redirect them to SigmaOS HAL equivalents                       │
│   Best for: running pre-built Linux .ko modules                  │
├──────────────────────────────────────────────────────────────────┤
│ Layer 1 — Ubuntu/BSD Compat ABI (drivers/linux/, drivers/bsd/)  │
│   Thin C-ABI shim for driver registration and MMIO access        │
│   Best for: vendor-supplied closed driver binaries               │
└──────────────────────────────────────────────────────────────────┘
```

---

## Layer 1: Ubuntu & BSD ABI Shims

### Ubuntu Compat (`drivers/linux/ubuntu_compat.rs`)

Wraps Linux module_init/module_exit-style driver registration for drivers
compiled against the Ubuntu kernel ABI. Tracks up to 32 wrapped drivers.

```rust
// Register a Linux-style driver
ubuntu_compat_init();
ubuntu_compat_register(
    b"rtl8169\0".as_ptr(),
    0x10EC,   // Realtek vendor ID
    0x8168,   // RTL8168 device ID
    0xFEBC0000, // MMIO BAR0 address
);
```

### BSD Compat (`drivers/bsd/bsd_compat.zig`)

Maps FreeBSD newbus driver model (attach/detach/intr) to SigmaOS SDF lifecycle.
Uses Zig's `comptime` to validate register alignment at compile time — any
misaligned register definition causes a build failure.

```zig
// All PCI config register offsets validated at compile time
comptime {
    comptimeValidateRegister(0x00, 4); // Vendor + Device ID
    comptimeValidateRegister(0x10, 4); // BAR0
}

bsd_compat_init();
bsd_compat_register("em0\x00", 4, 0x8086, 0x100E, bar0, mmio_base, 0); // Network
```

---

## Layer 2: Distro Compat Shim

`drivers/linux_distros/compat.rs` exports the exact symbol names that Linux
kernel modules expect to find when loaded. Each symbol redirects to the
SigmaOS HAL equivalent:

| Linux Symbol | SigmaOS Equivalent | Notes |
|---|---|---|
| `printk` | `sigma_log` | Kernel logging |
| `kmalloc` / `kfree` | `sigma_slab_alloc` / `sigma_slab_free` | Slab allocator |
| `ioremap` / `iounmap` | `sigma_iomap` / `sigma_iounmap` | MMIO mapping |
| `readl` / `writel` | `sigma_mmio_read32/write32` | 32-bit register I/O |
| `request_irq` / `free_irq` | `sigma_request_irq` / `sigma_free_irq` | IRQ registration |
| `dma_alloc_coherent` | `sigma_dma_alloc` | DMA buffer allocation |
| `pci_read_config_dword` | `sigma_pci_read_config32` | PCI config space |
| `pci_enable_device` | `sigma_pci_enable` | PCI device enable |
| `netif_carrier_on/off` | sigma-bus NIC events | Network state |

### Loading a Linux Driver Module

```bash

# Load a .ko built for Ubuntu into sigma-compat

sigma-compat load-driver /lib/modules/rtl8169.ko

# The shim:

# 1. Verifies ELF format

# 2. Resolves Linux kernel symbols → SigmaOS equivalents

# 3. Calls module_init()

# 4. Registers driver with SDF at ring-3 isolation

```

### Security Isolation

All Linux compat drivers run at **ring-3** with `sigma_pledge("stdio rpath inet")`
applied automatically. A Linux driver bug cannot crash the SigmaOS kernel.

---

## Layer 3: AI-Assisted Porting

`drivers/sigma/sigma_driver_ai_porter.nim` analyses a Linux driver's structure
and generates a SigmaOS SDF skeleton with the same hardware logic.

```bash

# Analyse a driver to understand its patterns

sigma-driver-porter analyse linux_e1000_main.c

# Output:

#   Patterns:   DpPciProbe, DpMmioRead, DpMmioWrite, DpIrqHandler, DpDmaAlloc

#   IRQ:        true  DMA: true

#   Complexity: 3/5

#   pledge:     stdio inet

#   Linux APIs: ioremap readl writel request_irq dma_alloc_coherent

# Generate SigmaOS skeleton (cleanroom — no GPL code copied)

sigma-driver-porter port linux_e1000_main.c

# Generated files:

#   sigma_drivers/e1000/Cargo.toml

#   sigma_drivers/e1000/src/lib.rs

#   sigma_drivers/e1000/sigma-shard.toml

```

The generated `lib.rs` contains:

- Correct SDF lifecycle (`probe/init/shutdown/irq`)

- `sigma_pledge` call with inferred capabilities

- `sigma_register_driver!` macro registration

- TODO comments for register definitions (filled from vendor datasheet)

- API mapping comments: `// Replace: ioremap → ddk::iomap`

### AI Translation Mode

```bash

# Full LLM-powered translation (requires sigma-agent daemon)

sigma-driver-porter port linux_e1000_main.c --ai

# sigma-agent analyses the full driver source, generates complete Rust code

# Falls back to rule-based skeleton if daemon not running

```

---

## Supported Linux Driver Patterns

The compat layer handles these common Linux driver patterns:

| Pattern | Linux Style | SigmaOS Style |
|---------|-------------|---------------|
| PCI probe | `pci_driver.probe()` callback | `fn_probe(bar: u64, irq: u8) -> i32` |
| MMIO access | `ioremap` + `readl/writel` | `ddk::iomap` + `mmio_read32/write32` |
| IRQ handling | `request_irq(handler, IRQF_SHARED)` | `ddk::request_irq` + `fn_irq → bool` |
| DMA | `dma_alloc_coherent` | `ddk::dma_alloc(size, phys_out)` |
| Networking | `alloc_etherdev` + `register_netdev` | sigma-bus channel 0x20 |
| Block | `register_blkdev` + `blk_mq` | sigma-bus channel 0x01 |
| USB | `usb_register(&driver)` | `sigma_register_driver!` |
| Platform | `platform_driver_register` | `sigma_register_driver!` |

---

## Distro Coverage

| Distribution | Kernel | Status | Notes |
|---|---|---|---|
| Ubuntu 22.04/24.04 | 5.15–6.8 | 🔄 Layer 1+2 | `ubuntu_compat.rs` |
| Debian 12 | 6.1 | 🔄 Layer 1+2 | Same ABI as Ubuntu |
| Fedora 40 | 6.8 | ⬜ Planned | Slightly different module ABI |
| Arch Linux | Rolling | ⬜ Planned | Layer 3 (AI port) recommended |
| FreeBSD 14 | — | 🔄 Layer 1 | `bsd_compat.zig` |

---

## Reference Implementation: Intel e1000

`kernel/linux_compat/e1000_main.rs` is the reference SDF port of the Intel e1000
NIC driver. It demonstrates the complete porting pattern:

```rust
// 1. Hardware register map (from Intel 8254x datasheet, not Linux source)
const E1000_CTRL:  u32 = 0x00000;
const E1000_RCTL:  u32 = 0x00100;

// 2. Descriptor rings allocated via sigma DMA
unsafe fn init_rx(&mut self) {
    let mut phys: u64 = 0;
    let _buf = sigma_dma_alloc(2048, &mut phys);
    // ... set up ring ...
    self.mmio_write(E1000_RDBAL, (ring_phys & 0xFFFF_FFFF) as u32);
}

// 3. IRQ handler forwards packets to sigma-bus
unsafe fn drain_rx(&mut self) {
    sigma_bus_send(IPC_CH_NET_RX, buf_ptr, pkt_len);
}

// 4. SDF exports
#[no_mangle] pub unsafe extern "C" fn e1000_probe(bar: u64, irq: u8) -> i32 { ... }
#[no_mangle] pub unsafe extern "C" fn e1000_init()    -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn e1000_shutdown()       { ... }
#[no_mangle] pub unsafe extern "C" fn e1000_irq()    -> bool { ... }
```

---

*See also: [Driver Framework](Driver-Framework) · [Windows-Linux-SigmaOS-Drivers](Windows-Linux-SigmaOS-Drivers) · [Driver Development Guide](Driver-Development-Guide)*
