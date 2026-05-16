/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN NETWORK SHARD (S-NET)
 * =========================================================================
 * Mission: Zero-trust, PQC-hardened, and hot-swappable TCP/IP networking.
 * Absorbing ideas from Linux networking stack but isolated per-shard.
 * =========================================================================
 */

#ifndef SIGMA_NETWORK_H
#define SIGMA_NETWORK_H

#include "../core/sigma_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Net {

typedef sigma_u32 sigma_ipv4_t;
typedef sigma_u16 sigma_port_t;

struct NetworkPacket {
    sigma_u8* payload;
    sigma_size_t length;
    sigma_ipv4_t src_ip;
    sigma_ipv4_t dest_ip;
    sigma_port_t src_port;
    sigma_port_t dest_port;
    sigma_u8 protocol; // e.g., TCP, UDP, ICMP
};

class SovereignNetworkShard : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignNetworkShard> {
    friend class SigmaOS::SigmaSingleton<SovereignNetworkShard>;
public:
    const char* type_name() const noexcept override { return "SovereignNetworkShard"; }

    sigma_status init();
    sigma_status shutdown();
    
    // Abstracted Socket API
    sigma_status socket_create(int domain, int type, int protocol, int* out_fd);
    sigma_status socket_bind(int fd, sigma_ipv4_t ip, sigma_port_t port);
    sigma_status socket_listen(int fd, int backlog);
    sigma_status socket_accept(int fd, int* out_client_fd);
    sigma_isize  socket_send(int fd, const void* buf, sigma_size_t len);
    sigma_isize  socket_recv(int fd, void* buf, sigma_size_t len);

private:
    SovereignNetworkShard() : m_initialized(false) {}
    bool m_initialized;
    
    // Internal Routing Table
    // Internal Socket Descriptors Table
};

} // namespace Net
} // namespace SigmaOS

#endif /* SIGMA_NETWORK_H */
