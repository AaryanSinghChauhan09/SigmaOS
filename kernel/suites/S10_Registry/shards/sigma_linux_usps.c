#include "../../include/sigma_base.h"

/*
 * Σ SIGMAOS: SOVEREIGN LINUX USP ENGINE v2.0 — MODULAR
 * Mission: Unified entry point for all absorbed Linux Kernel USPs.
 * Design: C11 / Zero-Dependency / Registry-Based.
 */
#include "../include/SovereignToolHeader.h"
#include "../include/SovereignUSP.h"
#include "../include/sigma_string.h"

/* Extern Registration Functions */
extern void SovereignBPF_Register(void);
extern void SovereignProcFS_Register(void);

void sigma_linux_usps_init_all(void) {
    SovereignUSP_InitRegistry();
    
    /* Register USP Shards */
    SovereignBPF_Register();
    /* (Future shards registered here) */
}

int sigma_linux_usps_main(int argc, char** argv) {
    sigma_linux_usps_init_all();

    if (argc < 2) {
        SovereignUSP_ListAll();
        sigma_printf("Usage: sigma linux-usps <module|all|list>\n");
        return 0;
    }

    const char* sub = argv[1];
    if (sigma_streq(sub, "list")) {
        SovereignUSP_ListAll();
        return 0;
    }

    if (sigma_streq(sub, "all")) {
        SovereignUSP_Show("all");
        return 0;
    }

    SovereignUSP_Show(sub);
    return 0;
}



