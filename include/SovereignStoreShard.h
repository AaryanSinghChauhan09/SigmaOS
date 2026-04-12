/* Σ SIGMAOS: SOVEREIGN STORE SHARD HEADER */
#ifndef SOVEREIGN_STORE_SHARD_H
#define SOVEREIGN_STORE_SHARD_H
#include "sigma_types.h"

typedef enum { ASSET_SHARD, ASSET_DATA, ASSET_PLUGIN } SigmaAssetType_t;

sigma_err_t sigma_store_register (const char* sku, const char* name, SigmaAssetType_t type);
sigma_err_t sigma_store_acquire  (const char* sku);
void        SovereignStoreShard_Init   (void);
void        SovereignStore_Audit       (void);

#endif
