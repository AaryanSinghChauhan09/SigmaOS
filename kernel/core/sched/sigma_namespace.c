#include "sigma_kernel_types.h"
#include "sigma_slab.h"

// Namespace abstraction (PID isolation)

#define MAX_NAMESPACES 16

typedef struct sigma_pid_namespace {
    int id;
    struct sigma_pid_namespace* parent;
    int pid_counter; // Local PID counter
} sigma_pid_namespace_t;

static sigma_pid_namespace_t namespaces[MAX_NAMESPACES];
static int ns_count = 0;

void sigma_namespace_init(void) {
    // Root namespace
    namespaces[0].id = 0;
    namespaces[0].parent = NULL;
    namespaces[0].pid_counter = 1;
    ns_count = 1;
}

int sigma_namespace_create(int parent_id) {
    if (ns_count >= MAX_NAMESPACES || parent_id >= ns_count || parent_id < 0) return -1;
    
    int id = ns_count++;
    namespaces[id].id = id;
    namespaces[id].parent = &namespaces[parent_id];
    namespaces[id].pid_counter = 1;
    
    return id;
}

int sigma_namespace_alloc_pid(int ns_id) {
    if (ns_id < 0 || ns_id >= ns_count) return -1;
    return namespaces[ns_id].pid_counter++;
}
