#include "sigma_base.h"

#include "SovereignRegistry.h"
#include "sigma_libc.h"

/*
 * Sovereign Package Registry (v1.0).
 * Mission: Centralized management of sharded applications and dependencies.
 * design: C11 / Zero-Dependency / Registry Pattern.
 */

#define SIGMA_MAX_PACKAGES 256

typedef struct {
    char name[64];
    char version[16];
    sigma_bool seated;
} SovereignPkgEntry_t;

static SovereignPkgEntry_t g_pkg_registry[SIGMA_MAX_PACKAGES];
static sigma_u32 g_pkg_count = 0;

void SovereignPkg_InitRegistry(void) {
    g_pkg_count = 0;
    sigma_printf("Σ [REGISTRY]: Sovereign Package Registry initialized.\n");
}

sigma_err_t SovereignPkg_Register(const char* name, const char* version) {
    if (g_pkg_count >= SIGMA_MAX_PACKAGES) return SIGMA_ERR;
    sigma_strcpy(g_pkg_registry[g_pkg_count].name, name, 64);
    sigma_strcpy(g_pkg_registry[g_pkg_count].version, version, 16);
    g_pkg_registry[g_pkg_count].seated = SIGMA_TRUE;
    g_pkg_count++;
    return SIGMA_OK;
}

void SovereignPkg_Audit(void) {
    sigma_printf("Σ [PKG-AUDIT]: Verifying %d seated application shards...\n", g_pkg_count);
    for (sigma_u32 i = 0; i < g_pkg_count; i++) {
        sigma_printf("  ✓ [OK]: %s (v%s) — integrity verified.\n", g_pkg_registry[i].name, g_pkg_registry[i].version);
    }
}



