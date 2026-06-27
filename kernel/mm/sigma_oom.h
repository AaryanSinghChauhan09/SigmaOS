// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_oom.h — Out-of-Memory killer
 *
 * Inspired by Linux OOM killer, FreeBSD vmspace, and Android LMKD.
 *
 * Policy:
 *   1. Per-process OOM score (0–1000); 1000 = kill first
 *   2. Score = (memory_rss_pages / total_pages * 1000)
 *              - oom_score_adj (set by service manager for critical daemons)
 *   3. Kernel daemons (sigma-busd, sigma-healthd) get oom_score_adj = -500
 *      so they are never killed first.
 *   4. When free memory < LOW_WATERMARK_PAGES: warn, throttle allocations
 *   5. When free memory < CRIT_WATERMARK_PAGES: invoke sigma_oom_kill()
 *   6. After kill: compact memory, retry the failed allocation
 *
 * Integration with cgroup v2 (sigma_cgroup.cpp):
 *   Per-workload memory limits trigger OOM locally before system-wide OOM.
 */

#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Watermarks ──────────────────────────────────────────────────────────── */
#define SIGMA_OOM_LOW_WATERMARK_PCT   10   /* % of total RAM              */
#define SIGMA_OOM_CRIT_WATERMARK_PCT   5   /* % — trigger kill            */

/* ── OOM score adjustment (like Linux /proc/<pid>/oom_score_adj) ─────────── */
#define SIGMA_OOM_ADJ_NEVER_KILL  (-1000) /* immortal — kernel threads   */
#define SIGMA_OOM_ADJ_PROTECT     (-500)  /* critical daemons            */
#define SIGMA_OOM_ADJ_DEFAULT         0
#define SIGMA_OOM_ADJ_ALWAYS_KILL  1000   /* sacrificial workloads       */

/* ── Per-process OOM context ─────────────────────────────────────────────── */
typedef struct {
    sigma_u32  pid;
    char       name[64];
    sigma_u64  rss_pages;          /* resident set size in pages          */
    sigma_s32  oom_score_adj;      /* set by service manager              */
    sigma_u32  oom_score;          /* computed: 0–1000                    */
    bool       protected_process;  /* true if adj <= SIGMA_OOM_ADJ_PROTECT */
} sigma_oom_proc_t;

/* ── OOM event ───────────────────────────────────────────────────────────── */
typedef struct {
    sigma_u64  timestamp_ns;
    sigma_u32  killed_pid;
    char       killed_name[64];
    sigma_u64  freed_pages;
    sigma_u64  free_pages_before;
    sigma_u64  free_pages_after;
} sigma_oom_event_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Called by mm allocator when page allocation fails. */
int sigma_oom_kill(const char *reason);

/* Set OOM score adjustment for a process (service manager uses this). */
int sigma_oom_set_adj(sigma_u32 pid, sigma_s32 adj);

/* Get OOM score for a process. */
sigma_u32 sigma_oom_score(sigma_u32 pid);

/* Dump current OOM scores for all processes, sorted worst→best. */
int sigma_oom_dump(sigma_oom_proc_t *out, int max, int *count_out);

/* Register a callback invoked before each OOM kill decision. */
typedef void (*sigma_oom_notify_fn)(const sigma_oom_proc_t *victim, void *ctx);
void sigma_oom_register_notify(sigma_oom_notify_fn fn, void *ctx);

/* Query last N OOM events. */
int sigma_oom_history(sigma_oom_event_t *out, int max, int *count_out);

/* Memory pressure levels (for cgroup soft limits). */
typedef enum {
    SIGMA_MEM_PRESSURE_NONE     = 0,
    SIGMA_MEM_PRESSURE_LOW      = 1,  /* 10–15% free             */
    SIGMA_MEM_PRESSURE_MEDIUM   = 2,  /* 5–10% free              */
    SIGMA_MEM_PRESSURE_CRITICAL = 3,  /* <5% free — OOM imminent */
} sigma_mem_pressure_t;

sigma_mem_pressure_t sigma_oom_pressure(void);
