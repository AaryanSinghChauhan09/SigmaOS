/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL)
 * =========================================================================
 * Mission: Zero-dependency silicon orchestration.
 * =========================================================================
 */

#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Silicon Lifecycle --- */
void hal_init(void);
void hal_shutdown(void);

/* --- Industrial Logging --- */
void sigma_log(const char* msg);
void sigma_printf(const char* format, ...);

/* --- CPU Control --- */
static inline void cpu_pause(void) {
    __asm__ __volatile__("pause" ::: "memory");
}

static inline void cpu_halt(void) {
    __asm__ __volatile__("hlt" ::: "memory");
}

/* --- Port I/O --- */
static inline void port_outb(sigma_u16 port, sigma_u8 val) {
    __asm__ __volatile__("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline sigma_u8 port_inb(sigma_u16 port) {
    sigma_u8 ret;
    __asm__ __volatile__("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAL_H */
