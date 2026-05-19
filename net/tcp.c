/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: TRANSMISSION CONTROL PROTOCOL (TCP)
 * =============================================================================
 * Inspired by: Linux kernel net/ipv4/tcp.c & net/ipv4/tcp_ipv4.c
 *              RFC 793 (Transmission Control Protocol)
 * =============================================================================
 * Implements the connection-oriented TCP state machine (LISTEN, SYN_SENT, etc).
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define TCP_MAX_SOCKETS 64

/* TCP States */
#define TCP_STATE_CLOSED      0
#define TCP_STATE_LISTEN      1
#define TCP_STATE_SYN_SENT    2
#define TCP_STATE_SYN_RECV    3
#define TCP_STATE_ESTABLISHED 4
#define TCP_STATE_FIN_WAIT1   5
#define TCP_STATE_FIN_WAIT2   6
#define TCP_STATE_TIME_WAIT   7
#define TCP_STATE_CLOSE_WAIT  8
#define TCP_STATE_LAST_ACK    9

#define TCP_FLAG_FIN 0x01
#define TCP_FLAG_SYN 0x02
#define TCP_FLAG_RST 0x04
#define TCP_FLAG_PSH 0x08
#define TCP_FLAG_ACK 0x10
#define TCP_FLAG_URG 0x20

typedef struct {
    sigma_u32 local_ip;
    sigma_u32 remote_ip;
    sigma_u16 local_port;
    sigma_u16 remote_port;
    sigma_u8  state;
    sigma_u32 seq_num;
    sigma_u32 ack_num;
    sigma_u16 window;
    sigma_bool active;
} sigma_tcp_sock_t;

static sigma_tcp_sock_t tcp_sockets[TCP_MAX_SOCKETS];

void tcp_init(void) {
    sigma_memset(tcp_sockets, 0, sizeof(tcp_sockets));
    sigma_printf("[tcp] Transmission Control Protocol (TCP) initialized\n");
}

int tcp_listen(sigma_u16 port) {
    for (sigma_u32 i = 0; i < TCP_MAX_SOCKETS; i++) {
        if (!tcp_sockets[i].active) {
            tcp_sockets[i].local_port = port;
            tcp_sockets[i].state = TCP_STATE_LISTEN;
            tcp_sockets[i].active = SIGMA_TRUE;
            sigma_printf("[tcp] Socket listening on port %u\n", port);
            return (int)i;
        }
    }
    return -1;
}

void tcp_process_packet(sigma_u32 src_ip, sigma_u16 src_port, sigma_u32 dst_ip, sigma_u16 dst_port, sigma_u8 flags, sigma_u32 seq, sigma_u32 ack) {
    /* Simplified State Machine dispatcher */
    for (sigma_u32 i = 0; i < TCP_MAX_SOCKETS; i++) {
        if (tcp_sockets[i].active && tcp_sockets[i].local_port == dst_port) {
            sigma_tcp_sock_t* sock = &tcp_sockets[i];
            
            switch (sock->state) {
                case TCP_STATE_LISTEN:
                    if (flags & TCP_FLAG_SYN) {
                        sigma_printf("[tcp] Received SYN from %u:%u. Transitioning to SYN_RECV.\n", src_ip, src_port);
                        sock->state = TCP_STATE_SYN_RECV;
                        sock->remote_ip = src_ip;
                        sock->remote_port = src_port;
                        sock->ack_num = seq + 1;
                        /* Would send SYN-ACK here */
                    }
                    break;
                    
                case TCP_STATE_SYN_RECV:
                    if (flags & TCP_FLAG_ACK) {
                        sigma_printf("[tcp] Received ACK from %u:%u. Connection ESTABLISHED.\n", src_ip, src_port);
                        sock->state = TCP_STATE_ESTABLISHED;
                    }
                    break;
                    
                case TCP_STATE_ESTABLISHED:
                    if (flags & TCP_FLAG_FIN) {
                        sigma_printf("[tcp] Received FIN from %u:%u. Transitioning to CLOSE_WAIT.\n", src_ip, src_port);
                        sock->state = TCP_STATE_CLOSE_WAIT;
                        /* Would send ACK here */
                    } else if (flags & TCP_FLAG_PSH) {
                        sigma_printf("[tcp] Received PSH/ACK. Processing payload.\n");
                        sock->ack_num = seq + 1; /* Simplification */
                    }
                    break;
            }
            return;
        }
    }
    
    /* No matching socket, send RST */
    if (!(flags & TCP_FLAG_RST)) {
        sigma_printf("[tcp] No socket on port %u. Dispatching RST.\n", dst_port);
    }
}
