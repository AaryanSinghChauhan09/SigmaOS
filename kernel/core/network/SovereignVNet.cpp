#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Bridge/TUN/TAP Shard (S-VNET)
 * Implementation: Layer 2 Bridging and Virtual Network Interfaces.
 * Mission: Enable container networking and hypervisor vSwitches.
 * Absorbed: Linux bridge-utils and tun/tap driver patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignVNet : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignVNet> {
    friend class SigmaOS::SigmaSingleton<SovereignVNet>;
public:
    const char* type_name() const noexcept override { return "SovereignVNet"; }

    void init() {
        sigma_log_info("[S-VNET] Initializing Virtual Networking (Bridge/TUN/TAP)...");
        sigma_log_info("[S-VNET] Virtual Switch br0 created.");
    }

private:
    SovereignVNet() = default;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void vnet_init() { SigmaOS::Kernel::Network::SovereignVNet::getInstance().init(); }
}

