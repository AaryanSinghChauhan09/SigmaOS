/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CGROUPS (v1.0 - Industrial Absorbtion: Linux)
 * =========================================================================
 * Mission: Resource Sharding. CPU/MEM Limiting for Industrial Processes.
 * Capability: Hierarchical Resource Sharding.
 * Principle: Zero-Overhead, Absolute Constraint. 
 * Standard: C11 (ISO/IEC 9899:2011) - Pure C.
 * =========================================================================
 */

#include "../libc/sigma_libc.h"

typedef struct sigma_cgroup {
    const char* name;
    sigma_u64 cpu_share; /* 0-1024 */
    sigma_u64 mem_limit;  /* In bytes */
    sigma_u64 mem_used;   /* In bytes */
    sigma_u64 proc_count;
    struct sigma_cgroup* parent;
} sigma_cgroup_t;

#define MAX_CGROUPS 256
static sigma_cgroup_t g_cgroups_table[MAX_CGROUPS];
static int g_cgroup_count = 0;

/* --- cgroup_mkdir (Linux parity) --- */
sigma_cgroup_t* sigma_cgroup_create(const char* name, sigma_cgroup_t* parent) {
    if (g_cgroup_count >= MAX_CGROUPS) return SIGMA_NULL;
    
    sigma_cgroup_t* cg = &g_cgroups_table[g_cgroup_count++];
    cg->name = name;
    cg->cpu_share = 1024;
    cg->mem_limit = 0; /* Unlimited by default */
    cg->mem_used = 0;
    cg->proc_count = 0;
    cg->parent = parent;
    
    sigma_printf("[KERNEL-CGROUP]: Matrix created: /sys/fs/cgroup/%s\n", name);
    return cg;
}

/* --- cgroup_write_mem (Linux parity: memory.limit_in_bytes) --- */
void sigma_cgroup_limit_mem(sigma_cgroup_t* cg, sigma_u64 limit) {
    cg->mem_limit = limit;
    sigma_printf("[KERNEL-CGROUP]: Set memory limit for [%s]: %llu bytes.\n", cg->name, limit);
}

/* --- cgroup_attach (Attach process to cgroup) --- */
void sigma_cgroup_attach(sigma_cgroup_t* cg, sigma_u64 pid) {
    cg->proc_count++;
    sigma_printf("[KERNEL-CGROUP]: Process %llu attached to matrix [%s].\n", pid, cg->name);
}

void sigma_cgroup_init(void) {
    g_cgroup_count = 0;
    sigma_cgroup_create("root", SIGMA_NULL);
    sigma_printf("[KERNEL-CGROUP]: Control Groups Sharding Active (Industrial Linux USP).\n");
}
