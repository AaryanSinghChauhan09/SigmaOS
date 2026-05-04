#ifndef SIGMA_MONITOR_H
#define SIGMA_MONITOR_H

#include "sigma_types.h"

typedef struct {
    sigma_u32 cpu_utilization;
    sigma_u32 memory_pressure;
    sigma_u32 network_throughput;
    sigma_u32 shard_migration_rate;
} sigma_system_load_t;

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Observability {

class SovereignMonitor {
public:
    static SovereignMonitor& getInstance();

    void init();
    sigma_system_load_t getLoadMatrix();
    void rebalanceLattice();

private:
    SovereignMonitor() : m_initialized(false) {}
    bool m_initialized;
};

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

void monitor_init(void);
sigma_system_load_t monitor_get_load_matrix(void);
void monitor_rebalance_lattice(void);

#ifdef __cplusplus
}
#endif

#endif
