/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL)
 * =========================================================================
 * Mission: Zero-dependency silicon orchestration.
 * NOTE: cpu_pause, cpu_halt, port_outb, port_inb, serial_init, serial_putc
 *       are defined in sigma_kernel_types.h (the silicon source-of-truth).
 *       This header only declares HAL lifecycle and logging functions.
 * =========================================================================
 */

#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

/* sigma_kernel_types.h is the single source of truth for all primitives */
#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Silicon Lifecycle --- */
void hal_init(void);
void hal_shutdown(void);

/* --- Industrial Logging --- */
void sigma_printf(const char* format, ...);

/* --- Assertions --- */
#define sigma_assert(condition) \
    do { if (!(condition)) { \
        sigma_printf("[PANIC] ASSERTION FAILED: %s at %s:%d\n", #condition, __FILE__, __LINE__); \
        hal_shutdown(); \
    } } while (0)

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAL_H */
