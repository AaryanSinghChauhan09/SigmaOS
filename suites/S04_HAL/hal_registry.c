/*
 * =============================================================================
 * Σ SIGMAOS: HAL GLOBAL REGISTRY (v1.0)
 * =============================================================================
 * Implements the registration and lookup functions declared in hal_contract.h.
 * At boot, each driver calls hal_register_*() to install its vtable.
 * The kernel accesses hardware ONLY via hal_get_*().
 * =============================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "sigma/hal_contract.h"

/* =========================================================================
 * Static vtable pointers (one per subsystem)
 * ========================================================================= */

static const SigmaDisplayOps*  g_display_ops  = (void*)0;
static const SigmaInputOps*    g_input_ops    = (void*)0;
static const SigmaStorageOps*  g_storage_ops  = (void*)0;
static const SigmaNetOps*      g_net_ops      = (void*)0;
static const SigmaTimerOps*    g_timer_ops    = (void*)0;
static const SigmaSerialOps*   g_serial_ops   = (void*)0;

/* =========================================================================
 * Registration
 * ========================================================================= */

void hal_register_display(const SigmaDisplayOps* ops) { g_display_ops = ops; }
void hal_register_input  (const SigmaInputOps*   ops) { g_input_ops   = ops; }
void hal_register_storage(const SigmaStorageOps* ops) { g_storage_ops = ops; }
void hal_register_net    (const SigmaNetOps*     ops) { g_net_ops     = ops; }
void hal_register_timer  (const SigmaTimerOps*   ops) { g_timer_ops   = ops; }
void hal_register_serial (const SigmaSerialOps*  ops) { g_serial_ops  = ops; }

/* =========================================================================
 * Lookup
 * ========================================================================= */

const SigmaDisplayOps*  hal_get_display(void) { return g_display_ops; }
const SigmaInputOps*    hal_get_input  (void) { return g_input_ops;   }
const SigmaStorageOps*  hal_get_storage(void) { return g_storage_ops; }
const SigmaNetOps*      hal_get_net    (void) { return g_net_ops;     }
const SigmaTimerOps*    hal_get_timer  (void) { return g_timer_ops;   }
const SigmaSerialOps*   hal_get_serial (void) { return g_serial_ops;  }
