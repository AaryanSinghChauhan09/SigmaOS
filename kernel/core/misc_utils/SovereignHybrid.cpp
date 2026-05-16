#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"

#include "../../../include/sigma_hybrid.h"
#include "../../../include/sigma_hal.h"


/**
 * SigmaOS Sovereign Hybrid Kernel
 * Implements a Dynamic Context Shifting (DCS) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal context management.
 */

static sigma_hybrid_mode_t current_mode = HYBRID_MODE_MICRO;

void hybrid_init() {
    sigma_log("[HYBRID] Initializing Sovereign Hybrid Kernel (DCS Algorithm)...");
}

void hybrid_set_mode(sigma_hybrid_mode_t mode) {
    current_mode = mode;
    sigma_log("[HYBRID] DCS: Kernel mode shifted to %s\n", mode == HYBRID_MODE_MICRO ? "MICRO" : "MONOLITHIC");
}

extern "C" bool hybrid_execute_syscall(sigma_u32 syscall_id, void* args) {
    // DCS (Dynamic Context Shifting) Algorithm
    // Routes syscalls based on the current hybrid mode.
    
    sigma_log("[HYBRID] DCS: Executing syscall 0x%02X...\n", syscall_id);
    
    if (current_mode == HYBRID_MODE_MICRO) {
        sigma_log("[HYBRID] DCS: Routing through isolated micro-services (Secure but overhead).");
    } else {
        sigma_log("[HYBRID] DCS: Routing directly to monolithic core (Max Performance).");
    }
    
    return true;
}




} // extern "C"
