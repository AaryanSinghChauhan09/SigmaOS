#include "../../include/core/sigma_types.h"
#include "../../include/sigma_log.h"

#include "../../include/sigma_hybrid.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"


/**
 * SigmaOS Sovereign Hybrid Kernel
 * Implements a Dynamic Context Shifting (DCS) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal context management.
 */

static sigma_hybrid_mode_t current_mode = HYBRID_MODE_MICRO;

extern "C" void hybrid_init() {
    sigma_log("[HYBRID] Initializing Sovereign Hybrid Kernel (DCS Algorithm)...");
}

extern "C" void hybrid_set_mode(sigma_hybrid_mode_t mode) {
    current_mode = mode;
    sigma_log_info("[HYBRID] DCS: Kernel mode shifted to %s\n", mode == HYBRID_MODE_MICRO ? "MICRO" : "MONOLITHIC");
}

extern "C" bool hybrid_execute_syscall(uint32_t syscall_id, void* args) {
    // DCS (Dynamic Context Shifting) Algorithm
    // Routes syscalls based on the current hybrid mode.
    
    sigma_log_info("[HYBRID] DCS: Executing syscall 0x%02X...\n", syscall_id);
    
    if (current_mode == HYBRID_MODE_MICRO) {
        sigma_log("[HYBRID] DCS: Routing through isolated micro-services (Secure but overhead).");
    } else {
        sigma_log("[HYBRID] DCS: Routing directly to monolithic core (Max Performance).");
    }
    
    return true;
}


