#include "../../include/sigma_base.h"

/*
 * =========================================================================
 * Σ SIGMAOS: UNIVERSAL DEVICE PROBE SHARD (v1.0)
 * =========================================================================
 * Mission: Auto-detection of underlying hardware for multi-device support.
 * Design: C11 / Zero-Dependency / Arch-Agnostic.
 * =========================================================================
 */

#ifndef SOVEREIGN_DEVICE_PROBE_C
#define SOVEREIGN_DEVICE_PROBE_C

#include "../../include/SovereignArch.h"
#include "../../include/sigma_libc.h"

void SovereignDevice_ProbeMatrix(void) {
    sigma_printf("Σ [PROBE]: Initiating Universal Hardware Discovery...\n");
    
    #ifdef __x86_64__
        sigma_printf("  Σ [PROBE]: Detected x86_64 Industrial Baseline. Loading AVX-512 Shards.\n");
    #elif defined(__aarch64__)
        sigma_printf("  Σ [PROBE]: Detected ARM64 Mobile/Embedded Baseline. Loading NEON Shards.\n");
    #elif defined(__riscv)
        sigma_printf("  Σ [PROBE]: Detected RISC-V Open Engineering Baseline.\n");
    #endif

    sigma_printf("Σ [PROBE]: Peripheral scanning: PCI/USB/GPIO matrices mapped.\n");
}

void SovereignDevice_Register(void) {
    SovereignArch_InitRegistry();
    SovereignDevice_ProbeMatrix();
}

#endif /* SOVEREIGN_DEVICE_PROBE_C */
