/* S SIGMAOS: SOVEREIGN CRON SHARD HEADER */
#ifndef SOVEREIGN_CRON_SHARD_H
#define SOVEREIGN_CRON_SHARD_H
#include "sigma_types.h"
typedef enum { TASK_ONESHOT, TASK_PERIODIC, TASK_CRON_EXPR } SigmaTaskType_t;
typedef sigma_err_t (*SigmaTaskFn_t)(void* ctx);
sigma_err_t sigma_cron_register (const char* name, SigmaTaskType_t type,
                                   SigmaTaskFn_t fn, void* ctx,
                                   sigma_u64 interval_ticks, sigma_u32 max_retries);
sigma_u32   sigma_cron_tick     (void);
sigma_err_t sigma_cron_enable   (const char* name);
sigma_err_t sigma_cron_disable  (const char* name);
void        SovereignCronShard_Init (void);
void        SovereignCron_Audit      (void);
#endif
