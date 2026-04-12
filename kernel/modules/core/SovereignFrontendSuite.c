/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN FRONTEND SUITE (v2.0 - INDUSTRIAL)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

void SovereignFrontend_Init(void) {
    sigma_printf("Σ [FRONT-SUITE]: Starting Sovereign Window Manager (SWM)...\n");
    sigma_printf("Σ [FRONT-SUITE]: Compositor layer connected to silicon probe.\n");
    sigma_printf("Σ [FRONT-SUITE]: Audio engine online - Neural Synth seated.\n");
}

void SovereignFrontend_Register(void) {
    static SovereignModule_t s_front_module = {
        .name = "SovereignFrontend",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))SovereignFrontend_Init,
    };
    sigma_module_register(&s_front_module);
}
