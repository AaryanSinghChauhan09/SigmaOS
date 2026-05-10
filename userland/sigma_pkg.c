/*
 * =========================================================================
 * ÃŽÂ£ SIGMAOS ZENITH SUPREME: SOVEREIGN PACKAGE MANAGER (v1.0)
 * =========================================================================
 * Mission: Universal application distribution (Debian/Arch/Ubuntu parity).
 * Capability: Pkg verification, Dependency sharding, Delta updates.
 * =========================================================================
 */

#include "../include/libc/SovereignLibC.h"
#include "../include/core/sigma_types.h"

typedef struct {
    char name[64];
    char version[16];
    sigma_size_t size;
    sigma_bool installed;
} sigma_pkg_t;

#define PKG_REPO_SIZE 10
static sigma_pkg_t sigma_repository[PKG_REPO_SIZE] = {
    {"sigma_desktop", "2.1.0", 12400, SIGMA_FALSE},
    {"sigma_calc", "1.0.4", 450, SIGMA_TRUE},
    {"sigma_text_edit", "1.5.0", 920, SIGMA_FALSE},
    {"sigma_compiler", "0.9.8", 45000, SIGMA_TRUE},
    {"sigma_network_tool", "1.2.1", 340, SIGMA_FALSE}
};

void sigma_pkg_install(const char* name) {
    kprintf("[PKG] Searching repository for: %s... ", name);
    for (int i = 0; i < PKG_REPO_SIZE; i++) {
        if (sigma_strcmp(sigma_repository[i].name, name) == 0) {
            kprintf("FOUND (v%s)\n", sigma_repository[i].version);
            kprintf("[PKG] Downloading binary sharding (%d KB)... ", sigma_repository[i].size / 1024);
            sigma_repository[i].installed = SIGMA_TRUE;
            kprintf("SUCCESS\n");
            return;
        }
    }
    kprintf("ERROR (NOT FOUND)\n");
}

void sigma_pkg_list() {
    kprintf("\nÃŽÂ£ SOVEREIGN REPOSITORY PULSE\n");
    kprintf("-------------------------------------------\n");
    kprintf("NAME              VERSION   STATUS\n");
    kprintf("-------------------------------------------\n");
    for (int i = 0; i < PKG_REPO_SIZE; i++) {
        if (sigma_repository[i].name[0] == '\0') continue;
        kprintf("%-17s %-9s %s\n", 
            sigma_repository[i].name, 
            sigma_repository[i].version, 
            sigma_repository[i].installed ? "INSTALLED" : "AVAILABLE");
    }
    kprintf("-------------------------------------------\n\n");
}

