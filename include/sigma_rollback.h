/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN AUTOMATED ROLLBACK (S-ROLLBACK)
 * =========================================================================
 * Mission: Zero-downtime automated state recovery and rollback.
 * =========================================================================
 */

#ifndef SIGMA_ROLLBACK_H
#define SIGMA_ROLLBACK_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t snapshot_id;
    uint32_t timestamp;
    uint32_t shard_state_root;
} sigma_rollback_token_t;

/* --- Rollback Primitives --- */
void rollback_init(void);
void rollback_capture_snapshot(void);
void rollback_execute_to_last_stable(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ROLLBACK_H */
