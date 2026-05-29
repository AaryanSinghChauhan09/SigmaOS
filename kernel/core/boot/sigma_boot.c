/*
 * SigmaOS boot sequence coordinator — normal boot vs safe mode.
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_immutable_root.h"

extern int init_kernel(void);
extern void start_normal_boot(void);
extern void load_safe_mode(void);
extern void sigma_puts(const char* s);

/* Implemented in kernel/resilience */
extern int sigma_rollback_check_fallback(void);
extern void sigma_boot_show_fix_it_menu(void);
void boot_sequence(void) {
    sigma_immutable_root_init();
    if (sigma_rollback_check_fallback() != 0) {
        sigma_puts("Kernel init failed. Entering Safe Mode...\n");
        load_safe_mode();
        return;
    }

    if (init_kernel() != 0) {
        sigma_puts("Kernel init failed. Entering Safe Mode...\n");
        load_safe_mode();
        return;
    }

    start_normal_boot();
}

/* Safe mode: minimal drivers + rescue shell only */
void load_safe_mode(void) {
    sigma_puts("[BOOT] Safe Mode: minimal HAL + serial rescue shell.\n");
    sigma_boot_show_fix_it_menu();
    /* Hand off to resilient fallback entry when linked */
}
