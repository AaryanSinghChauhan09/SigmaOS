// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_cgroup.h — cgroup v2 resource limits for all workloads (OCI runc fs2-inspired)
 *
 * Create a cgroup, set limits, enter PID, destroy on exit.
 * Prevents: fork bombs, memory exhaustion, CPU monopolization, I/O starvation.
 *
 * Usage in sigma_init / sigmad-process:
 *   sigma_cgroup_resources_t r = SIGMA_CGROUP_DEFAULTS;
 *   r.mem_limit_bytes = 512 * 1024 * 1024;
 *   r.pids_max = 128;
 *   sigma_cgroup_create("zenith-browser", &r);
 *   sigma_cgroup_enter("zenith-browser", zenith_pid);
 */
#include <sigma_kernel_types.h>
#include <sys/types.h>

typedef struct {
    sigma_i64  cpu_quota_us;      /* -1 = unlimited; cgroup: cpu.max quota   */
    sigma_i64  cpu_period_us;     /* default 100000 (100ms)                  */
    sigma_u64  cpu_shares;        /* relative weight; cgroup: cpu.weight     */
    sigma_i64  mem_limit_bytes;   /* -1 = unlimited; cgroup: memory.max      */
    sigma_i64  mem_swap_bytes;    /* -1 = unlimited; cgroup: memory.swap.max */
    sigma_i64  mem_low_bytes;     /* soft limit;     cgroup: memory.low      */
    sigma_i64  pids_max;          /* max PIDs; -1 = unlimited                */
    sigma_u16  io_weight;         /* 1–10000; cgroup: io.weight              */
} sigma_cgroup_resources_t;

typedef struct {
    sigma_u64 cpu_usage_ns;
    sigma_u64 mem_current_bytes;
    sigma_u64 mem_peak_bytes;
    sigma_u64 pids_current;
    sigma_u64 io_rbytes;
    sigma_u64 io_wbytes;
} sigma_cgroup_stat_t;

/* Sensible defaults — override individual fields as needed */
#define SIGMA_CGROUP_DEFAULTS { \
    .cpu_quota_us    = -1,      \
    .cpu_period_us   = 100000,  \
    .cpu_shares      = 1024,    \
    .mem_limit_bytes = -1,      \
    .mem_swap_bytes  = 0,       \
    .mem_low_bytes   = -1,      \
    .pids_max        = -1,      \
    .io_weight       = 100,     \
}

/* Tight defaults for untrusted workloads */
#define SIGMA_CGROUP_UNTRUSTED { \
    .cpu_quota_us    = 200000,              /* 2 CPUs */    \
    .cpu_period_us   = 100000,                              \
    .cpu_shares      = 512,                                 \
    .mem_limit_bytes = 512LL * 1024 * 1024, /* 512 MB */   \
    .mem_swap_bytes  = 0,                   /* no swap */   \
    .mem_low_bytes   = -1,                                  \
    .pids_max        = 128,                 /* fork bomb */ \
    .io_weight       = 50,                                  \
}

int sigma_cgroup_create(const char* name, const sigma_cgroup_resources_t* r);
int sigma_cgroup_enter(const char* name, pid_t pid);
int sigma_cgroup_destroy(const char* name);
int sigma_cgroup_stat(const char* name, sigma_cgroup_stat_t* out);
