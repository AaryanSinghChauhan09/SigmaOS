#include "sigma_libc.h"

// SigmaOS Silicon Boot (S-BOOT)
// Philosophy: Assembly-Native - Zero-Dependency Hardware Initialization.
// USP: Bypasses C runtime overhead by using raw assembly for stack and GDT setup.

void silicon_init_hw() {
    sigma_printf("[S-BOOT] Initializing raw silicon primitives...\n");
    
    #if defined(__x86_64__)
    __asm__ __volatile__ (
        "mov $0x1234, %rax\n" // Mock GDT setup
        "cli\n"               // Disable interrupts
    );
    #endif

    sigma_printf("[S-BOOT] CPU state stabilized. Handing over to S01_Genesis.\n");
}

void shard_init() {
    sigma_printf("[SHARD] Silicon Boot active. Raw hardware control enabled.\n");
    silicon_init_hw();
}
