/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE ABSTRACTION LAYER (HAL)
 * =========================================================================
 * Mission: Zero-dependency silicon orchestration.
 * =========================================================================
 */

#ifndef SIGMA_HAL_H
#define SIGMA_HAL_H

#include "SovereignLibC.h"

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Silicon Lifecycle --- */
void hal_init(void);
void hal_shutdown(void);

/* Intrinsics: cpu_pause, cpu_halt, port_inb/outb — sigma_kernel_types.h (via sigma_types.h) */

/* --- Serial I/O (Step 3: Kernel Debugging) --- */
#define COM1 0x3F8

static inline void serial_init(void) {
    port_outb(COM1 + 1, 0x00);    // Disable all interrupts
    port_outb(COM1 + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    port_outb(COM1 + 0, 0x03);    // Set divisor to 3 (38400 baud)
    port_outb(COM1 + 1, 0x00);    //                  hi byte
    port_outb(COM1 + 3, 0x03);    // 8 bits, no parity, one stop bit
    port_outb(COM1 + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
    port_outb(COM1 + 4, 0x0B);    // IRQs enabled, RTS/DSR set
}

static inline int is_transmit_empty(void) {
    return port_inb(COM1 + 5) & 0x20;
}

static inline void serial_putc(char c) {
    while (is_transmit_empty() == 0);
    port_outb(COM1, c);
}

/* --- Assertions (Step 3: Kernel Debugging) --- */
#define sigma_assert(condition) \
    if (!(condition)) { \
        sigma_printf("[PANIC] ASSERTION FAILED: %s at %s:%d\n", #condition, __FILE__, __LINE__); \
        hal_shutdown(); \
    }

#ifdef __cplusplus
}
#endif



#endif /* SIGMA_HAL_H */


