/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN WATCHDOG (S-WATCH)
 * =========================================================================
 * Mission: Hardware + software heartbeat for kernel liveness guarantee.
 * Competitor parity: Linux watchdog subsystem / Windows WHEA / macOS Panic.
 * ZERO-DEPENDENCY: Direct HPET / PIT timer register orchestration.
 * =========================================================================
 */

#ifndef SIGMA_WATCHDOG_H
#define SIGMA_WATCHDOG_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Watchdog Timeout Constants --- */
#define SIGMA_WDT_DEFAULT_TIMEOUT_MS  5000u  /* 5 s  */
#define SIGMA_WDT_MIN_TIMEOUT_MS       500u  /* 0.5 s */
#define SIGMA_WDT_MAX_TIMEOUT_MS     60000u  /* 60 s  */

/* --- Watchdog Action on Expiry --- */
#define SIGMA_WDT_ACTION_REBOOT      0x00u
#define SIGMA_WDT_ACTION_PANIC_DUMP  0x01u
#define SIGMA_WDT_ACTION_SHARD_HEAL  0x02u  /* SigmaOS-exclusive: trigger SHSR */

typedef struct {
    sigma_u32 timeout_ms;
    sigma_u32 action;         /* SIGMA_WDT_ACTION_* */
    sigma_u32 kick_count;     /* times watchdog was kicked alive */
    sigma_u32 expired_count;  /* times watchdog expired (for diagnostics) */
    sigma_u32 enabled;
} sigma_wdt_state_t;

/* --- Watchdog Primitives --- */
void wdt_init(sigma_u32 timeout_ms, sigma_u32 action);
void wdt_enable(void);
void wdt_disable(void);
void wdt_kick(void);  /* Feed / pet the watchdog */
sigma_u32 wdt_is_enabled(void);
const sigma_wdt_state_t* wdt_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_WATCHDOG_H */
