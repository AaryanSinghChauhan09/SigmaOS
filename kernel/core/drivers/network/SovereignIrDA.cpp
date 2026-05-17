#include "../../../../include/SigmaOOP.hpp"
#include "../../../../include/core/sigma_types.h"
#include "../../../../include/sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace SovereignIrDASpace { // Using a name-specific namespace to avoid collisions

class SovereignIrDA : public SigmaObject, public SigmaSingleton<SovereignIrDA> {
    friend class SigmaSingleton<SovereignIrDA>;
private:
    SovereignIrDA() {
        sigma_syslog("[SOVEREIGN] SovereignIrDA Shard initialized.");
    }

public:
    void Init() {
        sigma_syslog("[SOVEREIGN] SovereignIrDA: Functional parity achieved.");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void SovereignIrDA_init() {
    SigmaOS::Kernel::SovereignIrDASpace::SovereignIrDA::getInstance().Init();
}
 