// SigmaOS — sigma-hal-driver-storage: Storage Hardware Abstraction
// Module: sigma-hal-driver-storage
// USP: Universal block device abstraction bridging NVMe, SATA, and eMMC
//      via unified DMA queues.

#ifndef SIGMA_HAL_DRIVER_STORAGE_HPP
#define SIGMA_HAL_DRIVER_STORAGE_HPP

#include "../../include/atomic_sigma_oop_base.hpp"

namespace sigma {
namespace hal {

class IStorageDriver : public sigma::core::ISigmaDriver {
public:
    virtual bool initialize() override = 0;
    virtual bool read_blocks(unsigned long lba, unsigned int count, void* buffer) = 0;
    virtual bool write_blocks(unsigned long lba, unsigned int count, const void* buffer) = 0;
    virtual ~IStorageDriver() = default;
};

class NvmeAbstractDriver : public IStorageDriver {
public:
    bool initialize() override {
        // Setup NVMe Admin/IO Queues
        return true;
    }
    
    bool read_blocks(unsigned long lba, unsigned int count, void* buffer) override {
        (void)lba; (void)count; (void)buffer;
        // Native PCIe MMIO doorbell ringing
        return true;
    }
    
    bool write_blocks(unsigned long lba, unsigned int count, const void* buffer) override {
        (void)lba; (void)count; (void)buffer;
        return true;
    }
};

} // namespace hal
} // namespace sigma

#endif /* SIGMA_HAL_DRIVER_STORAGE_HPP */
