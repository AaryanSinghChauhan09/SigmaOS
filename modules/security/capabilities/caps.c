#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Capability-Based Security Model Prototype
// ---------------------------------------------------------

// Capability Rights Bitmask
#define CAP_READ    0x01
#define CAP_WRITE   0x02
#define CAP_EXECUTE 0x04
#define CAP_GRANT   0x08 // Right to delegate capability

typedef enum {
    OBJ_MEMORY_PAGE,
    OBJ_IPC_PORT,
    OBJ_DEVICE_IO
} object_type_t;

// The Capability Token (Unforgeable Kernel Object)
typedef struct capability {
    uint32_t cap_id;         // Unique ID
    object_type_t obj_type;  // Type of object being accessed
    uint64_t object_ref;     // Internal pointer/ID to the actual resource (e.g., physical page)
    uint8_t rights;          // Bitmask of allowed actions
    uint32_t owner_pid;      // Process that currently holds this capability
    struct capability* next; // Linked list for a process's capability table
} capability_t;

#define MAX_CAPABILITIES 1024
static capability_t cap_pool[MAX_CAPABILITIES];
static uint32_t cap_count = 0;

// Initialize the Capability subsystem
void caps_init() {
    for (int i = 0; i < MAX_CAPABILITIES; i++) {
        cap_pool[i].cap_id = 0;
        cap_pool[i].owner_pid = 0;
    }
}

// Kernel function to mint a new capability (e.g., when a process allocates a page)
capability_t* cap_mint(uint32_t pid, object_type_t type, uint64_t ref, uint8_t rights) {
    if (cap_count >= MAX_CAPABILITIES) return SIGMA_NULL;
    
    capability_t* new_cap = &cap_pool[cap_count++];
    new_cap->cap_id = 0xCAFEBABE ^ cap_count; // Mock unforgeable ID generation
    new_cap->obj_type = type;
    new_cap->object_ref = ref;
    new_cap->rights = rights;
    new_cap->owner_pid = pid;
    
    return new_cap;
}

// Check if a process has the right to perform an action on an object
int cap_check(uint32_t pid, uint32_t cap_id, uint8_t required_rights) {
    for (int i = 0; i < cap_count; i++) {
        if (cap_pool[i].cap_id == cap_id && cap_pool[i].owner_pid == pid) {
            if ((cap_pool[i].rights & required_rights) == required_rights) {
                return 1; // Access granted
            }
        }
    }
    return 0; // Access denied (or capability doesn't exist)
}

// Revoke a capability (e.g., when memory is freed)
void cap_revoke(uint32_t cap_id) {
    for (int i = 0; i < cap_count; i++) {
        if (cap_pool[i].cap_id == cap_id) {
            cap_pool[i].owner_pid = 0; // Nullify owner
            cap_pool[i].rights = 0;
            break;
        }
    }
}
