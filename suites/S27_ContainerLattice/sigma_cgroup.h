// SigmaOS — Sigma-CGroup: Resource Control Groups
// Inspired by: Linux cgroups v2 — but atomic, no filesystem mount required
// Module: sigma-sys-cgroups
// USP over Linux cgroups: No sysfs overhead, direct register-based enforcement
// Each cgroup is a lightweight struct — O(1) admission control

#ifndef SIGMA_CGROUP_H
#define SIGMA_CGROUP_H

#define SIGMA_CGROUP_MAX       32
#define SIGMA_CGROUP_MAX_PROCS 64
#define SIGMA_CGROUP_UNLIMITED 0xFFFFFFFFU

typedef struct SigmaCGroup {
    const char*   name;
    unsigned int  cpu_shares;       // relative CPU weight (1-1024)
    unsigned int  mem_limit_kb;     // max memory in KB
    unsigned int  io_weight;        // I/O bandwidth weight
    unsigned int  member_pids[SIGMA_CGROUP_MAX_PROCS];
    unsigned int  member_count;
    unsigned long cpu_usage_cycles; // tracked via RDTSC
} SigmaCGroup;

typedef struct SigmaCGroupRegistry {
    SigmaCGroup groups[SIGMA_CGROUP_MAX];
    unsigned int count;
} SigmaCGroupRegistry;

static inline void cgreg_init(SigmaCGroupRegistry* r) { r->count = 0; }

// Create a new cgroup with resource limits
static inline int cgroup_create(SigmaCGroupRegistry* r, const char* name,
                                  unsigned int cpu_shares,
                                  unsigned int mem_limit_kb,
                                  unsigned int io_weight) {
    if (r->count >= SIGMA_CGROUP_MAX) return -1;
    SigmaCGroup* g    = &r->groups[r->count++];
    g->name           = name;
    g->cpu_shares     = cpu_shares;
    g->mem_limit_kb   = mem_limit_kb;
    g->io_weight      = io_weight;
    g->member_count   = 0;
    g->cpu_usage_cycles = 0;
    return (int)(r->count - 1);
}

// Assign a process to a cgroup
static inline int cgroup_add_pid(SigmaCGroupRegistry* r, unsigned int gid,
                                   unsigned int pid) {
    if (gid >= r->count) return -1;
    SigmaCGroup* g = &r->groups[gid];
    if (g->member_count >= SIGMA_CGROUP_MAX_PROCS) return -1;
    g->member_pids[g->member_count++] = pid;
    return 0;
}

// Admission: check if process can allocate `kb` more memory
static inline int cgroup_admit_mem(SigmaCGroupRegistry* r, unsigned int gid,
                                    unsigned int current_kb, unsigned int req_kb) {
    if (gid >= r->count) return 1; // no cgroup = unlimited
    SigmaCGroup* g = &r->groups[gid];
    if (g->mem_limit_kb == SIGMA_CGROUP_UNLIMITED) return 1;
    return (current_kb + req_kb <= g->mem_limit_kb) ? 1 : 0;
}

// Track CPU usage for a cgroup (call with RDTSC delta)
static inline void cgroup_charge_cpu(SigmaCGroupRegistry* r, unsigned int gid,
                                      unsigned long cycles) {
    if (gid < r->count)
        r->groups[gid].cpu_usage_cycles += cycles;
}

#endif /* SIGMA_CGROUP_H */
