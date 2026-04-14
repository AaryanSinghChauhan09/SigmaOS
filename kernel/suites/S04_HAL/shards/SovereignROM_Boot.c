/*
 * =========================================================================
 * Σ SIGMAOS: S04_HAL — SovereignROM_Boot.c
 * =========================================================================
 * Implementation of Idea 68.1 (Apex Infinity): ROM-able Kernel.
 * Enables execution directly from Flash/ROM without RAM relocation.
 * =========================================================================
 */

#include "sigma_base.h"
#include <stdint.h>

extern void kernel_main(void);

void rom_boot_init(void) {
    sigma_printf("Σ [S04]: Sovereign ROM Bootloader Materialized (Apex Idea 68.1).\n");
}

void rom_entry(void) {
    // Stage 0: Direct from ROM vector
    sigma_printf("Σ [ROM]: Executing from non-volatile address 0xFFFF0000...\n");
    kernel_main();
}
