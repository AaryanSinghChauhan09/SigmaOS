/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN LATTICE ROLLING UPDATE (S-LRU)
 * =========================================================================
 * Mission: Zero-downtime, rolling shard updates and machine-state migration.
 * Inspired by Arch Linux / Rolling Release.
 * =========================================================================
 */

#ifndef SIGMA_LRU_H
#define SIGMA_LRU_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    LRU_IDLE,
    LRU_MIGRATING,
    LRU_STABILIZING
} sigma_lru_state_t;

/* --- LRU Primitives --- */
void lru_init(void);
void lru_trigger_update(uint32_t shard_id, void* new_binary, uint32_t size);
sigma_lru_state_t lru_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LRU_H */
