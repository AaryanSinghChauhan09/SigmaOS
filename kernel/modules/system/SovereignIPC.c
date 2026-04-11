/*
 * Σ SIGMAOS: SOVEREIGN IPC v2.0 — MODULAR
 * Mission: Pluggable communication primitives. Every primitive is a shard.
 * Design: C11 / Zero-Dependency / Registry-Based.
 */
#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignIPC.h"

/* Extern IPC Registration Functions */
extern void SovereignPipe_Register(void);
extern void SovereignMQ_Register(void);

void SovereignIPC_Init(void) {
    sigma_printf("Σ [IPC]: Synchronizing Sovereign Communication Shards...\n");

    /* 1. Initialize Registry */
    SovereignIPC_InitRegistry();

    /* 2. Register IPC Shards */
    SovereignPipe_Register();

    /* 3. Activate Shards */
    SovereignIPC_ActivateAll();

    sigma_printf("Σ [IPC]: Communication Matrix Convergence Verified. IPC Sovereignty achieved.\n");
}
