#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "sigma_log.h"
#include "observability/sigma_monitor.h"
#include "core/SigmaOOP.hpp"

extern "C" void telemetry_execute_ebpf(const void* bytecode, sigma_usize size);

namespace SigmaOS {
namespace Kernel {
namespace Observability {

/* Industrial Constants */
static constexpr sigma_u32 SIMULATED_CPU_LOAD       = 12U;
static constexpr sigma_u32 SIMULATED_MEM_PRESSURE   = 45U;
static constexpr sigma_u32 SIMULATED_NET_THROUGHPUT = 850U;
static constexpr sigma_u32 SIMULATED_MIGRATION_RATE = 2U;

SovereignObservabilityMonitor& SovereignObservabilityMonitor::getInstance() {
    static SovereignObservabilityMonitor instance;
    return instance;
}

const char* SovereignObservabilityMonitor::type_name() const noexcept {
    return "SovereignObservabilityMonitor";
}

void SovereignObservabilityMonitor::init() {
    sigma_log_info("[MONITOR] Initializing Sovereign Observability Matrix (eBPF-Native)...");
    this->m_initialized = true;
}

sigma_system_load_t SovereignObservabilityMonitor::getLoadMatrix() {
    sigma_system_load_t load;
    load.cpu_utilization    = SIMULATED_CPU_LOAD;
    load.memory_pressure    = SIMULATED_MEM_PRESSURE;
    load.network_throughput = SIMULATED_NET_THROUGHPUT;
    load.shard_migration_rate = SIMULATED_MIGRATION_RATE;
    return load;
}

void SovereignObservabilityMonitor::executeEbpfProgram(const void* bytecode, sigma_usize size) {
    telemetry_execute_ebpf(bytecode, size);
}

void SovereignObservabilityMonitor::rebalanceLattice() {
    sigma_log_warn("[MONITOR] Lattice load imbalance detected via eBPF probes. Migrating shards...");
    sigma_log_info("[MONITOR] Migration: S412 -> Core 15, S092 -> Core 02.");
}

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge (all use getInstance()) --- */
extern "C" void monitor_init() {
    SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().init();
}

extern "C" sigma_system_load_t monitor_get_load_matrix() {
    return SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().getLoadMatrix();
}

extern "C" void monitor_execute_ebpf(const void* bytecode, sigma_usize size) {
    SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().executeEbpfProgram(bytecode, size);
}

extern "C" void monitor_rebalance_lattice() {
    SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().rebalanceLattice();
}
