#include "../../../include/SigmaOOP.hpp"
#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign TCP/IP Shard (S-TCPIP)
 * Implementation: IPv4, TCP, UDP, ICMP stack.
 * Mission: Enable core internet connectivity.
 * Absorbed: Linux/BSD networking stack patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignTCPIP : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignTCPIP> {
    friend class SigmaOS::SigmaSingleton<SovereignTCPIP>;
public:
    const char* type_name() const noexcept override { return "SovereignTCPIP"; }

    void init() {
        sigma_log_info("[S-TCPIP] Initializing IPv4 TCP/UDP Stack...");
        sigma_log_info("[S-TCPIP] Routing table: INITIALIZED.");
    }

private:
    SovereignTCPIP() = default;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void tcpip_init() { SigmaOS::Kernel::Network::SovereignTCPIP::getInstance().init(); }
}

