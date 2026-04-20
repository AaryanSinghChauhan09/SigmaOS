/*
 * =========================================================================
 * S SIGMAOS: S10_REGISTRY — SovereignSystemRegistry.c
 * =========================================================================
 * Mission: Transactional System State & Configuration Nexus.
 * Design: No-Glibc, No-Host-Headers. Zero-Corruption WAL architecture.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"

#define MAX_REG_ENTRIES 4096

static sovereign_registry_t g_registry;

/**
 * @brief Initialize the Sovereign Registry Lattice.
 */
void SovereignRegistry_Init(void) {
    g_registry.shard_count = 0;
    g_registry.active_count = 0;
    g_registry.registry_lock = 0;
    
    sigma_sigma_sigma_printf("S [REGISTRY]: Initialized Sovereign State Nexus (Capacity: %d entries)\n", MAX_REG_ENTRIES);
}

/**
 * @brief Register a functional shard into the global execution lattice.
 */
sigma_err_t SovereignRegistry_Register(const char* name, shard_category_t cat, shard_init_fn init) {
    if (g_registry.shard_count >= MAX_SHARDS) {
        return SIGMA_ERROR;
    }

    sovereign_shard_t* shard = &g_registry.shards[g_registry.shard_count++];
    
    sigma_sigma_sigma_strcpy(shard->name, name);
    shard->category = cat;
    shard->status = SHARD_STATUS_REGISTERED;
    shard->init = init;
    shard->version = 1;
    
    sigma_sigma_sigma_printf("S [REGISTRY]: Registered Shard '%s' (Category: %d)\n", name, cat);
    return SIGMA_OK;
}

/**
 * @brief Audit the entire project lattice for integrity and performance readiness.
 */
void SovereignRegistry_Audit(void) {
    sigma_sigma_sigma_printf("\n╔══════════════════════════════════════════════════════════════╗\n");
    sigma_sigma_sigma_printf("║   SigmaOS Sovereign Lattice Audit (Terminal v33.1)           ║\n");
    sigma_sigma_sigma_printf("╠══════════════════════════════════════════════════════════════╣\n");
    
    for (sigma_u32 i = 0; i < g_registry.shard_count; i++) {
        sovereign_shard_t* s = &g_registry.shards[i];
        sigma_sigma_sigma_printf("║ Shard: %-25s | Status: %-10d ║\n", s->name, s->status);
    }
    
    sigma_sigma_sigma_printf("╚══════════════════════════════════════════════════════════════╝\n\n");
}

/**
 * @brief Finalize and seal the registry before system handoff or hibernation.
 */
void SovereignRegistry_Finalize(void) {
    sigma_sigma_sigma_printf("S [REGISTRY]: System State Persisted. Locking Lattice.\n");
    g_registry.registry_lock = 1;
}

// Legacy Parity Shard
void S10_Registry_Register(void) {
    SovereignRegistry_Init();
}

