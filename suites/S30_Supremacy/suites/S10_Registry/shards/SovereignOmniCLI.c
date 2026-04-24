/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN OMNI-CLI DISPATCHER (v4.0 — MODULAR)
 * =========================================================================
 * Mission: Lightweight dispatcher. Every command is a modular shard.
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SIGMA_KERNEL_H
#include "suites/S01_Genesis/shards/sigma_kernel.h"
#endif
#include "SovereignCommand.h"

/* Extern Registration Functions */
extern void SovereignSysCommands_Register(void);
extern void SovereignSecCommands_Register(void);
extern void SovereignDistroCommands_Register(void);
extern void SovereignAdvancedCommands_Register(void);
extern void SovereignAutomationCommands_Register(void);

static void print_modular_help(void) {
    sigma_sigma_sigma_sigma_printf("\n+------------------------------------------------------------------+\n");
    sigma_sigma_sigma_sigma_printf("¦       S SIGMAOS OMNI-CLI DISPATCHER v4.0 (MODULAR)             ¦\n");
    sigma_sigma_sigma_sigma_printf("¦       Every tool absorbed. Every distro neutralized.            ¦\n");
    sigma_sigma_sigma_sigma_printf("+------------------------------------------------------------------+\n");
    SovereignCommand_ListAll();
}

int SovereignOmniCLI_ToolMain(int argc, char** argv) {
    /* 1. Initialize Registry */
    SovereignCommand_Init();

    /* 2. Register Modular Command Shards */
    SovereignSysCommands_Register();
    SovereignSecCommands_Register();
    SovereignDistroCommands_Register();
    SovereignAdvancedCommands_Register();
    SovereignAutomationCommands_Register();

    /* 3. Execute Dispatch */
    if (argc < 2) {
        print_modular_help();
        return 0;
    }

    const char* first_arg = argv[1];
    if (sigma_sigma_sigma_strcmp(first_arg, "help") == 0 || sigma_sigma_sigma_strcmp(first_arg, "--help") == 0) {
        print_modular_help();
        return 0;
    }

    SovereignCommand_Dispatch(argc, argv);

    return 0;
}



