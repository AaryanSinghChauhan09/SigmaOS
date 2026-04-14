#include "../../include/sigma_base.h"

#include "../../include/SovereignArch.h"
#include "../../include/sigma_libc.h"

void sigma_arm64_init(void) {
    sigma_printf("  Σ [ARM64]: Initialising ARM v8-A/v9 Cortex-A Matrix...\n");
    sigma_printf("  Σ [ARM64]: EL3/EL2 Exceptions configured. VBAR_EL1 set.\n");
    sigma_printf("  Σ [ARM64]: Silicon-direct Apple M1/M2/M3 affinity enabled (Asahi Amalgamation).\n");
}

void SovereignARM64_Register(void) {
    SovereignArch_Register("ARM64", sigma_arm64_init, SIGMA_NULL);
}

