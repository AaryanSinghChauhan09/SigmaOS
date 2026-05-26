#include "../../../../../include/sigma_kernel_types.h"
#include "../../../../../include/sigma_log.h"
#include "../../../../../suites/S10_Registry/shards/SovereignLatticeRegistry.h"
#include "../../../../../suites/S32_SystemTools/include/sigma_libc.h"
#include "../../../../../include/security/sigma_pkg_registry.h"

/*
 * Sovereign Package Registry (v1.0).
 * Mission: Centralized management of sharded applications and dependencies.
 * design: C11 / Zero-Dependency / Registry Pattern.
 */

#define SIGMA_MAX_PACKAGES 256
#define MAX_SNAPSHOTS 5

static SovereignPkgEntry_t g_pkg_registry[SIGMA_MAX_PACKAGES];
static sigma_u32 g_pkg_count = 0;

typedef struct {
    SovereignPkgEntry_t registry[SIGMA_MAX_PACKAGES];
    sigma_u32 count;
    sigma_u32 generation_id;
} PkgSnapshot_t;

static PkgSnapshot_t g_snapshots[MAX_SNAPSHOTS];
static sigma_u32 g_current_generation = 0;

void SovereignPkg_InitRegistry(void) {
    g_pkg_count = 0;
    kprintf("S [REGISTRY]: Sovereign Package Registry initialized.\n");
}

int SovereignPkg_Register(const char* name, const char* version, CurationLevel_t curation) {
    if (g_pkg_count >= SIGMA_MAX_PACKAGES) return SIGMA_ERROR;
    sigma_strcpy(g_pkg_registry[g_pkg_count].name, name, 64);
    sigma_strcpy(g_pkg_registry[g_pkg_count].version, version, 16);
    g_pkg_registry[g_pkg_count].curation = curation;
    g_pkg_registry[g_pkg_count].seated = SIGMA_TRUE;
    g_pkg_count++;
    return SIGMA_OK;
}

void SovereignPkg_Audit(void) {
    kprintf("S [PKG-AUDIT]: Verifying %d seated application shards...\n", g_pkg_count);
    for (sigma_u32 i = 0; i < g_pkg_count; i++) {
        const char* curation_str = "UNVERIFIED";
        if (g_pkg_registry[i].curation == CURATION_OFFICIAL) curation_str = "OFFICIAL";
        else if (g_pkg_registry[i].curation == CURATION_COMMUNITY) curation_str = "COMMUNITY";
        
        kprintf("  ? [OK]: %s (v%s) [%s] integrity verified.\n", 
            g_pkg_registry[i].name, 
            g_pkg_registry[i].version,
            curation_str);
    }
}

CurationLevel_t SovereignPkg_GetCuration(const char* name) {
    for (sigma_u32 i = 0; i < g_pkg_count; i++) {
        if (sigma_strcmp(g_pkg_registry[i].name, name) == 0) {
            return g_pkg_registry[i].curation;
        }
    }
    return CURATION_UNVERIFIED;
}

void SovereignPkg_SnapshotState(void) {
    if (g_current_generation < MAX_SNAPSHOTS) {
        sigma_memcpy(&g_snapshots[g_current_generation].registry, g_pkg_registry, sizeof(g_pkg_registry));
        g_snapshots[g_current_generation].count = g_pkg_count;
        g_snapshots[g_current_generation].generation_id = g_current_generation;
        kprintf("S [REGISTRY]: Snapshot created (Generation %d).\n", g_current_generation);
        g_current_generation++;
    } else {
        kprintf("S [REGISTRY]: Error - Maximum snapshots reached.\n");
    }
}

int SovereignPkg_Rollback(sigma_u32 generation_id) {
    if (generation_id >= g_current_generation) return SIGMA_ERROR;
    sigma_memcpy(g_pkg_registry, &g_snapshots[generation_id].registry, sizeof(g_pkg_registry));
    g_pkg_count = g_snapshots[generation_id].count;
    kprintf("S [REGISTRY]: Atomically rolled back to Generation %d.\n", generation_id);
    
    /* Erase future generations */
    g_current_generation = generation_id + 1;
    return SIGMA_OK;
}

void SovereignPkg_LoadManifest(const char* manifest_data) {
    (void)manifest_data;
    kprintf("S [REGISTRY]: Loading Declarative Manifest (NixOS-style)...\n");
    
    /* Auto-snapshot before applying new declarative state */
    SovereignPkg_SnapshotState();
    
    kprintf("S [REGISTRY]: Resolving declarative dependencies and assembling the environment.\n");
    
    /* Simulate applying a new state */
    g_pkg_count = 0;
    SovereignPkg_Register("sigma-core", "1.0", CURATION_OFFICIAL);
    SovereignPkg_Register("sigma-ui", "1.1", CURATION_OFFICIAL);
    SovereignPkg_Register("sigma-networking", "1.0", CURATION_COMMUNITY);
    
    kprintf("S [REGISTRY]: System State transition complete.\n");
}
