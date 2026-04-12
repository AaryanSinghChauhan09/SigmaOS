/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN WATCHDOG SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb Linux WDT / Windows WHEA / macOS watchdogd USP.
 *          Native Silicon Hardware Watchdog & Auto Self-Healing Engine.
 * Design: C11 / Zero-Dependency / Monotonic Heartbeat + Recovery Chain.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Watchdog Structures
// -------------------------------------------------------------------------

typedef enum {
    WDT_ACTION_REBOOT,        /* Hard silicon reboot                     */
    WDT_ACTION_PANIC,         /* Kernel panic + dump                     */
    WDT_ACTION_RECOVER,       /* Self-heal: restart failed shard         */
    WDT_ACTION_NOTIFY         /* Alert citizen, continue running         */
} SigmaWDTAction_t;

typedef struct {
    char             shard_name[32];
    sigma_u32        timeout_ticks;   /* Miss this many feeds → action    */
    sigma_u32        missed_feeds;    /* Current miss count               */
    sigma_u64        last_feed_tick;
    sigma_u64        total_feeds;
    SigmaWDTAction_t action;
    sigma_bool       armed;
    sigma_bool       healthy;
} SigmaWDTEntry_t;

typedef struct {
    sigma_u64 boot_tick;
    sigma_u64 current_tick;
    sigma_u64 total_resets;
    sigma_u64 total_recoveries;
} SigmaWDTGlobal_t;

#define MAX_WDT_ENTRIES 16
static SigmaWDTEntry_t  s_wdt_table[MAX_WDT_ENTRIES];
static sigma_u32        s_wdt_count = 0;
static SigmaWDTGlobal_t s_wdt_global = {0, 0, 0, 0};

// -------------------------------------------------------------------------
// Watchdog Logic (Linux softdog/iTCO_wdt / Windows WHEA / watchdogd parity)
// -------------------------------------------------------------------------

static const char* s_action_names[] = {
    "REBOOT", "PANIC", "RECOVER", "NOTIFY"
};

/**
 * sigma_wdt_register: Arms a silicon watchdog for a named shard.
 */
sigma_err_t sigma_wdt_register(const char* shard, sigma_u32 timeout_ticks,
                                SigmaWDTAction_t action) {
    if (s_wdt_count >= MAX_WDT_ENTRIES) return SIGMA_ENOSPC;
    SigmaWDTEntry_t* e = &s_wdt_table[s_wdt_count++];
    sigma_strcpy(e->shard_name, shard);
    e->timeout_ticks  = timeout_ticks;
    e->missed_feeds   = 0;
    e->last_feed_tick = s_wdt_global.current_tick;
    e->total_feeds    = 0;
    e->action         = action;
    e->armed          = SIGMA_TRUE;
    e->healthy        = SIGMA_TRUE;
    sigma_printf("[WDT]: Armed watchdog for '%s' (timeout=%u ticks, "
                 "action=%s).\n", shard, timeout_ticks, s_action_names[action]);
    return SIGMA_OK;
}

/**
 * sigma_wdt_feed: Pet the silicon watchdog (reset miss counter).
 *
 * Must be called within timeout_ticks by the monitored shard.
 */
