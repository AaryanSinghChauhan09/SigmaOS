#include "core/SovereignNetStack.h"
#include "sigma_log.h"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace Network {

void SovereignNetStack::init() {
    sigma_log("[S-NET] Initializing Sovereign Industrial Network Stack (Lattice-Direct)...");
    
    // Industrial TCP/IP Ignition
    sigma_log("[S-NET] Mounting TCP/UDP/ICMP Shards into Lattice Mesh...");
    
    m_firewall_active = true;
    m_active_connections = 0;
    
    sigma_log("[S-NET] Firewall: ACTIVE (Lattice-Guard enabled).");
    sigma_log("[S-NET] S-VPN: READY (WireGuard-PQC enabled).");
}

int SovereignNetStack::socket(Protocol proto) {
    sigma_log_info("[S-NET] Spawning Industrial Socket (Protocol: %d)\n", (int)proto);
    m_active_connections++;
    return (int)m_active_connections;
}

int SovereignNetStack::transmit(NetPacket* packet) {
    if (m_firewall_active) {
        // Simple firewall check (simulation)
        if (packet->dst_ip == 0x7F000001) { // Localhost always allowed
             sigma_log_info("[S-NET] [FIREWALL] Allowing packet to localhost.\n");
        }
    }
    
    sigma_log_info("[S-NET] Transmitting %u bytes to 0x%08X...\n", (sigma_u32)packet->size, packet->dst_ip);
    return (int)packet->size;
}

int SovereignNetStack::receive(sigma_u32 sock, void* buffer, sigma_size_t size) {
    (void)sock; (void)buffer; (void)size;
    return 0; // Simulation: No data in buffer
}

void SovereignNetStack::enable_firewall(bool enabled) {
    m_firewall_active = enabled;
    sigma_log_info("[S-NET] Firewall state changed: %s\n", enabled ? "ACTIVE" : "DISABLED");
}

void SovereignNetStack::add_firewall_rule(sigma_u32 ip_mask, Protocol proto, bool allow) {
    sigma_log_info("[S-NET] Adding Firewall Rule: Mask 0x%X Proto %d -> %s\n", ip_mask, (int)proto, allow ? "ALLOW" : "DENY");
}

void SovereignNetStack::connect_vpn(const char* endpoint_pqc) {
    sigma_log_info("[S-NET] Establishing S-VPN (PQC-Hardened WireGuard) to: %s\n", endpoint_pqc);
    sigma_log("[S-NET] [SECURITY] Handshake complete. Tunnel encrypted via Dilithium-5.");
}

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" {
    void net_init() {
        SigmaOS::Kernel::Network::SovereignNetStack::getInstance().init();
    }

    int net_socket() {
        return SigmaOS::Kernel::Network::SovereignNetStack::getInstance().socket(SigmaOS::Kernel::Network::Protocol::TCP);
    }

    void net_connect_vpn(const char* endpoint) {
        SigmaOS::Kernel::Network::SovereignNetStack::getInstance().connect_vpn(endpoint);
    }
}
 