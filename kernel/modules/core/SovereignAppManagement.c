/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN APPLICATION MANAGEMENT SUITE (v2.0 - INTEGRATED)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

typedef struct {
    char app_id[64];
    int is_sandboxed;
    char root_mount[128];
} SovereignAppContainer_t;

static SovereignAppContainer_t s_app_matrix[128];
static int s_active_apps = 0;

void sigma_universal_packaging_init() {
    sigma_memset(s_app_matrix, 0, sizeof(s_app_matrix));
    s_active_apps = 0;
}

void sigma_deploy_sandboxed_app(const char* identifier, int sandbox_level) {
    if (s_active_apps >= 128) return;
    sigma_strncpy(s_app_matrix[s_active_apps].app_id, identifier, 64);
    s_app_matrix[s_active_apps].is_sandboxed = (sandbox_level > 0);
    sigma_printf("Σ [APP-MGMT]: Deployed [%s] (Sandbox: %d)\n", identifier, s_app_matrix[s_active_apps].is_sandboxed);
    s_active_apps++;
}

void SovereignUniversalPackaging_Register(void) {
    static SovereignModule_t s_pkg_module = {
        .name = "SovereignUniversalPackaging",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))sigma_universal_packaging_init,
    };
    sigma_module_register(&s_pkg_module);
}
