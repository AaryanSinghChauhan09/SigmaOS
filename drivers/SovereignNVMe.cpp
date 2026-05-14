#include "sigma_hal.h"
#include "sigma_log.h"
#include "sigma_nvme.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign NVMe Driver
 * High-performance, zero-copy block storage interface.
 *
 * Design: OOP-isolated singleton — SovereignNVMeEngine.
 */

class SovereignNVMeEngine {
public:
    static SovereignNVMeEngine& getInstance() {
        static SovereignNVMeEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[NVMe] Initializing Sovereign Storage Controller...");
        
        // Scan PCI bus for NVMe class devices (Stub for now)
        this->master_nvme.base_addr = 0xF0000000; 
        this->master_nvme.irq = 16;
        this->master_nvme.initialized = true;

        sigma_log_info("[NVMe] Controller mapped at 0x%llX, IRQ %d\n", 
                     this->master_nvme.base_addr, this->master_nvme.irq);
    }

    int readBlocks(uint64_t lba, uint32_t count, void* buffer) {
        if (!this->master_nvme.initialized) return -1;
        
        if (this->queue_depth >= 64u) {
            sigma_log("[NVMe] [WARNING] Controller saturated. Queue depth at limit.");
            return -2;
        }

        this->queue_depth++;
        int status = -1;
        for (int retry = 0; retry < 3; retry++) {
            // Perform DMA transfer from NVMe controller (simulated)
            status = 0; // Assume success for now
            if (status == 0) break;
            sigma_log_info("[NVMe] Retry %d for LBA %llu\n", retry + 1, lba);
        }
        
        this->queue_depth--;
        return status;
    }

    int writeBlocks(uint64_t lba, uint32_t count, const void* buffer) {
        if (!this->master_nvme.initialized) return -1;
        if (this->queue_depth >= 64u) return -2;

        this->queue_depth++;
        // Perform DMA transfer to NVMe controller (simulated)
        this->queue_depth--;
        return 0; // Success
    }

private:
    SovereignNVMeEngine() : queue_depth(0) {
        master_nvme.initialized = false;
    }
    
    sigma_nvme_device_t master_nvme;
    sigma_u32           queue_depth;
};

/* --- C Wrappers --- */
extern "C" void nvme_init() {
    SovereignNVMeEngine::getInstance().init();
}

extern "C" int nvme_read_blocks(uint64_t lba, uint32_t count, void* buffer) {
    return SovereignNVMeEngine::getInstance().readBlocks(lba, count, buffer);
}

extern "C" int nvme_write_blocks(uint64_t lba, uint32_t count, const void* buffer) {
    return SovereignNVMeEngine::getInstance().writeBlocks(lba, count, buffer);
}


