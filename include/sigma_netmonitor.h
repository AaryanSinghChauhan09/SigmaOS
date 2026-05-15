/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NETWORK MONITOR (S-NETMONITOR)
 * =========================================================================
 * Mission: Real-time, per-process network traffic visualization with 
 * anomaly detection and bandwidth throttling per shard.
 * =========================================================================
 */

#ifndef SIGMA_NETMONITOR_H
#define SIGMA_NETMONITOR_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Network Monitor Primitives --- */
void netmonitor_init(void);
void netmonitor_poll_traffic(void);
void netmonitor_throttle_shard(uint32_t shard_id, uint32_t max_kbps);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NETMONITOR_H */
