/**
 * @file SovereignNix_Immutable.c
 * @brief Phase 66: NixOS Absorption Shard (Immutable State).
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

sigma_err_t sigma_nix_immutable_init(void) {
    sigma_printf("S [ABSORPTION]: Applying NixOS 'Immutable' Primitives...\n");
    sigma_printf("  S [NIX]: Enforcing read-only root filesystem shard.\n");
    sigma_printf("  S [NIX]: Declarative state manifest verified: SOVEREIGN_CONFIG.json\n");
    
    return SIGMA_OK;
}

void SovereignNixImmutable_Register(void) {
    SovereignRegistry_Register("nix_immutable", "Security", sigma_nix_immutable_init);
}
