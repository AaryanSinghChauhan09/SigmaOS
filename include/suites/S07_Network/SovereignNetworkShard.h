/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN NETWORK SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_NETWORK_SHARD_H
#define SOVEREIGN_NETWORK_SHARD_H

#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

typedef struct {
    char      iface_name[16];
    sigma_u32 ip_addr;
    sigma_u32 packets_switched;
    sigma_bool link_up;
} SigmaNetIface_t;

typedef struct {
    SigmaObject_t core;
    SigmaNetIface_t eth0;
} SovereignNetworkShard_t;

SovereignNetworkShard_t SovereignNetworkShard_Create(void);
void sigma_net_zero_copy_dispatch(void* packet_ring, sigma_u32 count);
void SovereignNetworkShard_Audit(SovereignNetworkShard_t* self);
void SovereignNetworkShard_Init(void);

#endif /* SOVEREIGN_NETWORK_SHARD_H */
