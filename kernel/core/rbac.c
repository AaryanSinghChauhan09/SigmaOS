/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: ROLE-BASED ACCESS CONTROL (v1.0)
 * =============================================================================
 * Principles: Shard-Level Permissions & Zero-Trust Resource Access.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef enum Role {
    ROLE_KERNEL,
    ROLE_DRIVER,
    ROLE_SERVICE,
    ROLE_USER
} role_t;

typedef struct Permission {
    u64     shard_id;
    role_t  role;
    bool_t  can_io;
    bool_t  can_net;
    bool_t  can_fs;
} perm_t;

#define MAX_PERMS 256
static perm_t perm_lattice[MAX_PERMS];
static u32 perm_count = 0;

void rbac_init() {
    perm_count = 0;
}

/* Check if a shard has permission to perform a specific action */
bool_t rbac_check(u64 shard_id, bool_t net_req, bool_t fs_req) {
    for (u32 i = 0; i < perm_count; i++) {
        if (perm_lattice[i].shard_id == shard_id) {
            if (net_req && !perm_lattice[i].can_net) return FALSE;
            if (fs_req && !perm_lattice[i].can_fs) return FALSE;
            return TRUE;
        }
    }
    return FALSE; /* Default Deny */
}

void rbac_grant(u64 shard_id, role_t role, bool_t io, bool_t net, bool_t fs) {
    if (perm_count < MAX_PERMS) {
        perm_lattice[perm_count++] = (perm_t){shard_id, role, io, net, fs};
    }
}
