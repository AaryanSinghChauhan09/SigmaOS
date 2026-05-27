/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NETWORKING STACK (S-NET)
 * =========================================================================
 * Mission: A ground-up TCP/IP networking stack designed for security and speed.
 * Competitor parity: Linux net/ipv4, BSD sockets.
 * ZERO-DEPENDENCY: Direct ring-buffer manipulation, no POSIX sockets.
 * =========================================================================
 */

#ifndef SIGMA_NET_H
#define SIGMA_NET_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define SIGMA_MAC_ADDR_LEN 6
#define SIGMA_IPV4_ADDR_LEN 4

/* --- Network Interface Structure --- */
typedef struct {
    sigma_u8 mac_address[SIGMA_MAC_ADDR_LEN];
    sigma_u8 ipv4_address[SIGMA_IPV4_ADDR_LEN];
    sigma_u8 subnet_mask[SIGMA_IPV4_ADDR_LEN];
    sigma_u8 gateway[SIGMA_IPV4_ADDR_LEN];
    bool is_up;
    char name[16];
} sigma_net_interface_t;

/* --- Socket Stub --- */
typedef struct {
    sigma_u32 socket_id;
    sigma_u16 local_port;
    sigma_u16 remote_port;
    sigma_u8 remote_ip[SIGMA_IPV4_ADDR_LEN];
    sigma_u8 protocol; /* TCP / UDP */
    sigma_u32 state;
} sigma_net_socket_t;

/* --- Networking Primitives --- */
void sigma_net_init();
bool sigma_net_register_interface(sigma_net_interface_t* iface);
sigma_net_socket_t* sigma_net_socket_create(sigma_u8 protocol);
bool sigma_net_bind(sigma_net_socket_t* sock, sigma_u16 port);
bool sigma_net_connect(sigma_net_socket_t* sock, const sigma_u8* ip, sigma_u16 port);
sigma_u32 sigma_net_send(sigma_net_socket_t* sock, const void* data, sigma_u32 len);
sigma_u32 sigma_net_recv(sigma_net_socket_t* sock, void* buffer, sigma_u32 max_len);
void sigma_net_close(sigma_net_socket_t* sock);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NET_H */
