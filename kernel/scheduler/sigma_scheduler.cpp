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
#define MAX_CORES    32
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
    u32  cpu_affinity;   /* Bitmask of allowed CPUs, or specific CPU */
    u32  current_cpu;    /* CPU currently executing this proc */
};

/* Per-core runqueue */
struct RunQueue {
    u32 current_pid;
    u32 load_weight;     /* For load balancer */
    u64 last_idle_time;
};

static SigmaPCB proc_table[MAX_PROCS];
static RunQueue core_rq[MAX_CORES];
static u32 next_pid = 1;
static u32 proc_count = 0;

/* Create a new process entry */
extern "C" u32 sigma_sched_create_proc(const char* name, u32 sched_class, u32 priority, u32 initial_cpu) {
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (!proc_table[i].active) {
            proc_table[i].pid = next_pid++;
            proc_table[i].ppid = 0; // Root by default, can be set later
            proc_table[i].sched_class = sched_class;
            proc_table[i].priority = priority;
            proc_table[i].deadline_ms = 0;
            proc_table[i].runtime_ms = 0;
            proc_table[i].vruntime = 0;
            proc_table[i].state = PROC_READY;
            proc_table[i].active = true;
            proc_table[i].cpu_affinity = 0xFFFFFFFF; // All CPUs
            proc_table[i].current_cpu = initial_cpu;
            /* Copy name */
            u32 j = 0;
            while (name[j] && j < 31) { proc_table[i].name[j] = name[j]; j++; }
            proc_table[i].name[j] = '\0';
            proc_count++;
            
            sigma_vga_printf("[Sched] Created %s (PID: %d) on CPU %d\n", name, proc_table[i].pid, initial_cpu);
            return proc_table[i].pid;
        }
    }
    return 0;
}

/* Select next process to run on a specific core */
extern "C" SigmaPCB* sigma_sched_next(u32 cpu_id) {
    SigmaPCB* chosen = 0;
    u64 now = sigma_get_uptime_ms();

    /* First, check real-time EDF candidates bound to this CPU */
    u64 earliest_deadline = 0xFFFFFFFFFFFFFFFFULL;
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (proc_table[i].active && proc_table[i].state == PROC_READY
            && proc_table[i].sched_class == SCHED_RT
            && (proc_table[i].cpu_affinity & (1 << cpu_id))
            && proc_table[i].current_cpu == cpu_id) {
            
            if (proc_table[i].deadline_ms < earliest_deadline) {
                earliest_deadline = proc_table[i].deadline_ms;
                chosen = &proc_table[i];
            }
        }
    }
    if (chosen) goto found;

    /* CFS-style: lowest vruntime wins among NORMAL tasks bound to this CPU */
    u64 min_vruntime = 0xFFFFFFFFFFFFFFFFULL;
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (proc_table[i].active && proc_table[i].state == PROC_READY
            && proc_table[i].sched_class == SCHED_NORMAL
            && (proc_table[i].cpu_affinity & (1 << cpu_id))
            && proc_table[i].current_cpu == cpu_id) {
            
            if (proc_table[i].vruntime < min_vruntime) {
                min_vruntime = proc_table[i].vruntime;
                chosen = &proc_table[i];
            }
        }
    }

found:
    if (chosen) {
        chosen->state = PROC_RUNNING;
        core_rq[cpu_id].current_pid = chosen->pid;
    } else {
        core_rq[cpu_id].current_pid = 0; // Idle
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
            sigma_vga_printf("[Sched] Process %d exited.\n", pid);
            return;
        }
    }
}

/* Voluntary preemption */
extern "C" void sigma_sched_yield(u32 cpu_id) {
    u32 curr_pid = core_rq[cpu_id].current_pid;
    if (curr_pid != 0) {
        for (u32 i = 0; i < MAX_PROCS; i++) {
            if (proc_table[i].pid == curr_pid) {
                proc_table[i].state = PROC_READY;
                // Add a small penalty to vruntime to ensure others get a chance
                proc_table[i].vruntime += 10; 
                break;
            }
        }
    }
    // Context switch logic would follow here
}

/* CPU Pinning */
extern "C" int sigma_sched_set_affinity(u32 pid, u32 cpu_mask) {
    for (u32 i = 0; i < MAX_PROCS; i++) {
        if (proc_table[i].active && proc_table[i].pid == pid) {
            proc_table[i].cpu_affinity = cpu_mask;
            sigma_vga_printf("[Sched] Set affinity for PID %d to 0x%x\n", pid, cpu_mask);
            return 0;
        }
    }
    return -1;
}

/* Energy-Aware Scheduling: ACPI P-State scaling stub */
extern "C" void sigma_sched_scale_cpu_freq(u32 cpu_id, u32 utilization_pct) {
    // Stub: Scale CPU frequency based on load
    // E.g., if utilization > 80%, request P0 state (max freq)
    // If utilization < 20%, request Pn state (min freq)
    /* 
    if (utilization_pct > 80) {
        write_msr(IA32_PERF_CTL, P0_STATE);
    } else {
        write_msr(IA32_PERF_CTL, Pn_STATE);
    }
    */
}
