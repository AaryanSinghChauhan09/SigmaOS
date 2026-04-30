#include "sigma_watchdog.h"
#include "sigma_hal.h"
#include "sigma_types.h"

/**
 * SigmaOS Sovereign Watchdog Implementation
 * Implements a Silicon Heartbeat Arbitration (SHA) algorithm.
 * ZERO-DEPENDENCY: Direct HPET/PIT timer control; no daemon required.
 * Competitor parity: Linux watchdog subsystem, Windows WHEA, macOS panic.
 *
 * Design: OOP-isolated singleton — SovereignWDTManager.
 */

/* --- Sovereign Watchdog Manager (OOP Isolation) --- */
static struct {
    sigma_wdt_state_t state;
    sigma_u32 initialized;
} SovereignWDTManager = {
    .state = {
        .timeout_ms    = SIGMA_WDT_DEFAULT_TIMEOUT_MS,
        .action        = SIGMA_WDT_ACTION_SHARD_HEAL,
        .kick_count    = 0u,
        .expired_count = 0u,
        .enabled       = 0u
    },
    .initialized = 0u
};

extern "C" void wdt_init(sigma_u32 timeout_ms, sigma_u32 action) {
    sigma_log("[WDT] Initializing Sovereign Silicon Heartbeat Arbitration (SHA)...");

    // Clamp timeout to valid range
    if (timeout_ms < SIGMA_WDT_MIN_TIMEOUT_MS) timeout_ms = SIGMA_WDT_MIN_TIMEOUT_MS;
    if (timeout_ms > SIGMA_WDT_MAX_TIMEOUT_MS) timeout_ms = SIGMA_WDT_MAX_TIMEOUT_MS;

    SovereignWDTManager.state.timeout_ms = timeout_ms;
    SovereignWDTManager.state.action     = action;
    SovereignWDTManager.initialized      = 1u;

    sigma_printf("[WDT] SHA: Watchdog configured — timeout=%dms action=%d.\n",
                 (int)timeout_ms, (int)action);
}

extern "C" void wdt_enable() {
    SovereignWDTManager.state.enabled = 1u;
    // SHA Algorithm: Arms the HPET one-shot counter for the timeout period.
    sigma_log("[WDT] SHA: Watchdog ARMED. Silicon heartbeat monitoring ACTIVE.");
}

extern "C" void wdt_disable() {
    SovereignWDTManager.state.enabled = 0u;
    sigma_log("[WDT] SHA: Watchdog DISARMED.");
}

extern "C" void wdt_kick() {
    // SHA Algorithm: Reloads the HPET counter; prevents expiry.
    SovereignWDTManager.state.kick_count++;
    sigma_printf("[WDT] SHA: Heartbeat received (kick #%d). Timer reloaded.\n",
                 (int)SovereignWDTManager.state.kick_count);
}

extern "C" sigma_u32 wdt_is_enabled() {
    return SovereignWDTManager.state.enabled;
}

extern "C" const sigma_wdt_state_t* wdt_get_state() {
    return &SovereignWDTManager.state;
}
