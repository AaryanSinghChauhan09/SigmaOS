/* S SIGMAOS: SOVEREIGN VAULT SHARD HEADER */
#ifndef SOVEREIGN_VAULT_SHARD_H
#define SOVEREIGN_VAULT_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_vault_seal   (const char* path, const char* val, sigma_u32 clearance);
const char* sigma_vault_unseal (const char* path, sigma_u32 requester_clearance);
void        SovereignVaultShard_Init (void);
void        SovereignVault_Audit     (void);

#endif
