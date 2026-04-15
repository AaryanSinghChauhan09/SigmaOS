/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SCHEDULER INTERFACE (v2.0)
 * =========================================================================
 * Mission: Pluggable tasks scheduling (CFS, RT, Deadline).
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_SCHEDULER_H
#define SOVEREIGN_SCHEDULER_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef void (*sigma_schedule_fn)(sigma_u32 cpu_id, sigma_u64 now_ns);

typedef struct {
    char name[32];
    sigma_schedule_fn schedule;
} sovereign_scheduler_shard_t;

/* Registry API */
void SovereignScheduler_InitRegistry(void);
sigma_err_t SovereignScheduler_Register(const char* name, sigma_schedule_fn schedule);
void sigma_schedule(sigma_u32 cpu_id, sigma_u64 now_ns);

#endif /* SOVEREIGN_SCHEDULER_H */
