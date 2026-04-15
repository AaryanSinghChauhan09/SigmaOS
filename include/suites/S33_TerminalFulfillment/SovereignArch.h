/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN ARCHITECTURE INTERFACE (v1.0)
 * =========================================================================
 * Mission: Multi-Architecture, Multi-Device Support Matrix.
 * Design: C11 / Zero-Dependency / Registry-Based.
 * Supported: x86_64, ARM64, RISC-V.
 * =========================================================================
 */

#ifndef SOVEREIGN_ARCH_H
#define SOVEREIGN_ARCH_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef void (*sigma_arch_halt_fn)(void);
typedef void (*sigma_arch_init_fn)(void);

typedef struct {
    char name[32];
    sigma_arch_init_fn init;
    sigma_arch_halt_fn halt;
} sovereign_arch_shard_t;

/* Registry API */
void SovereignArch_InitRegistry(void);
sigma_err_t SovereignArch_Register(const char* name, sigma_arch_init_fn init, sigma_arch_halt_fn halt);
void SovereignArch_InitializeCPU(const char* arch_name);

#endif /* SOVEREIGN_ARCH_H */
