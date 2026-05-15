/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ENERGY-AWARE SCHEDULER (S-ENERGYSCHED)
 * =========================================================================
 * Mission: Prioritize low-power operations for IoT and mobile devices,
 * making SigmaOS greener than traditional Linux.
 * =========================================================================
 */

#ifndef SIGMA_ENERGYSCHED_H
#define SIGMA_ENERGYSCHED_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    ENERGY_STATE_ACTIVE,
    ENERGY_STATE_THROTTLED,
    ENERGY_STATE_SLEEP
} sigma_energy_state_t;

/* --- Energy Scheduler Primitives --- */
void energysched_init(void);
void energysched_evaluate_power(void);
void energysched_set_shard_state(uint32_t shard_id, sigma_energy_state_t state);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ENERGYSCHED_H */
