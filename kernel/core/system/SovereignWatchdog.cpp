#include "core/SigmaOOP.hpp"
#include "core/sigma_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace SovereignWatchdogSpace {

class SovereignWatchdog : public SigmaObject, public SigmaSingleton<SovereignWatchdog> {
    friend class SigmaSingleton<SovereignWatchdog>;
private:
    SovereignWatchdog() {
        sigma_log_info("[SOVEREIGN] SovereignWatchdog Shard initialized.");
    }

public:
    void Init() {
        sigma_log_info("[SOVEREIGN] SovereignWatchdog: Monitoring/Active.");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void SovereignWatchdog_init() {
    SigmaOS::Kernel::SovereignWatchdogSpace::SovereignWatchdog::getInstance().Init();
}
