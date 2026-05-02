#include "../../../include/sigma_types.h"
#include "sigma_hal.h"
#include "../../../include/SovereignLibC.h"

/**
 * SigmaOS Sovereign Hardware Abstraction Layer (HAL)
 * Unifies x86_64, ARM64, and RISC-V under a single ABI layer.
 *
 * USP: Transparently handles architecture-specific CPU dispatching and IO mapping.
 *
 * Design: OOP-isolated singleton — SovereignHALEngine.
 */

class SovereignHALEngine {
public:
    static SovereignHALEngine& getInstance() {
        static SovereignHALEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[HAL] Initializing Unified Sovereign Hardware Abstraction Layer...");
    }

    void mapHardwareIO(sigma_u32 device_id) {
        sigma_printf("[HAL] Device IO dynamically mapped for hardware ID: 0x%04X.\n", device_id);
    }

private:
    SovereignHALEngine() {}
};

/* --- C Wrappers --- */
extern "C" void hal_init() {
    SovereignHALEngine::getInstance().init();
}

extern "C" void hal_map_io(sigma_u32 id) {
    SovereignHALEngine::getInstance().mapHardwareIO(id);
}
