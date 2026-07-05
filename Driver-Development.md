# SigmaOS Driver Development Guide

All drivers use the **Sovereign Driver Framework (SDF)** — a clean lifecycle API that isolates drivers from the core kernel, enabling Ring-3 driver hosting (Phase G) for fault isolation.

---

## SDF Driver Template

```cpp
// drivers/subsystem/sigma_mydriver.cpp
#include "include/SovereignDriver.h"

class SigmaMyDriver : public SovereignDriverBase {
public:
    // Called during PCI enumeration — return SIGMA_OK if this device matches
    sigma_status probe(SigmaDeviceInfo* dev) override {
        if (dev->vendor_id != MY_VENDOR || dev->device_id != MY_DEVICE)
            return SIGMA_ERR_NO_MATCH;
        return SIGMA_OK;
    }

    // Allocate resources, map MMIO, install IRQ handler
    sigma_status init() override {
        bar0_ = sigma_pci_map_bar(dev_, 0);
        sigma_irq_request(dev_->irq, irq_handler, this);
        return SIGMA_OK;
    }

    // Release all resources
    sigma_status shutdown() override {
        sigma_irq_free(dev_->irq);
        sigma_pci_unmap_bar(bar0_);
        return SIGMA_OK;
    }

    // Device-specific ops
    sigma_status read(void* buf, size_t len) override { ... }
    sigma_status write(const void* buf, size_t len) override { ... }

private:
    static void irq_handler(void* ctx) { ... }
    volatile uint8_t* bar0_;
};

// Auto-register with SDF
SIGMA_SDF_REGISTER_DRIVER(SigmaMyDriver, "my_driver", MY_VENDOR, MY_DEVICE);
```

---

## Driver Categories

| Category | Directory | Examples |
|----------|-----------|---------|
| Display | `drivers/display/` | VESA, VirtIO-GPU, DRM/KMS |
| Graphics | `drivers/graphics/` | Intel i915, AMD amdgpu |
| Network | `drivers/net/` | e1000, iwlwifi, rtl8xxxu |
| Storage | `drivers/storage/` | NVMe, AHCI |
| USB | `drivers/usb/` | xHCI, HID |
| Audio | `drivers/audio/` | Intel HDA |
| Input | `drivers/input/` | PS/2, USB HID keyboard/mouse |
| GPU Compute | `drivers/gpu/` | CUDA-compat, OpenCL stubs |

---

## Current Driver Status

| Driver | File | Status |
|--------|------|--------|
| NVMe | `drivers/storage/sigma_nvme.cpp` | ✅ Done |
| USB xHCI | `drivers/usb/sigma_xhci.cpp` | ✅ Done |
| Intel e1000 NIC | `kernel/core/drivers/SovereignE1000.cpp` | ✅ Done |
| VESA/GOP framebuffer | `drivers/display/sigma_vesa.cpp` | ⬜ Phase G |
| VirtIO-GPU | `drivers/display/sigma_virtio_gpu.cpp` | ⬜ Phase G |
| Intel i915 KMS | `drivers/graphics/sigma_i915.cpp` | ⬜ Phase G |
| AMD amdgpu | `drivers/graphics/sigma_amdgpu.cpp` | ⬜ Phase G |
| Intel iwlwifi Wi-Fi 6 | `drivers/net/sigma_iwlwifi.cpp` | ⬜ Phase G |
| Realtek rtl8xxxu | `drivers/net/sigma_rtl8xxxu.cpp` | ⬜ Phase G |
| Intel HDA audio | `drivers/audio/sigma_hda.cpp` | ⬜ Phase G |
| Bluetooth HCI | `drivers/bt/sigma_hci_usb.cpp` | ⬜ Phase G |
| ARM64 BCM2711 (RPi 4) | `arch/arm64/sigma_bcm2711.cpp` | ⬜ Phase G |
| ARM64 BCM2712 (RPi 5) | `arch/arm64/sigma_bcm2712.cpp` | ⬜ Phase G |

---

## Writing a New Driver — Step by Step

1. **Create the file** in the appropriate `drivers/subsystem/` directory
2. **Inherit** from `SovereignDriverBase`
3. **Implement** `probe()`, `init()`, `shutdown()`
4. **Register** with `SIGMA_SDF_REGISTER_DRIVER`
5. **Add a test** in `tests/unit/drivers/`
6. **Run in QEMU**: `make DRIVER=my_driver qemu-driver-test`
7. **Update** `CURRENT_PROBLEMS_MANIFEST.md` — mark the driver as resolved

## PCI Driver Helpers

```cpp
// Map a BAR region
volatile uint8_t* bar = sigma_pci_map_bar(dev, bar_index);

// Request an IRQ (MSI-X preferred)
sigma_irq_request(dev->irq, handler, ctx);

// DMA allocation (physically contiguous)
void* dma_buf = sigma_dma_alloc(size, &phys_addr);
```

---

## Submitting a Driver PR

1. PR title: `driver: add <name> <subsystem> driver`
2. Must pass `make fuzz DRIVER=<name>`
3. Must pass QEMU smoke test (include screenshot/log in PR)
4. Update `CURRENT_PROBLEMS_MANIFEST.md`
5. Add wiki entry to [Driver-Shards](Driver-Shards) page

---

*See also: [DriverAPI](DriverAPI) · [Hardware-Support](Hardware-Support) · [Branch-Development-Roadmap](Branch-Development-Roadmap)*
