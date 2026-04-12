/* Σ SIGMAOS: SOVEREIGN AIRDROP SHARD HEADER */
#ifndef SOVEREIGN_AIRDROP_SHARD_H
#define SOVEREIGN_AIRDROP_SHARD_H
#include "sigma_types.h"

void        sigma_airdrop_scan     (void);
sigma_err_t sigma_airdrop_send     (const char* target_peer, const char* filepath);
void        SovereignAirDropShard_Init (void);
void        SovereignAirDrop_Audit     (void);

#endif
