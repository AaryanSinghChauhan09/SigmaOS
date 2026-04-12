/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DISTRO SLINGER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_DISTRO_SLINGER_H
#define SOVEREIGN_DISTRO_SLINGER_H

#include "SigmaOOP.h"

CLASS_DECLARE(SovereignDistroSlinger) {
    SigmaObject_t core;
    char current_distro[64];
    sigma_u32 active_instances;
    sigma_bool parity_mapped;
    
    VIRTUAL(sigma_err_t, load_shard, struct SovereignDistroSlinger* self, const char* path, const char* name);
    VIRTUAL(sigma_err_t, map_syscalls, struct SovereignDistroSlinger* self);
    VIRTUAL(void, spawn_autonomous, struct SovereignDistroSlinger* self);
    VIRTUAL(void, audit_shards, struct SovereignDistroSlinger* self);
};

SovereignDistroSlinger_t SovereignDistroSlinger_Create(void);
void SovereignDistroSlinger_Init(void);

#endif /* SOVEREIGN_DISTRO_SLINGER_H */
