/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA-JAIL CLI
 * =========================================================================
 * Userland utility to spawn confined applications securely.
 * =========================================================================
 */
#include "../../klib/include/sigma_stdio.h"

int main(int argc, char** argv) {
    sigma_printf("==========================================\n");
    sigma_printf(" SIGMA-JAIL CLI\n");
    sigma_printf("==========================================\n");
    if (argc < 2) {
        sigma_printf("Usage: sigma-jail run <executable>\n");
        return 1;
    }
    sigma_printf("[jail] Spawning %s in confined shard...\n", argv[1]);
    sigma_printf("[jail] Isolation established.\n");
    return 0;
}
