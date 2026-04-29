/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM RECOVERY (S-RECOVER)
 * =========================================================================
 * Mission: Self-healing shard restoration and automated lattice recovery.
 * =========================================================================
 */

#ifndef SIGMA_RECOVER_H
#define SIGMA_RECOVER_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_RECOVER_HEALTHY,
    SIGMA_RECOVER_HEALING,
    SIGMA_RECOVER_FAILSAFE
} sigma_recovery_state_t;

/* --- Recovery Primitives --- */
void recover_init(void);
void recover_trigger_healing(uint32_t shard_id);
sigma_recovery_state_t recover_get_lattice_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_RECOVER_H */
