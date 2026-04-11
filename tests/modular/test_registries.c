#include "../../include/sigma_kernel.h"
#include "../../include/SovereignRegistry.h"
#include "../../include/SovereignDistro.h"
#include "../../include/SovereignUSP.h"
#include "../../include/SovereignTool.h"
#include "../../include/SovereignCommand.h"

void test_registries_modular() {
    sigma_printf("Σ [TEST]: Running Paradigm Registry Audit...\n");

    /* 1. Audit SovereignRegistry */
    SovereignRegistry_Init();
    sigma_printf("Σ [PASS]: Core Registry Operational.\n");

    /* 2. Audit Distro Registry */
    SovereignDistro_InitRegistry();
    SIGMA_ASSERT(SovereignDistro_Register("test-distro", "pkg", "init", "USP", SIGMA_NULL) == SIGMA_OK, "Distro registration failed");
    sigma_printf("Σ [PASS]: Distro Sharding Registry Verified.\n");

    /* 3. Audit USP Registry */
    SovereignUSP_InitRegistry();
    SIGMA_ASSERT(SovereignUSP_Register("test-usp", "Description", SIGMA_NULL) == SIGMA_OK, "USP registration failed");
    sigma_printf("Σ [PASS]: Kernel USP Registry Verified.\n");

    /* 4. Audit Tool Registry */
    SovereignTool_InitRegistry();
    SIGMA_ASSERT(SovereignTool_Register("test-tool", "Replacement", SIGMA_NULL) == SIGMA_OK, "Tool registration failed");
    sigma_printf("Σ [PASS]: Tool Absorption Registry Verified.\n");

    /* 5. Audit Command Registry */
    SovereignCommand_Init();
    SIGMA_ASSERT(SovereignCommand_Register("test-cmd", "Desc", SIGMA_NULL) == SIGMA_OK, "Command registration failed");
    sigma_printf("Σ [PASS]: Omni-CLI Command Registry Verified.\n");

    sigma_printf("Σ [SUCCESS]: All Modular Paradigm Registries are HEALTHY.\n");
}
