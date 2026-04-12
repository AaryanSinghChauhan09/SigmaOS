/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA KERNEL AGGREGATOR (v2.0 — MASTER)
 * =========================================================================
 * Mission: Master initialization orchestrator only.
 * =========================================================================
 */

#ifndef SIGMA_KERNEL_H
#define SIGMA_KERNEL_H

#include "sigma_base.h"
#include "SovereignRegistry.h"

static inline void SovereignMaster_InitAll(void) {
    SovereignRegistry_Init();
    
    /* Registration Hooks */
    extern void SovereignMemory_Register(void);
    extern void SovereignSecurity_Register(void);
    extern void SovereignCrypto_Register(void);
    extern void SovereignAppManagement_Register(void);
    extern void SovereignServiceControl_Register(void);
    extern void SovereignIntelligence_Register(void);
    extern void SovereignFrontend_Register(void);
    extern void SovereignEcosystem_Register(void);
    extern void SovereignBackend_Register(void);
    extern void SovereignConfig_Register(void);
    extern void SovereignCLI_Register(void);
    extern void SovereignPrinciple_Register(void);

    /* Activation Sequence */
    SovereignMemory_Register();
    SovereignSecurity_Register();
    SovereignCrypto_Register();
    SovereignAppManagement_Register();
    SovereignServiceControl_Register();
    SovereignIntelligence_Register();
    SovereignFrontend_Register();
    SovereignEcosystem_Register();
    SovereignBackend_Register();
    SovereignConfig_Register();
    SovereignCLI_Register();
    SovereignPrinciple_Register();

    sigma_printf("\nΣ [SOVEREIGN-MASTER]: Global Mesh Orchestration COMPLETE.\n");
    sigma_modules_init_all();
}

#endif /* SIGMA_KERNEL_H */
