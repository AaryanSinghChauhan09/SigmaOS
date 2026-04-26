#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Shard Isolation & Sandboxing Module (Phase 2)
// ---------------------------------------------------------

#define MAX_SHARDS 1024
#define MAX_CAPABILITIES 64

typedef enum {
    CAP_NETWORK_ACCESS = 1,
    CAP_FILE_READ      = 2,
    CAP_FILE_WRITE     = 4,
    CAP_IPC_SEND       = 8,
    CAP_IPC_RECV       = 16,
    CAP_HW_IO          = 32
} capability_type_t;

typedef struct {
    int shard_id;
    int is_active;
    
    // 1. Memory Boundaries
    uint64_t vmm_page_directory; 
    uint64_t memory_limit_bytes;
    uint64_t memory_used_bytes;
    
    // 2. Namespace Separation
    int namespace_id;
    
    // 3. Permission Models (Capability Tokens)
    uint32_t capabilities_mask;
} isolated_shard_t;

static isolated_shard_t shard_table[MAX_SHARDS];

void isolation_init() {
    for (int i = 0; i < MAX_SHARDS; i++) {
        shard_table[i].is_active = 0;
    }
}

// Create a new sandboxed shard
int isolation_create_shard(uint64_t memory_limit, int namespace_id) {
    for (int i = 0; i < MAX_SHARDS; i++) {
        if (!shard_table[i].is_active) {
            shard_table[i].shard_id = i;
            shard_table[i].is_active = 1;
            shard_table[i].vmm_page_directory = 0; // Would call vmm_create_directory()
            shard_table[i].memory_limit_bytes = memory_limit;
            shard_table[i].memory_used_bytes = 0;
            shard_table[i].namespace_id = namespace_id;
            shard_table[i].capabilities_mask = 0; // Default: Zero trust
            return i;
        }
    }
    return -1; // Out of shards
}

// 3. Permission Models: Grant a capability token
int isolation_grant_capability(int shard_id, uint32_t capability) {
    if (shard_id < 0 || shard_id >= MAX_SHARDS || !shard_table[shard_id].is_active) return -1;
    shard_table[shard_id].capabilities_mask |= capability;
    // Log audit trail
    // sigma_log_audit("Granted capability to shard");
    return 0;
}

// 4. Isolation Enforcement: Check capability before action
int isolation_check_capability(int shard_id, uint32_t capability) {
    if (shard_id < 0 || shard_id >= MAX_SHARDS || !shard_table[shard_id].is_active) return 0; // Deny
    
    if ((shard_table[shard_id].capabilities_mask & capability) == capability) {
        return 1; // Allow
    }
    
    // Log isolation breach attempt
    // sigma_log_violation("Shard attempted unauthorized action");
    return 0; // Deny
}

// Hardware Memory Fault Containment
void isolation_handle_fault(int shard_id, uint64_t fault_addr) {
    // If a shard causes a memory fault outside its permitted VMM region:
    if (shard_id >= 0 && shard_id < MAX_SHARDS) {
        shard_table[shard_id].is_active = 0; // Terminate shard immediately (Crash containment)
        // Log "Shard sandboxed and terminated due to illegal memory access"
    }
}
