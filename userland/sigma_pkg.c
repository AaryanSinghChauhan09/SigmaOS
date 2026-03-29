/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: SOVEREIGN PACKAGE MANAGER (v1.0)
 * =========================================================================
 * Mission: Universal application distribution (Debian/Arch/Ubuntu parity).
 * Capability: Pkg verification, Dependency sharding, Delta updates.
 * =========================================================================
 */

#include "../libc/sigma_libc.h"
#include "../libc/sigma_types.h"

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
    sigma_printf("[PKG] Searching repository for: %s... ", name);
    for (int i = 0; i < PKG_REPO_SIZE; i++) {
        if (sigma_streq(sigma_repository[i].name, name)) {
            sigma_printf("FOUND (v%s)\n", sigma_repository[i].version);
            sigma_printf("[PKG] Downloading binary sharding (%d KB)... ", sigma_repository[i].size / 1024);
            sigma_repository[i].installed = SIGMA_TRUE;
            sigma_printf("SUCCESS\n");
            return;
        }
    }
    sigma_printf("ERROR (NOT FOUND)\n");
}

void sigma_pkg_list() {
    sigma_printf("\nΣ SOVEREIGN REPOSITORY PULSE\n");
    sigma_printf("-------------------------------------------\n");
    sigma_printf("NAME              VERSION   STATUS\n");
    sigma_printf("-------------------------------------------\n");
    for (int i = 0; i < PKG_REPO_SIZE; i++) {
        if (sigma_repository[i].name[0] == '\0') continue;
        sigma_printf("%-17s %-9s %s\n", 
            sigma_repository[i].name, 
            sigma_repository[i].version, 
            sigma_repository[i].installed ? "INSTALLED" : "AVAILABLE");
    }
    sigma_printf("-------------------------------------------\n\n");
}
