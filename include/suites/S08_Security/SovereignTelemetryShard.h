/* Σ SIGMAOS: SOVEREIGN TELEMETRY SHARD HEADER */
#ifndef SOVEREIGN_TELEMETRY_SHARD_H
#define SOVEREIGN_TELEMETRY_SHARD_H
#include "sigma_types.h"

void sigma_telemetry_emit   (const char* metric, sigma_u32 value);
void sigma_telemetry_export (void);
void SovereignTelemetryShard_Init (void);
void SovereignTelemetry_Audit     (void);

#endif
