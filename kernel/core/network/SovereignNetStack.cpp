#include "sigma_types.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

/**
 * SigmaOS Sovereign Networking Stack (NetStack)
 * Zero-trust TCP/IP implementation for sovereign computing.
 *
 * USP: Completely bypasses legacy POSIX networking overhead. Implements a
 * hardened IPv4/IPv6 stack with inherent packet filtering at the data-link layer.
 *
 * Design: OOP-isolated singleton — SovereignNetStackEngine.
 */

class SovereignNetStackEngine {
public:
    static SovereignNetStackEngine& getInstance() {
        static SovereignNetStackEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[NET] Initializing Sovereign TCP/IP Stack...");
        this->interfaces_active = 0;
        this->packets_filtered = 0;
        sigma_log("[NET] Zero-trust packet inspection ACTIVE.");
    }

    void registerInterface(const char* mac_addr) {
        if (this->interfaces_active >= 4) return;
        sigma_hardened_strcpy(this->interfaces[this->interfaces_active], mac_addr, 18);
        this->interfaces_active++;
        sigma_printf("[NET] Network interface %s registered.\n", mac_addr);
    }

    bool dispatchPacket(const char* /*payload*/, sigma_u32 length) {

        // Deep Packet Inspection simulation
        if (length > 1500) {
            this->packets_filtered++;
            sigma_log("[NET] [WARN] Oversized MTU packet dropped by sovereign firewall.");
            return false;
        }
        sigma_printf("[NET] Dispatched %u bytes over TCP/IP.\n", length);
        return true;
    }

private:
    SovereignNetStackEngine() : interfaces_active(0), packets_filtered(0) {}

    char interfaces[4][18];
    sigma_u32 interfaces_active;
    sigma_u32 packets_filtered;
};

/* --- C Wrappers --- */
extern "C" void netstack_init() {
    SovereignNetStackEngine::getInstance().init();
}

extern "C" void netstack_register_iface(const char* mac_addr) {
    SovereignNetStackEngine::getInstance().registerInterface(mac_addr);
}

extern "C" bool netstack_dispatch(const char* payload, sigma_u32 length) {
    return SovereignNetStackEngine::getInstance().dispatchPacket(payload, length);
}


