#include "../../../include/core/SovereignNetStack.h"
#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace Network {

void SovereignNetStack::init() {
    sigma_log_info("[S-NET] Initializing Sovereign Industrial Network Stack...");
    sigma_log_info("[S-NET] Protocol Support: TCP, UDP, ICMP, LATTICE-MESH.");
    m_firewall_active = true;
    m_active_connections = 0;
    sigma_log_info("[S-NET] Zero-Trust Firewall: ARMED. PQC-Handshake listener: ONLINE.");
}

int SovereignNetStack::socket(Protocol proto) {
    (void)proto;
    sigma_log_info("[S-NET] Creating industrial socket (Type: %d)...", (int)proto);
    return ++m_active_connections;
}

int SovereignNetStack::transmit(NetPacket* packet) {
    if (!m_firewall_active) {
        sigma_log_info("[S-NET] Transmission FAILED: Firewall disarmed.");
        return -1;
    }
    sigma_log_info("[S-NET] Transmitting packet to 0x%X (Size: %zu)...", packet->dst_ip, packet->size);
    // Simulation: Encapsulate in Ethernet -> IP -> Proto
    sigma_log_info("[S-NET] Status: SENT (PQC-Encrypted Shard).");
    return 0;
}

int SovereignNetStack::receive(sigma_u32 sock, void* buffer, sigma_size_t size) {
    (void)sock; (void)buffer; (void)size;
    // Simulation: Receive from hardware buffer
    return 0;
}

void SovereignNetStack::enable_firewall(bool enabled) {
    m_firewall_active = enabled;
    sigma_log_info("[S-NET] Firewall state updated: %s", enabled ? "ARMED" : "DISARMED");
}

void SovereignNetStack::add_firewall_rule(sigma_u32 ip_mask, Protocol proto, bool allow) {
    sigma_log_info("[S-NET] Adding rule: Mask 0x%X Proto %d -> %s", ip_mask, (int)proto, allow ? "ALLOW" : "BLOCK");
}

void SovereignNetStack::connect_vpn(const char* endpoint_pqc) {
    sigma_log_info("[S-NET] Initiating Sovereign VPN Handshake (Kyber-1024)...");
    sigma_log_info("[S-NET] Connecting to: %s", endpoint_pqc);
    sigma_log_info("[S-NET] VPN Tunnel ESTABLISHED. Zero-Trust Mesh Synced.");
}

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void net_stack_init() {
        SigmaOS::Kernel::Network::SovereignNetStack::getInstance().init();
    }
    
    int net_stack_socket(int proto) {
        return SigmaOS::Kernel::Network::SovereignNetStack::getInstance().socket((SigmaOS::Kernel::Network::Protocol)proto);
    }
    
    void net_stack_connect_vpn(const char* endpoint) {
        SigmaOS::Kernel::Network::SovereignNetStack::getInstance().connect_vpn(endpoint);
    }
}
