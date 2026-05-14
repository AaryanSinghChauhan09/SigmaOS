#include "core/SigmaOOP.hpp"
#include "core/sigma_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace SovereignSwapSpace {

class SovereignSwap : public SigmaObject, public SigmaSingleton<SovereignSwap> {
    friend class SigmaSingleton<SovereignSwap>;
private:
    SovereignSwap() {
        sigma_log_info("[SOVEREIGN] SovereignSwap Shard initialized.");
    }

public:
    void Init() {
        sigma_log_info("[SOVEREIGN] SovereignSwap: Monitoring/Active.");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void SovereignSwap_init() {
    SigmaOS::Kernel::SovereignSwapSpace::SovereignSwap::getInstance().Init();
}
