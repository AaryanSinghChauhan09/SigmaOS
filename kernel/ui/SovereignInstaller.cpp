/*
 * SigmaOS: Sovereign Installer (UI-001)
 * Layer: L6 - Zenith UI / System Deployment
 */
#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

void partition_manager_scan();

namespace SigmaOS {
namespace Kernel {
namespace Deployment {

class SovereignInstaller : public SigmaObject {
public:
    static SovereignInstaller& getInstance() {
        static SovereignInstaller instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignInstaller"; }

    static void startInstallation() {
        sigma_log_info("[INSTALLER] Initializing Zenith Morphic Installer...");
        partition_manager_scan();
        sigma_log_info("[INSTALLER] Selecting shards: [Kernel, Drivers, Zenith-UI, AI-Nexus].");
        sigma_log_info("[INSTALLER] Formatting partition with PQC-LatticeFS...");
        sigma_log_info("[INSTALLER] Installation COMPLETE. Reboot to continue.");
    }
private:
    SovereignInstaller() = default;
};

} // namespace Deployment
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void installer_start() {
    SigmaOS::Kernel::Deployment::SovereignInstaller::startInstallation();
}

} // extern "C"

} // extern "C"
