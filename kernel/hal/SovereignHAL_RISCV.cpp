/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL) - RISC-V
 * =========================================================================
 * Architecture-specific intrinsics for RISC-V (rv64).
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_hal.h"

static sigma_bool g_interrupts_enabled = SIGMA_FALSE;

extern "C" {

void hal_init(void) {
    sigma_log("[HAL] Hardware Abstraction Layer initialized (Arch: RISC-V rv64).");
}

void hal_disable_interrupts(void) {
#if defined(__riscv)
    __asm__ volatile("csrci sstatus, 2"); /* Disable SIE */
#endif
    g_interrupts_enabled = SIGMA_FALSE;
}

void hal_enable_interrupts(void) {
#if defined(__riscv)
    __asm__ volatile("csrsi sstatus, 2"); /* Enable SIE */
#endif
    g_interrupts_enabled = SIGMA_TRUE;
}

sigma_bool hal_interrupts_enabled(void) {
    return g_interrupts_enabled;
}

/* MMIO stubs */
sigma_u8  hal_inb(sigma_u16 port) { return 0; }
sigma_u16 hal_inw(sigma_u16 port) { return 0; }
sigma_u32 hal_inl(sigma_u16 port) { return 0; }
void      hal_outb(sigma_u16 port, sigma_u8 val) {}
void      hal_outw(sigma_u16 port, sigma_u16 val) {}
void      hal_outl(sigma_u16 port, sigma_u32 val) {}

sigma_u64 hal_read_tsc(void) {
    sigma_u64 val = 0;
#if defined(__riscv) && (__riscv_xlen == 64)
    __asm__ volatile("rdtime %0" : "=r" (val));
#endif
    return val;
}

void hal_cpu_relax(void) {
    /* RISC-V doesn't have a direct pause equivalent yet, use nop */
#if defined(__riscv)
    __asm__ volatile("nop" ::: "memory");
#endif
}

void hal_cpu_halt(void) {
#if defined(__riscv)
    __asm__ volatile("wfi");
#endif
}

void hal_tlb_flush_single(sigma_vaddr_t addr) {
#if defined(__riscv)
    __asm__ volatile("sfence.vma %0, zero" :: "r"(addr) : "memory");
#endif
}

void hal_tlb_flush_all(void) {
#if defined(__riscv)
    __asm__ volatile("sfence.vma zero, zero" ::: "memory");
#endif
}

} // extern "C"
