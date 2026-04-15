/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S03_Orchestrator/shards/sigma_sched.h
 * =========================================================================
 * Sovereign CFS Scheduler — gap-closes:
 *   Linux  : Completely Fair Scheduler (CFS), cgroups v2 bandwidth
 *   Windows: NT Dispatcher, Thread Quantum, Priority Boosting
 *   macOS  : GCD (Grand Central Dispatch), QoS classes
 *   FreeBSD: ULE scheduler, idle/interrupt priorities
 *   RTOS   : Rate-Monotonic (RM), Earliest Deadline First (EDF)
 * =========================================================================
 */

#ifndef SIGMA_SCHED_H
#define SIGMA_SCHED_H

typedef unsigned long long sc_u64;
typedef unsigned int       sc_u32;
typedef signed   int       sc_i32;
typedef unsigned char      sc_bool;
#define SCHED_TRUE  ((sc_bool)1)
#define SCHED_FALSE ((sc_bool)0)

/* ── QoS classes (macOS GCD parity) ────────────────────────────────────── */
typedef enum {
    QOS_USER_INTERACTIVE = 0,  /* UI thread — max performance           */
    QOS_USER_INITIATED   = 1,  /* user-triggered work — high priority   */
    QOS_UTILITY          = 2,  /* background downloads — medium         */
    QOS_BACKGROUND       = 3,  /* sync, indexing — lowest               */
    QOS_REALTIME         = 4   /* audio/video — hard deadline           */
} sigma_qos_t;

/* ── Scheduler policy ────────────────────────────────────────────────────── */
typedef enum {
    POLICY_CFS   = 0,  /* Linux CFS: vruntime-based fairness            */
    POLICY_FIFO  = 1,  /* POSIX SCHED_FIFO: run until preempted/blocked */
    POLICY_RR    = 2,  /* POSIX SCHED_RR: time-sliced realtime          */
    POLICY_EDF   = 3,  /* Earliest Deadline First (RTOS)                */
    POLICY_IDLE  = 4   /* run only when nothing else wants CPU          */
} sigma_sched_policy_t;

/* ── Run-queue entry ─────────────────────────────────────────────────────── */
#define SIGMA_SCHED_MAX_TASKS 1024
#define SIGMA_SCHED_TIMESLICE_NS 4000000ULL  /* 4ms default quantum       */

typedef struct {
    sc_u32              pid;
    sigma_sched_policy_t policy;
    sigma_qos_t         qos;
    sc_i32              nice;           /* -20 to +19 (lower = higher)   */
    sc_u64              vruntime;       /* CFS virtual runtime (ns)      */
    sc_u64              deadline_ns;    /* EDF absolute deadline         */
    sc_u64              timeslice_ns;   /* remaining quantum             */
    sc_u64              total_cpu_ns;   /* cumulative CPU time           */
    sc_bool             on_cpu;         /* currently executing           */
    sc_bool             preemptible;    /* can be preempted mid-quantum  */
} sigma_task_t;

/* ── CPU run-queue ───────────────────────────────────────────────────────── */
typedef struct {
    sc_u32        cpu_id;
    sigma_task_t  tasks[SIGMA_SCHED_MAX_TASKS];
    sc_u32        task_count;
    sc_u64        min_vruntime;          /* CFS: leftmost leaf value     */
    sc_u64        clock_ns;              /* monotonic clock for this CPU */
    sc_u32        current_pid;           /* currently running PID        */
    sc_u64        context_switches;      /* telemetry                    */
    sc_u64        preemptions;
} sigma_runqueue_t;

#define SIGMA_MAX_CPUS 256

/* ── Public API ─────────────────────────────────────────────────────────── */
void sigma_sched_init(sc_u32 num_cpus);
void sigma_sched_enqueue(sc_u32 cpu_id, sc_u32 pid,
                          sigma_sched_policy_t policy,
                          sigma_qos_t qos, sc_i32 nice);
void sigma_sched_dequeue(sc_u32 cpu_id, sc_u32 pid);

/* Pick next task to run */
sc_u32 sigma_sched_pick_next(sc_u32 cpu_id);
void   sigma_sched_tick(sc_u32 cpu_id, sc_u64 elapsed_ns);
void   sigma_sched_yield(sc_u32 cpu_id, sc_u32 pid);
void   sigma_sched_set_deadline(sc_u32 pid, sc_u64 deadline_ns);

/* Load balancing (work-stealing — Linux SMP parity) */
void   sigma_sched_balance(void);

/* Reporting */
void   sigma_sched_stats(sc_u32 cpu_id);
void   sigma_sched_global_stats(void);

#endif /* SIGMA_SCHED_H */
