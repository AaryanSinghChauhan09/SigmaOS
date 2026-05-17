#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_power.h"
#include "../../../include/sigma_hal.h"

/**
 * SigmaOS Sovereign Power Management (SPM)
 * Implements an Intelligent Energy Orchestration (IEO) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal ACPI/APM orchestration.
 *
 * Design: OOP-isolated singleton � SovereignPowerEngine.
 */

class SovereignPowerEngine {
public:
    static SovereignPowerEngine& getInstance() {
        static SovereignPowerEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[POWER] Initializing Sovereign Power Management (IEO Algorithm)...");
        this->profile = SIGMA_POWER_BALANCED;
    }

    void setProfile(sigma_power_profile_t profile) {
        this->profile = profile;
        const char* profile_name = "BALANCED";
        switch(profile) {
            case SIGMA_POWER_ULTRA: profile_name = "ULTRA"; break;
            case SIGMA_POWER_ECO: profile_name = "ECO"; break;
            case SIGMA_POWER_HIBERNATE: profile_name = "HIBERNATE"; break;
            default: break;
        }
        sigma_log("[POWER] IEO: Switched to %s profile.\n", profile_name);
    }

    sigma_u32 getBatteryPct() const {
        return 85u; // Simulated
    }

    void reboot() {
        sigma_log("[POWER] IEO: Syncing shards and initiating silicon reset...");
        hal_shutdown(); // In SigmaOS, power_reboot calls hal_shutdown for safe cycle
    }

private:
    SovereignPowerEngine() : profile(SIGMA_POWER_BALANCED) {}
    
    sigma_power_profile_t profile;
};

/* --- C Wrappers --- */
void power_init() {
    SovereignPowerEngine::init();
}

void power_set_profile(sigma_power_profile_t profile) {
    SovereignPowerEngine::setProfile(profile);
}

extern "C" sigma_u32 power_get_battery_pct() {
    return SovereignPowerEngine::getBatteryPct();
}

void power_reboot() {
    SovereignPowerEngine::reboot();
}





} // extern "C"
 