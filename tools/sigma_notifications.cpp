/*
 * =========================================================================
 * Σ SIGMAOS: SIGMA NOTIFICATION CENTER (sigma_notifications) v1.0
 * =========================================================================
 * Mission: Unified alerts across shards.
 * Inspiration: macOS Notification Center + dunst.
 * Principle: Cross-shard event bus for UI-independent notifications.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {

class SigmaNotificationCenter : public SigmaObject, public SigmaSingleton<SigmaNotificationCenter> {
    friend class SigmaSingleton<SigmaNotificationCenter>;
public:
    const char* type_name() const noexcept override { return "SigmaNotificationCenter"; }

    void init() {
        m_unread_count = 0;
        sigma_log_info("[NOTIFY] Sigma Notification Center v1.0 initialized.");
    }

    void push_alert(const char* title, const char* message, sigma_u8 priority) {
        if (m_unread_count >= 128) return;
        m_unread_count++;
        const char* prio_str = (priority > 1) ? "CRITICAL" : (priority == 1) ? "HIGH" : "NORMAL";
        sigma_log_info("[NOTIFY] New Alert [%s]: %s - %s", prio_str, title, message);
    }

    void clear_all() {
        sigma_log_info("[NOTIFY] Cleared %u unread notifications.", m_unread_count);
        m_unread_count = 0;
    }

private:
    SigmaNotificationCenter() : m_unread_count(0) {}
    sigma_u32 m_unread_count;
};

} // namespace Tools
} // namespace SigmaOS

extern "C" {
void notify_init()                                                          { SigmaOS::Tools::SigmaNotificationCenter::getInstance().init(); }
void notify_push(const char* t, const char* m, sigma_u8 p)                  { SigmaOS::Tools::SigmaNotificationCenter::getInstance().push_alert(t, m, p); }
void notify_clear()                                                         { SigmaOS::Tools::SigmaNotificationCenter::getInstance().clear_all(); }
}
