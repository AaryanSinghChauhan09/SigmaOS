/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SHARD MANAGER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SHARD_MANAGER_H
#define SOVEREIGN_SHARD_MANAGER_H

#include "sigma_types.h"

sigma_err_t sigma_shard_start          (const char* name, sigma_bool essential);
void        sigma_shard_stop           (const char* name);
void        SovereignShardManager_Init (void);
void        SovereignShardManager_Audit(void);

#endif /* SOVEREIGN_SHARD_MANAGER_H */
