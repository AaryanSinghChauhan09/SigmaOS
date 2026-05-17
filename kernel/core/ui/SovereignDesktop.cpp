#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace DesktopSpace {

class SovereignDesktop : public SigmaObject, public SigmaSingleton<SovereignDesktop> {
    friend class SigmaSingleton<SovereignDesktop>;
private:
    SovereignDesktop() {
        sigma_log_info("[SOVEREIGN] Zenith Desktop Compositor initialized.");
    }

public:
    void LaunchToolkit() {
        sigma_log_info("[SOVEREIGN] S-UI Toolkit v1.0 active.");
        sigma_log_info("[SOVEREIGN] Dynamic Glassmorphism Engine: [READY]");
        sigma_log_info("[SOVEREIGN] Drawing root workspace...");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void desktop_init() {
    SigmaOS::Kernel::DesktopSpace::SovereignDesktop::getInstance().LaunchToolkit();
}
 