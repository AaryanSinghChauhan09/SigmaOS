#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS TCP/IP Stack Prototype
// ---------------------------------------------------------

typedef struct {
    uint32_t src_ip;
    uint32_t dest_ip;
    uint16_t src_port;
    uint16_t dest_port;
    uint32_t seq_num;
    uint32_t ack_num;
    uint8_t flags;
    uint16_t window_size;
} tcp_header_t;

typedef struct {
    int socket_id;
    int state; // 0: CLOSED, 1: LISTEN, 2: ESTABLISHED
    uint32_t local_ip;
    uint16_t local_port;
    uint32_t remote_ip;
    uint16_t remote_port;
} tcp_socket_t;

#define MAX_SOCKETS 128
static tcp_socket_t active_sockets[MAX_SOCKETS];

void tcp_init() {
    for (int i = 0; i < MAX_SOCKETS; i++) {
        active_sockets[i].state = 0; // CLOSED
    }
}

int tcp_connect(uint32_t ip, uint16_t port) {
    // Find free socket
    for (int i = 0; i < MAX_SOCKETS; i++) {
        if (active_sockets[i].state == 0) {
            active_sockets[i].remote_ip = ip;
            active_sockets[i].remote_port = port;
            active_sockets[i].state = 2; // ESTABLISHED (Mock)
            return i;
        }
    }
    return -1; // Out of sockets
}

int tcp_send(int socket_id, const char* data, int len) {
    if (socket_id < 0 || socket_id >= MAX_SOCKETS || active_sockets[socket_id].state != 2) {
        return -1; // Invalid socket
    }
    // Mock send
    return len;
}
