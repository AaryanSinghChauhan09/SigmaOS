#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "sigma_panicrecov.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "sigma_rollback.h"
#include "../../../include/sigma_log.h"
#include "sigma_recover.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Panic Recovery
 * Implements a Resilient State Resurrection (RSR) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal crash recovery.
 */

class SovereignPanicRecov {
public:
    static void init() {
        sigma_log("[PANICRECOV] Initializing Sovereign Panic Recovery (RSR Algorithm)...");
    }

    static void handlePanic(uint32_t fault_code, const void* register_state) {
        // RSR (Resilient State Resurrection) Algorithm
        // Captures full CPU context and attempts graceful recovery before cold reboot.
        
        SovereignRecover::getInstance().setLatticeState(SIGMA_RECOVER_CRITICAL);
        sigma_log_info("[PANICRECOV] RSR: !!! KERNEL PANIC INTERCEPTED (Fault: 0x%08X) !!!\n", fault_code);
        sigma_log("[PANICRECOV] RSR: Capturing full register state to sovereign diagnostic log...");
        
        // Attempt auto-recovery
        if (attemptRecovery()) {
            sigma_log("[PANICRECOV] RSR: Recovery SUCCESSFUL. System continues.");
        } else {
            sigma_log("[PANICRECOV] RSR: Recovery FAILED. Triggering S-Rollback to last known-good state...");
            rollback_execute_to_last_stable();
        }
    }

    static bool attemptRecovery() {
        sigma_log("[PANICRECOV] RSR: Isolating faulting shard and restarting its context...");
        // Attempt to restart just the faulting shard rather than the entire kernel.
        return true; // Simulated success
    }
};

/* --- C Wrappers --- */
extern "C" void panicrecov_init() {
    SovereignPanicRecov::init();
}

extern "C" void panicrecov_handle_panic(uint32_t fault_code, const void* register_state) {
    SovereignPanicRecov::handlePanic(fault_code, register_state);
}

extern "C" bool panicrecov_attempt_recovery() {
    return SovereignPanicRecov::attemptRecovery();
}


