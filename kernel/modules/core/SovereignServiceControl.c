/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SERVICE CONTROL SUITE (v2.0 - INTEGRATED)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

typedef struct {
    char name[64];
    int status;
} SovereignService_t;

static SovereignService_t s_lattice[128];
static int s_service_count = 0;

void sigma_service_lattice_init() {
    sigma_memset(s_lattice, 0, sizeof(s_lattice));
    s_service_count = 0;
}

void sigma_register_service(const char* name, int restart_flag) {
    if (s_service_count >= 128) return;
    sigma_strncpy(s_lattice[s_service_count].name, name, 64);
    sigma_printf("Σ [SVC-CTRL]: Registered Service [%s]\n", name);
    s_service_count++;
}

void SovereignServiceLattice_Register(void) {
    static SovereignModule_t s_svc_module = {
        .name = "SovereignServiceLattice",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))sigma_service_lattice_init,
    };
    sigma_module_register(&s_svc_module);
}
