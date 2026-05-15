#ifndef SIGMA_MONITOR_H
#define SIGMA_MONITOR_H

#include "include/sigma_types.h"

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

class SovereignObservabilityMonitor {
public:
    static SovereignObservabilityMonitor& getInstance();

    const char* type_name() const noexcept;

    void init();
    sigma_system_load_t getLoadMatrix();
    void executeEbpfProgram(const void* bytecode, sigma_usize size);
    void rebalanceLattice();

    virtual ~SovereignObservabilityMonitor() {}

private:
    SovereignObservabilityMonitor() : m_initialized(false) {}
    bool m_initialized;
};

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS
#endif /* __cplusplus */

#ifdef __cplusplus
extern "C" {
#endif

void monitor_init(void);
sigma_system_load_t monitor_get_load_matrix(void);
void monitor_rebalance_lattice(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MONITOR_H */
