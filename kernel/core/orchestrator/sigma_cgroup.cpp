/**
 * =========================================================================
 * Σ SIGMAOS: CGROUP v2 RESOURCE CONTROLLER  [#900]
 * =========================================================================
 * Implements CPU, memory, and I/O resource enforcement for sigma-pod
 * containers.  Modelled on Linux cgroups v2 but tightly integrated into
 * the SigmaOS Lattice Shard scheduler.
 *
 * Architecture
 * ─────────────
 *  sigma_cgroup_create()
 *     └─ allocates a sigma_cgroup shard with resource slots
 *  sigma_cgroup_attach_pid()
 *     └─ binds a kernel process shard to the cgroup
 *  sigma_cgroup_enforce_cpu()
 *     └─ called by the Lattice scheduler on each tick to enforce CPU quotas
 *  sigma_cgroup_enforce_memory()
 *     └─ called by the PMM on allocation paths to enforce memory limits
 *  sigma_cgroup_enforce_io()
 *     └─ called by the block I/O layer (NVMe/SATA queue submission paths)
 *
 * cgroup_apply_pod_limits() ── C-linkage wrapper called by the orchestrator.
 *
 * =========================================================================
 */

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/sigma_error_codes.h"
#include "sigma_cgroup.h"

namespace SigmaOS {
namespace Cgroup {

/* -------------------------------------------------------------------------
 * Constants
 * ---------------------------------------------------------------------- */
#define CGROUP_MAX_GROUPS        64u
#define CGROUP_MAX_PIDS_PER      32u
#define CGROUP_SCHED_PERIOD_US   100000u   /* 100 ms scheduling period */
#define CGROUP_IO_WEIGHT_DEFAULT 100u      /* relative I/O weight (1-1000) */

/* -------------------------------------------------------------------------
 * Internal cgroup shard structure
 * ---------------------------------------------------------------------- */
struct CgroupShard {
    sigma_u32 id;
    char      name[40];
    bool      active;

    /* CPU controller */
    sigma_u32 cpu_quota_us;     /* microseconds of CPU per period */
    sigma_u32 cpu_period_us;    /* scheduling period (default 100 ms) */
    sigma_u64 cpu_runtime_us;   /* accumulated runtime in current period */
    sigma_u64 cpu_throttle_count;

    /* Memory controller */
    sigma_u64 mem_limit_bytes;  /* hard limit (0 = unlimited) */
    sigma_u64 mem_current_bytes;/* currently allocated */
    sigma_u64 mem_peak_bytes;
    sigma_u64 oom_kill_count;

    /* I/O controller */
    sigma_u32 io_weight;        /* 1–1000, relative weight (cfq/bfq-like) */
    sigma_u64 io_bytes_written;
    sigma_u64 io_bytes_read;
    sigma_u64 io_throttle_count;

