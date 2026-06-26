/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN TASK SCHEDULER (v1.0 — MLFQ + EDF Hybrid)
 * =============================================================================
 * Mission: Production-grade scheduler combining Multi-Level Feedback Queue
 *          for general tasks with Earliest Deadline First for real-time.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_SCHEDULER_H
#define SIGMA_SCHEDULER_H

#include "../sigma_kernel_types.h"

#define SCHED_MAX_TASKS      1024
#define SCHED_MLFQ_LEVELS       8
#define SCHED_MAX_CPUS         16
#define SCHED_BASE_QUANTUM_US 4000   /* 4ms base time quantum */

typedef enum {
    TASK_STATE_READY      = 0,
    TASK_STATE_RUNNING    = 1,
    TASK_STATE_BLOCKED    = 2,
    TASK_STATE_SLEEPING   = 3,
    TASK_STATE_TERMINATED = 4
} sigma_task_state_t;

typedef enum {
    SCHED_POLICY_MLFQ        = 0,   /* Multi-Level Feedback Queue */
    SCHED_POLICY_EDF         = 1,   /* Earliest Deadline First (real-time) */
    SCHED_POLICY_ROUND_ROBIN = 2,
    SCHED_POLICY_FIFO        = 3
} sigma_sched_policy_t;

typedef struct {
    sigma_u32            tid;           /* thread ID */
    sigma_u32            pid;           /* owning process ID */
    sigma_task_state_t   state;
    sigma_sched_policy_t policy;
    sigma_u8             mlfq_level;    /* current MLFQ queue level (0 = highest) */
    sigma_u8             base_priority; /* 0 = highest, 255 = lowest */
    sigma_u64            deadline_us;   /* for EDF: absolute deadline in μs */
    sigma_u64            period_us;     /* for periodic real-time tasks */
    sigma_u64            time_slice_us; /* remaining time slice */
    sigma_u64            total_cpu_us;  /* total CPU time consumed */
    sigma_u32            cpu_affinity;  /* bitmask of allowed CPUs */
    sigma_u64            last_run_tsc;  /* TSC of last dispatch */
    sigma_u64            wake_time_us;  /* for sleeping tasks */
    sigma_vaddr_t        stack_ptr;     /* saved stack pointer */
} sigma_task_t;

typedef struct {
    sigma_u32  cpu_id;
    sigma_u32  current_tid;          /* TID of currently running task */
    sigma_u64  idle_time_us;
    sigma_u64  total_context_switches;
    sigma_u64  last_tick_tsc;
} sigma_cpu_state_t;

#ifdef __cplusplus
extern "C" {
#endif

void            sched_init(sigma_u32 num_cpus);
sigma_u32       sched_add_task(sigma_u32 pid, sigma_sched_policy_t policy,
                               sigma_u8 priority, sigma_u64 deadline_us);
int             sched_remove_task(sigma_u32 tid);
void            sched_tick(sigma_u32 cpu_id);
void            sched_yield(void);
sigma_u32       sched_get_current(sigma_u32 cpu_id);
int             sched_set_policy(sigma_u32 tid, sigma_sched_policy_t policy);
int             sched_set_affinity(sigma_u32 tid, sigma_u32 cpu_mask);
int             sched_sleep(sigma_u32 tid, sigma_u64 duration_us);
void            sched_priority_boost(void);
void            sched_print_queues(void);
void            sched_print_cpu_stats(void);
sigma_u32       sched_get_task_count(void);
sigma_u64       sched_get_total_switches(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SCHEDULER_H */
