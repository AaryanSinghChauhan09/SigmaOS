#ifndef SOVEREIGN_NET_STACK_H
#define SOVEREIGN_NET_STACK_H

#include "./sigma_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Network {

enum class Protocol {
    TCP,
    UDP,
    ICMP,
    LATTICE_MESH
};

struct NetPacket {
    sigma_u32 src_ip;
    sigma_u32 dst_ip;
    sigma_u16 src_port;
    sigma_u16 dst_port;
    Protocol protocol;
    void* payload;
    sigma_size_t size;
};

class SovereignNetStack : public SigmaObject, public SigmaSingleton<SovereignNetStack> {
public:
    void init();
    
    // Core Stack
    int socket(Protocol proto);
    int transmit(NetPacket* packet);
    int receive(sigma_u32 sock, void* buffer, sigma_size_t size);
    
    // Industrial Features
    void enable_firewall(bool enabled);
    void add_firewall_rule(sigma_u32 ip_mask, Protocol proto, bool allow);
    void connect_vpn(const char* endpoint_pqc);

    virtual const char* type_name() const noexcept override { return "SovereignNetStack"; }

private:
    friend class SigmaSingleton<SovereignNetStack>;
    SovereignNetStack() = default;
    
    bool m_firewall_active;
    sigma_u32 m_active_connections;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void net_stack_init();
    int net_stack_socket(int proto);
    void net_stack_connect_vpn(const char* endpoint);
}

#endif // SOVEREIGN_NET_STACK_H
