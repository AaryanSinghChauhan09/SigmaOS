/* =========================================================================
 * S SIGMAOS: SOVEREIGN ROLLBACK SHARD HEADER
 * ========================================================================= */
#ifndef SOVEREIGN_ROLLBACK_SHARD_H
#define SOVEREIGN_ROLLBACK_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"
sigma_err_t sigma_rollback_snap     (const char* path);
sigma_err_t sigma_rollback_restore  (const char* snap_id);
void        sigma_rollback_prune    (sigma_u32 keep_last_n);
void        SovereignRollbackShard_Init (void);
void        SovereignRollback_Audit      (void);
#endif
