#include "../../../../../include/libc/SovereignLibC.h"
#include "../../../../../include/libc/sigma_libc.h"
#include "../../../../../include/core/sigma_types.h"

/**
 * SigmaOS Sovereign Interrupt Vectoring
 * Subsystem: S01 (Genesis)
 * Mission: Zero-latency hardware interrupt dispatch via Direct-Vectoring.
 */

#define MAX_INTERRUPTS 256

typedef void (*interrupt_handler_t)(void);

static interrupt_handler_t vector_table[MAX_INTERRUPTS];

void genesis_register_handler(uint8_t vector, interrupt_handler_t handler) {
    if (vector >= MAX_INTERRUPTS) return;
    vector_table[vector] = handler;
    sigma_printf("S01 [GENESIS]: Vector 0x%02X bound to handler 0x%p\n", vector, handler);
}

void genesis_dispatch_interrupt(uint8_t vector) {
    if (vector_table[vector]) {
        vector_table[vector]();
    } else {
        sigma_printf("S01 [GENESIS]: [IRQ-STUB] Unhandled vector 0x%02X\n", vector);
    }
}

void S01_Register_InterruptVector(void) {
    sigma_printf("S01 [GENESIS]: Sovereign Interrupt Vectoring Online.\n");
    // Default system timer handler simulation
    genesis_register_handler(0x20, (interrupt_handler_t)0xDEADBEEF); 
}
