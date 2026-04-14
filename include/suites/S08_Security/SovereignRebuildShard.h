/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN REBUILD SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_REBUILD_SHARD_H
#define SOVEREIGN_REBUILD_SHARD_H

#include "sigma_types.h"

sigma_err_t sigma_rebuild_system        (void);
void        sigma_rebuild_rollback      (void);
void        SovereignRebuildShard_Init  (void);
void        SovereignRebuild_Audit      (void);

#endif /* SOVEREIGN_REBUILD_SHARD_H */
