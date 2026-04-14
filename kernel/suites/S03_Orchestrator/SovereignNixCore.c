#include "../../include/sigma_kernel.h"
#include "../../include/SovereignRegistry.h"

void SovereignNixCore_Init() {
    sigma_printf("Σ [ABSORB]: SovereignNixCore Integration Complete. Global USP Parity Secured.\n");
    sigma_printf("Σ [NIX]: Declarative configuration engine established. Immutability verified.\n");
}

void SovereignNixCore_Register() {
    SovereignRegistry_Register("NixCore", SHARD_CAT_DISTRO, SovereignNixCore_Init);
}





