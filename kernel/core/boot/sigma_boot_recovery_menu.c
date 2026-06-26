/*
 * SigmaOS bootloader / safe-mode "Fix it" recovery menu
 */
#include "../../../include/sigma_kernel_types.h"

extern void sigma_puts(const char* s);

void sigma_boot_show_fix_it_menu(void) {
    sigma_puts("\n=== SigmaOS Fix It Menu ===\n");
    sigma_puts("  1) Boot Minimal (SIGMA_MINIMAL_MODE)\n");
    sigma_puts("  2) Roll back last kernel update\n");
    sigma_puts("  3) Reload network driver\n");
    sigma_puts("  4) Open rescue shell\n");
    sigma_puts("  5) Reboot\n");
    sigma_puts("Select option via serial console (default: 4)\n");
}
