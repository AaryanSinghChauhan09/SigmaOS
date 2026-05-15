#ifndef SIGMA_MONITOR_H
#define SIGMA_MONITOR_H

#include "../include/core/sigma_types.h"

typedef struct {
    sigma_u32 cpu_utilization;
    sigma_u32 memory_pressure;
    sigma_u32 network_throughput;
    sigma_u32 shard_migration_rate;
} sigma_system_load_t;

#endif
