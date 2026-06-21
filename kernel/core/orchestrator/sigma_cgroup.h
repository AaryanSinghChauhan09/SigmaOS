/**
 * =========================================================================
 * Σ SIGMAOS: CGROUP PUBLIC HEADER
 * =========================================================================
 */
#pragma once

#include "../../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/** Opaque cgroup type (callers hold pointers; internals are private) */
struct sigma_cgroup;

typedef struct {
    sigma_u64 cpu_throttle_count;
    sigma_u64 mem_current_bytes;
    sigma_u64 mem_peak_bytes;
    sigma_u64 mem_limit_bytes;
    sigma_u64 oom_kill_count;
    sigma_u64 io_bytes_read;
    sigma_u64 io_bytes_written;
    sigma_u64 io_throttle_count;
    sigma_u32 pid_count;
} sigma_cgroup_stats_t;

void         sigma_cgroup_init(void);
sigma_status sigma_cgroup_create(const char* name, sigma_u32 cpu_millis,
                                  sigma_u32 mem_mb, sigma_u32 io_weight,
                                  sigma_u32* id_out);
sigma_status sigma_cgroup_attach_pid(sigma_u32 cgroup_id, sigma_u32 pid);
sigma_status sigma_cgroup_enforce_cpu(sigma_u32 cgroup_id, sigma_u32 elapsed_us);
sigma_status sigma_cgroup_enforce_memory(sigma_u32 cgroup_id, sigma_u64 alloc_bytes);
void         sigma_cgroup_release_memory(sigma_u32 cgroup_id, sigma_u64 freed_bytes);
sigma_status sigma_cgroup_enforce_io(sigma_u32 cgroup_id, sigma_u64 bytes, bool is_write);
sigma_status sigma_cgroup_get_stats(sigma_u32 cgroup_id, sigma_cgroup_stats_t* out);
sigma_status sigma_cgroup_destroy(sigma_u32 cgroup_id);

/** Called by the orchestrator during pod creation. */
struct sigma_cgroup* cgroup_apply_pod_limits(const char* pod_name,
                                              sigma_u32   cpu_millis,
                                              sigma_u32   mem_mb);

#ifdef __cplusplus
}
#endif
