// =============================================================================
// SigmaOS  kernel/core/security  SovereignWatchdog.cpp  v2.0
// Hardware Watchdog + Atomic Rollback on kernel deadlock
// =============================================================================
#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/core/SigmaOOP.hpp"

/* Forward declaration of rollback (defined in SovereignRollbackShard.cpp) */
extern "C" void rollback_execute(void);

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignWatchdog
    : public SigmaOS::SigmaObject
    , public SigmaOS::SigmaSingleton<SovereignWatchdog>
{
    friend class SigmaOS::SigmaSingleton<SovereignWatchdog>;
public:
    const char* type_name() const noexcept override { return "SovereignWatchdog"; }

    void init(sigma_u32 timeout_ms = 5000) {
        m_timeout_ms = timeout_ms;
        m_counter    = 0;
        m_triggered  = SIGMA_FALSE;
        sigma_log_info("[WATCHDOG] Sovereign Industrial Watchdog v2.0 initialized.");
        sigma_log_info("[WATCHDOG] Timeout: %u ms | Fallback: ATOMIC ROLLBACK\n", m_timeout_ms);
        sigma_log_info("[WATCHDOG] Heartbeat monitoring ACTIVE.");
    }

    /* Called periodically by the scheduler - resets the counter */
    void feed() {
        m_counter = 0;
        sigma_log_info("[WATCHDOG] Heartbeat OK.");
    }

    /* Called on each timer tick - increments counter, triggers on expiry */
    void tick(sigma_u32 elapsed_ms) {
        if (m_triggered) return;
        m_counter += elapsed_ms;
        if (m_counter >= m_timeout_ms) {
            onTimeout();
        }
    }

    bool isTriggered() const { return m_triggered != SIGMA_FALSE; }

private:
    sigma_u32 m_timeout_ms;
    sigma_u32 m_counter;
    sigma_bool m_triggered;

    void onTimeout() {
        m_triggered = SIGMA_TRUE;
        sigma_log_err("[WATCHDOG CRITICAL] HEARTBEAT LOST - kernel deadlock suspected!");
        sigma_log_info("[WATCHDOG CRITICAL] Initiating Sovereign Atomic Rollback...");
        rollback_execute();
        sigma_log_info("[WATCHDOG CRITICAL] Rollback complete. Resuming sovereign execution.");
        m_triggered = SIGMA_FALSE;
        m_counter   = 0;
    }
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" {
    void watchdog_init(sigma_u32 timeout_ms) {
        SigmaOS::Kernel::Security::SovereignWatchdog::getInstance().init(timeout_ms);
    }
    void watchdog_feed() {
        SigmaOS::Kernel::Security::SovereignWatchdog::getInstance().feed();
    }
    void watchdog_tick(sigma_u32 elapsed_ms) {
        SigmaOS::Kernel::Security::SovereignWatchdog::getInstance().tick(elapsed_ms);
    }
    int watchdog_triggered() {
        return SigmaOS::Kernel::Security::SovereignWatchdog::getInstance().isTriggered() ? 1 : 0;
    }
}
