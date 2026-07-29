/**
 * =========================================================================
 * Σ SIGMAOS: CGROUP ENFORCEMENT (Phase E)
 * =========================================================================
 * Binds CPU, memory, and I/O limits from pod specifications to kernel
 * cgroup shards.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_error_codes.h"
#include <sigma_libc.h>

namespace SigmaOS {
namespace Orchestrator {

// -------------------------------------------------------------------------
// Types
// -------------------------------------------------------------------------
struct CgroupShard {
    sigma_u32 id;
    sigma_u32 cpu_quota_pct; // 0-100%
    sigma_u64 memory_limit_mb;
    sigma_u32 io_weight;     // 1-1000
    bool active;
};

#define MAX_CGROUP_SHARDS 256
static CgroupShard g_shards[MAX_CGROUP_SHARDS];
static sigma_u32 g_next_shard_id = 1;

// -------------------------------------------------------------------------
// Initialization
// -------------------------------------------------------------------------
void sigma_cgroup_init() {
    sys_print("[CGroup] Initializing Kernel CGroup Shards...\n");
    sigma_memset(g_shards, 0, sizeof(g_shards));
}

// -------------------------------------------------------------------------
// Enforcement
// -------------------------------------------------------------------------
sigma_u32 sigma_cgroup_create_shard(sigma_u32 cpu_pct, sigma_u64 mem_mb, sigma_u32 io_w) {
    for (int i = 0; i < MAX_CGROUP_SHARDS; i++) {
        if (!g_shards[i].active) {
            g_shards[i].id = g_next_shard_id++;
            g_shards[i].cpu_quota_pct = cpu_pct > 100 ? 100 : cpu_pct;
            g_shards[i].memory_limit_mb = mem_mb;
            g_shards[i].io_weight = io_w;
            g_shards[i].active = true;
            
            sys_print("[CGroup] Created Shard %u: CPU %u%%, Mem %llu MB, IO %u\n",
                      g_shards[i].id, g_shards[i].cpu_quota_pct, 
                      g_shards[i].memory_limit_mb, g_shards[i].io_weight);
                      
            return g_shards[i].id;
        }
    }
    
    sys_print("[CGroup] ERROR: Max cgroup shards reached!\n");
    return 0; // Invalid ID
}

sigma_status sigma_cgroup_bind_task(sigma_u32 shard_id, sigma_u32 task_id) {
    // In a real implementation, this would link the task_struct to the cgroup.
    sys_print("[CGroup] Binding Task %u to CGroup Shard %u.\n", task_id, shard_id);
    return SIGMA_SUCCESS;
}

sigma_status sigma_cgroup_enforce_memory(sigma_u32 shard_id, sigma_u64 requested_mb) {
    for (int i = 0; i < MAX_CGROUP_SHARDS; i++) {
        if (g_shards[i].active && g_shards[i].id == shard_id) {
            // Check if requested memory exceeds limit
            if (requested_mb > g_shards[i].memory_limit_mb) {
                sys_print("[CGroup] Shard %u OOM! Requested %llu MB, Limit %llu MB.\n",
                          shard_id, requested_mb, g_shards[i].memory_limit_mb);
                return K_ERR_NO_MEM;
            }
            return SIGMA_SUCCESS;
        }
    }
    return K_ERR_INVAL;
}

} // namespace Orchestrator
} // namespace SigmaOS

extern "C" {
    void sigma_cgroup_init_c() {
        SigmaOS::Orchestrator::sigma_cgroup_init();
    }
}
