#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Nuclear Engineering Shard (S-NUKE)
 * Purpose: Professional environment for nuclear engineers and reactor safety specialists.
 * Features: Neutron-flux lattice, decay-heat calculator, SCRAM-failsafe state machine.
 */

namespace SigmaOS {
namespace Kernel {
namespace Industrial {

// --- Reactor State Machine ---
enum class ReactorState : sigma_u8 {
    OFFLINE      = 0,
    COLD_STANDBY = 1,
    CRITICAL     = 2,  // Self-sustaining chain reaction
    SCRAM        = 3,  // Emergency shutdown
};

struct NeutronFlux {
    sigma_u64 fission_count;
    sigma_u32 neutrons_per_sec;
    sigma_u32 temp_kelvin;
    ReactorState state;
};

class SovereignNuke : public SigmaOS::SigmaObject {
public:
    static SovereignNuke& getInstance() {
        static SovereignNuke instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignNuke";
    }

    void init() {
        sigma_log_info("[S-NUKE] Initializing Nuclear Engineering Nexus...");
        flux_.fission_count = 0;
        flux_.neutrons_per_sec = 0;
        flux_.temp_kelvin = 293;
        flux_.state = ReactorState::OFFLINE;
    }

    void calculateDecayHeat(sigma_u32 power_mw, sigma_u32 seconds_shutdown) {
        // ANS-5.1 decay heat approximation: P_decay = 0.066 * P0 * t^(-0.2)
        // Using integer arithmetic for bare-metal; multiply by 1000 for milli-units
        sigma_u64 p0 = (sigma_u64)power_mw * 1000ULL;
        sigma_u64 factor = 66; // 0.066 * 1000
        // Approximate t^-0.2 for common shutdown times using lookup
        sigma_u32 decay_factor; // per-mille (1000 = 1.0)
        if (seconds_shutdown < 10)       decay_factor = 60;
        else if (seconds_shutdown < 100) decay_factor = 42;
        else if (seconds_shutdown < 1000) decay_factor = 30;
        else                              decay_factor = 20;

        sigma_u64 decay_mw = (p0 * factor * decay_factor) / (1000ULL * 1000ULL);
        sigma_log_info("[S-NUKE] Decay heat at t=%us: %llu kW (ANS-5.1 estimate)",
                       seconds_shutdown, decay_mw);
    }

    void triggerSCRAM(const char* reason) {
        flux_.state = ReactorState::SCRAM;
        sigma_log_err("[S-NUKE] SCRAM INITIATED: %s", reason);
        sigma_log_info("[S-NUKE] Control rods INSERTED. Moderator coolant ENGAGED.");
    }

    void assessFlux(sigma_u32 neutrons_per_sec, sigma_u32 temp_kelvin) {
        flux_.neutrons_per_sec = neutrons_per_sec;
        flux_.temp_kelvin = temp_kelvin;

        if (temp_kelvin > 1800) {
            triggerSCRAM("Core temperature exceeded 1800K safety threshold");
            return;
        }
        if (neutrons_per_sec > 1000000000U) {
            triggerSCRAM("Neutron flux exceeded prompt-critical threshold");
            return;
        }
        flux_.state = ReactorState::CRITICAL;
        sigma_log_info("[S-NUKE] Flux: %u n/s | Temp: %u K | State: CRITICAL (stable)",
                       neutrons_per_sec, temp_kelvin);
    }

private:
    SovereignNuke() = default;
    NeutronFlux flux_{};
};

} // namespace Industrial
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void nuke_init() {
    SigmaOS::Kernel::Industrial::SovereignNuke::getInstance().init();
}

void nuke_decay_heat(sigma_u32 power_mw, sigma_u32 seconds) {
    SigmaOS::Kernel::Industrial::SovereignNuke::getInstance().calculateDecayHeat(power_mw, seconds);
}

void nuke_scram(const char* reason) {
    SigmaOS::Kernel::Industrial::SovereignNuke::getInstance().triggerSCRAM(reason);
}

void nuke_assess_flux(sigma_u32 n_per_sec, sigma_u32 temp_k) {
    SigmaOS::Kernel::Industrial::SovereignNuke::getInstance().assessFlux(n_per_sec, temp_k);
}

} // extern "C"
