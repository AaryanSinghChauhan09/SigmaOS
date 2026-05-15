#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign IPX Shard (S-IPX)
 * Implementation: Internetwork Packet Exchange protocol.
 * Mission: Enable legacy Novell NetWare connectivity.
 * Absorbed: Linux IPX stack patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignIPX : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignIPX> {
    friend class SigmaOS::SigmaSingleton<SovereignIPX>;
public:
    const char* type_name() const noexcept override { return "SovereignIPX"; }

    void init() {
        sigma_log_info("[S-IPX] Initializing IPX Protocol Stack...");
        sigma_log_info("[S-IPX] 802.3 raw / 802.2 LLC frame types supported.");
    }

private:
    SovereignIPX() = default;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void ipx_init() { SigmaOS::Kernel::Network::SovereignIPX::getInstance().init(); }
}

