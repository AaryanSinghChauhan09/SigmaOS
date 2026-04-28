#ifndef SIGMA_SYSTEM_SHARDS_H
#define SIGMA_SYSTEM_SHARDS_H

#include "sigma_types.h"

/* Sovereign Net Mesh Shard */
typedef struct SovereignNetZenith {
    const char* type_name;
    sigma_u64   handshakes;
    sigma_u64   dns_queries;
    sigma_u32   active_connections;
    sigma_u32   packets_sequenced;
    sigma_bool  firewall_shard_active;
} SovereignNetZenith;

/* Sovereign Scheduler Shard */
typedef struct SovereignScheduler {
    const char* type_name;
    sigma_u32 task_count;
    sigma_u32 context_switches;
    sigma_u8  cpu_affinity;
} SovereignScheduler;

/* Sovereign UI Engine Shard */
typedef struct SovereignUIEngine {
    const char* type_name;
    sigma_u32 layers_composited;
    sigma_u32 fps_zenith;
    sigma_bool glass_blur_active;
    sigma_u64   frames_rendered;
} SovereignUIEngine;

#endif /* SIGMA_SYSTEM_SHARDS_H */
