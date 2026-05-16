#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"
#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Microkernel Orchestrator
 * Highly modular, message-passing microkernel architecture abstraction.
 *
 * USP: Allows the OS to dynamically switch between Monolithic and Microkernel
 * execution models based on security requirements. In microkernel mode,
 * drivers and filesystems are pushed to user space, orchestrated by IPC.
 *
 * Design: OOP-isolated singleton — SovereignMicrokernelEngine.
 */

class SovereignMicrokernelEngine {
public:
    static SovereignMicrokernelEngine& getInstance() {
        static SovereignMicrokernelEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[MICROKERNEL] Initializing Sovereign Microkernel Architecture...");
        this->microkernel_mode = false;
        this->ipc_channels_active = 0;
        sigma_log("[MICROKERNEL] Ready to orchestrate user-space IPCs.");
    }

    void enableMicrokernelMode() {
        this->microkernel_mode = true;
        sigma_log("[MICROKERNEL] Mode switched to MICROKERNEL. Pushing drivers to user-space boundaries.");
        // Simulated teardown of monolithic structures
    }

    void disableMicrokernelMode() {
        this->microkernel_mode = false;
        sigma_log("[MICROKERNEL] Mode switched to MONOLITHIC. Re-absorbing drivers into ring-0.");
    }

    sigma_u32 allocateIPCChannel(sigma_u32 service_a, sigma_u32 service_b) {
        if (!this->microkernel_mode) {
            sigma_log("[MICROKERNEL] [WARNING] Cannot allocate IPC in Monolithic Mode.");
            return 0;
        }
        
        sigma_u32 channel_id = ++this->ipc_channels_active;
        sigma_log_info("[MICROKERNEL] IPC Channel %u allocated between Service 0x%04X and Service 0x%04X.\n", 
                     channel_id, service_a, service_b);
        return channel_id;
    }

private:
    SovereignMicrokernelEngine() : microkernel_mode(false), ipc_channels_active(0) {}

    bool microkernel_mode;
    sigma_u32 ipc_channels_active;
};

/* --- C Wrappers --- */
extern "C" void microkernel_init() {
    SovereignMicrokernelEngine::getInstance().init();
}

extern "C" void microkernel_enable() {
    SovereignMicrokernelEngine::getInstance().enableMicrokernelMode();
}

extern "C" void microkernel_disable() {
    SovereignMicrokernelEngine::getInstance().disableMicrokernelMode();
}

extern "C" sigma_u32 microkernel_allocate_ipc(sigma_u32 service_a, sigma_u32 service_b) {
    return SovereignMicrokernelEngine::getInstance().allocateIPCChannel(service_a, service_b);
}


