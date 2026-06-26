/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL) - ARM64
 * =========================================================================
 * Architecture-specific intrinsics for ARMv8-A (AArch64).
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_hal.h"

static sigma_bool g_interrupts_enabled = SIGMA_FALSE;

extern "C" {

void hal_init(void) {
    sigma_log("[HAL] Hardware Abstraction Layer initialized (Arch: ARM64).");
}

void hal_disable_interrupts(void) {
#if defined(__aarch64__)
    __asm__ volatile("msr daifset, #2"); /* Disable IRQ */
#endif
    g_interrupts_enabled = SIGMA_FALSE;
}

void hal_enable_interrupts(void) {
#if defined(__aarch64__)
    __asm__ volatile("msr daifclr, #2"); /* Enable IRQ */
#endif
    g_interrupts_enabled = SIGMA_TRUE;
}

sigma_bool hal_interrupts_enabled(void) {
    return g_interrupts_enabled;
}

/* Port IO doesn't exist on ARM (Memory-Mapped IO is used). These are stubs. */
sigma_u8  hal_inb(sigma_u16 port) { return 0; }
sigma_u16 hal_inw(sigma_u16 port) { return 0; }
sigma_u32 hal_inl(sigma_u16 port) { return 0; }
void      hal_outb(sigma_u16 port, sigma_u8 val) {}
void      hal_outw(sigma_u16 port, sigma_u16 val) {}
void      hal_outl(sigma_u16 port, sigma_u32 val) {}

sigma_u64 hal_read_tsc(void) {
    sigma_u64 val = 0;
#if defined(__aarch64__)
    __asm__ volatile("mrs %0, cntvct_el0" : "=r" (val));
#endif
    return val;
}

void hal_cpu_relax(void) {
#if defined(__aarch64__)
    __asm__ volatile("yield" ::: "memory");
#endif
}

void hal_cpu_halt(void) {
#if defined(__aarch64__)
    __asm__ volatile("wfi");
#endif
}

void hal_tlb_flush_single(sigma_vaddr_t addr) {
#if defined(__aarch64__)
    __asm__ volatile("tlbi vae1, %0; dsb sy; isb" :: "r" (addr >> 12) : "memory");
#endif
}

void hal_tlb_flush_all(void) {
#if defined(__aarch64__)
    __asm__ volatile("tlbi vmalle1; dsb sy; isb" ::: "memory");
#endif
}

} // extern "C"
