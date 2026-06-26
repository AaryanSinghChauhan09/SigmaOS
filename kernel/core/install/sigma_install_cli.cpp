/**
 * ===========================================================================
 * Σ SIGMAOS: INSTALLER CLI (sigma-install)
 * ===========================================================================
 * Mission: Command-line wizard for bootstrapping SigmaOS onto bare metal.
 * ===========================================================================
 */

#include "../../../include/sigma_log.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

extern "C" void installer_init();
extern "C" bool installer_run_guided(const char* target_disk);
extern "C" bool installer_run_advanced(const char* target_disk, bool encrypt, bool lvm, const char* fs, bool ab_part, bool secure);

void print_banner() {
    printf("\n");
    printf("   _____ _                       ____  _____\n");
    printf("  / ____(_)                     / __ \\/ ____|\n");
    printf(" | (___  _  __ _ _ __ ___   ___| |  | | (___ \n");
    printf("  \\___ \\| |/ _` | '_ ` _ \\ / _ \\ |  | |\\___ \\\n");
    printf("  ____) | | (_| | | | | | |  __/ |__| |____) |\n");
    printf(" |_____/|_|\\__, |_| |_| |_|\\___|\\____/|_____/\n");
    printf("            __/ |                            \n");
    printf("           |___/  Bare-Metal Installer v1.0   \n");
    printf("\n");
}

int main(int argc, char** argv) {
    print_banner();
    installer_init();

    if (argc < 2) {
        printf("Usage: sigma_install_cli <mode> <target_disk> [options]\n");
        printf("Modes:\n");
        printf("  --guided     Use beginner-friendly defaults (A/B partitions, LUKS, Btrfs)\n");
        printf("  --advanced   Custom installation settings\n");
        return 1;
    }

    const char* mode = argv[1];
    
    if (strcmp(mode, "--guided") == 0) {
        if (argc < 3) {
            printf("Error: Missing target disk. Example: sigma_install_cli --guided /dev/nvme0n1\n");
            return 1;
        }
        const char* disk = argv[2];
        installer_run_guided(disk);
    } 
    else if (strcmp(mode, "--advanced") == 0) {
        // Simple mock parsing for advanced mode
        const char* disk = argc > 2 ? argv[2] : "/dev/sda";
        printf("Entering advanced configuration for %s...\n", disk);
        installer_run_advanced(disk, true, false, "zfs", true, true);
    }
    else {
        printf("Unknown mode: %s\n", mode);
        return 1;
    }

    return 0;
}
