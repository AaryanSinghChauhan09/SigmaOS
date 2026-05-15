/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GLOBAL LATTICE SYNC (S-SYNC)
 * =========================================================================
 * Mission: Zero-latency global state synchronisation across lattice shards.
 * Competitor parity: Chrony/NTP, Git-sync, AWS Global Accelerator (concept).
 * ZERO-DEPENDENCY: Strictly silicon-native sync protocols.
 * =========================================================================
 */

#ifndef SIGMA_GLOBAL_SYNC_H
#define SIGMA_GLOBAL_SYNC_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Sync Status --- */
#define SIGMA_SYNC_IDLE       0x00u
#define SIGMA_SYNC_PUSHING    0x01u
#define SIGMA_SYNC_PULLING    0x02u
#define SIGMA_SYNC_RECONCILE  0x03u

typedef struct {
    sigma_u32 sync_status;
    sigma_u64 last_sync_us;
    sigma_u64 total_payload_bytes;
    sigma_u32 drift_ms;
} sigma_sync_state_t;

/* --- Sync Primitives --- */
void      sync_init(void);
void      sync_lattice_push(sigma_u32 shard_id, const void* data, sigma_size_t size);
void      sync_lattice_pull(sigma_u32 shard_id, void* out_data, sigma_size_t size);
void      sync_reconcile_all(void);
const sigma_sync_state_t* sync_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_GLOBAL_SYNC_H */
