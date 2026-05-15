/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM TELEMETRY (SST)
 * =========================================================================
 * Mission: Silicon-direct performance monitoring and shard-state observability.
 * =========================================================================
 */

#ifndef SIGMA_TELEMETRY_H
#define SIGMA_TELEMETRY_H

#include "include/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 cpu_load_pct;
    sigma_u32 mem_usage_kb;
    sigma_u32 active_shards;
    sigma_u32 lattice_temp_c;
} sigma_telemetry_data_t;

/* --- Telemetry Primitives --- */
void telemetry_init(void);
sigma_telemetry_data_t telemetry_get_snapshot(void);
void telemetry_log_shard_event(uint32_t shard_id, const char* event);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TELEMETRY_H */
