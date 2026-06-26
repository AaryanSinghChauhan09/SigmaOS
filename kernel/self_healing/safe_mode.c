/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SAFE MODE
 * =========================================================================
 * A rescue environment that boots when the main kernel initialization fails.
 * Minimal drivers, no graphical compositor, direct raw serial access.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_libc.h"

static void safe_mode_shell() {
    sys_print("\n");
    sys_print("==============================================\n");
    sys_print(" Σ SOVEREIGN SAFE MODE (Rescue Environment)\n");
    sys_print("==============================================\n");
    sys_print("Type 'help' for commands.\n");
    
    /* In a real implementation, this would poll the keyboard directly via port I/O */
    while (1) {
        sys_print("rescue> ");
        /* Simulated busy loop */
        for (volatile int i = 0; i < 10000000; i++) {}
        sys_print("reset\n");
        sys_print("Resetting configuration...\n");
        break;
    }
}

void load_safe_mode(void) {
    /* Step 1: Disable all complex interrupts */
    __asm__ __volatile__("cli");
    
    /* Step 2: Load basic VGA/Serial text drivers only */
    sys_print("[safe_mode] Loading minimal I/O drivers...\n");
    
    /* Step 3: Launch rescue shell */
    safe_mode_shell();
    
    /* Step 4: Halt if shell exits */
    while (1) {
        __asm__ __volatile__("hlt");
    }
}
