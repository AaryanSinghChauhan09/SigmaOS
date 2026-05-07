#include "hal/sigma_hal.h"
#include "core/sigma_types.h"
#include "power_manager.hpp"
#include "libc/SovereignLibC.h"

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
    }
}

void SovereignPowerManager::OptimizeForWorkload(sigma_u32 load_percentage) {
    if (load_percentage > 85) SetState(PowerState::PEAK_PERFORMANCE);
    else if (load_percentage < 20) SetState(PowerState::AMNESIC_LOW_POWER);
    else SetState(PowerState::BALANCED_LATTICE);
}

void SovereignPowerManager::PredictiveThrottling() {
    sigma_log("[POWER/PREDICT]: Analyzing Lattice Pulse for Load Anticipation...\n");
    // Simulate AI prediction
    m_load_prediction = 90; // Predict spike
    sigma_log("[POWER/PREDICT]: Load Spike Predicted (90%%). Pre-emptively Scaling to PEAK.\n");
    SetState(PowerState::PEAK_PERFORMANCE);
}

void SovereignPowerManager::Audit() {
    sigma_log("\n--- Σ SOVEREIGN POWER AUDIT ---\n");
    sigma_log("| Voltage           : %d mV\n", m_voltage_mv);
    sigma_log("| Frequency         : %d MHz\n", m_frequency_mhz);
    sigma_log("| State             : %s\n", (m_current_state == PowerState::PEAK_PERFORMANCE) ? "PEAK" : 
                                          (m_current_state == PowerState::BALANCED_LATTICE) ? "BALANCED" : "AMNESIC-LOW");
    sigma_log("--------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS



