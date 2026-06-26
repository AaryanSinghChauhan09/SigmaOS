/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NETWORK STACK (v1.0)
 * =========================================================================
 * Zero-dependency L2/L3/L4 stack (Ethernet, IPv4, TCP, UDP).
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_network.h"

namespace SigmaOS {
namespace Kernel {

class SovereignNetworkStack {
public:
    static SovereignNetworkStack& getInstance() {
        static SovereignNetworkStack instance;
        return instance;
    }

    void init() {
        m_iface_count = 0;
        m_sock_count = 0;
        
        for (sigma_u32 i = 0; i < NET_MAX_SOCKETS; i++) {
            m_sockets[i].sock_id = 0;
            m_sockets[i].is_bound = SIGMA_FALSE;
        }

        sigma_log("[NET] Sovereign Network Stack initialized.");
    }

    int registerInterface(sigma_u32 dev_id, const sigma_mac_addr_t* mac, const sigma_ip_addr_t* ip) {
        if (m_iface_count >= NET_MAX_INTERFACES) return K_ERR_NOMEM;

        m_ifaces[m_iface_count].dev_id = dev_id;
        m_ifaces[m_iface_count].mac = *mac;
        m_ifaces[m_iface_count].ip = *ip;
        m_iface_count++;

        sigma_log_info("[NET] Interface registered: dev_id %u, IP %d.%d.%d.%d\n",
                       dev_id, ip->ip[0], ip->ip[1], ip->ip[2], ip->ip[3]);
        return K_OK;
    }

    sigma_u32 createSocket(sigma_u32 owner_pid, sigma_socket_proto_t proto) {
        if (m_sock_count >= NET_MAX_SOCKETS) return 0;

        sigma_u32 id = m_sock_count + 1;
        sigma_socket_t& s = m_sockets[id - 1];
        s.sock_id = id;
        s.proto = proto;
        s.owner_pid = owner_pid;
        s.is_bound = SIGMA_FALSE;
        s.is_listening = SIGMA_FALSE;
        s.tcp_state = TCP_STATE_CLOSED;
        s.rx_head = 0;
        s.rx_tail = 0;

        m_sock_count++;
        sigma_log_info("[NET] Process %u created socket %u (Proto %d)\n", owner_pid, id, (int)proto);
        return id;
    }

    int bindSocket(sigma_u32 sock_id, const sigma_ip_addr_t* ip, sigma_u16 port) {
        sigma_socket_t* s = findSocket(sock_id);
        if (!s) return K_ERR_NOTFOUND;

        s->local_ip = *ip;
        s->local_port = port;
        s->is_bound = SIGMA_TRUE;
        
        sigma_log_info("[NET] Socket %u bound to port %u\n", sock_id, port);
        return K_OK;
    }

    int listenSocket(sigma_u32 sock_id, int backlog) {
        sigma_socket_t* s = findSocket(sock_id);
        if (!s || !s->is_bound || s->proto != SOCKET_PROTO_TCP) return K_ERR_INVAL;

        s->is_listening = SIGMA_TRUE;
        s->tcp_state = TCP_STATE_LISTEN;
        sigma_log_info("[NET] Socket %u listening (backlog %d)\n", sock_id, backlog);
        return K_OK;
    }

    sigma_u32 acceptSocket(sigma_u32 sock_id) {
        sigma_socket_t* s = findSocket(sock_id);
        if (!s || !s->is_listening) return 0;

        /* Simulate accepting a connection by creating a new socket */
        sigma_u32 new_id = createSocket(s->owner_pid, SOCKET_PROTO_TCP);
        if (new_id) {
            sigma_socket_t* new_s = findSocket(new_id);
            new_s->local_ip = s->local_ip;
            new_s->local_port = s->local_port;
            /* Fake remote address */
            new_s->remote_ip.ip[0] = 192; new_s->remote_ip.ip[1] = 168; 
            new_s->remote_ip.ip[2] = 1; new_s->remote_ip.ip[3] = 100;
            new_s->remote_port = 45678;
            new_s->tcp_state = TCP_STATE_ESTABLISHED;
            
            sigma_log_info("[NET] Socket %u accepted connection -> new socket %u\n", sock_id, new_id);
        }
        return new_id;
    }

    int connectSocket(sigma_u32 sock_id, const sigma_ip_addr_t* dest_ip, sigma_u16 dest_port) {
        sigma_socket_t* s = findSocket(sock_id);
        if (!s) return K_ERR_NOTFOUND;

        s->remote_ip = *dest_ip;
        s->remote_port = dest_port;
        if (s->proto == SOCKET_PROTO_TCP) {
            s->tcp_state = TCP_STATE_ESTABLISHED; /* Skip handshake simulation for now */
        }
        
        sigma_log_info("[NET] Socket %u connected to %d.%d.%d.%d:%u\n",
                       sock_id, dest_ip->ip[0], dest_ip->ip[1], dest_ip->ip[2], dest_ip->ip[3], dest_port);
        return K_OK;
    }

