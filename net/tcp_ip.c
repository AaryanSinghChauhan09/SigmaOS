#include "../sigma_libc.h"

/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: TCP/IP PROTOCOL SUITE (v1.0)
 * =============================================================================
 * Handles L3 (IPv4) packet headers and L4 (TCP/UDP) port bindings.
 * Interfaces with POSIX system call layer.
 * =============================================================================
 */

#define SOCK_MAX 16

typedef enum {
    CONN_CLOSED,
    CONN_LISTEN,
    CONN_SYN_SENT,
    CONN_ESTABLISHED,
    CONN_FIN_WAIT
} conn_state_t;

typedef struct {
    int id;
    sigma_u32 local_ip;
    sigma_u16 local_port;
    sigma_u32 remote_ip;
    sigma_u16 remote_port;
    conn_state_t state;
} net_socket_t;

static net_socket_t sockets_table[SOCK_MAX];
static int socket_counter = 0;

void init_tcp_ip(void) {
    sigma_printf("[tcp_ip] Initializing TCP/IP Protocol Stack...\n");
    for (int i = 0; i < SOCK_MAX; i++) {
        sockets_table[i].id = -1;
        sockets_table[i].state = CONN_CLOSED;
    }
    socket_counter = 100; // FD starts at 100
    sigma_printf("[tcp_ip] Protocol table initialized (Max: %d active sockets).\n", SOCK_MAX);
}

sigma_i32 net_socket(sigma_i32 domain, sigma_i32 type, sigma_i32 protocol) {
    (void)domain; (void)type; (void)protocol;
    
    // Find empty socket slot
    for (int i = 0; i < SOCK_MAX; i++) {
        if (sockets_table[i].id == -1) {
            sockets_table[i].id = socket_counter++;
            sockets_table[i].local_ip = 0x7F000001; // 127.0.0.1
            sockets_table[i].local_port = 5000 + i;
            sockets_table[i].state = CONN_CLOSED;
            sigma_printf("[tcp_ip] Socket created. FD: %d, local port: %d\n", sockets_table[i].id, sockets_table[i].local_port);
            return sockets_table[i].id;
        }
    }
    return -1; // Busy
}

sigma_i32 net_connect(sigma_i32 fd, sigma_u32 remote_ip, sigma_u16 remote_port) {
    for (int i = 0; i < SOCK_MAX; i++) {
        if (sockets_table[i].id == fd) {
            sockets_table[i].remote_ip = remote_ip;
            sockets_table[i].remote_port = remote_port;
            sockets_table[i].state = CONN_SYN_SENT;
            sigma_printf("[tcp_ip] FD %d: Sent SYN connection request -> %u.%u.%u.%u:%d\n",
                         fd,
                         (remote_ip >> 24) & 0xFF,
                         (remote_ip >> 16) & 0xFF,
                         (remote_ip >> 8) & 0xFF,
                         remote_ip & 0xFF,
                         remote_port);
            
            // Simulate receiving SYN-ACK
            sockets_table[i].state = CONN_ESTABLISHED;
            sigma_printf("[tcp_ip] FD %d: Handshake complete. State: ESTABLISHED\n", fd);
            return 0; // Success
        }
    }
    return -1; // FD not found
}

sigma_i32 net_send(sigma_i32 fd, const void* data, sigma_size_t size) {
    for (int i = 0; i < SOCK_MAX; i++) {
        if (sockets_table[i].id == fd) {
            if (sockets_table[i].state != CONN_ESTABLISHED) {
                sigma_printf("[tcp_ip] ERR: Socket FD %d not connected.\n", fd);
                return -1;
            }
            sigma_printf("[tcp_ip] FD %d: Sending %u bytes of TCP payload...\n", fd, (sigma_u32)size);
            // Route through loopback
            extern sigma_i32 loopback_transmit(const void* data, sigma_size_t size);
            loopback_transmit(data, size);
            return (sigma_i32)size;
        }
    }
    return -1;
}
