#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN PANIC HANDLER (v100.0)
 * =============================================================================
 * Principles: Industrial Logging & Post-Mortem Diagnostics.
 * =============================================================================
 */
#include "../../../include/core/sigma_kernel_types.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_log.h"

extern void vga_clear(sigma_u8 color);

void sigma_panic(const char* reason, sigma_u64 error_code, sigma_u64 rip) {
    /* Halt Interrupts */
    __asm__ __volatile__ ("cli");

    /* Visual Alarm (Industrial Red) */
    vga_clear(0x40); 

    sigma_log("\n\nΣ [PANIC]: CRITICAL ARCHITECTURAL FAILURE\n");
    sigma_log("REASON: %s\n", reason);
    sigma_log("ERROR: 0x%lx | RIP: 0x%lx\n", error_code, rip);
    sigma_log("Σ [DIAG]: LOGGING TO SOVEREIGN REMOTE AGENT...\n");

    /* Log to Sovereign Log Engine */
    log_emit(LOG_CRITICAL, reason);

    /* Infinite Halt */
    for(;;) { __asm__ __volatile__ ("hlt"); }
}
