#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: ROLE-BASED ACCESS CONTROL (v1.0)
 * =============================================================================
 * Principles: Shard-Level Permissions & Zero-Trust Resource Access.
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

typedef enum Role {
    ROLE_KERNEL,
    ROLE_DRIVER,
    ROLE_SERVICE,
    ROLE_USER
} role_t;

typedef struct Permission {
    sigma_u64     shard_id;
    role_t  role;
    sigma_bool  can_io;
    sigma_bool  can_net;
    sigma_bool  can_fs;
} perm_t;

#define MAX_PERMS 256
static perm_t perm_lattice[MAX_PERMS];
static sigma_u32 perm_count = 0;

void rbac_init() {
    perm_count = 0;
}

/* Check if a shard has permission to perform a specific action */
sigma_bool rbac_check(sigma_u64 shard_id, sigma_bool net_req, sigma_bool fs_req) {
    for (sigma_u32 i = 0; i < perm_count; i++) {
        if (perm_lattice[i].shard_id == shard_id) {
            if (net_req && !perm_lattice[i].can_net) return SIGMA_FALSE;
            if (fs_req && !perm_lattice[i].can_fs) return SIGMA_FALSE;
            return SIGMA_TRUE;
        }
    }
    return SIGMA_FALSE; /* Default Deny */
}

void rbac_grant(sigma_u64 shard_id, role_t role, sigma_bool io, sigma_bool net, sigma_bool fs) {
    if (perm_count < MAX_PERMS) {
        perm_lattice[perm_count++] = (perm_t){shard_id, role, io, net, fs};
    }
}
