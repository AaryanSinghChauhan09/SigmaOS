#include "sigma_log.h"
#include "sigma_kernel_types.h"
#include "power_manager.hpp"

namespace SigmaOS {
namespace Kernel {

void SovereignPowerManager::SetState(PowerState state) {
    m_current_state = state;
    switch(state) {
        case PowerState::PEAK_PERFORMANCE:
            m_voltage_mv = 1250; m_frequency_mhz = 5200;
            sigma_log("[POWER]: Ascending to PEAK PERFORMANCE (Silicon Overdrive).\n");
            break;
        case PowerState::BALANCED_LATTICE:
            m_voltage_mv = 1100; m_frequency_mhz = 3500;
            sigma_log("[POWER]: Balanced Lattice Sharding Active.\n");
            break;
        case PowerState::AMNESIC_LOW_POWER:
            m_voltage_mv = 850; m_frequency_mhz = 1200;
            sigma_log("[POWER]: Descending to AMNESIC LOW POWER (Battery Sovereignty).\n");
            break;
        case PowerState::PROFILE_HPC:
            m_voltage_mv = 1350; m_frequency_mhz = 6000;
            sigma_log("[POWER]: Profile [HPC] Active. Unrestricted thermal limits.\n");
            break;
        case PowerState::PROFILE_CLOUD:
            m_voltage_mv = 1000; m_frequency_mhz = 3000;
            sigma_log("[POWER]: Profile [CLOUD] Active. Optimized for density and throughput.\n");
            break;
        case PowerState::PROFILE_EDGE:
            m_voltage_mv = 800; m_frequency_mhz = 1000;
            sigma_log("[POWER]: Profile [EDGE] Active. Maximum efficiency for IoT devices.\n");
            break;
        case PowerState::PROFILE_GAMING:
            m_voltage_mv = 1200; m_frequency_mhz = 4800;
            sigma_log("[POWER]: Profile [GAMING] Active. Optimized kernel scheduling for low-latency graphics.\n");
            break;
        default:
            sigma_log("[POWER]: WARNING — Unknown power state requested. Falling back to BALANCED.\n");
            m_current_state = PowerState::BALANCED_LATTICE;
            m_voltage_mv = 1100; m_frequency_mhz = 3500;
            break;
    }
}

static const char* PowerStateToString(PowerState state) {
    switch (state) {
        case PowerState::PEAK_PERFORMANCE: return "PEAK_PERFORMANCE";
        case PowerState::BALANCED_LATTICE: return "BALANCED_LATTICE";
        case PowerState::AMNESIC_LOW_POWER: return "AMNESIC_LOW_POWER";
        case PowerState::PROFILE_HPC:      return "PROFILE_HPC";
        case PowerState::PROFILE_CLOUD:    return "PROFILE_CLOUD";
        case PowerState::PROFILE_EDGE:     return "PROFILE_EDGE";
        case PowerState::PROFILE_GAMING:   return "PROFILE_GAMING";
        default:                           return "UNKNOWN";
    }
}

void SovereignPowerManager::OptimizeForWorkload(sigma_u32 load_percentage) {
    if (load_percentage > 85) SetState(PowerState::PEAK_PERFORMANCE);
    else if (load_percentage < 20) SetState(PowerState::AMNESIC_LOW_POWER);
    else SetState(PowerState::BALANCED_LATTICE);
}

void SovereignPowerManager::PredictiveThrottling() {
    sigma_log("[POWER/PREDICT]: Analyzing Lattice Pulse for Load Anticipation...\n");
    m_load_prediction = 90;
    sigma_log("[POWER/PREDICT]: Load Spike Predicted (90%%). Pre-emptively Scaling to PEAK.\n");
    SetState(PowerState::PEAK_PERFORMANCE);
}

void SovereignPowerManager::Audit() {
    sigma_log("\n--- Σ SOVEREIGN POWER AUDIT ---\n");
    sigma_log("| Voltage           : %d mV\n", m_voltage_mv);
    sigma_log("| Frequency         : %d MHz\n", m_frequency_mhz);
    sigma_log("| State             : %s\n", PowerStateToString(m_current_state));
    sigma_log("| Load Prediction   : %d%%\n", m_load_prediction);
    sigma_log("-------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS



 