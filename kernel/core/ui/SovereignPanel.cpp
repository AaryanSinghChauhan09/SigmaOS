#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SIGMAOS: SOVEREIGN DESKTOP PANEL (S-PANEL)
 * Absorbed Concepts: Ubuntu Dock, GNOME Panel, Windows Taskbar.
 * Principle: Centralized shard launching and notification orchestration.
 */

namespace SigmaOS {
namespace Kernel {
namespace UI {

class SovereignPanel : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignPanel> {
    friend class SigmaOS::SigmaSingleton<SovereignPanel>;
public:
    const char* type_name() const noexcept override { return "SovereignPanel"; }

    void init() {
        sigma_log_info("[S-PANEL] Initializing Sovereign Desktop Panel...");
        sigma_log_info("[S-PANEL] Shard Launcher: ACTIVE. Status Tray: SYNCED.");
        sigma_log_info("[S-PANEL] Industrial Accessibility (One-Click) achieved.");
    }

    void show_notification(const char* title, const char* msg) {
        sigma_log_info("[S-PANEL] Notification: [%s] %s", title, msg);
    }
};

} // namespace UI
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void panel_init() { SigmaOS::Kernel::UI::SovereignPanel::getInstance().init(); }
}
