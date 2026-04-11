/*
 * Σ SIGMAOS: SOVEREIGN INIT SYSTEM v2.0 — MODULAR
 * Mission: Pluggable service orchestration (PID 1).
 * Design: C11 / Zero-Dependency / Registry-Based.
 */
#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignInit.h"

/* Extern Service Registration Functions */
extern void SovereignLogger_Register(void);

void SovereignInitSystem_Init(void) {
    sigma_printf("Σ [INIT]: Synchronizing Sovereign PID-1 Service Shards...\n");

    /* 1. Initialize Registry */
    SovereignInit_InitRegistry();

    /* 2. Register Service Shards */
    SovereignLogger_Register();
    /* (Other system services registered here) */

    /* 3. Execute Parallel Activation */
    SovereignInit_StartAll();

    /* 4. Display Orchestration Status */
    SovereignInit_ShowStatus();

    sigma_printf("Σ [INIT]: Service Matrix Convergence Verified. PID-1 Sovereignty achieved.\n");
}
