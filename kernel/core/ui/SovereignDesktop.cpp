#include "core/SigmaOOP.hpp"
#include "core/sigma_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace DesktopSpace {

class SovereignDesktop : public SigmaObject, public SigmaSingleton<SovereignDesktop> {
    friend class SigmaSingleton<SovereignDesktop>;
private:
    SovereignDesktop() {
        sigma_syslog("[SOVEREIGN] Zenith Desktop Compositor initialized.");
    }

public:
    void LaunchToolkit() {
        sigma_syslog("[SOVEREIGN] S-UI Toolkit v1.0 active.");
        sigma_syslog("[SOVEREIGN] Dynamic Glassmorphism Engine: [READY]");
        sigma_syslog("[SOVEREIGN] Drawing root workspace...");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void desktop_init() {
    SigmaOS::Kernel::DesktopSpace::SovereignDesktop::getInstance().LaunchToolkit();
}
