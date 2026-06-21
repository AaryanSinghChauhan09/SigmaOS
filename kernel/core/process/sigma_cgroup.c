/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: CONTROL GROUPS (CGROUPS)
 * =============================================================================
 * Inspired by: Linux kernel kernel/cgroup/cgroup.c
 * =============================================================================
 * Hierarchical resource tracking and limiting for task isolation.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_CGROUPS 32
#define CGROUP_NAME_LEN 32

/* Resource Controllers */
#define CGROUP_SUBSYS_CPU    0x01
#define CGROUP_SUBSYS_MEMORY 0x02

typedef struct {
    sigma_u64 memory_limit_bytes;
    sigma_u64 memory_usage_bytes;
    sigma_u32 cpu_weight; /* For proportional CPU time */
} sigma_cgroup_res_t;

typedef struct sigma_cgroup {
    char name[CGROUP_NAME_LEN];
    struct sigma_cgroup* parent;
    
    sigma_u32 active_subsys; /* Bitmask of enabled controllers */
    sigma_cgroup_res_t resources;
    
    sigma_bool active;
} sigma_cgroup_t;

static sigma_cgroup_t cgroup_pool[MAX_CGROUPS];
static sigma_cgroup_t* root_cgroup = SIGMA_NULL;

void cgroup_init(void) {
    sigma_memset(cgroup_pool, 0, sizeof(cgroup_pool));
    
    root_cgroup = &cgroup_pool[0];
    sigma_strcpy(root_cgroup->name, "/", CGROUP_NAME_LEN);
    root_cgroup->parent = SIGMA_NULL;
    root_cgroup->active_subsys = CGROUP_SUBSYS_CPU | CGROUP_SUBSYS_MEMORY;
    root_cgroup->resources.memory_limit_bytes = 0xFFFFFFFFFFFFFFFF; /* No limit */
    root_cgroup->resources.cpu_weight = 100;
    root_cgroup->active = SIGMA_TRUE;
    
    sigma_printf("[cgroup] Control Groups subsystem initialized\n");
}

sigma_cgroup_t* cgroup_create(const char* name, sigma_cgroup_t* parent) {
    for (sigma_u32 i = 1; i < MAX_CGROUPS; i++) {
        if (!cgroup_pool[i].active) {
            sigma_strcpy(cgroup_pool[i].name, name, CGROUP_NAME_LEN);
            cgroup_pool[i].parent = parent ? parent : root_cgroup;
            
            /* Inherit subsystems from parent */
            cgroup_pool[i].active_subsys = cgroup_pool[i].parent->active_subsys;
            
            /* Default resource limits */
            cgroup_pool[i].resources.memory_limit_bytes = cgroup_pool[i].parent->resources.memory_limit_bytes;
            cgroup_pool[i].resources.cpu_weight = 100;
            cgroup_pool[i].resources.memory_usage_bytes = 0;
            
            cgroup_pool[i].active = SIGMA_TRUE;
            
            sigma_printf("[cgroup] Created cgroup '%s' under '%s'\n", 
                         name, cgroup_pool[i].parent->name);
            return &cgroup_pool[i];
        }
    }
    sigma_printf("[cgroup] ERR: Max cgroups reached\n");
    return SIGMA_NULL;
}

int cgroup_charge_memory(sigma_cgroup_t* cg, sigma_u64 bytes) {
    if (!cg || !cg->active) return 0;
    
    sigma_cgroup_t* current = cg;
    
    /* Traverse up the hierarchy to check limits */
    while (current) {
        if (current->active_subsys & CGROUP_SUBSYS_MEMORY) {
            if (current->resources.memory_usage_bytes + bytes > current->resources.memory_limit_bytes) {
                sigma_printf("[cgroup] ERR: Memory limit exceeded in cgroup '%s'\n", current->name);
                /* Trigger OOM or return error */
                return -1; 
            }
        }
        current = current->parent;
    }
    
    /* If we passed limits, apply the charge hierarchically */
    current = cg;
    while (current) {
        if (current->active_subsys & CGROUP_SUBSYS_MEMORY) {
            current->resources.memory_usage_bytes += bytes;
        }
        current = current->parent;
    }
    return 0;
}

void cgroup_set_memory_limit(sigma_cgroup_t* cg, sigma_u64 limit) {
    if (cg && cg->active) {
        cg->resources.memory_limit_bytes = limit;
        sigma_printf("[cgroup] Set memory limit for '%s' to %llu bytes\n", cg->name, limit);
    }
}

void cgroup_set_cpu_weight(sigma_cgroup_t* cg, sigma_u32 weight) {
    if (cg && cg->active) {
        if (weight < 1) weight = 1;
        if (weight > 10000) weight = 10000;
        cg->resources.cpu_weight = weight;
        sigma_printf("[cgroup] Set CPU weight for '%s' to %u\n", cg->name, weight);
    }
}

sigma_cgroup_t* cgroup_apply_pod_limits(const char* pod_name,
                                      sigma_u32 cpu_millis,
                                      sigma_u32 mem_mb) {
    sigma_cgroup_t* cg = cgroup_create(pod_name, root_cgroup);
    if (!cg) return SIGMA_NULL;

    if (mem_mb > 0) {
        cgroup_set_memory_limit(cg, (sigma_u64)mem_mb * 1024ULL * 1024ULL);
    }
    if (cpu_millis > 0 && cpu_millis <= 1000) {
        cgroup_set_cpu_weight(cg, cpu_millis);
    } else if (cpu_millis > 1000) {
        cgroup_set_cpu_weight(cg, 1000);
    }
    return cg;
}
