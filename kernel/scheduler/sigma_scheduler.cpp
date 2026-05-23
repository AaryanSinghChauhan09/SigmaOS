/*
 * Σ SigmaOS — sigma_scheduler: Sovereign Round-Robin + EDF Hybrid Scheduler
 * Zero-Dependency: No POSIX threads, no pthreads.
 * Absorbs: Linux CFS (Completely Fair Scheduler), L4Re RTOS EDF scheduling,
 *          Minix process table design, Plan 9 proc management.
 */

typedef unsigned int   u32;
typedef unsigned long long u64;
typedef unsigned char  u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" u64  sigma_get_uptime_ms();

#define MAX_PROCS    256
#define SCHED_NORMAL   0   /* Round-robin fair (like CFS) */
#define SCHED_RT       1   /* Real-time / EDF (for RTOS branch) */
#define PROC_READY     0
#define PROC_RUNNING   1
#define PROC_BLOCKED   2
#define PROC_ZOMBIE    3

/* Process Control Block */
struct SigmaPCB {
    u32  pid;
    u32  ppid;
    u32  sched_class;    /* SCHED_NORMAL or SCHED_RT */
    u32  priority;       /* 0=lowest, 99=highest */
    u64  deadline_ms;    /* For EDF: absolute deadline */
    u64  runtime_ms;     /* Total CPU time consumed */
    u64  vruntime;       /* Virtual runtime for CFS-style fairness */
    u32  state;
    char name[32];
    u64  stack_ptr;      /* Saved stack pointer for context switch */
    bool active;
};

static SigmaPCB proc_table[MAX_PROCS];
static u32 current_pid = 0;
static u32 proc_count = 0;

/* Create a new process entry */
extern "C" u32 sigma_sched_create_proc(const char* name, u32 sched_class, u32 priority) {
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (!proc_table[i].active) {
            proc_table[i].pid = i + 1;
            proc_table[i].ppid = current_pid;
            proc_table[i].sched_class = sched_class;
            proc_table[i].priority = priority;
            proc_table[i].deadline_ms = 0;
            proc_table[i].runtime_ms = 0;
            proc_table[i].vruntime = 0;
            proc_table[i].state = PROC_READY;
            proc_table[i].active = true;
            /* Copy name */
            u32 j = 0;
            while (name[j] && j < 31) { proc_table[i].name[j] = name[j]; j++; }
            proc_table[i].name[j] = '\0';
            proc_count++;
            return proc_table[i].pid;
        }
    }
    return 0;
}

/* Select next process to run: EDF for RT tasks, lowest vruntime for NORMAL */
extern "C" SigmaPCB* sigma_sched_next() {
    SigmaPCB* chosen = 0;
    u64 now = sigma_get_uptime_ms();

    /* First, check real-time EDF candidates */
    u64 earliest_deadline = 0xFFFFFFFFFFFFFFFFULL;
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (proc_table[i].active && proc_table[i].state == PROC_READY
            && proc_table[i].sched_class == SCHED_RT) {
            if (proc_table[i].deadline_ms < earliest_deadline) {
                earliest_deadline = proc_table[i].deadline_ms;
                chosen = &proc_table[i];
            }
        }
    }
    if (chosen) return chosen;

    /* CFS-style: lowest vruntime wins among NORMAL tasks */
    u64 min_vruntime = 0xFFFFFFFFFFFFFFFFULL;
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (proc_table[i].active && proc_table[i].state == PROC_READY
            && proc_table[i].sched_class == SCHED_NORMAL) {
            if (proc_table[i].vruntime < min_vruntime) {
                min_vruntime = proc_table[i].vruntime;
                chosen = &proc_table[i];
            }
        }
    }
    return chosen;
}

/* Mark process as done */
extern "C" void sigma_sched_exit(u32 pid) {
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (proc_table[i].active && proc_table[i].pid == pid) {
            proc_table[i].state = PROC_ZOMBIE;
            proc_table[i].active = false;
            proc_count--;
            return;
        }
    }
}
