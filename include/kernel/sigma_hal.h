/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL v1.0)
 * =============================================================================
 * Mission: Abstract architecture-specific intrinsics, port I/O, and CPU
 *          management (x86_64 initially).
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

#include "../sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

void       hal_init(void);

/* Interrupt Management */
void       hal_disable_interrupts(void);
void       hal_enable_interrupts(void);
sigma_bool hal_interrupts_enabled(void);

/* Port I/O */
sigma_u8   hal_inb(sigma_u16 port);
sigma_u16  hal_inw(sigma_u16 port);
sigma_u32  hal_inl(sigma_u16 port);

void       hal_outb(sigma_u16 port, sigma_u8 val);
void       hal_outw(sigma_u16 port, sigma_u16 val);
void       hal_outl(sigma_u16 port, sigma_u32 val);

/* CPU & Timing */
sigma_u64  hal_read_tsc(void);
void       hal_cpu_relax(void);
void       hal_cpu_halt(void);

/* MMU / TLB */
void       hal_tlb_flush_single(sigma_vaddr_t addr);
void       hal_tlb_flush_all(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAL_H */
