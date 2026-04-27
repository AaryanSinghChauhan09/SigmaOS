/*
 * =============================================================================
 * Σ SIGMAOS SHELL: SIGMAPM PACKAGE MANAGER (v1.0)
 * =============================================================================
 * Principles: Shard-Native Packages & Dependency Integrity.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct Package {
    char    name[32];
    u32     version;
    u64     shard_id;
    bool_t  installed;
} pkg_t;

#define MAX_PACKAGES 128
static pkg_t package_db[MAX_PACKAGES];
static u32 pkg_count = 0;

void sigmapm_init() {
    pkg_count = 0;
    kprintf("Σ [SIGMAPM]: Package database initialized.\n");
}

/* Install a verified shard package into the system */
int sigmapm_install(const char* name, u64 shard_id) {
    if (pkg_count >= MAX_PACKAGES) return -1;

    pkg_t* p = &package_db[pkg_count++];
    sigma_memcpy(p->name, name, sigma_strlen(name));
    p->shard_id = shard_id;
    p->installed = TRUE;
    p->version = 100; /* v1.0.0 */

    kprintf("Σ [SIGMAPM]: Installed package '%s' (Shard 0x%x)\n", name, shard_id);
    return 0;
}

/* Check if a package is present in the lattice */
bool_t sigmapm_check(const char* name) {
    for (u32 i = 0; i < pkg_count; i++) {
        if (sigma_strcmp(package_db[i].name, name) == 0 && package_db[i].installed) {
            return TRUE;
        }
    }
    return FALSE;
}
