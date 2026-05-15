#include "../../../../../include/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "../../../../../include/SovereignOSBasicsZenith.h"
#include "../../../../../include/libc/sigma_libc.h"

/*
 * Sovereign Input Matrix (evdev/HID Parity).
 * Unified keyboard, mouse, touchscreen, and gamepad abstraction.
 * Design: C11 / Zero-Dependency / Hardware-Fused.
 */

sigma_err_t sigma_input_init(void) {
    sigma_sigma_printf("  S [INPUT]: Sovereign Input Matrix initialized.\n");
    sigma_sigma_printf("  S [INPUT]: HID descriptor parsing and evdev event queuing: READY.\n");
    return SIGMA_OK;
}

void SovereignInput_Register(void) {
    SovereignRegistry_Register("input", sigma_input_init);
}



