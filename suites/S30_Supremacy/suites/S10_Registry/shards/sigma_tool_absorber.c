#include "../../../../../include/libc/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

/*
 * S SIGMAOS: SOVEREIGN TOOL ABSORBER v3.0  MODULAR
 * Mission: Unified entry point for all software tool absorption shards.
 * Design: C11 / Zero-Dependency / Registry-Based.
 */
#include "../../../../../include/SovereignToolHeader.h"
#include "../../../../../include/SovereignTool.h"
#include "../../../../../include/sigma_string.h"

/* Extern Registration Functions */
extern void SovereignGit_Register(void);
extern void SovereignDocker_Register(void);

void sigma_tool_init_all(void) {
    SovereignTool_InitRegistry();
    
    /* Register Tool Shards */
    SovereignGit_Register();
    /* (Future tool shards registered here) */
}

int sigma_tool_absorber_main(int argc, char** argv) {
    sigma_tool_init_all();

    if (argc < 2) {
        SovereignTool_ListAll();
        sigma_sigma_printf("Usage: sigma tools <absorb|list> [name]\n");
        return 0;
    }

    const char* sub = argv[1];
    if (sigma_streq(sub, "list")) {
        SovereignTool_ListAll();
        return 0;
    }

    if (sigma_streq(sub, "absorb")) {
        const char* target = argc > 2 ? argv[2] : "all";
        SovereignTool_Absorb(target);
        return 0;
    }

    sigma_sigma_printf("[ERROR] Unknown subcommand: %s\n", sub);
    return 1;
}



