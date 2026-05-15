#include "../../../include/SovereignLibC.h"
#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-CGROUP-SHARD (v1.0 - RESOURCE ISOLATION)
 * =============================================================================
 * Algorithm: Sharded-Resource Limiting (SRL)
 * Principles:
 *   - Kernel-native resource isolation for shards (Absorbing Linux cgroups).
 *   - Absolute industrial sovereignty in sharded performance management.
 *   - O(1) CPU/Memory budget verification per-shard-group.
 * Reference: Linux Control Groups (cgroups).
 * =============================================================================
 */

#include "../../../include/core/sigma_kernel_types.h"

#define MAX_CGROUPS 16

typedef struct SovereignCGroup {
    char        name[32];
    sigma_u32         cpu_weight;
    sigma_u64         memory_limit;
    sigma_u64         memory_usage;
    sigma_bool      active;
} SovereignCGroup;

static SovereignCGroup g_cgroups[MAX_CGROUPS];
static sigma_u32 g_cgroup_count = 0;

/* =========================================================================
 * CGROUP Engine (The Budget Shard)
 * ========================================================================= */

void cgroup_init(void) {
<<<<<<<< HEAD:suites/S30_Supremacy/cgroup_shard.c
    for (int i = 0; i < MAX_CGROUPS; i++) g_cgroups[i].active = FALSE;
    // ksigma_printf("[CGROUP]: Sovereign Resource-Isolation Shard Online.\n");
========
    for (int i = 0; i < MAX_CGROUPS; i++) g_cgroups[i].active = SIGMA_FALSE;
    // kprintf("[CGROUP]: Sovereign Resource-Isolation Shard Online.\n");
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/shards/system/cgroup_shard.c
}

sigma_status cgroup_create(const char* name, sigma_u32 weight, sigma_u64 mem_limit) {
    if (g_cgroup_count >= MAX_CGROUPS) return K_ERR_NOMEM;
    
    SovereignCGroup* cg = &g_cgroups[g_cgroup_count++];
    sigma_usize i = 0; while (i < 31 && name[i]) { cg->name[i] = name[i]; i++; }
    cg->name[i]   = '\0';
    cg->cpu_weight = weight;
    cg->memory_limit = mem_limit;
    cg->memory_usage = 0;
    cg->active    = SIGMA_TRUE;
    
    // ksigma_printf("[CGROUP]: Industrial Budget Created: %s (Weight: %u)\n", name, weight);
    return K_OK;
}

sigma_bool cgroup_limit_check(sigma_u32 cg_id, sigma_u64 mem_req) {
    if (cg_id >= MAX_CGROUPS || !g_cgroups[cg_id].active) return SIGMA_TRUE;
    
    if (g_cgroups[cg_id].memory_usage + mem_req > g_cgroups[cg_id].memory_limit) {
        return SIGMA_FALSE; /* Absolute Industrial Limit */
    }
    return SIGMA_TRUE;
}
