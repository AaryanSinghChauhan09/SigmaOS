#ifndef SIGMA_MONITOR_H
#define SIGMA_MONITOR_H
#include "sigma_types.h"
#ifdef __cplusplus
extern "C" {
#endif
typedef struct { sigma_u32 cpu_usage; sigma_u32 mem_usage; } sigma_system_load_t;
#ifdef __cplusplus
}
#endif
#endif
