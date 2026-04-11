#include "../../include/SovereignScheduler.h"
#include "../../include/sigma_libc.h"
#include "../../include/sigma_string.h"

#define MAX_SCHED_SHARDS 4
static sovereign_scheduler_shard_t g_sched_shards[MAX_SCHED_SHARDS];
static sigma_u32 g_sched_shard_count = 0;
static sigma_u32 g_active_sched = 0;

void SovereignScheduler_InitRegistry(void) {
    sigma_memset(g_sched_shards, 0, sizeof(g_sched_shards));
    g_sched_shard_count = 0;
    sigma_printf("Σ [SCHED]: Sovereign Scheduler Registry Operational.\n");
}

sigma_err_t SovereignScheduler_Register(const char* name, sigma_schedule_fn schedule) {
    if (g_sched_shard_count >= MAX_SCHED_SHARDS) return SIGMA_ENOSPC;

    sovereign_scheduler_shard_t* s = &g_sched_shards[g_sched_shard_count++];
    sigma_strncpy(s->name, name, 32);
    s->schedule = schedule;
    
    sigma_printf("Σ [SCHED]: Registered Scheduler Shard '%s'\n", name);
    return SIGMA_OK;
}

void sigma_schedule(sigma_u32 cpu_id, sigma_u64 now_ns) {
    if (g_sched_shard_count > 0 && g_sched_shards[g_active_sched].schedule) {
        g_sched_shards[g_active_sched].schedule(cpu_id, now_ns);
    }
}
