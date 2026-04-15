/* S SIGMAOS: SOVEREIGN WATCHDOG SHARD HEADER */
#ifndef SOVEREIGN_WATCHDOG_SHARD_H
#define SOVEREIGN_WATCHDOG_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"
typedef enum { WDT_ACTION_REBOOT, WDT_ACTION_PANIC,
               WDT_ACTION_RECOVER, WDT_ACTION_NOTIFY } SigmaWDTAction_t;
sigma_err_t sigma_wdt_register (const char* shard, sigma_u32 timeout_ticks, SigmaWDTAction_t action);
sigma_err_t sigma_wdt_feed     (const char* shard);
void        sigma_wdt_tick     (void);
void        SovereignWatchdogShard_Init (void);
void        SovereignWatchdog_Audit      (void);
#endif
