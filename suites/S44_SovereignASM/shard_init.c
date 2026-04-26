#include "sigma_libc.h"

// SigmaOS Sovereign ASM (S-ASM)
// Philosophy: Zero-Runtime - Pure Machine Instruction Primitives.
// USP: Provides a suite of raw assembly wrappers to eliminate high-level overhead for critical lattice operations.

void s_asm_yield() {
    #if defined(__x86_64__)
    __asm__ __volatile__ ("pause" ::: "memory");
    #endif
}

void s_asm_fence() {
    #if defined(__x86_64__)
    __asm__ __volatile__ ("mfence" ::: "memory");
    #endif
}

void shard_init() {
    SIGMA_SHARD_INIT();
    sigma_printf("[SHARD] Sovereign ASM active. Zero-runtime primitives enabled.\n");
}
