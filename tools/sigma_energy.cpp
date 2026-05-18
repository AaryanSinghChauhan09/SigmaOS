/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA ENERGY SAVER (sigma_energy) v1.0
 * =========================================================================
 * Mission: Intelligent power scaling for laptops/servers.
 * Inspiration: TLP + Intel P-state driver.
 * Principle: Telemetry-driven frequency scaling and device sleep.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

enum class PowerProfile : sigma_u8 {
    PERFORMANCE = 0,
    BALANCED    = 1,
    POWERSAVE   = 2,
    ULTRA_ECO   = 3,
};

class SigmaEnergySaver : public SigmaObject, public SigmaSingleton<SigmaEnergySaver> {
    friend class SigmaSingleton<SigmaEnergySaver>;
public:
    const char* type_name() const noexcept override { return "SigmaEnergySaver"; }

    void init() {
        m_profile = PowerProfile::BALANCED;
        m_battery_pct = 100;
        m_is_plugged_in = true;
        sigma_printf("[ENERGY] Sigma Energy Saver v1.0 initialized.");
        apply_profile(m_profile);
    }

    void set_battery_state(sigma_u8 pct, bool plugged_in) {
        m_battery_pct = pct;
        m_is_plugged_in = plugged_in;
        sigma_printf("[ENERGY] Battery: %u%% | %s", pct, plugged_in ? "AC Power" : "Battery");
        
        /* Auto-switch profiles based on power source */
        if (plugged_in && m_profile != PowerProfile::PERFORMANCE && m_profile != PowerProfile::BALANCED) {
            sigma_printf("[ENERGY] AC power detected. Auto-switching to BALANCED.");
            apply_profile(PowerProfile::BALANCED);
        } else if (!plugged_in && pct <= 20 && m_profile != PowerProfile::ULTRA_ECO) {
            sigma_printf("[ENERGY] Critical battery. Auto-switching to ULTRA_ECO.");
            apply_profile(PowerProfile::ULTRA_ECO);
        } else if (!plugged_in && pct > 20 && m_profile == PowerProfile::PERFORMANCE) {
            sigma_printf("[ENERGY] On battery. Auto-switching to POWERSAVE.");
            apply_profile(PowerProfile::POWERSAVE);
        }
    }

    void force_profile(PowerProfile p) {
        sigma_printf("[ENERGY] User override power profile.");
        apply_profile(p);
    }

    void report() const {
        const char* p_str = "UNKNOWN";
        switch (m_profile) {
            case PowerProfile::PERFORMANCE: p_str = "PERFORMANCE"; break;
            case PowerProfile::BALANCED:    p_str = "BALANCED";    break;
            case PowerProfile::POWERSAVE:   p_str = "POWERSAVE";   break;
            case PowerProfile::ULTRA_ECO:   p_str = "ULTRA_ECO";   break;
        }
        sigma_printf("[ENERGY] === Power Status ===");
        sigma_printf("[ENERGY] Profile : %s", p_str);
        sigma_printf("[ENERGY] Battery : %u%% (%s)", m_battery_pct, m_is_plugged_in ? "AC" : "DC");
    }

private:
    SigmaEnergySaver() : m_profile(PowerProfile::BALANCED), m_battery_pct(100), m_is_plugged_in(true) {}
    
    void apply_profile(PowerProfile p) {
        m_profile = p;
        /* Simulate CPU frequency scaling and device sleep toggles */
        switch (p) {
            case PowerProfile::PERFORMANCE:
                sigma_printf("[ENERGY] Applied: Max CPU freq, no auto-suspend.");
                break;
            case PowerProfile::BALANCED:
                sigma_printf("[ENERGY] Applied: Dynamic CPU freq, 5m screen timeout.");
                break;
            case PowerProfile::POWERSAVE:
                sigma_printf("[ENERGY] Applied: Capped CPU freq, aggressive PCI ASPM.");
                break;
            case PowerProfile::ULTRA_ECO:
                sigma_printf("[ENERGY] Applied: Min CPU freq, disable background tasks, dim display.");
                break;
        }
    }

    PowerProfile m_profile;
    sigma_u8     m_battery_pct;
    bool         m_is_plugged_in;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void energy_init()                                    { SigmaOS::Tools::SigmaEnergySaver::getInstance().init(); }
void energy_set_state(sigma_u8 pct, sigma_u8 plugged) { SigmaOS::Tools::SigmaEnergySaver::getInstance().set_battery_state(pct, plugged != 0); }
void energy_force_powersave()                         { SigmaOS::Tools::SigmaEnergySaver::getInstance().force_profile(SigmaOS::Tools::PowerProfile::POWERSAVE); }
void energy_report()                                  { SigmaOS::Tools::SigmaEnergySaver::getInstance().report(); }
}