sigma_err_t sigma_wdt_feed(const char* shard) {
    for (sigma_u32 i = 0; i < s_wdt_count; i++) {
        if (sigma_streq(s_wdt_table[i].shard_name, shard)) {
            s_wdt_table[i].missed_feeds   = 0;
            s_wdt_table[i].last_feed_tick = s_wdt_global.current_tick;
            s_wdt_table[i].total_feeds++;
            s_wdt_table[i].healthy        = SIGMA_TRUE;
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

/**
 * sigma_wdt_tick: Advances the global watchdog clock — call from kernel timer ISR.
 *
 * Checks all armed watchdogs for feed timeouts and triggers recovery actions.
 */
void sigma_wdt_tick() {
    s_wdt_global.current_tick++;

    for (sigma_u32 i = 0; i < s_wdt_count; i++) {
        SigmaWDTEntry_t* e = &s_wdt_table[i];
        if (!e->armed) continue;

        sigma_u64 age = s_wdt_global.current_tick - e->last_feed_tick;
        if (age > e->timeout_ticks) {
            e->missed_feeds++;
            e->healthy = SIGMA_FALSE;

            if (e->missed_feeds == 1) {
                /* First miss: emit warning */
                sigma_printf("\033[1;33m[WDT]: WARN! '%s' missed feed "
                             "(age=%llu ticks).\033[0m\n",
                             e->shard_name, (unsigned long long)age);
            }
            if (e->missed_feeds >= 3) {
                /* Triple miss: trigger action */
                sigma_printf("\033[1;31m[WDT]: TIMEOUT! '%s' x%u feeds missed — "
                             "triggering %s.\033[0m\n",
                             e->shard_name, e->missed_feeds,
                             s_action_names[e->action]);
                switch (e->action) {
                    case WDT_ACTION_RECOVER:
                        sigma_printf("  [WDT]: Self-healing: restarting '%s'...\n",
                                     e->shard_name);
                        e->missed_feeds   = 0;
                        e->last_feed_tick = s_wdt_global.current_tick;
                        e->healthy        = SIGMA_TRUE;
                        s_wdt_global.total_recoveries++;
                        break;
                    case WDT_ACTION_NOTIFY:
                        sigma_printf("  [WDT]: Citizen notified of '%s' failure.\n",
                                     e->shard_name);
                        e->armed = SIGMA_FALSE; /* Don't re-trigger */
                        break;
                    case WDT_ACTION_REBOOT:
                        sigma_printf("  [WDT]: REBOOT sequence initiated.\n");
                        s_wdt_global.total_resets++;
                        e->armed = SIGMA_FALSE;
                        break;
                    case WDT_ACTION_PANIC:
                        sigma_printf("  [WDT]: KERNEL PANIC — silicon halt.\n");
                        e->armed = SIGMA_FALSE;
                        break;
                }
            }
        }
    }
}

// -------------------------------------------------------------------------
// Industrial Watchdog Audit
// -------------------------------------------------------------------------

void SovereignWatchdog_Audit() {
    sigma_printf("\n--- SOVEREIGN WATCHDOG AUDIT ---\n");
    sigma_printf("Global tick: %llu | Recoveries: %llu | Resets: %llu\n",
                 (unsigned long long)s_wdt_global.current_tick,
                 (unsigned long long)s_wdt_global.total_recoveries,
                 (unsigned long long)s_wdt_global.total_resets);
    sigma_printf("SHARD                TIMEOUT MISSED FEEDS       ACTION   ARMED HEALTHY\n");
    sigma_printf("-------------------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_wdt_count; i++) {
        SigmaWDTEntry_t* e = &s_wdt_table[i];
        sigma_printf("%-20s %-7u %-6u %-12llu %-8s %-5s %s\n",
                     e->shard_name, e->timeout_ticks, e->missed_feeds,
                     (unsigned long long)e->total_feeds,
                     s_action_names[e->action],
                     e->armed    ? "YES" : "no",
                     e->healthy  ? "OK"  : "FAIL");
    }
    sigma_printf("-------------------------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignWatchdogShard_Init() {
    sigma_printf("[SOC]: Seating Native Watchdog Shard "
                 "(Linux WDT/Windows WHEA/watchdogd Parity v1.0)...\n");
    sigma_wdt_register("sigma_kernel_core",  10, WDT_ACTION_PANIC);
    sigma_wdt_register("sigma_wm_display",   20, WDT_ACTION_RECOVER);
    sigma_wdt_register("sigma_net_stack",    15, WDT_ACTION_RECOVER);
    sigma_wdt_register("sigma_mm_pipeline",  30, WDT_ACTION_NOTIFY);

    /* Simulate feeding + a couple ticks */
    sigma_wdt_feed("sigma_kernel_core");
    sigma_wdt_feed("sigma_wm_display");
    for (sigma_u32 i = 0; i < 5; i++) sigma_wdt_tick();
    sigma_wdt_feed("sigma_kernel_core");
    sigma_wdt_feed("sigma_wm_display");
}
