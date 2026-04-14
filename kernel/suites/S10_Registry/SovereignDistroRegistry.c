#include "../../include/sigma_base.h"

#include "../include/SovereignDistro.h"
#include "../include/sigma_libc.h"
#include "../include/sigma_string.h"

static sovereign_distro_registry_t g_distro_registry;

void SovereignDistro_InitRegistry(void) {
    sigma_memset(&g_distro_registry, 0, sizeof(sovereign_distro_registry_t));
    sigma_printf("Σ [DISTRO]: Sovereign Distro Registry Online.\n");
}

sigma_err_t SovereignDistro_Register(const char* name, const char* pkg, const char* init, const char* usp, sigma_distro_absorb_fn absorb) {
    if (g_distro_registry.distro_count >= MAX_DISTROS) return SIGMA_ENOSPC;

    sovereign_distro_t* d = &g_distro_registry.distros[g_distro_registry.distro_count++];
    sigma_strncpy(d->name, name, DISTRO_NAME_MAX);
    sigma_strncpy(d->pkg_mgr, pkg, 16);
    sigma_strncpy(d->init_system, init, 16);
    sigma_strncpy(d->usp_summary, usp, 128);
    d->absorb = absorb;
    
    return SIGMA_OK;
}

void SovereignDistro_Absorb(const char* name) {
    sigma_bool all = (sigma_streq(name, "all"));
    
    for (sigma_u32 i = 0; i < g_distro_registry.distro_count; i++) {
        if (all || sigma_streq(g_distro_registry.distros[i].name, name)) {
            sigma_printf("Σ [ABSORB]: Fragmenting and absorbing USP from Distro: %s\n", g_distro_registry.distros[i].name);
            if (g_distro_registry.distros[i].absorb) {
                g_distro_registry.distros[i].absorb();
            }
            if (!all) return;
        }
    }
    if (!all) sigma_printf("[DISTRO/ERR]: Unknown distro '%s'.\n", name);
}

void SovereignDistro_ListAll(void) {
    sigma_printf("\nΣ SIGMAOS: GLOBAL DISTRO USP MATRIX\n");
    sigma_printf("--------------------------------------------------------------------------------\n");
    sigma_printf("%-12s | %-8s | %-8s | %-40s\n", "Distro", "Pkg", "Init", "Notable USP");
    sigma_printf("--------------------------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < g_distro_registry.distro_count; i++) {
        sovereign_distro_t* d = &g_distro_registry.distros[i];
        sigma_printf("%-12s | %-8s | %-8s | %-40s\n", d->name, d->pkg_mgr, d->init_system, d->usp_summary);
    }
    sigma_printf("--------------------------------------------------------------------------------\n");
}

