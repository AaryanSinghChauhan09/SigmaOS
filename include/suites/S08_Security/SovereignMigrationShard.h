/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN MIGRATION SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_MIGRATION_SHARD_H
#define SOVEREIGN_MIGRATION_SHARD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_migrate_checkpoint (const char* shard_id);
void        sigma_migrate_push       (const char* shard_id, const char* target_node);
void        SovereignMigrationShard_Init (void);
void        SovereignMigration_Audit     (void);

#endif /* SOVEREIGN_MIGRATION_SHARD_H */
