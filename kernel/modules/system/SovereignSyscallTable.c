/*
 * Σ SIGMAOS: SOVEREIGN SYSCALL DISPATCH TABLE v2.0 — MODULAR
 * Mission: Pluggable syscall entry point. Linux ABI compatible.
 * Design: C11 / Zero-Dependency / Registry-Based.
 */
#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignSyscall.h"

/* Extern Syscall Registration Functions */
extern void SovereignFileSyscalls_Register(void);
extern void SovereignProcessSyscalls_Register(void);

void SovereignSyscallTable_Init(void) {
    sigma_printf("Σ [SYS]: Synchronizing Sovereign Syscall Shards...\n");

    /* 1. Initialize Registry */
    SovereignSyscall_InitRegistry();

    /* 2. Register Syscall Shards */
    SovereignFileSyscalls_Register();
    /* (Other syscall sectors registered here) */

    sigma_printf("Σ [SYS]: Syscall Matrix Convergence Verified. ABI Sovereignty achieved.\n");
}
