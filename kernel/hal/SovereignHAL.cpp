/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL v1.0)
 * =========================================================================
 * Architecture-specific intrinsics (x86_64 default).
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_hal.h"

static sigma_bool g_interrupts_enabled = SIGMA_FALSE;

extern "C" {

void hal_init(void) {
    sigma_log("[HAL] Hardware Abstraction Layer initialized (Arch: x86_64).");
}

void hal_disable_interrupts(void) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("cli");
#endif
    g_interrupts_enabled = SIGMA_FALSE;
}

void hal_enable_interrupts(void) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("sti");
#endif
    g_interrupts_enabled = SIGMA_TRUE;
}

sigma_bool hal_interrupts_enabled(void) {
    return g_interrupts_enabled;
}

sigma_u8 hal_inb(sigma_u16 port) {
    sigma_u8 ret = 0;
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
#endif
    return ret;
}

sigma_u16 hal_inw(sigma_u16 port) {
    sigma_u16 ret = 0;
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("inw %1, %0" : "=a"(ret) : "Nd"(port));
#endif
    return ret;
}

sigma_u32 hal_inl(sigma_u16 port) {
    sigma_u32 ret = 0;
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("inl %1, %0" : "=a"(ret) : "Nd"(port));
#endif
    return ret;
}

void hal_outb(sigma_u16 port, sigma_u8 val) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("outb %0, %1" : : "a"(val), "Nd"(port));
#endif
}

void hal_outw(sigma_u16 port, sigma_u16 val) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("outw %0, %1" : : "a"(val), "Nd"(port));
#endif
}

void hal_outl(sigma_u16 port, sigma_u32 val) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("outl %0, %1" : : "a"(val), "Nd"(port));
#endif
}

sigma_u64 hal_read_tsc(void) {
    sigma_u32 lo, hi;
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("rdtsc" : "=a"(lo), "=d"(hi));
    return ((sigma_u64)hi << 32) | lo;
#else
    return 0;
#endif
}

void hal_cpu_relax(void) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("pause" ::: "memory");
#endif
}

void hal_cpu_halt(void) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("hlt");
#endif
}

void hal_tlb_flush_single(sigma_vaddr_t addr) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ volatile("invlpg (%0)" ::"r" (addr) : "memory");
#endif
}

void hal_tlb_flush_all(void) {
#if defined(__x86_64__) || defined(__i386__)
    sigma_u64 cr3;
    __asm__ volatile("mov %%cr3, %0" : "=r" (cr3));
    __asm__ volatile("mov %0, %%cr3" :: "r" (cr3) : "memory");
#endif
}

} // extern "C"
