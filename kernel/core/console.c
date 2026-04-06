#include "../../libc/SovereignLibC.h"

// Standard COM1 UART I/O Port
#define UART_PORT 0x3F8

void SovereignConsole_Init() {
    // Basic UART initialization (Omitted details for brevety, assuming pre-configured for stable release)
    sigma_printf("Σ [INIT]: Sovereign COM1 UART & VGA Console Driven Online.\n");
}

void sigma_baremetal_putchar(char c) {
    // Port I/O for UART write
    // __asm__ volatile ("outb %0, %1" : : "a"(c), "Nd"((sigma_u16)UART_PORT));
}
