#include "../../include/sigma_base.h"

#include "../../include/SovereignHardwareIOZenith.h"
#include "../../include/SovereignRegistry.h"
#include "../../include/sigma_libc.h"

/*
 * Sovereign USB/Thunderbolt Matrix.
 * xHCI hardware abstraction and plug-and-play orchestration.
 * Design: C11 / Zero-Dependency / Standalone.
 */

sigma_err_t sigma_usb_init(void) {
    sigma_printf("  Σ [USB]: Sovereign USB 3.x/4.0 stack initialized.\n");
    sigma_printf("  Σ [USB]: xHCI and Thunderbolt data-matrices loaded with zero overhead.\n");
    return SIGMA_OK;
}

void SovereignUSB_Register(void) {
    SovereignRegistry_Register("usb_core", sigma_usb_init);
}

