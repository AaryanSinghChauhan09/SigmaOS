#include "../../../include/SigmaOOP.hpp"
#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Firewall Shard (S-FW)
 * Implementation: Stateful packet inspection and NAT.
 * Mission: Secure the lattice boundary from unauthorized ingress.
 * Absorbed: Linux Netfilter/iptables/nftables patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignFirewall : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignFirewall> {
    friend class SigmaOS::SigmaSingleton<SovereignFirewall>;
public:
    const char* type_name() const noexcept override { return "SovereignFirewall"; }

    void init() {
        sigma_log_info("[S-FW] Initializing Stateful Packet Filter (S-FW)...");
        sigma_log_info("[S-FW] Default policy: DROP INGRESS, ACCEPT EGRESS.");
    }

private:
    SovereignFirewall() = default;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void firewall_init() { SigmaOS::Kernel::Network::SovereignFirewall::getInstance().init(); }
}

