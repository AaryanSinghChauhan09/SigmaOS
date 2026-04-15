#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignUSP.h"
#include "sigma_libc.h"
#include "sigma_string.h"

static sovereign_usp_registry_t g_usp_registry;

void SovereignUSP_InitRegistry(void) {
    sigma_memset(&g_usp_registry, 0, sizeof(sovereign_usp_registry_t));
    sigma_printf("S [USP]: Sovereign Linux USP Registry Online. Capacity: %d.\n", MAX_USPS);
}

sigma_err_t SovereignUSP_Register(const char* name, const char* desc, sigma_usp_show_fn show) {
    if (g_usp_registry.usp_count >= MAX_USPS) return SIGMA_ENOSPC;

    sovereign_usp_t* u = &g_usp_registry.usps[g_usp_registry.usp_count++];
    sigma_strncpy(u->name, name, USP_NAME_MAX);
    sigma_strncpy(u->description, desc, 128);
    u->show = show;
    
    return SIGMA_OK;
}

void SovereignUSP_Show(const char* name) {
    sigma_bool all = (sigma_streq(name, "all"));
    
    for (sigma_u32 i = 0; i < g_usp_registry.usp_count; i++) {
        if (all || sigma_streq(g_usp_registry.usps[i].name, name)) {
            if (g_usp_registry.usps[i].show) {
                g_usp_registry.usps[i].show();
            }
            if (!all) return;
        }
    }
    if (!all) sigma_printf("[USP/ERR]: Unknown Kernel USP '%s'.\n", name);
}

void SovereignUSP_ListAll(void) {
    sigma_printf("\nS SIGMAOS: CORE KERNEL USP CATALOG (LINUX PARITY)\n");
    sigma_printf("--------------------------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < g_usp_registry.usp_count; i++) {
        sigma_printf("%-16s | %s\n", g_usp_registry.usps[i].name, g_usp_registry.usps[i].description);
    }
    sigma_printf("--------------------------------------------------------------------------------\n");
}



