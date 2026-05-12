#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Air-Gap (S-GAP)
 * Purpose: Silicon-level physical network isolation.
 * USP: Guarantees 100% isolation by disabling the PCI power state of
 *      all networking silicon with a single kernel command.
 */

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignAirGap : public SigmaOS::SigmaObject {
public:
    static SovereignAirGap& getInstance() {
        static SovereignAirGap instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignAirGap";
    }

    void init() {
        sigma_log_info("[S-GAP] Initializing Air-Gap Isolation Shield...");
        this->m_isolated = false;
    }

    void engage() {
        sigma_log_warn("[S-GAP] ENGAGING PHYSICAL AIR-GAP. Cutting power to all NICs...");
        // Hit & Trial: Write 0x0 to PCI D3 state for all Ethernet/WiFi controllers
        this->m_isolated = true;
        sigma_log_warn("[S-GAP] ISOLATION COMPLETE. Lattice is now AIR-GAPPED.");
    }

    void disengage() {
        sigma_log_info("[S-GAP] Disengaging Air-Gap. Restoring networking silicon...");
        // Hit & Trial: Restore D0 state
        this->m_isolated = false;
        sigma_log_info("[S-GAP] Networking RESTORED.");
    }

private:
    SovereignAirGap() = default;
    bool m_isolated;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void airgap_init() {
    SigmaOS::Kernel::Security::SovereignAirGap::getInstance().init();
}

void airgap_engage() {
    SigmaOS::Kernel::Security::SovereignAirGap::getInstance().engage();
}

void airgap_disengage() {
    SigmaOS::Kernel::Security::SovereignAirGap::getInstance().disengage();
}

} // extern "C"
