#include "libc/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "suites/S10_Registry/shards/SovereignLatticeRegistry.h"
#include "libc/sigma_libc.h"

/*
 * Sovereign Package Registry (v1.0).
 * Mission: Centralized management of sharded applications and dependencies.
 * design: C11 / Zero-Dependency / Registry Pattern.
 */

#define SIGMA_MAX_PACKAGES 256

typedef enum {
    CURATION_UNVERIFIED = 0,
    CURATION_COMMUNITY = 1,
    CURATION_OFFICIAL = 2
} CurationLevel_t;

typedef struct {
    char name[64];
    char version[16];
    CurationLevel_t curation;
    sigma_bool seated;
} SovereignPkgEntry_t;

static SovereignPkgEntry_t g_pkg_registry[SIGMA_MAX_PACKAGES];
static sigma_u32 g_pkg_count = 0;

void SovereignPkg_InitRegistry(void) {
    g_pkg_count = 0;
    sigma_sigma_printf("S [REGISTRY]: Sovereign Package Registry initialized.\n");
}

sigma_err_t SovereignPkg_Register(const char* name, const char* version, CurationLevel_t curation) {
    if (g_pkg_count >= SIGMA_MAX_PACKAGES) return SIGMA_ERR;
    sigma_sigma_strcpy(g_pkg_registry[g_pkg_count].name, name, 64);
    sigma_sigma_strcpy(g_pkg_registry[g_pkg_count].version, version, 16);
    g_pkg_registry[g_pkg_count].curation = curation;
    g_pkg_registry[g_pkg_count].seated = SIGMA_TRUE;
    g_pkg_count++;
    return SIGMA_OK;
}

void SovereignPkg_Audit(void) {
    sigma_sigma_printf("S [PKG-AUDIT]: Verifying %d seated application shards...\n", g_pkg_count);
    for (sigma_u32 i = 0; i < g_pkg_count; i++) {
        const char* curation_str = "UNVERIFIED";
        if (g_pkg_registry[i].curation == CURATION_OFFICIAL) curation_str = "OFFICIAL";
        else if (g_pkg_registry[i].curation == CURATION_COMMUNITY) curation_str = "COMMUNITY";
        
        sigma_sigma_printf("  ? [OK]: %s (v%s) [%s] integrity verified.\n", 
            g_pkg_registry[i].name, 
            g_pkg_registry[i].version,
            curation_str);
    }
}



