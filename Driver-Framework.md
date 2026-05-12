# Sovereign Driver Framework (S-DF)

The S-DF provides a standardized way to write device drivers that are compatible with the Sovereign Lattice.

## 🛠 Driver Structure

A Sovereign Driver is a specialized shard that inherits from `SigmaOS::Kernel::Hardware::SovereignDriver`.

```cpp
#include "hal/sigma_hal.h"

class MyDriver : public SovereignDriver {
public:
    void init() override {
        // Initialize MMIO, IRQs, etc.
    }

    void handle_irq() override {
        // High-priority interrupt logic
    }
};
```

## 🔌 Supported Interfaces

- **PCIe-Sov**: High-speed peripheral interconnect with PQC-attestation.
- **USB-Lattice**: Generic driver class for HID, Mass Storage, and Professional peripherals (e.g., DICOM scanners).
- **I2C/SPI**: Low-level sensor bridges for Industrial IoT shards.

## 🧪 Template Example

See `kernel/shards/hardware/SovereignWiFi.cpp` for a production example of a network driver shard.

---

### Next: [Security & Reliability](Security-Safety.md)
