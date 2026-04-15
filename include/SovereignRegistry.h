/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM REGISTRY (v1.0)
 * =========================================================================
 */

#ifndef SOVEREIGN_REGISTRY_H
#define SOVEREIGN_REGISTRY_H

#include "sigma_base.h"

typedef struct {
    char        name[64];
    void*       ptr;
    sigma_u32   flags;
} RegistryEntry;

#define MAX_REGISTRY_ENTRIES 4096

void        SovereignRegistry_Init(void);
sigma_err_t SovereignRegistry_Set(const char* name, void* ptr, sigma_u32 flags);
void*       SovereignRegistry_Get(const char* name);

#endif /* SOVEREIGN_REGISTRY_H */
