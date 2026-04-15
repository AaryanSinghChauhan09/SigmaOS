/* S SIGMAOS: SOVEREIGN RESTORE SHARD HEADER */
#ifndef SOVEREIGN_RESTORE_SHARD_H
#define SOVEREIGN_RESTORE_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_restore_checkpoint (const char* label);
sigma_err_t sigma_restore_rollback   (sigma_u32 rid);
void        SovereignRestoreShard_Init (void);
void        SovereignRestore_Audit     (void);

#endif
