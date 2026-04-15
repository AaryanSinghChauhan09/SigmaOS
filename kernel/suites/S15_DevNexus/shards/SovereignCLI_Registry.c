#include "sigma_base.h"

#include "SovereignCLI_Core.h"
#include "../../include/SovereignCLI.h"

void SovereignCLI_Register(void) {
    /* 1. Seat Core Shell Logic */
    extern void SovereignCLI_Core_Register(void);
    SovereignCLI_Core_Register();

    /* 2. Seat Essential Commands (ls, cat, cd, echo) */
    extern void SovereignCLI_Essential_Register(void);
    SovereignCLI_Essential_Register();

    /* 3. Seat Linux-Grade Distro Tools (apt, pacman, top, neofetch, grep) */
    extern void SovereignCLI_DistroSuite_Register(void);
    SovereignCLI_DistroSuite_Register();

    /* 4. Seat Developer-Grade Tools (git, python, cc, make) */
    extern void SovereignCLI_DevSuite_Register(void);
    SovereignCLI_DevSuite_Register();

    /* 5. Seat Cyber-Security Tools (nmap, whoami, iptables, defender, vault) */
    extern void SovereignCLI_CyberSuite_Register(void);
    SovereignCLI_CyberSuite_Register();
    
    sigma_printf("Σ [CLI-REGISTRY]: Industrial Command Omnibus (100+ Matrix) Seated.\n");

    static SovereignModule_t s_cli_module = {
        .name = "SovereignCLI",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))SovereignCLI_Init,
    };
    sigma_module_register(&s_cli_module);
}



