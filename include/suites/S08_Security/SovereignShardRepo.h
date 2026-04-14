/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SHARD REPOSITORY HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SHARD_REPO_H
#define SOVEREIGN_SHARD_REPO_H

#include "sigma_types.h"

sigma_err_t sigma_repo_pull          (const char* name);
void        sigma_repo_list          (void);
void        SovereignShardRepo_Init  (void);

#endif /* SOVEREIGN_SHARD_REPO_H */
