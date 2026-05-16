/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN PANIC HANDLER (v1.0)
 * =============================================================================
 * Principles: Industrial Logging & Post-Mortem Diagnostics.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

extern void vga_clear(sigma_u8 color);
extern void kprintf(const char* fmt, ...);
extern void serial_puts(const char* s);

void sigma_panic(const char* reason, sigma_u64 error_code, sigma_u64 rip) {
    /* Halt Interrupts */
    __asm__ __volatile__ ("cli");

    /* Visual Alarm (Industrial Red) */
    vga_clear(0x40); 

    kprintf("\n\nÎ£ [PANIC]: CRITICAL ARCHITECTURAL FAILURE\n");
    kprintf("REASON: %s\n", reason);
    kprintf("ERROR: 0x%x | RIP: 0x%x\n", error_code, rip);
    kprintf("Î£ [DIAG]: LOGGING TO SOVEREIGN REMOTE AGENT...\n");

    /* Log to Serial UART for external diagnostics */
    serial_puts("Î£ [PANIC]: ");
    serial_puts(reason);
    serial_puts("\n");

    /* Infinite Halt */
    for(;;) { __asm__ __volatile__ ("hlt"); }
}
