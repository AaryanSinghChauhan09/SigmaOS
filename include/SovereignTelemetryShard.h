/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TELEMETRY SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_TELEMETRY_SHARD_H
#define SOVEREIGN_TELEMETRY_SHARD_H

#include "sigma_types.h"

typedef enum {
    PROBE_KPROBE,
    PROBE_UPROBE,
    PROBE_TRACEPOINT,
    PROBE_PERF_EVENT
} SigmaProbeType_t;

sigma_err_t sigma_tele_probe_arm    (const char* name, sigma_u64 addr, SigmaProbeType_t type);
void        sigma_tele_sample       (void);
void        sigma_tele_map_flush    (void);
void        SovereignTelemetryShard_Init (void);
void        SovereignTelemetry_Audit     (void);

#endif /* SOVEREIGN_TELEMETRY_SHARD_H */
