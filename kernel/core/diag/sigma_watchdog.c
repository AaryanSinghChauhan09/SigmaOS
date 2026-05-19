/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: HARDWARE WATCHDOG TIMER
 * =============================================================================
 * Inspired by: Linux kernel drivers/watchdog/watchdog_core.c
 *              Intel iTCO watchdog (iTCO_wdt.c)
 *              systemd watchdog daemon (sd_watchdog_enabled)
 * =============================================================================
 * Prevents system hangs by rebooting if the kernel fails to pet the watchdog
 * within a configured timeout period.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define WDT_STATE_DISABLED  0
#define WDT_STATE_RUNNING   1
#define WDT_STATE_EXPIRED   2

typedef struct {
    sigma_u32  timeout_sec;
    sigma_u32  remaining_sec;
    sigma_u32  state;
    sigma_u32  pet_count;
    sigma_u32  expiry_count;
    sigma_bool nowayout;      /* If true, watchdog cannot be stopped once started */
} sigma_watchdog_t;

static sigma_watchdog_t wdt;

void sigma_wdt_init(sigma_u32 timeout_sec) {
    sigma_memset(&wdt, 0, sizeof(wdt));
    wdt.timeout_sec   = timeout_sec;
    wdt.remaining_sec = timeout_sec;
    wdt.state         = WDT_STATE_DISABLED;
    wdt.nowayout      = SIGMA_FALSE;
    sigma_printf("[watchdog] Initialized (timeout=%u sec, nowayout=%s)\n",
                 timeout_sec, wdt.nowayout ? "yes" : "no");
}

int sigma_wdt_start(void) {
    if (wdt.state == WDT_STATE_RUNNING) {
        sigma_printf("[watchdog] Already running\n");
        return 0;
    }
    wdt.state         = WDT_STATE_RUNNING;
    wdt.remaining_sec = wdt.timeout_sec;
    sigma_printf("[watchdog] Started (timeout=%u sec)\n", wdt.timeout_sec);
    return 0;
}

int sigma_wdt_stop(void) {
    if (wdt.nowayout) {
        sigma_printf("[watchdog] ERR: Cannot stop — CONFIG_WATCHDOG_NOWAYOUT is set\n");
        return -1;
    }
    wdt.state = WDT_STATE_DISABLED;
    sigma_printf("[watchdog] Stopped\n");
    return 0;
}

void sigma_wdt_pet(void) {
    if (wdt.state != WDT_STATE_RUNNING) return;
    wdt.remaining_sec = wdt.timeout_sec;
    wdt.pet_count++;
    sigma_printf("[watchdog] Petted (reset to %u sec, total pets: %u)\n",
                 wdt.timeout_sec, wdt.pet_count);
}

void sigma_wdt_tick(void) {
    if (wdt.state != WDT_STATE_RUNNING) return;

    if (wdt.remaining_sec > 0) {
        wdt.remaining_sec--;
    }

    if (wdt.remaining_sec == 0) {
        wdt.state = WDT_STATE_EXPIRED;
        wdt.expiry_count++;
        sigma_printf("[watchdog] *** EXPIRED *** — system would reboot (count=%u)\n",
                     wdt.expiry_count);
        /* In real hardware: trigger system reset via MMIO/port write */
        wdt.remaining_sec = wdt.timeout_sec;
        wdt.state = WDT_STATE_RUNNING;
    }
}

void sigma_wdt_set_timeout(sigma_u32 timeout_sec) {
    wdt.timeout_sec = timeout_sec;
    if (wdt.state == WDT_STATE_RUNNING) {
        wdt.remaining_sec = timeout_sec;
    }
    sigma_printf("[watchdog] Timeout updated to %u sec\n", timeout_sec);
}

void sigma_wdt_set_nowayout(sigma_bool nowayout) {
    wdt.nowayout = nowayout;
    sigma_printf("[watchdog] nowayout=%s\n", nowayout ? "yes" : "no");
}

void sigma_wdt_status(void) {
    const char* state_str = "DISABLED";
    if (wdt.state == WDT_STATE_RUNNING) state_str = "RUNNING";
    else if (wdt.state == WDT_STATE_EXPIRED) state_str = "EXPIRED";

    sigma_printf("\n--- Σ WATCHDOG STATUS ---\n");
    sigma_printf("| State     : %s\n", state_str);
    sigma_printf("| Timeout   : %u sec\n", wdt.timeout_sec);
    sigma_printf("| Remaining : %u sec\n", wdt.remaining_sec);
    sigma_printf("| Pet Count : %u\n", wdt.pet_count);
    sigma_printf("| Expiries  : %u\n", wdt.expiry_count);
    sigma_printf("| Nowayout  : %s\n", wdt.nowayout ? "YES" : "NO");
    sigma_printf("-------------------------\n");
}
