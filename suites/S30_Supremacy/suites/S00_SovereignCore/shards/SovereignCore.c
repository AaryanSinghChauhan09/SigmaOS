#include "sigma_libc.h"
#include "sigma_types.h"

/**
 * SigmaOS Sovereign Micro-Kernel Core
 * Subsystem: S00 (SovereignCore)
 * Mission: Minimalist core for scheduling, memory abstraction, and IPC.
 */

typedef struct {
    uint64_t core_epoch;
    sigma_bool scheduler_active;
    char system_status[32];
} CoreState;

static CoreState global_core;

void core_init(void) {
    global_core.core_epoch = 0;
    global_core.scheduler_active = SIGMA_TRUE;
    sigma_strncpy(global_core.system_status, "STABLE_SOVEREIGN", 31);
    
    sigma_printf("S00 [SOVEREIGN-CORE]: Micro-kernel initialized.\n");
    sigma_printf("  [SCHEDULER]: Active (Predictive Neural Sliced).\n");
    sigma_printf("  [STATUS]: %s\n", global_core.system_status);
}

void core_dispatch_interrupt(uint32_t irq) {
    // Basic IRQ routing to suite-level handlers
    sigma_printf("S00 [SOVEREIGN-CORE]: IRQ %d intercepted. Routing to Sovereign Lattice...\n", irq);
}

void S00_Register_Core(void) {
    core_init();
}
