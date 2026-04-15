#include "sigma_base.h"

#include "SovereignLatticeRegistry.h"
#include "sigma_libc.h"

static sovereign_registry_t g_registry;

void SovereignRegistry_Init(void) {
    sigma_memset(&g_registry, 0, sizeof(sovereign_registry_t));
    sigma_printf("Σ [REGISTRY]: Sovereign Shard Registry Online. Capacity: %d shards.\n", MAX_SHARDS);
}

sigma_err_t SovereignRegistry_Register(const char* name, shard_category_t cat, shard_init_fn init) {
    if (g_registry.shard_count >= MAX_SHARDS) {
        sigma_printf("Σ [REGISTRY/ERR]: Maximum shard capacity reached! Cannot register %s.\n", name);
        return SIGMA_ENOSPC;
    }

    sovereign_shard_t* shard = &g_registry.shards[g_registry.shard_count++];
    sigma_strncpy(shard->name, name, SHARD_NAME_MAX);
    shard->category = cat;
    shard->init = init;
    shard->status = SHARD_STATUS_REGISTERED;

    sigma_printf("Σ [REGISTRY]: Registered Shard [%s] in Category %d.\n", name, cat);
    
    /* Auto-initialize if requested or wait for Finalize */
    if (init) {
        shard->status = SHARD_STATUS_INITIALIZING;
        init();
        shard->status = SHARD_STATUS_ACTIVE;
        g_registry.active_count++;
    }

    return SIGMA_OK;
}

void SovereignRegistry_Finalize(void) {
    sigma_printf("Σ [REGISTRY]: Finalizing Shard Matrix. Shards: %d | Active: %d.\n", 
                 g_registry.shard_count, g_registry.active_count);
}

void SovereignRegistry_Audit(void) {
    sigma_printf("--- Σ SOVEREIGN REGISTRY AUDIT REPORT ---\n");
    for (sigma_u32 i = 0; i < g_registry.shard_count; i++) {
        sovereign_shard_t* s = &g_registry.shards[i];
        sigma_printf("Shard %d: %s | Status: %d | Category: %d\n", i, s->name, s->status, s->category);
    }
}



