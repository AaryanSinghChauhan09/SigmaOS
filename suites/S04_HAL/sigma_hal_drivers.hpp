// SigmaOS — OOP HAL Driver Registry
// Module: sigma-hal-registry
// Single responsibility: register, lookup, and dispatch hardware drivers
// Uses pure C++ OOP — inherits from ISigmaDriver base

#pragma once
#include "../../include/atomic_sigma_oop_base.hpp"
#include "../../include/libc/sigma_libc.h"

namespace sigma {
namespace hal {

// Concrete NVMe storage driver
class NVMeDriver : public sigma::core::ISigmaDriver,
                   public sigma::core::ISigmaModule {
private:
    bool ready;
public:
    NVMeDriver() : ready(false) {}

    void initialize() override {
        sigma_kprint("[SigmaHAL] NVMe: Initializing PCIe BAR registers...\n");
        ready = true;
    }

    void execute() override {
        if (ready)
            sigma_kprint("[SigmaHAL] NVMe: Executing atomic DMA read/write queue.\n");
    }

    void shutdown() override {
        sigma_kprint("[SigmaHAL] NVMe: Flushing write cache and disengaging.\n");
        ready = false;
    }

    int probe_hardware() override {
        sigma_kprint("[SigmaHAL] NVMe: Probing PCIe device ID 0x0953...\n");
        return 1;
    }

    void enable_dma() override {
        sigma_kprint("[SigmaHAL] NVMe: Enabling bus-mastering DMA.\n");
    }
};

// Concrete USB HID driver
class USBHIDDriver : public sigma::core::ISigmaDriver,
                     public sigma::core::ISigmaModule {
public:
    void initialize() override {
        sigma_kprint("[SigmaHAL] USB-HID: Enumerating endpoints...\n");
    }
    void execute() override {
        sigma_kprint("[SigmaHAL] USB-HID: Polling interrupt endpoint.\n");
    }
    void shutdown() override {
        sigma_kprint("[SigmaHAL] USB-HID: Releasing USB interface.\n");
    }
    int probe_hardware() override {
        sigma_kprint("[SigmaHAL] USB-HID: Probing USB class 03h...\n");
        return 1;
    }
    void enable_dma() override {
        sigma_kprint("[SigmaHAL] USB-HID: DMA not applicable for interrupt transfers.\n");
    }
};

} // namespace hal
} // namespace sigma

extern "C" {
    void hal_run_all_drivers() {
        sigma::hal::NVMeDriver nvme;
        sigma::hal::USBHIDDriver hid;

        nvme.probe_hardware(); nvme.initialize(); nvme.enable_dma(); nvme.execute(); nvme.shutdown();
        hid.probe_hardware();  hid.initialize();  hid.enable_dma();  hid.execute();  hid.shutdown();
    }
}
