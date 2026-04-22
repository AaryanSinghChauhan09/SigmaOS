/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DSA SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_DSA_SHARD_H
#define SOVEREIGN_DSA_SHARD_H

#include "sigma_types.h"
#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

typedef struct SovereignDSAShard {
    SigmaObject_t core;
    const char* active_algo;
    sigma_u64 total_ops;
    
    void (*sort_quicksort)(struct SovereignDSAShard* self, sigma_u32* arr, sigma_sz_t size);
    void* (*map_silicon_shard)(struct SovereignDSAShard* self, sigma_u64 phys_addr, sigma_sz_t size);
    void (*audit_complexity)(struct SovereignDSAShard* self);
} SovereignDSAShard_t;

SovereignDSAShard_t SovereignDSA_Create(void);
void SovereignDSA_Register(void);

#endif /* SOVEREIGN_DSA_SHARD_H */
