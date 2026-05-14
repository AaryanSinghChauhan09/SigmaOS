#include "core/SigmaOOP.hpp"
#include "core/sigma_types.h"
#include "sigma_log.h"

/**
 * SIGMAOS: SovereignInstaller Deployment Shard
 * Mission: Adapt the Sovereign Lattice for SovereignInstaller environments.
 */

namespace SigmaOS {
namespace Kernel {
namespace Deployment {

class SovereignInstaller : public SigmaObject, public SigmaSingleton<SovereignInstaller> {
    friend class SigmaSingleton<SovereignInstaller>;
private:
    SovereignInstaller() {
        sigma_log_info("[DEPLOY] Initializing SovereignInstaller engine..." );
    }

public:
    void activate() {
        sigma_log_info("[DEPLOY] SovereignInstaller: Mode ACTIVE. Lattice adapted.");
    }
};

} // namespace
} // namespace Kernel
} // namespace SigmaOS

extern "C" void SovereignInstaller_activate() {
    SigmaOS::Kernel::Deployment::SovereignInstaller::getInstance().activate();
}
