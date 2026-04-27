/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-CGROUP-SHARD (v1.0 - RESOURCE ISOLATION)
 * =============================================================================
 * Algorithm: Sharded-Resource Limiting (SRL)
 * Principles:
 *   - Kernel-native resource isolation for shards (Absorbing Linux cgroups).
 *   - Absolute industrial sovereignty in sharded performance management.
 *   - O(1) CPU/Memory budget verification per-shard-group.
 * Reference: Linux Control Groups (cgroups).
 * =============================================================================
 */

#include "../include/sigma_kernel_types.h"

#define MAX_CGROUPS 16

typedef struct SovereignCGroup {
    char        name[32];
    u32         cpu_weight;
    u64         memory_limit;
    u64         memory_usage;
    bool_t      active;
} SovereignCGroup;

static SovereignCGroup g_cgroups[MAX_CGROUPS];
static u32 g_cgroup_count = 0;

/* =========================================================================
 * CGROUP Engine (The Budget Shard)
 * ========================================================================= */

void cgroup_init(void) {
    for (int i = 0; i < MAX_CGROUPS; i++) g_cgroups[i].active = FALSE;
    // kprintf("[CGROUP]: Sovereign Resource-Isolation Shard Online.\n");
}

k_status cgroup_create(const char* name, u32 weight, u64 mem_limit) {
    if (g_cgroup_count >= MAX_CGROUPS) return K_ERR_NOMEM;
    
    SovereignCGroup* cg = &g_cgroups[g_cgroup_count++];
    usize i = 0; while (i < 31 && name[i]) { cg->name[i] = name[i]; i++; }
    cg->name[i]   = '\0';
    cg->cpu_weight = weight;
    cg->memory_limit = mem_limit;
    cg->memory_usage = 0;
    cg->active    = TRUE;
    
    // kprintf("[CGROUP]: Industrial Budget Created: %s (Weight: %u)\n", name, weight);
    return K_OK;
}

bool_t cgroup_limit_check(u32 cg_id, u64 mem_req) {
    if (cg_id >= MAX_CGROUPS || !g_cgroups[cg_id].active) return TRUE;
    
    if (g_cgroups[cg_id].memory_usage + mem_req > g_cgroups[cg_id].memory_limit) {
        return FALSE; /* Absolute Industrial Limit */
    }
    return TRUE;
}
