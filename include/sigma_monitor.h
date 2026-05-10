#pragma once
/* sigma_monitor.h - Sovereign Monitor Shard API */
#include "core/sigma_types.h"
#ifdef __cplusplus
extern "C" {
#endif
void sigma_monitor_init(void);
void sigma_monitor_poll(void);
#ifdef __cplusplus
}
#endif
