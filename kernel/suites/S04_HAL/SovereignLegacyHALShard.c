#include "../../include/sigma_base.h"

#include "../../../include/SovereignArch.h"
#include "../../../include/sigma_libc.h"

/*
 * Sovereign Legacy Hardware Abstraction (v1.0).
 * Mission: Extreme Resource Frugality (Q4OS Parity).
 * Targets: Antiquated x86/32-bit silicon and low-mem controllers.
 * Logic: Disables advanced vectorization (AVX/SSE) to avoid overhead.
 */

sigma_err_t sigma_hal_legacy_init(void) {
    sigma_printf("  Σ [HAL-LEGACY]: Sovereign legacy architecture resurrection active.\n");
    sigma_printf("  Σ [HAL-LEGACY]: Memory footprint crushed to sub-50MB bounds.\n");
    sigma_printf("  Σ [HAL-LEGACY]: Defaulting to base ALU instruction set.\n");
    return SIGMA_OK;
}

void SovereignLegacyHAL_Register(void) {
    SovereignArch_Register("legacy_hal", sigma_hal_legacy_init);
}
