/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN NETWORK STACK (v1.0)
 * =============================================================================
 * Mission: Zero-dependency L2/L3/L4 stack (Ethernet, IPv4, TCP, UDP).
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_NETWORK_H
#define SIGMA_NETWORK_H

#include "../sigma_kernel_types.h"

#define NET_MAX_SOCKETS     128
#define NET_MAX_INTERFACES    4
#define NET_MTU_SIZE       1500

typedef enum {
    SOCKET_PROTO_TCP = 0,
    SOCKET_PROTO_UDP = 1,
    SOCKET_PROTO_RAW = 2
} sigma_socket_proto_t;

typedef enum {
    TCP_STATE_CLOSED      = 0,
    TCP_STATE_LISTEN      = 1,
    TCP_STATE_SYN_SENT    = 2,
    TCP_STATE_SYN_RECV    = 3,
    TCP_STATE_ESTABLISHED = 4,
    TCP_STATE_FIN_WAIT1   = 5,
    TCP_STATE_FIN_WAIT2   = 6,
    TCP_STATE_TIME_WAIT   = 7,
    TCP_STATE_CLOSE_WAIT  = 8,
    TCP_STATE_LAST_ACK    = 9
} sigma_tcp_state_t;

typedef struct {
    sigma_u8 mac[6];
} sigma_mac_addr_t;

typedef struct {
    sigma_u8 ip[4];
} sigma_ip_addr_t;

typedef struct {
    sigma_u32            sock_id;
    sigma_socket_proto_t proto;
    sigma_u32            owner_pid;
    
    sigma_ip_addr_t      local_ip;
    sigma_u16            local_port;
    sigma_ip_addr_t      remote_ip;
    sigma_u16            remote_port;

    sigma_tcp_state_t    tcp_state;
    sigma_u32            seq_num;
    sigma_u32            ack_num;

    sigma_u8             rx_buffer[8192];
    sigma_u32            rx_head;
    sigma_u32            rx_tail;
    
    sigma_bool           is_bound;
    sigma_bool           is_listening;
} sigma_socket_t;

#ifdef __cplusplus
extern "C" {
#endif

void      net_init(void);
int       net_register_interface(sigma_u32 dev_id, const sigma_mac_addr_t* mac, const sigma_ip_addr_t* ip);

sigma_u32 socket_create(sigma_u32 owner_pid, sigma_socket_proto_t proto);
int       socket_bind(sigma_u32 sock_id, const sigma_ip_addr_t* ip, sigma_u16 port);
int       socket_listen(sigma_u32 sock_id, int backlog);
sigma_u32 socket_accept(sigma_u32 sock_id);
int       socket_connect(sigma_u32 sock_id, const sigma_ip_addr_t* dest_ip, sigma_u16 dest_port);
int       socket_send(sigma_u32 sock_id, const void* buf, sigma_usize len);
int       socket_recv(sigma_u32 sock_id, void* buf, sigma_usize len);
int       socket_close(sigma_u32 sock_id);

void      net_print_interfaces(void);
void      net_print_sockets(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NETWORK_H */
