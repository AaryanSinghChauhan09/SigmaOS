/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM MONITORING (S-MONITOR)
 * =========================================================================
 * Mission: Real-time silicon performance and load balancing orchestration.
 * =========================================================================
 */

#ifndef SIGMA_MONITOR_H
#define SIGMA_MONITOR_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t cpu_utilization;
    uint32_t memory_pressure;
    uint32_t network_throughput;
    uint32_t shard_migration_rate;
} sigma_system_load_t;

/* --- Monitor Primitives --- */
void monitor_init(void);
sigma_system_load_t monitor_get_load_matrix(void);
void monitor_rebalance_lattice(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MONITOR_H */
