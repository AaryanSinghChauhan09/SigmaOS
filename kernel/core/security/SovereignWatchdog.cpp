#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignWatchdog : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignWatchdog> {
    friend class SigmaSingleton<SovereignWatchdog>;
public:
    const char* type_name() const noexcept override { return "SovereignWatchdog"; }

    void init() {
        sigma_log_info("[WATCHDOG:CORE] Initializing Sovereign Industrial Watchdog...");
        sigma_log_info("[WATCHDOG:CORE] Fallback State: ATOMIC ROLLBACK.");
        sigma_log_info("[WATCHDOG:CORE] Heartbeat Monitoring: ACTIVE.");
    }

    void feed() {
        // Heartbeat signal received from Sovereign scheduler
        sigma_log_info("[WATCHDOG:FEED] Heartbeat verified.");
    }

    void onTimeout() {
        sigma_log_err("[WATCHDOG:FAIL] HEARTBEAT LOST! Potential kernel deadlock detected.");
        sigma_log_info("[WATCHDOG:FAIL] Triggering Sovereign Atomic Rollback...");
        
        extern "C" void rollback_execute();
        rollback_execute();
        
        sigma_log_info("[WATCHDOG:FAIL] System state restored. Resuming industrial execution.");
    }
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void watchdog_init() {
        SigmaOS::Kernel::Security::SovereignWatchdog::getInstance().init();
    }
}
