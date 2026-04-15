/* S SIGMAOS: SOVEREIGN DOCK SHARD HEADER */
#ifndef SOVEREIGN_DOCK_SHARD_H
#define SOVEREIGN_DOCK_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_dock_pin     (const char* name, const char* cmd);
void        sigma_dock_launch  (const char* name);
void        SovereignDockShard_Init (void);
void        SovereignDock_Audit     (void);

#endif
