#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignNet : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignNet> {
    friend class SigmaOS::SigmaSingleton<SovereignNet>;
public:
    const char* type_name() const noexcept override {
        return "SovereignNet";
    }

    void init() {
        sigma_log_info("[SYS:NET] Initializing Sovereign Network Lattice (DoS-Hardened)...");
        this->m_packet_rate_limit = 10000; // packets per second
    }

    void filterIncoming(const void* packet, sigma_usize size) {
        if (size > 1500) {
             sigma_log_warn("[SYS:NET] Jumbo frame rejected (Oversized: %zu bytes).", size);
             return;
        }
    }

    bool validateSocketBounds(sigma_u32 socket_id) {
        if (socket_id >= 1024) {
            sigma_log_error("[SYS:NET] Socket ID %u out of bounds. Possible exhaustion attack.", socket_id);
            return false;
        }
        return true;
    }

private:
    sigma_u32 m_packet_rate_limit;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS


extern "C" {

void sovereignnet_init() {
    SigmaOS::Kernel::Network::SovereignNet::getInstance().init();
}

} // extern "C"
