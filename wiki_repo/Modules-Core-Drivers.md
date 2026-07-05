# Σ core/drivers — Sovereign Hardware Driver Layer

Isolates hardware drivers into **capability-gated loadable shards** so a faulty
driver cannot crash the kernel or corrupt unrelated subsystems.

## Architecture

```
┌──────────────────────────────────────────────────────┐
│                  Kernel Core (Ring 0)                │
├──────────────────────────────────────────────────────┤
│  Driver Registry  ←→  SovereignHAL (ext/hal)         │
│      │                                               │
│   ┌──┴──┐  ┌────┐  ┌─────┐  ┌──────┐  ┌────────┐   │
│   │ PCI │  │ USB│  │ GPU │  │Audio │  │  WiFi  │   │
│   └─────┘  └────┘  └─────┘  └──────┘  └────────┘   │
└──────────────────────────────────────────────────────┘
```

## Source Files

| File | Exported Function | Description |
|---|---|---|
| `audio.rs` | `audio_init()` | HD Audio / AC97 subsystem init |
| `gpu.rs` | `gpu_init()` | GPU framebuffer + DRM/KMS stub |
| `pci.rs` | `pci_init()` | PCIe bus enumeration & BAR mapping |
| `usb.rs` | `usb_init()` | XHCI/EHCI USB host controller |
| `wifi.rs` | `wifi_init()` | 802.11 wireless stack (nl80211-style) |
| `sigma_libc.rs` | — | Shared no-alloc primitives |

## API Interface

All driver shards expose a common C-compatible ABI:

```c
// Register a driver with the SovereignHAL registry
void sigma_register_driver(const char *name, sigma_driver_t *drv);

// Allocate DMA-coherent memory for a driver
void *sigma_alloc_dma_region(size_t size);

// Bind an interrupt vector to a handler
int sigma_irq_install(uint32_t vector, void (*handler)(void));

// Core drivers entry points
void pci_init(void);
void usb_init(void);
void gpu_init(void);
void audio_init(void);
void wifi_init(void);
```

## Capability Model

Each driver shard must declare its required capabilities in `module.json`:

```json
{
  "capabilities_required": ["CAP_PCI_ACCESS", "CAP_IRQ_BIND"],
  "capabilities_provided": ["CAP_NIC_DRIVER"]
}
```

## Roadmap

- [x] PCI bus enumeration (`pci.rs`)
- [x] USB XHCI stub (`usb.rs`)
- [x] GPU framebuffer stub (`gpu.rs`)
- [x] Audio HD-Audio stub (`audio.rs`)
- [x] WiFi 802.11 stub (`wifi.rs`)
- [ ] Full DMA ring-buffer implementation (NIC)
- [ ] GPU DRM/KMS mode-setting
- [ ] NVMe storage driver
- [ ] Bluetooth HCI driver
- [ ] Formal DDK API header (`ddk_api.h`)
- [ ] CBMC/Kani safety proofs for DMA paths

## Related Modules

- [`modules/ext/hal`](../../ext/hal/README.md) — Hardware Abstraction Layer
- [`modules/tools/diag`](../../tools/diag/README.md) — Driver diagnostics & tracing
