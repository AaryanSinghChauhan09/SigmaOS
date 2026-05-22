#include "sigma_kernel_types.h"
#include "sigma_slab.h"
#include "sigma_mlfq.h"

// CoreOS/Docker inspired cgroup implementation

#define MAX_CGROUPS 32

typedef struct sigma_cgroup {
    int id;
    char name[32];
    uint32_t memory_limit_kb;
    uint32_t memory_usage_kb;
    uint32_t cpu_shares;      // Weight for scheduler
    struct sigma_cgroup* parent;
    struct sigma_cgroup* children;
    struct sigma_cgroup* sibling;
} sigma_cgroup_t;

static sigma_cgroup_t cgroups[MAX_CGROUPS];
static int cgroup_count = 0;

void sigma_cgroup_init(void) {
    // Init root cgroup
    cgroups[0].id = 0;
    cgroups[0].name[0] = '/';
    cgroups[0].name[1] = '\0';
    cgroups[0].memory_limit_kb = 0xFFFFFFFF; // Unlimited
    cgroups[0].memory_usage_kb = 0;
    cgroups[0].cpu_shares = 1024;
    cgroups[0].parent = NULL;
    cgroups[0].children = NULL;
    cgroups[0].sibling = NULL;
    cgroup_count = 1;
}

int sigma_cgroup_create(const char* name, int parent_id) {
    if (cgroup_count >= MAX_CGROUPS || parent_id >= cgroup_count || parent_id < 0) return -1;
    
    int id = cgroup_count++;
    sigma_cgroup_t* cg = &cgroups[id];
    
    cg->id = id;
    int i = 0;
    while(name[i] && i < 31) {
        cg->name[i] = name[i];
        i++;
    }
    cg->name[i] = '\0';
    
    cg->memory_limit_kb = cgroups[parent_id].memory_limit_kb; // Inherit
    cg->memory_usage_kb = 0;
    cg->cpu_shares = 1024;
    
    cg->parent = &cgroups[parent_id];
    cg->children = NULL;
    
    // Insert into sibling list
    cg->sibling = cgroups[parent_id].children;
    cgroups[parent_id].children = cg;
    
    return id;
}

int sigma_cgroup_charge_mem(int cgroup_id, uint32_t kb) {
    if (cgroup_id < 0 || cgroup_id >= cgroup_count) return -1;
    
    sigma_cgroup_t* cg = &cgroups[cgroup_id];
    
    // Bubble up hierarchy to check limits
    sigma_cgroup_t* curr = cg;
    while (curr) {
        if (curr->memory_usage_kb + kb > curr->memory_limit_kb) {
            return -1; // OOM for this cgroup
        }
        curr = curr->parent;
    }
    
    // Apply charge
    curr = cg;
    while (curr) {
        curr->memory_usage_kb += kb;
        curr = curr->parent;
    }
    
    return 0;
}
