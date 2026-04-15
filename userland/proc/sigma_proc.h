/*
 * =========================================================================
 * Σ SIGMAOS userland/proc/sigma_proc.h
 * =========================================================================
 * Process & Thread Manager — gap-closes:
 *   Linux  : task_struct, clone(), cgroups, namespaces, seccomp
 *   macOS  : NSTask, GCD queues
 *   Windows: CreateProcess, Job Objects, Thread Pools
 *   Android: Zygote forking model
 * =========================================================================
 */

#ifndef SIGMA_PROC_H
#define SIGMA_PROC_H

typedef unsigned int       proc_u32;
typedef signed   int       proc_i32;
typedef unsigned long long proc_u64;
typedef unsigned char      proc_bool;
#define PROC_TRUE  ((proc_bool)1)
#define PROC_FALSE ((proc_bool)0)
#define PROC_NULL  ((void*)0)
#define PROC_OK    ((proc_i32) 0)
#define PROC_ERR   ((proc_i32)-1)

/* ── Process states ─────────────────────────────────────────────────────── */
typedef enum {
    PROC_RUNNING   = 0,
    PROC_SLEEPING,      /* waiting for I/O                     */
    PROC_STOPPED,       /* SIGSTOP / ptrace                    */
    PROC_ZOMBIE,        /* exited, not reaped                  */
    PROC_DEAD
} sigma_proc_state_t;

/* ── Scheduling class (Linux CFS / Windows priority) ────────────────────── */
typedef enum {
    SCHED_NORMAL   = 0,  /* CFS SCHED_OTHER                   */
    SCHED_REALTIME = 1,  /* CFS SCHED_FIFO/RR                 */
    SCHED_IDLE     = 2,  /* CFS SCHED_IDLE                    */
    SCHED_BATCH    = 3   /* CFS SCHED_BATCH                   */
} sigma_sched_class_t;

/* ── Namespace isolation flags (Linux unshare gaps) ─────────────────────── */
#define NS_PID   (1 << 0)
#define NS_NET   (1 << 1)
#define NS_MNT   (1 << 2)
#define NS_UTS   (1 << 3)
#define NS_IPC   (1 << 4)
#define NS_USER  (1 << 5)

#define SIGMA_PROC_MAX  512
#define SIGMA_CMD_LEN   256

/* ── Process Control Block (PCB) ────────────────────────────────────────── */
typedef struct {
    proc_u32            pid;
    proc_u32            ppid;         /* parent PID                    */
    proc_u32            uid;          /* owner UID                     */
    char                cmd[SIGMA_CMD_LEN];
    sigma_proc_state_t  state;
    sigma_sched_class_t sched;
    proc_i32            priority;     /* -20 (high) to +19 (low)       */
    proc_u32            ns_flags;     /* namespace isolation bitmask   */
    proc_u64            vm_rss_kb;    /* resident set size (KB)        */
    proc_u64            cpu_ticks;    /* CPU time consumed             */
    proc_u64            start_tsc;    /* TSC at process creation       */
    proc_bool           seccomp_on;   /* seccomp filter active         */
} sigma_pcb_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
proc_i32  sigma_proc_spawn(const char *cmd, proc_u32 ppid,
                            sigma_sched_class_t sched, proc_i32 prio,
                            proc_u32 ns_flags);
void      sigma_proc_kill(proc_u32 pid, proc_i32 signal);
void      sigma_proc_reap(proc_u32 pid);
sigma_pcb_t *sigma_proc_get(proc_u32 pid);
void      sigma_proc_top(void);       /* PS / htop style report        */
void      sigma_proc_set_sched(proc_u32 pid, sigma_sched_class_t cls,
                                proc_i32 prio);
void      sigma_proc_enable_seccomp(proc_u32 pid);

#endif /* SIGMA_PROC_H */
