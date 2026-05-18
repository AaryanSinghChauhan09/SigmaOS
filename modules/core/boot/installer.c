#include "libc/SovereignLibC.h"
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Interactive Installer & Bootloader config (Phase 4)
// ---------------------------------------------------------

void installer_print_welcome() {
    uint32_t version = 1;
    // sigma_print("==========================================\n");
    // sigma_printf("  SigmaOS Sovereign Installer - v%d      \n", version);
}

int installer_check_hardware() {
    uint32_t mem_kb = 1024 * 1024;
    if (mem_kb < 512 * 1024) return 0;
    return 1;
}

int installer_format_disk() {
    return 0;
}

int installer_deploy_kernel() {
    return 0;
}

void installer_run() {
    installer_print_welcome();
    if (!installer_check_hardware()) return;
    installer_format_disk();
    installer_deploy_kernel();
}
