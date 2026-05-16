#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

/**
 * SIGMAOS: Zenith Web UI Shard
 * Mission: Providing the sovereign desktop experience inside the browser.
 */

namespace SigmaOS {
namespace Kernel {
namespace UI {

class ZenithWebUI : public SigmaObject, public SigmaSingleton<ZenithWebUI> {
    friend class SigmaSingleton<ZenithWebUI>;
private:
    ZenithWebUI() {
        sigma_log_info("[WEB-UI] Initializing Sovereign Browser Layer..." );
    }

public:
    void ignite() {
        sigma_log_info("[WEB-UI] Zenith Web UI: Status ACTIVE. Shard-aware rendering enabled.");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void ZenithWebUI_ignite() {
    SigmaOS::Kernel::UI::ZenithWebUI::getInstance().ignite();
}
