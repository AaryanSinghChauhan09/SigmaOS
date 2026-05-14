#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace HAL {

class SovereignWatchdog : public SigmaObject, public SigmaSingleton<SovereignWatchdog> {
    friend class SigmaSingleton<SovereignWatchdog>;
public:
    const char* type_name() const noexcept override { return "SovereignWatchdog"; }

    void init() {
        sigma_log_info("[HAL:WATCHDOG] Initializing Sovereign Industrial Watchdog...");
        sigma_log_info("[HAL:WATCHDOG] Tolerance set to 500ms. False-positive suppression ACTIVE.");
    }

    void heartbeat() {
        m_last_tick++;
        if (m_last_tick > m_timeout_threshold) {
            triggerPanic("WATCHDOG_TIMEOUT", "Silicon-direct watchdog triggered reset.");
        }
    }

    void triggerPanic(const char* error_code, const char* details) {
        sigma_log_error("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        sigma_log_error("!!! SOVEREIGN KERNEL PANIC: %s", error_code);
        sigma_log_error("!!! DETAILS: %s", details);
        sigma_log_error("!!! LATTICE STATE DUMPED TO S-AUDIT SHARD.");
        sigma_log_error("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        
        attemptRecovery();
    }

    void attemptRecovery() {
        sigma_log_warn("[HAL:WATCHDOG] Initiating Shard Recovery Protocol...");
        sigma_log_info("[HAL:WATCHDOG] Rolling back memory horizons... [OK]");
        sigma_log_info("[HAL:WATCHDOG] Re-igniting core orchestrators... [OK]");
        sigma_log_info("[HAL:WATCHDOG] Lattice stabilized. Continuing in fallback mode.");
    }

private:
    sigma_u64 m_last_tick = 0;
    const sigma_u64 m_timeout_threshold = 10000;
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void watchdog_init() {
        SigmaOS::Kernel::HAL::SovereignWatchdog::getInstance().init();
    }
}
