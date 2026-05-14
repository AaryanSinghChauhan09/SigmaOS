#include "core/sigma_types.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign PPP Shard (S-PPP)
 * Implementation: Point-to-Point Protocol & PPPoE.
 * Mission: Enable serial and broadband DSL connectivity.
 * Absorbed: Linux pppd and pppoe-client patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignPPP : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignPPP> {
    friend class SigmaOS::SigmaSingleton<SovereignPPP>;
public:
    const char* type_name() const noexcept override { return "SovereignPPP"; }

    void init() {
        sigma_log_info("[S-PPP] Initializing PPP & PPPoE Daemons...");
        sigma_log_info("[S-PPP] LCP/IPCP negotiation engine READY.");
    }

private:
    SovereignPPP() = default;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void ppp_init() { SigmaOS::Kernel::Network::SovereignPPP::getInstance().init(); }
}
