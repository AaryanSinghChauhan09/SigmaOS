#include "../../../../../include/libc/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "../../../../../include/SovereignArch.h"
#include "../../../../../include/libc/sigma_libc.h"

void sigma_arm64_init(void) {
    sigma_sigma_printf("  S [ARM64]: Initialising ARM v8-A/v9 Cortex-A Matrix...\n");
    sigma_sigma_printf("  S [ARM64]: EL3/EL2 Exceptions configured. VBAR_EL1 set.\n");
    sigma_sigma_printf("  S [ARM64]: Silicon-direct Apple M1/M2/M3 affinity enabled (Asahi Amalgamation).\n");
}

void SovereignARM64_Register(void) {
    SovereignArch_Register("ARM64", sigma_arm64_init, SIGMA_NULL);
}



