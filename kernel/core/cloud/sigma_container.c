/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: CONTAINER NAMESPACES (ORCHESTRATION)
 * =============================================================================
 * Inspired by: Linux kernel kernel/nsproxy.c & net/core/net_namespace.c
 *              Docker / Kubernetes underpinnings
 * =============================================================================
 * Provides kernel-level isolation for networking, IPC, and mounts, forming
 * the foundation for Docker-style containers and cloud orchestration.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define CLONE_NEWNET   0x40000000
#define CLONE_NEWIPC   0x08000000
#define CLONE_NEWPID   0x20000000
#define CLONE_NEWNS    0x00020000 /* Mount namespace */

#define MAX_NAMESPACES 64

typedef struct {
    sigma_u32 ns_id;
    sigma_u32 type;
    sigma_u32 refcount;
    char      name[32];
    sigma_bool active;
    
    /* Subsystem specific pointers would go here (e.g. routing table, IPC shm list) */
} sigma_namespace_t;

static sigma_namespace_t namespace_pool[MAX_NAMESPACES];
static sigma_u32 next_ns_id = 1;

void namespace_init(void) {
    sigma_memset(namespace_pool, 0, sizeof(namespace_pool));
    
    /* Create default root namespaces */
    sigma_namespace_t* root_net = &namespace_pool[0];
    root_net->ns_id = next_ns_id++;
    root_net->type = CLONE_NEWNET;
    root_net->refcount = 1;
    root_net->active = SIGMA_TRUE;
    sigma_strcpy(root_net->name, "net_init", 32);
    
    sigma_namespace_t* root_pid = &namespace_pool[1];
    root_pid->ns_id = next_ns_id++;
    root_pid->type = CLONE_NEWPID;
    root_pid->refcount = 1;
    root_pid->active = SIGMA_TRUE;
    sigma_strcpy(root_pid->name, "pid_init", 32);
    
    sigma_printf("[cloud] Container Namespace subsystem initialized (Root NS created)\n");
}

int namespace_create(sigma_u32 type, const char* name) {
    for (sigma_u32 i = 0; i < MAX_NAMESPACES; i++) {
        if (!namespace_pool[i].active) {
            namespace_pool[i].ns_id = next_ns_id++;
            namespace_pool[i].type = type;
            namespace_pool[i].refcount = 1;
            namespace_pool[i].active = SIGMA_TRUE;
            
            sigma_u32 j = 0;
            while (j < 31 && name[j]) { namespace_pool[i].name[j] = name[j]; j++; }
            namespace_pool[i].name[j] = '\0';
            
            const char* type_str = "UNKNOWN";
            if (type == CLONE_NEWNET) type_str = "NET";
            if (type == CLONE_NEWIPC) type_str = "IPC";
            if (type == CLONE_NEWPID) type_str = "PID";
            if (type == CLONE_NEWNS)  type_str = "MNT";
            
            sigma_printf("[cloud] Created %s Namespace ID: %u ('%s')\n", 
                         type_str, namespace_pool[i].ns_id, namespace_pool[i].name);
            return (int)namespace_pool[i].ns_id;
        }
    }
    sigma_printf("[cloud] ERR: Max namespaces reached\n");
    return -1;
}

int namespace_attach_task(sigma_u32 pid, sigma_u32 ns_id) {
    /* Lookup NS */
    sigma_namespace_t* target_ns = SIGMA_NULL;
    for (sigma_u32 i = 0; i < MAX_NAMESPACES; i++) {
        if (namespace_pool[i].active && namespace_pool[i].ns_id == ns_id) {
            target_ns = &namespace_pool[i];
            break;
        }
    }
    
    if (!target_ns) {
        sigma_printf("[cloud] ERR: Namespace ID %u not found\n", ns_id);
        return -1;
    }
    
    /* In a real kernel, we update the task_struct's nsproxy pointer */
    target_ns->refcount++;
    
    const char* type_str = "UNKNOWN";
    if (target_ns->type == CLONE_NEWNET) type_str = "NET";
    if (target_ns->type == CLONE_NEWIPC) type_str = "IPC";
    if (target_ns->type == CLONE_NEWPID) type_str = "PID";
    if (target_ns->type == CLONE_NEWNS)  type_str = "MNT";
    
    sigma_printf("[cloud] Process %u entered %s Namespace '%s'\n", 
                 pid, type_str, target_ns->name);
                 
    return 0;
}
