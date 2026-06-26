// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_sched_sovereign.cpp — SCHED_SOVEREIGN hard real-time scheduling class
 * (kernel-exp branch — release/rtos will consume this)
 *
 * Tasks with priority >= SIGMA_RT_THRESHOLD are promoted to SCHED_SOVEREIGN:
 *   - Strict deadline-based execution (EDF within the RT class)
 *   - Priority inheritance via SovereignMutex (no unbounded priority inversion)
 *   - Lock-free SPSC handoff on context switch
 *   - Preempts all SCHED_MLFQ tasks without yielding
 *
 * Runtime tunable:
 *   sigma-sysctl kernel.sched.rt_threshold       # priority cutoff (default 80)
 *   sigma-sysctl kernel.sched.rt_timeslice_us    # RT time slice in µs (default 500)
 */

#include "sigma_sched_sovereign.h"
#include "sigma_log.h"
#include "include/sigma_sysctl.h"

/* ── Tunables (exposed via sysctl) ─────────────────────────────────────────── */
static int g_rt_threshold    = 80;   /* priority >= this → SCHED_SOVEREIGN  */
static int g_rt_timeslice_us = 500;  /* hard real-time time slice in µs      */

SIGMA_SYSCTL(rt_threshold,    "kernel.sched.rt_threshold",    SYSCTL_TYPE_INT, &g_rt_threshold,    false)
SIGMA_SYSCTL(rt_timeslice_us, "kernel.sched.rt_timeslice_us", SYSCTL_TYPE_INT, &g_rt_timeslice_us, false)

/* ── RT run queue ───────────────────────────────────────────────────────────── */
#define SIGMA_RT_QUEUE_SIZE 64

typedef struct {
    sigma_u32  pid;
    int        priority;      /* higher = more urgent                        */
    sigma_u64  deadline_ns;   /* absolute deadline in monotonic nanoseconds  */
    sigma_u64  period_ns;     /* task period (0 = aperiodic)                 */
    bool       active;
} sigma_rt_task_t;

static sigma_rt_task_t g_rt_queue[SIGMA_RT_QUEUE_SIZE];
static int             g_rt_count = 0;

static sigma_sysctl_node_t g_sysctl_rt_count;

static void register_rt_count_sysctl(void) {
    sigma_sysctl_register(&g_sysctl_rt_count,
                          "kernel.sched.rt_active_tasks",
                          SYSCTL_TYPE_INT, &g_rt_count, true);
}

/* ── Admit a task to SCHED_SOVEREIGN ────────────────────────────────────────── */

int sigma_sched_rt_admit(sigma_u32 pid, int priority,
                          sigma_u64 period_ns, sigma_u64 deadline_ns) {
    if (priority < g_rt_threshold) {
        sigma_log_warn("[RT] pid=%u priority=%d < threshold=%d — stays in MLFQ\n",
                       pid, priority, g_rt_threshold);
        return -1;
    }
    if (g_rt_count >= SIGMA_RT_QUEUE_SIZE) {
        sigma_log_err("[RT] RT queue full (%d tasks) — rejecting pid=%u\n",
                      SIGMA_RT_QUEUE_SIZE, pid);
        return -1;
    }

    sigma_rt_task_t* t = &g_rt_queue[g_rt_count++];
    t->pid         = pid;
    t->priority    = priority;
    t->deadline_ns = deadline_ns;
    t->period_ns   = period_ns;
    t->active      = true;

    sigma_log_info("[RT] pid=%u admitted to SCHED_SOVEREIGN (prio=%d deadline=%llu ns)\n",
                   pid, priority, (unsigned long long)deadline_ns);
    return 0;
}

/* ── EDF: pick the task with the earliest deadline ──────────────────────────── */

sigma_u32 sigma_sched_rt_pick_next(sigma_u64 now_ns) {
    int     best_idx      = -1;
    sigma_u64 best_deadline = UINT64_MAX;

    for (int i = 0; i < g_rt_count; i++) {
        if (!g_rt_queue[i].active) continue;
        if (g_rt_queue[i].deadline_ns < best_deadline) {
            best_deadline = g_rt_queue[i].deadline_ns;
            best_idx      = i;
        }
    }

    if (best_idx < 0) return 0;  /* no RT tasks ready */

    sigma_rt_task_t* t = &g_rt_queue[best_idx];

    /* Deadline miss detection */
    if (now_ns > t->deadline_ns) {
        sigma_log_err("[RT] DEADLINE MISS: pid=%u deadline=%llu now=%llu (delta=%llu ns)\n",
                      t->pid,
                      (unsigned long long)t->deadline_ns,
                      (unsigned long long)now_ns,
                      (unsigned long long)(now_ns - t->deadline_ns));
    }

    /* Renew periodic deadline */
    if (t->period_ns > 0) {
        t->deadline_ns += t->period_ns;
    } else {
        t->active = false;  /* aperiodic — run once */
        g_rt_count--;
    }

    return t->pid;
}

/* ── Priority inheritance (SovereignMutex) ──────────────────────────────────── */

void sigma_sched_rt_boost_priority(sigma_u32 holder_pid, sigma_u32 waiter_pid) {
    int waiter_prio = -1;

    /* Find waiter's priority */
    for (int i = 0; i < g_rt_count; i++) {
        if (g_rt_queue[i].pid == waiter_pid) {
            waiter_prio = g_rt_queue[i].priority;
            break;
        }
    }
    if (waiter_prio < 0) return;

    /* Boost holder if waiter has higher priority */
    for (int i = 0; i < g_rt_count; i++) {
        if (g_rt_queue[i].pid == holder_pid) {
            if (waiter_prio > g_rt_queue[i].priority) {
                sigma_log_info("[RT] priority inheritance: pid=%u boosted %d→%d (waiter=%u)\n",
                               holder_pid, g_rt_queue[i].priority, waiter_prio, waiter_pid);
                g_rt_queue[i].priority = waiter_prio;
            }
            return;
        }
    }
}

/* ── Init ────────────────────────────────────────────────────────────────────── */

void sigma_sched_rt_init(void) {
    for (int i = 0; i < SIGMA_RT_QUEUE_SIZE; i++) {
        g_rt_queue[i].active = false;
    }
    register_rt_count_sysctl();
    sigma_log_info("[RT] SCHED_SOVEREIGN initialized: threshold=%d timeslice=%d µs\n",
                   g_rt_threshold, g_rt_timeslice_us);
}
