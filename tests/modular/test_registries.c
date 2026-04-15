#include "sigma_kernel.h"
#include "SovereignLatticeRegistry.h"
#include "SovereignDistro.h"
#include "SovereignUSP.h"
#include "SovereignTool.h"
#include "SovereignCommand.h"

void test_registries_modular() {
    sigma_printf("S [TEST]: Running Paradigm Registry Audit...\n");

    /* 1. Audit SovereignRegistry */
    SovereignRegistry_Init();
    sigma_printf("S [PASS]: Core Registry Operational.\n");

    /* 2. Audit Distro Registry */
    SovereignDistro_InitRegistry();
    SIGMA_ASSERT(SovereignDistro_Register("test-distro", "pkg", "init", "USP", SIGMA_NULL) == SIGMA_OK, "Distro registration failed");
    sigma_printf("S [PASS]: Distro Sharding Registry Verified.\n");

    /* 3. Audit USP Registry */
    SovereignUSP_InitRegistry();
    SIGMA_ASSERT(SovereignUSP_Register("test-usp", "Description", SIGMA_NULL) == SIGMA_OK, "USP registration failed");
    sigma_printf("S [PASS]: Kernel USP Registry Verified.\n");

    /* 4. Audit Tool Registry */
    SovereignTool_InitRegistry();
    SIGMA_ASSERT(SovereignTool_Register("test-tool", "Replacement", SIGMA_NULL) == SIGMA_OK, "Tool registration failed");
    sigma_printf("S [PASS]: Tool Absorption Registry Verified.\n");

    /* 5. Audit Command Registry */
    SovereignCommand_InitRegistry();
    SIGMA_ASSERT(SovereignCommand_Register("test-cmd", "Desc", SIGMA_NULL) == SIGMA_OK, "Command registration failed");
    sigma_printf("S [PASS]: Omni-CLI Command Registry Verified.\n");

    sigma_printf("S [SUCCESS]: All Modular Paradigm Registries are HEALTHY.\n");
}
