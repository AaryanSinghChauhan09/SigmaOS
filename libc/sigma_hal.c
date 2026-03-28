/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (sigma_hal.c)
 * =========================================================================
 * USP Absorbed: Linux (Arch-specific bits), FreeBSD (HAL/PM), seL4 (Arch-HAL)
 * Principle: Unified interface for multi-architecture sovereign compute.
 * Languages: C, architecture-aware inline ASM.
 * =========================================================================
 */

#include "sigma_types.h"
#include "sigma_libc.h"

/* Sovereign CPU Context State (Poly-Arch) */
struct CpuContext {
#if defined(SIGMA_ARCH_X86_64)
    sigma_u64 rip, rsp, rax, rbx, rcx, rdx, rsi, rdi;
    sigma_u64 r8, r9, r10, r11, r12, r13, r14, r15;
    sigma_u64 rflags;
#elif defined(SIGMA_ARCH_ARM64)
    sigma_u64 pc, sp, x[31], pstate;
#elif defined(SIGMA_ARCH_RISCV64)
    sigma_u64 pc, sp, ra, x[31], status;
#endif
};

/* Architecture-specific I/O Gateways */
void sigma_hal_outb(sigma_u16 port, sigma_u8 val) {
#if defined(SIGMA_ARCH_X86_64) || defined(SIGMA_ARCH_X86_32)
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
#else
    (void)port; (void)val; // Memory-mapped I/O on non-x86
#endif
}

sigma_u8 sigma_hal_inb(sigma_u16 port) {
#if defined(SIGMA_ARCH_X86_64) || defined(SIGMA_ARCH_X86_32)
    sigma_u8 res;
    __asm__ volatile ("inb %1, %0" : "=a"(res) : "Nd"(port));
    return res;
#else
    (void)port; return 0;
#endif
}

/* Multi-Arch Memory Barrier Implementation */
void sigma_hal_barrier() {
#if defined(SIGMA_ARCH_X86_64)
    __asm__ volatile ("mfence" ::: "memory");
#elif defined(SIGMA_ARCH_ARM64)
    __asm__ volatile ("dmb sy" ::: "memory");
#elif defined(SIGMA_ARCH_RISCV64)
    __asm__ volatile ("fence" ::: "memory");
#else
    __asm__ volatile ("" ::: "memory");
#endif
}

/* Sovereign CPU ID / Feature Shard Probing */
void sigma_hal_probe_features() {
    sigma_printf("[HAL]: Probing sovereign execution shards for %s...\n", 
#if defined(SIGMA_ARCH_X86_64)
        "x86_64"
#elif defined(SIGMA_ARCH_ARM64)
        "AARCH64"
#elif defined(SIGMA_ARCH_RISCV64)
        "RISCV64"
#else
        "UNKNOWN_ARCH"
#endif
    );
    sigma_hal_barrier();
}

/* Global HAL Dispatcher Entry */
void sigma_hal_init() {
    sigma_hal_probe_features();
}

