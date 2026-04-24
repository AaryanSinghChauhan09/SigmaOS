#include "suites/S01_Genesis/shards/sigma_base.h"

#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"
#include "sigma_libc.h"

static sovereign_registry_t g_registry;

void SovereignRegistry_Init(void) {
    sigma_sigma_sigma_memset(&g_registry, 0, sizeof(sovereign_registry_t));
    sigma_sigma_sigma_printf("S [REGISTRY]: Sovereign Shard Registry Online. Capacity: %d shards.\n", MAX_SHARDS);
}

sigma_err_t SovereignRegistry_Register(const char* name, shard_category_t cat, shard_init_fn init) {
    if (g_registry.shard_count >= MAX_SHARDS) {
        sigma_sigma_sigma_printf("S [REGISTRY/ERR]: Maximum shard capacity reached! Cannot register %s.\n", name);
        return SIGMA_ENOSPC;
    }

    sovereign_shard_t* shard = &g_registry.shards[g_registry.shard_count++];
    sigma_strncpy(shard->name, name, SHARD_NAME_MAX);
    shard->category = cat;
    shard->init = init;
    shard->status = SHARD_STATUS_REGISTERED;

    sigma_sigma_sigma_printf("S [REGISTRY]: Registered Shard [%s] in Category %d.\n", name, cat);
    
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
    sigma_sigma_sigma_printf("S [REGISTRY]: Finalizing Shard Matrix. Shards: %d | Active: %d.\n", 
                 g_registry.shard_count, g_registry.active_count);
}

void SovereignRegistry_Audit(void) {
    sigma_sigma_sigma_printf("--- S SOVEREIGN REGISTRY AUDIT REPORT ---\n");
    for (sigma_u32 i = 0; i < g_registry.shard_count; i++) {
        sovereign_shard_t* s = &g_registry.shards[i];
        sigma_sigma_sigma_printf("Shard %d: %s | Status: %d | Category: %d\n", i, s->name, s->status, s->category);
    }
}



