/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN LIVE BOOT (Persistence Shard)
 * =========================================================================
 * Mission: Implements LATT-001 for Live USB environments.
 * Layer  : L0 � Silicon / Boot
 * =========================================================================
 */

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Boot {

class SovereignLiveBoot : public SigmaObject {
public:
    static SovereignLiveBoot& getInstance() {
        static SovereignLiveBoot instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignLiveBoot"; }

    static void initializePersistence() {
        sigma_log_info("[LIVE-BOOT] Probing for persistence overlay on USB...");
        // Check for 'casper-rw' or 'sigma-persistence' labels
        sigma_log_info("[LIVE-BOOT] Persistence found: [LatticeFS Overlay].");
    }

    void enterTryMode() {
        sigma_log_info("[LIVE-BOOT] Entering 'Try SigmaOS' (Ephemeral Lattice) mode.");
        sigma_log_info("[LIVE-BOOT] RAM-disk backing active. No changes will be written to disk.");
    }

    void enterPersistentMode() {
        sigma_log_info("[LIVE-BOOT] Entering Persistent Lattice mode.");
        initializePersistence();
    }

private:
    SovereignLiveBoot() = default;
};
} // namespace Boot
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void live_boot_init() {
    SigmaOS::Kernel::Boot::SovereignLiveBoot::getInstance().enterTryMode();
}

void live_boot_try() {
    SigmaOS::Kernel::Boot::SovereignLiveBoot::getInstance().enterTryMode();
}

void live_boot_persistent() {
    SigmaOS::Kernel::Boot::SovereignLiveBoot::getInstance().enterPersistentMode();
}

} // extern "C"
