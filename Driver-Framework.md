# Sovereign Driver Framework (SDF)

The **Sovereign Driver Framework (SDF)** is SigmaOS's unified hardware abstraction
and driver lifecycle system. It combines the best ideas from Windows WDM/WDDM (stable
ABI, vendor support) and Linux (open source, security isolation) while adding features
neither OS offers: Ring-3 crash isolation and AI-assisted porting.

---

## Why SDF Exists

| Problem | Windows | Linux | SigmaOS SDF |
|---------|---------|-------|-------------|
| Stable ABI | ✅ per-version | ❌ breaks every update | ✅ frozen forever (DDK v1.0) |
| Open source | ❌ vendor blobs | ✅ kernel-integrated | ✅ open encouraged + closed allowed |
| Driver crash isolation | ❌ ring-0 crash = BSOD | ❌ ring-0 crash = kernel panic | ✅ Ring-3 isolation option |
| Security per driver | ❌ full kernel access | ❌ full kernel access | ✅ sigma_pledge per driver |
| AI-assisted porting | ❌ | ❌ | ✅ sigma-driver-porter |

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  HARDWARE  (CPU · NVMe · GPU · NIC · USB · TPM2)        │
├─────────────────────────────────────────────────────────┤
│  SovereignHAL  (port I/O · MMIO · PCI · ACPI · DMA)    │
├─────────────────────────────────────────────────────────┤
│  SDF CORE                                               │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐  │
│  │  DDK     │ │ kabi/    │ │ Ring-3   │ │ sigma-   │  │
│  │  API     │ │ ABI      │ │ sandbox  │ │ bus IPC  │  │
│  │ v1.0     │ │ stable   │ │ isolation│ │ channels │  │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘  │
├─────────────────────────────────────────────────────────┤
│  DRIVER LAYER                                           │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────────┐  │
│  │  Open   │ │ Closed  │ │AI-Ported│ │  Linux/BSD  │  │
│  │ drivers │ │ vendor  │ │ drivers │ │  compat     │  │
│  │ (FOSS)  │ │  blobs  │ │         │ │  shim       │  │
│  └─────────┘ └─────────┘ └─────────┘ └─────────────┘  │
├─────────────────────────────────────────────────────────┤
│  KERNEL  (SovereignIPC · pledge · DeviceManager)        │
└─────────────────────────────────────────────────────────┘
```

---

## SDF Driver Lifecycle

Every SDF driver implements four lifecycle functions:

```rust
// 1. probe() — check if hardware is present (PCI ID match)
pub extern "C" fn my_driver_probe(pci_bar: u64, irq: u8) -> i32;

// 2. init() — map MMIO, set up DMA rings, declare pledge
pub extern "C" fn my_driver_init() -> i32;

// 3. irq() — handle hardware interrupt (optional)
pub extern "C" fn my_driver_irq() -> bool;  // true = IRQ was ours

// 4. shutdown() — quiesce hardware, free DMA, unmap MMIO
pub extern "C" fn my_driver_shutdown();
```

Registration is done with a single macro that places the descriptor in the
`.sigma_drivers` ELF section for automatic kernel discovery:

```rust
sigma_register_driver!(SigmaDriverDescriptor {
    magic:       SIGMA_DDK_MAGIC,
    abi_version: DDK_ABI_VERSION,   // 1 — frozen forever
    vendor_id:   0x8086,
    device_id:   0x100E,
    flags:       SIGMA_DRV_FLAG_OPEN_SOURCE,
    ring:        3,                 // Ring-3 isolated (safe default)
    fn_probe:    Some(my_driver_probe),
    fn_init:     Some(my_driver_init),
    fn_shutdown: Some(my_driver_shutdown),
    fn_irq:      Some(my_driver_irq),
    ..Default::default()
});
```

---

## Stable ABI (kabi/)

The `kabi/` library freezes the binary interface between drivers and the kernel.
A driver compiled against DDK v1.0 will load on any future SigmaOS version.

```rust
// kabi/src/lib.rs
pub const KABI_VERSION_MAJOR: u32 = 1;  // bump only for breaking changes
pub const KABI_VERSION_MINOR: u32 = 0;  // bump when adding new fields

