/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN PANIC RECOVERY (S-PANICRECOV)
 * =========================================================================
 * Mission: Industrial-grade kernel panic handler that captures full 
 * register state, generates a diagnostic report, and attempts automatic
 * recovery via S-Rollback before resorting to a cold reboot.
 * =========================================================================
 */

#ifndef SIGMA_PANICRECOV_H
#define SIGMA_PANICRECOV_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Panic Recovery Primitives --- */
void panicrecov_init(void);
void panicrecov_handle_panic(uint32_t fault_code, const void* register_state);
bool panicrecov_attempt_recovery(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PANICRECOV_H */
