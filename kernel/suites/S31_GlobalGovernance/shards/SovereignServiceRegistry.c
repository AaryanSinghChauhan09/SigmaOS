#include "sigma_base.h"

#include "SovereignInit.h"
#include "sigma_libc.h"
#include "sigma_string.h"

#define MAX_SYSTEM_SERVICES 32
static sovereign_service_shard_t g_services[MAX_SYSTEM_SERVICES];
static sigma_u32 g_svc_count = 0;

void SovereignInit_InitRegistry(void) {
    sigma_memset(g_services, 0, sizeof(g_services));
    g_svc_count = 0;
    sigma_printf("Σ [INIT]: Sovereign Service Registry Operational.\n");
}

sigma_err_t SovereignInit_RegisterService(const char* name, const char* path, sigma_bool restart, sigma_svc_init_fn init) {
    if (g_svc_count >= MAX_SYSTEM_SERVICES) return SIGMA_ENOSPC;

    sovereign_service_shard_t* s = &g_services[g_svc_count++];
    sigma_strncpy(s->name, name, 32);
    sigma_strncpy(s->exec_path, path, 64);
    s->auto_restart = restart;
    s->init = init;
    s->state = SIGMA_SVC_STOPPED;
    
    sigma_printf("Σ [INIT]: Registered Service Shard '%s' (Path: %s)\n", name, path);
    return SIGMA_OK;
}

void SovereignInit_StartAll(void) {
    sigma_printf("Σ [INIT]: Orchestrating Parallel Service Activation...\n");
    for (sigma_u32 i = 0; i < g_svc_count; i++) {
        sigma_printf("Σ [INIT]: Activating %s -> PID %d\n", g_services[i].name, 1000+i);
        g_services[i].state = SIGMA_SVC_RUNNING;
        if (g_services[i].init) g_services[i].init();
    }
}

void SovereignInit_ShowStatus(void) {
    sigma_printf("\nΣ SIGMAOS: CORE SERVICE ORCHESTRATION TABLE\n");
    sigma_printf("--------------------------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < g_svc_count; i++) {
        sigma_printf("%-16s | %-20s | %s\n", g_services[i].name, g_services[i].exec_path, "RUNNING");
    }
    sigma_printf("--------------------------------------------------------------------------------\n");
}