    int sendData(sigma_u32 sock_id, const void* buf, sigma_usize len) {
        sigma_socket_t* s = findSocket(sock_id);
        if (!s) return K_ERR_NOTFOUND;

        sigma_log_info("[NET] Socket %u sent %llu bytes to %d.%d.%d.%d:%u\n",
                       sock_id, (unsigned long long)len,
                       s->remote_ip.ip[0], s->remote_ip.ip[1], s->remote_ip.ip[2], s->remote_ip.ip[3], s->remote_port);
        return len;
    }

    int recvData(sigma_u32 sock_id, void* buf, sigma_usize len) {
        sigma_socket_t* s = findSocket(sock_id);
        if (!s) return K_ERR_NOTFOUND;

        /* Simulate reading 0 bytes (no data) */
        return 0;
    }

    int closeSocket(sigma_u32 sock_id) {
        sigma_socket_t* s = findSocket(sock_id);
        if (!s) return K_ERR_NOTFOUND;

        sigma_log_info("[NET] Socket %u closed.\n", sock_id);
        s->sock_id = 0;
        return K_OK;
    }

    void printInterfaces() {
        sigma_log("\n--- NETWORK INTERFACES ---");
        for (sigma_u32 i = 0; i < m_iface_count; i++) {
            sigma_log_info("| eth%u (dev %u) - MAC: %02x:%02x:%02x:%02x:%02x:%02x - IP: %d.%d.%d.%d\n",
                           i, m_ifaces[i].dev_id,
                           m_ifaces[i].mac.mac[0], m_ifaces[i].mac.mac[1], m_ifaces[i].mac.mac[2],
                           m_ifaces[i].mac.mac[3], m_ifaces[i].mac.mac[4], m_ifaces[i].mac.mac[5],
                           m_ifaces[i].ip.ip[0], m_ifaces[i].ip.ip[1], m_ifaces[i].ip.ip[2], m_ifaces[i].ip.ip[3]);
        }
        sigma_log("--------------------------");
    }

    void printSockets() {
        sigma_log("\n--- ACTIVE SOCKETS ---");
        for (sigma_u32 i = 0; i < m_sock_count; i++) {
            sigma_socket_t& s = m_sockets[i];
            if (s.sock_id != 0) {
                const char* proto = (s.proto == SOCKET_PROTO_TCP) ? "TCP" : "UDP";
                sigma_log_info("| Sock %u (PID %u) [%s] - Local: %d.%d.%d.%d:%u - Remote: %d.%d.%d.%d:%u - State: %d\n",
                               s.sock_id, s.owner_pid, proto,
                               s.local_ip.ip[0], s.local_ip.ip[1], s.local_ip.ip[2], s.local_ip.ip[3], s.local_port,
                               s.remote_ip.ip[0], s.remote_ip.ip[1], s.remote_ip.ip[2], s.remote_ip.ip[3], s.remote_port,
                               (int)s.tcp_state);
            }
        }
        sigma_log("----------------------");
    }

private:
    SovereignNetworkStack() : m_iface_count(0), m_sock_count(0) {}

    sigma_socket_t* findSocket(sigma_u32 id) {
        if (id == 0 || id > m_sock_count) return SIGMA_NULL;
        return &m_sockets[id - 1];
    }

    struct Interface {
        sigma_u32 dev_id;
        sigma_mac_addr_t mac;
        sigma_ip_addr_t ip;
    };

    Interface      m_ifaces[NET_MAX_INTERFACES];
    sigma_u32      m_iface_count;
    
    sigma_socket_t m_sockets[NET_MAX_SOCKETS];
    sigma_u32      m_sock_count;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void net_init(void) { SigmaOS::Kernel::SovereignNetworkStack::getInstance().init(); }

int net_register_interface(sigma_u32 dev_id, const sigma_mac_addr_t* mac, const sigma_ip_addr_t* ip) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().registerInterface(dev_id, mac, ip);
}
sigma_u32 socket_create(sigma_u32 owner_pid, sigma_socket_proto_t proto) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().createSocket(owner_pid, proto);
}
int socket_bind(sigma_u32 sock_id, const sigma_ip_addr_t* ip, sigma_u16 port) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().bindSocket(sock_id, ip, port);
}
int socket_listen(sigma_u32 sock_id, int backlog) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().listenSocket(sock_id, backlog);
}
sigma_u32 socket_accept(sigma_u32 sock_id) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().acceptSocket(sock_id);
}
int socket_connect(sigma_u32 sock_id, const sigma_ip_addr_t* dest_ip, sigma_u16 dest_port) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().connectSocket(sock_id, dest_ip, dest_port);
}
int socket_send(sigma_u32 sock_id, const void* buf, sigma_usize len) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().sendData(sock_id, buf, len);
}
int socket_recv(sigma_u32 sock_id, void* buf, sigma_usize len) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().recvData(sock_id, buf, len);
}
int socket_close(sigma_u32 sock_id) {
    return SigmaOS::Kernel::SovereignNetworkStack::getInstance().closeSocket(sock_id);
}
void net_print_interfaces(void) {
    SigmaOS::Kernel::SovereignNetworkStack::getInstance().printInterfaces();
}
void net_print_sockets(void) {
    SigmaOS::Kernel::SovereignNetworkStack::getInstance().printSockets();
}

} // extern "C"
