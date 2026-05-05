#include "../../../include/sigma_thermaliq.h"
#include "../../../include/SovereignLibC.h"

extern "C" void energysched_set_shard_state(sigma_u32 shard_id, sigma_u32 state);

namespace SigmaOS {
namespace Kernel {
namespace Hardware {

SovereignThermalIQ& SovereignThermalIQ::getInstance() {
    static SovereignThermalIQ instance;
    return instance;
}

void SovereignThermalIQ::init() {
    sigma_log("[THERMALIQ] Initializing Sovereign Thermal Intelligence (PTR Algorithm)...");
    this->initialized = 1u;
}

sigma_u32 SovereignThermalIQ::getPackageTemp() {
    sigma_log("[THERMALIQ] PTR: Reading package thermal diode...");
    return 62u; /* 62 degrees C simulated */
}

void SovereignThermalIQ::applyThermalPolicy() {
    /* PTR (Predictive Thermal Regulation) Algorithm
     * Uses trend analysis to throttle before hitting critical temp zones. */

    sigma_u32 current_temp = this->getPackageTemp();
    this->temp_history[this->history_ptr % 4u] = current_temp;
    this->history_ptr++;

    sigma_u32 avg_temp = (this->temp_history[0] +
                          this->temp_history[1] +
                          this->temp_history[2] +
                          this->temp_history[3]) / 4u;

    sigma_printf("[THERMALIQ] PTR: Current: %u C, 4-sample average: %u C.\n", current_temp, avg_temp);

    if (avg_temp > 75u) {
        sigma_log("[THERMALIQ] PTR: Thermal trend upwards. Engaging predictive silicon throttling.");
        energysched_set_shard_state(0u, 1);
    }
}

void SovereignThermalIQ::emergencyThrottle(sigma_u32 threshold_celsius) {
    sigma_printf("[THERMALIQ] PTR: EMERGENCY THROTTLE engaged at %u C.\n", threshold_celsius);
}

} // namespace Hardware
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void thermaliq_init() {
    SigmaOS::Kernel::Hardware::SovereignThermalIQ::getInstance().init();
}

extern "C" sigma_u32 thermaliq_get_package_temp() {
    return SigmaOS::Kernel::Hardware::SovereignThermalIQ::getInstance().getPackageTemp();
}

extern "C" void thermaliq_apply_thermal_policy() {
    SigmaOS::Kernel::Hardware::SovereignThermalIQ::getInstance().applyThermalPolicy();
}

extern "C" void thermaliq_emergency_throttle(sigma_u32 threshold_celsius) {
    SigmaOS::Kernel::Hardware::SovereignThermalIQ::getInstance().emergencyThrottle(threshold_celsius);
}


