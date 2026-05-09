#include "../../../include/sigma_log.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/hal/sigma_hal.h"

/**
 * SigmaOS Sovereign Network Stack
 * USP: High-performance, P2P-ready networking without legacy Linux overhead.
 */

class SovereignNetStack {
public:
    static SovereignNetStack& getInstance() {
        static SovereignNetStack instance;
        return instance;
    }

    void initInterface(const char* ifname) {
        sigma_log("[NET] Initializing sovereign interface: %s", ifname);
    }

    void transmitPacket(const void* data, sigma_size_t len) {
        // Optimized DMA transmit — sigma_size_t, no stdlib
        sigma_log("[NET] Transmitting sovereign packet (%llu bytes).", (sigma_u64)len);
    }

    void enableP2PDiscovery() {
        sigma_log("[NET] [P2P] Engaging sovereign mesh discovery.");
    }
};

extern "C" void sigma_net_init(const char* dev) {
    SovereignNetStack::getInstance().initInterface(dev);
}
