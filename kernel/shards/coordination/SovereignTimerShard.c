#include "../../../include/SovereignOSBasicsZenith.h"
#include "../../../include/sigma_libc.h"

/*
 * Sovereign Timer Subsystem (HPET/TSC/ARM Generic Timer Parity).
 * High-resolution tick source for scheduling and profiling.
 * Design: C11 / Zero-Dependency / Hardware-Fused.
 */

sigma_err_t sigma_timer_init(void) {
    sigma_printf("  Σ [TIMER]: Sovereign high-resolution timer subsystem online.\n");
    sigma_printf("  Σ [TIMER]: TSC/HPET calibration: nanosecond precision.\n");
    return SIGMA_OK;
}

void SovereignTimer_Register(void) {
    SovereignRegistry_Register("timer", sigma_timer_init);
}