    /* Process membership */
    sigma_u32 pids[CGROUP_MAX_PIDS_PER];
    sigma_u32 pid_count;
};

/* -------------------------------------------------------------------------
 * Subsystem state
 * ---------------------------------------------------------------------- */
static CgroupShard  s_groups[CGROUP_MAX_GROUPS];
static sigma_u32    s_group_count = 0;
static bool         s_cgroup_ready = false;

/* -------------------------------------------------------------------------
 * Helper: find cgroup by name
 * ---------------------------------------------------------------------- */
static CgroupShard* find_by_name(const char* name)
{
    for (sigma_u32 i = 0; i < s_group_count; i++) {
        if (s_groups[i].active) {
            /* simple strcmp */
            const char* a = s_groups[i].name;
            const char* b = name;
            while (*a && *a == *b) { a++; b++; }
            if (*a == *b) return &s_groups[i];
        }
    }
    return nullptr;
}

/* =========================================================================
 * Public API
 * ======================================================================= */

/**
 * sigma_cgroup_init() — Initialise the cgroup subsystem.
 * Call once during kernel boot after the scheduler is up.
 */
void sigma_cgroup_init(void)
{
    sigma_memset(s_groups, 0, sizeof(s_groups));
    s_group_count = 0;
    s_cgroup_ready = true;
    sigma_log_info("[Cgroup] Cgroup v2 controller online (max %u groups).", CGROUP_MAX_GROUPS);
}

/**
 * sigma_cgroup_create() — Create a new resource-controlled group.
 *
 * @name         : human-readable name (pod name, container name, etc.)
 * @cpu_millis   : CPU quota in milli-CPU (1000 = 1 full core per period)
 * @mem_mb       : memory limit in MiB (0 = unlimited)
 * @io_weight    : I/O weight 1-1000 (100 = default)
 * @id_out       : filled with the new cgroup ID on success
 */
sigma_status sigma_cgroup_create(const char*  name,
                                  sigma_u32    cpu_millis,
                                  sigma_u32    mem_mb,
                                  sigma_u32    io_weight,
                                  sigma_u32*   id_out)
{
    if (!s_cgroup_ready) return K_ERR_INVAL;
    if (s_group_count >= CGROUP_MAX_GROUPS) {
        sigma_log_err("[Cgroup] Cannot create '%s': cgroup table full.", name);
        return K_ERR_NOMEM;
    }

    CgroupShard* g = &s_groups[s_group_count];
    sigma_memset(g, 0, sizeof(*g));

    g->id     = s_group_count;
    g->active = true;
    sigma_strncpy(g->name, name, sizeof(g->name) - 1);

    /* CPU: convert milli-CPU to microseconds per period */
    g->cpu_period_us = CGROUP_SCHED_PERIOD_US;
    if (cpu_millis > 0 && cpu_millis <= 64000u) {
        /* cpu_quota_us = (cpu_millis / 1000) * period_us */
        g->cpu_quota_us = (cpu_millis * CGROUP_SCHED_PERIOD_US) / 1000u;
    } else {
        g->cpu_quota_us = CGROUP_SCHED_PERIOD_US; /* default: 1 full core */
    }

    /* Memory */
    g->mem_limit_bytes  = (sigma_u64)mem_mb * 1024ULL * 1024ULL;

    /* I/O */
    g->io_weight = (io_weight >= 1 && io_weight <= 1000) ?
                    io_weight : CGROUP_IO_WEIGHT_DEFAULT;

    if (id_out) *id_out = g->id;
    s_group_count++;

    sigma_log_info("[Cgroup] Created group[%u] '%s' — cpu=%u ms/period mem=%u MB io_weight=%u",
                   g->id, g->name, cpu_millis, mem_mb, g->io_weight);
    return K_OK;
}

/**
 * sigma_cgroup_attach_pid() — Add a kernel process/task to a cgroup.
 */
sigma_status sigma_cgroup_attach_pid(sigma_u32 cgroup_id, sigma_u32 pid)
{
    if (cgroup_id >= s_group_count) return K_ERR_INVAL;
    CgroupShard* g = &s_groups[cgroup_id];

    if (g->pid_count >= CGROUP_MAX_PIDS_PER) {
        sigma_log_err("[Cgroup] Group '%s': PID table full (max %u).",
                      g->name, CGROUP_MAX_PIDS_PER);
        return K_ERR_NOMEM;
    }
    g->pids[g->pid_count++] = pid;
    sigma_log_info("[Cgroup] PID %u attached to group '%s'.", pid, g->name);
    return K_OK;
}

/**
 * sigma_cgroup_enforce_cpu() — Called by the scheduler on every tick.
 * If the group has exhausted its quota, it is throttled (marked for descheduling).
 *
 * @cgroup_id  : group to check
 * @elapsed_us : microseconds of CPU consumed since last call
 * Returns:  K_OK if the task may continue, K_ERR_BUSY if throttled.
 */
sigma_status sigma_cgroup_enforce_cpu(sigma_u32 cgroup_id, sigma_u32 elapsed_us)
{
    if (cgroup_id >= s_group_count) return K_OK; /* no group — no limit */
    CgroupShard* g = &s_groups[cgroup_id];

    g->cpu_runtime_us += elapsed_us;

    if (g->cpu_quota_us > 0 && g->cpu_runtime_us > g->cpu_quota_us) {
        g->cpu_throttle_count++;
        /* Reset accumulated runtime at start of next period */
        g->cpu_runtime_us = 0;
        sigma_log_info("[Cgroup] '%s': CPU throttled (quota %u us/period, throttle #%llu).",
                       g->name, g->cpu_quota_us, g->cpu_throttle_count);
        return K_ERR_BUSY; /* caller deschedules task for remainder of period */
    }
    return K_OK;
}

/**
 * sigma_cgroup_enforce_memory() — Called by PMM on allocation.
 * Returns K_ERR_NOMEM if the limit would be exceeded (triggers OOM handler).
 *
 * @cgroup_id  : group to check
 * @alloc_bytes: bytes about to be allocated
 */
sigma_status sigma_cgroup_enforce_memory(sigma_u32 cgroup_id, sigma_u64 alloc_bytes)
{
    if (cgroup_id >= s_group_count) return K_OK;
    CgroupShard* g = &s_groups[cgroup_id];

    if (g->mem_limit_bytes == 0) return K_OK; /* unlimited */

    if (g->mem_current_bytes + alloc_bytes > g->mem_limit_bytes) {
        g->oom_kill_count++;
        sigma_log_warn("[Cgroup] '%s': Memory limit exceeded! current=%llu MB limit=%llu MB OOM#%llu",
                       g->name,
                       g->mem_current_bytes / (1024*1024),
                       g->mem_limit_bytes   / (1024*1024),
                       g->oom_kill_count);
        return K_ERR_NOMEM; /* caller triggers OOM reclaim / kill */
    }

    g->mem_current_bytes += alloc_bytes;
    if (g->mem_current_bytes > g->mem_peak_bytes)
        g->mem_peak_bytes = g->mem_current_bytes;

    return K_OK;
}

/**
 * sigma_cgroup_release_memory() — Called by PMM on free().
 */
void sigma_cgroup_release_memory(sigma_u32 cgroup_id, sigma_u64 freed_bytes)
{
    if (cgroup_id >= s_group_count) return;
    CgroupShard* g = &s_groups[cgroup_id];
    if (g->mem_current_bytes >= freed_bytes)
        g->mem_current_bytes -= freed_bytes;
    else
        g->mem_current_bytes = 0;
}

/**
 * sigma_cgroup_enforce_io() — Called by NVMe/SATA queue submission paths.
 * Accounts I/O bytes; returns K_ERR_BUSY if the device-wide scheduler should
 * throttle this group's request.
 *
 * Throttling algorithm: proportional-share (weight / sum_of_weights).
 * Here we log and return K_OK; the block layer uses io_weight for CFQ-like scheduling.
 */
sigma_status sigma_cgroup_enforce_io(sigma_u32 cgroup_id,
                                      sigma_u64 bytes,
                                      bool      is_write)
{
    if (cgroup_id >= s_group_count) return K_OK;
    CgroupShard* g = &s_groups[cgroup_id];

    if (is_write) g->io_bytes_written += bytes;
    else          g->io_bytes_read    += bytes;

    return K_OK; /* weight-based scheduling handled by block I/O layer */
}

/**
 * sigma_cgroup_get_stats() — Fill a stats struct for userland / procfs.
 */
sigma_status sigma_cgroup_get_stats(sigma_u32 cgroup_id, sigma_cgroup_stats_t* out)
{
    if (cgroup_id >= s_group_count || !out) return K_ERR_INVAL;
    CgroupShard* g = &s_groups[cgroup_id];

    out->cpu_throttle_count  = g->cpu_throttle_count;
    out->mem_current_bytes   = g->mem_current_bytes;
    out->mem_peak_bytes      = g->mem_peak_bytes;
    out->mem_limit_bytes     = g->mem_limit_bytes;
    out->oom_kill_count      = g->oom_kill_count;
    out->io_bytes_read       = g->io_bytes_read;
    out->io_bytes_written    = g->io_bytes_written;
    out->io_throttle_count   = g->io_throttle_count;
    out->pid_count           = g->pid_count;
    return K_OK;
}

/**
 * sigma_cgroup_destroy() — Remove a cgroup (detaches all PIDs first).
 */
sigma_status sigma_cgroup_destroy(sigma_u32 cgroup_id)
{
    if (cgroup_id >= s_group_count) return K_ERR_INVAL;
    CgroupShard* g = &s_groups[cgroup_id];
    sigma_log_info("[Cgroup] Destroying group[%u] '%s'.", cgroup_id, g->name);
    g->active    = false;
    g->pid_count = 0;
    return K_OK;
}

} // namespace Cgroup
} // namespace SigmaOS

