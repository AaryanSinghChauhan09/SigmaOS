#include "../../include/sigma_base.h"

/*
 * Σ SIGMAOS: SOVEREIGN DISTRO ABSORBER v3.0 — MODULAR
 * Mission: Unified entry point for all distro personality shards.
 * Design: C11 / Zero-Dependency / Registry-Based.
 */
#include "../include/SovereignToolHeader.h"
#include "../include/SovereignDistro.h"
#include "../include/sigma_string.h"

/* Extern Registration Functions */
extern void SovereignArch_Register(void);
extern void SovereignNix_Register(void);
extern void SovereignGentoo_Register(void);
extern void SovereignGaruda_Register(void);

void sigma_distro_init_all(void) {
    SovereignDistro_InitRegistry();
    
    /* Register Shards */
    SovereignArch_Register();
    SovereignGaruda_Register();
    /* (Future shards will be registered here) */
}

int sigma_distro_absorber_main(int argc, char** argv) {
    sigma_distro_init_all();

    if (argc < 2) {
        SovereignDistro_ListAll();
        sigma_printf("Usage: sigma distro <absorb|personality|list|info> [name]\n");
        return 0;
    }

    const char* sub = argv[1];
    if (sigma_streq(sub, "list")) {
        SovereignDistro_ListAll();
        return 0;
    }

    if (sigma_streq(sub, "absorb")) {
        const char* target = argc > 2 ? argv[2] : "all";
        SovereignDistro_Absorb(target);
        return 0;
    }

    if (sigma_streq(sub, "personality")) {
        const char* p = argc > 2 ? argv[2] : "arch";
        sigma_printf("[SIGMA-DISTRO] Activating '%s' personality via modular registry...\n", p);
        /* Logic for personality mapping */
        return 0;
    }

    sigma_printf("[ERROR] Unknown subcommand: %s\n", sub);
    return 1;
}

