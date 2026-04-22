/* S SIGMAOS: SOVEREIGN RTOS SHARD HEADER */
#ifndef SOVEREIGN_RTOS_SHARD_H
#define SOVEREIGN_RTOS_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_rtos_schedule (const char* name, sigma_u32 period, sigma_bool critical);
void        SovereignRTOSShard_Init (void);
void        SovereignRTOS_Audit     (void);

#endif