/* =========================================================================
 * C-linkage API — consumed by orchestrator and kernel subsystems
 * ======================================================================= */
extern "C" {

void sigma_cgroup_init(void) {
    SigmaOS::Cgroup::sigma_cgroup_init();
}

sigma_status sigma_cgroup_create(const char* name, sigma_u32 cpu_millis,
                                  sigma_u32 mem_mb, sigma_u32 io_weight,
                                  sigma_u32* id_out)
{
    return SigmaOS::Cgroup::sigma_cgroup_create(name, cpu_millis, mem_mb, io_weight, id_out);
}

sigma_status sigma_cgroup_attach_pid(sigma_u32 cgroup_id, sigma_u32 pid) {
    return SigmaOS::Cgroup::sigma_cgroup_attach_pid(cgroup_id, pid);
}

sigma_status sigma_cgroup_enforce_cpu(sigma_u32 cgroup_id, sigma_u32 elapsed_us) {
    return SigmaOS::Cgroup::sigma_cgroup_enforce_cpu(cgroup_id, elapsed_us);
}

sigma_status sigma_cgroup_enforce_memory(sigma_u32 cgroup_id, sigma_u64 alloc_bytes) {
    return SigmaOS::Cgroup::sigma_cgroup_enforce_memory(cgroup_id, alloc_bytes);
}

void sigma_cgroup_release_memory(sigma_u32 cgroup_id, sigma_u64 freed_bytes) {
    SigmaOS::Cgroup::sigma_cgroup_release_memory(cgroup_id, freed_bytes);
}

sigma_status sigma_cgroup_enforce_io(sigma_u32 cgroup_id, sigma_u64 bytes, bool is_write) {
    return SigmaOS::Cgroup::sigma_cgroup_enforce_io(cgroup_id, bytes, is_write);
}

sigma_status sigma_cgroup_get_stats(sigma_u32 cgroup_id, sigma_cgroup_stats_t* out) {
    return SigmaOS::Cgroup::sigma_cgroup_get_stats(cgroup_id, out);
}

sigma_status sigma_cgroup_destroy(sigma_u32 cgroup_id) {
    return SigmaOS::Cgroup::sigma_cgroup_destroy(cgroup_id);
}

/**
 * cgroup_apply_pod_limits() — C wrapper called directly by sigma_orchestrator.cpp.
 * Creates a new cgroup and returns the opaque pointer (or nullptr on failure).
 */
struct sigma_cgroup* cgroup_apply_pod_limits(const char* pod_name,
                                              sigma_u32   cpu_millis,
                                              sigma_u32   mem_mb)
{
    sigma_u32 id = 0;
    sigma_status st = SigmaOS::Cgroup::sigma_cgroup_create(
        pod_name, cpu_millis, mem_mb, 100 /* default io_weight */, &id);

    if (st != K_OK) {
        sigma_log_err("[Cgroup] cgroup_apply_pod_limits: failed for pod '%s' (err=%d)",
                      pod_name, (int)st);
        return nullptr;
    }

    /* Return a stable pointer into the internal array (valid for lifetime of pod) */
    return reinterpret_cast<struct sigma_cgroup*>(&SigmaOS::Cgroup::s_groups[id]);
}

} // extern "C"
