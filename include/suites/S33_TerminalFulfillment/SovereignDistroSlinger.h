/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DISTRO SLINGER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_DISTRO_SLINGER_H
#define SOVEREIGN_DISTRO_SLINGER_H

#include "sigma_types.h"
#include "SigmaOOP.h"

typedef enum {
    PERSONA_SIGMA,
    PERSONA_LINUX,
    PERSONA_DARWIN,
    PERSONA_WINDOWS
} SigmaPersona_t;

typedef struct SovereignDistroSlinger {
    SigmaObject_t core;
    char current_distro[64];
    sigma_u32 active_instances;
    sigma_bool parity_mapped;
    SigmaPersona_t active_persona;
    
    sigma_err_t (*load_shard)(struct SovereignDistroSlinger* self, const char* path, const char* name);
    void        (*switch_persona)(struct SovereignDistroSlinger* self, SigmaPersona_t persona);
    sigma_err_t (*map_syscalls)(struct SovereignDistroSlinger* self);
    void        (*spawn_autonomous)(struct SovereignDistroSlinger* self);
    void        (*audit_shards)(struct SovereignDistroSlinger* self);
} SovereignDistroSlinger_t;

SovereignDistroSlinger_t SovereignDistroSlinger_Create(void);
void SovereignDistroSlinger_Init(void);

#endif /* SOVEREIGN_DISTRO_SLINGER_H */
