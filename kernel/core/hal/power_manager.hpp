#ifndef POWER_MANAGER_HPP
#define POWER_MANAGER_HPP

#include "sigma_kernel_types.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

enum class PowerState { PEAK_PERFORMANCE, BALANCED_LATTICE, AMNESIC_LOW_POWER, PROFILE_HPC, PROFILE_CLOUD, PROFILE_EDGE, PROFILE_GAMING };

/*
 * =========================================================================
 * SOVEREIGN POWER MANAGER (Energy-Aware Kernel)
 * =========================================================================
 * Industrial-grade silicon power orchestration. Dynamically adjusts 
 * voltage and frequency shards based on lattice throughput requirements.
 */
class SovereignPowerManager : public SigmaObject {
private:
    PowerState m_current_state;
    sigma_u32 m_voltage_mv;
    sigma_u32 m_frequency_mhz;
    sigma_u32 m_load_prediction; // AI-driven prediction shard

public:
    SovereignPowerManager() 
        : m_current_state(PowerState::BALANCED_LATTICE), m_voltage_mv(1100), m_frequency_mhz(3500) {}

    const char* type_name() const noexcept override { return "SovereignPowerManager"; }

    void SetState(PowerState state);
    void OptimizeForWorkload(sigma_u32 load_percentage);
    void PredictiveThrottling(); // AI-driven pre-emptive scaling
    void Audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif

 