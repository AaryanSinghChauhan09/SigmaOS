/* =========================================================================
 * Σ SIGMAOS: SOVEREIGN POWER SHARD HEADER
 * ========================================================================= */
#ifndef SOVEREIGN_POWER_SHARD_H
#define SOVEREIGN_POWER_SHARD_H
#include "sigma_types.h"
typedef enum { POWER_PLAN_PERFORMANCE, POWER_PLAN_BALANCED,
               POWER_PLAN_POWER_SAVER, POWER_PLAN_ULTRA_LOW_LATENCY } SigmaPowerPlan_t;
void sigma_power_init_cpu    (sigma_u32 cpu_id);
void sigma_power_set_plan    (SigmaPowerPlan_t plan);
void sigma_power_auto_govern (void);
void SovereignPowerShard_Init(void);
void SovereignPower_Audit    (void);
#endif