// Validated at driver load time:
pub fn validate(&self) -> KabiResult {
    if self.magic != KABI_MAGIC          { return Err(KabiError::BadMagic); }
    if self.version_major != KABI_VERSION_MAJOR { return Err(MajorMismatch); }
    Ok(())
}
```

New fields are added **at the end of structs only**. Old drivers see only the
fields they know about — the `struct_size` field lets the loader detect truncated
older drivers and zero-fill the new fields.

---

## Ring-3 Driver Isolation

By setting `ring: 3` in the descriptor, a driver runs as an isolated userspace
process communicating with the kernel through sigma-bus IPC:

```
Kernel ring-0          sigma-bus              Driver ring-3
     │                    │                        │
     │──── IRQ event ────►│                        │
     │                    │──── IPC message ───────►│
     │                    │                        │── handle IRQ
     │                    │◄─── result ────────────│
     │◄─── return ───────►│                        │
```

Benefits:
- Driver crash → kernel keeps running, driver process restarted
- Driver bug → contained in sandbox, no kernel memory corruption
- `sigma_pledge` per driver limits syscall access
- Easier debugging (driver is a normal process)

Cost: ~1–5µs per operation for IPC crossing.

---

## Security: sigma_pledge per Driver

```rust
// NIC driver — can only do network I/O
sigma_pledge("stdio inet");

// GPU driver — can only access video memory
sigma_pledge("stdio video");

// Audio driver — can only access audio
sigma_pledge("stdio audio");
```

A compromised NIC driver with `pledge("stdio inet")` **cannot** read your files,
spawn processes, or access the GPU. Windows drivers run in ring-0 with full kernel
access — one exploit = full system compromise.

---

## Driver Flags

| Flag | Meaning |
|------|---------|
| `SIGMA_DRV_FLAG_OPEN_SOURCE` | Source available — community can audit and fix |
| `SIGMA_DRV_FLAG_CERTIFIED` | Vendor-signed, tested by SigmaOS team |
| `SIGMA_DRV_FLAG_RING3` | Runs in ring-3 isolated process |
| `SIGMA_DRV_FLAG_HOT_PLUG` | Supports runtime attach/detach |
| `SIGMA_DRV_FLAG_AI_PORTED` | Generated by sigma-driver-porter |
| `SIGMA_DRV_FLAG_COMPAT_LX` | Wraps a Linux driver via distro compat layer |

---

## Hardware Support Status

| Hardware | Driver | Status |
|----------|--------|--------|
| Intel e1000/e1000e NIC | `kernel/linux_compat/e1000_main.rs` | ✅ Working |
| NVMe SSD | `drivers/sovereignnvme.rs` | ✅ Working |
| USB HID | `drivers/sovereignusb.rs` | 🔄 Partial |
| Intel Wi-Fi 6 (iwlwifi) | `drivers/net/sigma_wifi_driver.rs` | 🔄 Partial |
| Intel GPU (i915) | `drivers/gpu/` | ⬜ Phase C |
| AMD GPU (amdgpu) | `drivers/gpu/` | ⬜ Phase C |
| NVIDIA (nouveau) | — | ⬜ Phase D |
| HD Audio | `drivers/audio/sigma_hda.rs` | 🔄 Partial |
| Printer (CUPS) | `drivers/printing/cups.nim` | ✅ Working |
| AHCI/SATA | `drivers/storage/sigma_ahci.rs` | 🔄 Partial |

---

## Building a Driver

```bash
# Generate skeleton from Linux driver (cleanroom study)
sigma-driver-porter analyse /path/to/linux_rtl8169.c
sigma-driver-porter port    /path/to/linux_rtl8169.c

# Or with AI translation
sigma-driver-porter port /path/to/linux_rtl8169.c --ai

# Build
cd sigma_drivers/rtl8169
cargo build --release --target x86_64-sigmaos.json

# Validate
sigma-ddk validate target/x86_64-sigmaos/release/librtl8169.a

# View vendor registry
sigma-ddk-vendors list
sigma-ddk-vendors score
```

---

*See also: [Driver Development Guide](Driver-Development-Guide) · [Linux Driver Compat](Linux-Driver-Compat) · [Windows-Linux-SigmaOS-Drivers](Windows-Linux-SigmaOS-Drivers) · [Kernel ABI Stability](kabi)*
