/*
 * Σ SigmaOS — sigma_net_socket: Sovereign Socket API
 * Zero-Dependency: No POSIX sockets (no sys/socket.h, AF_INET, SOCK_STREAM).
 * Provides a highly optimized, state-machine-driven lightweight interface.
 */

typedef unsigned int   u32;
typedef unsigned short u16;
typedef unsigned char  u8;
typedef unsigned long long u64;

/* Sovereign Socket Protocols */
#define SIGMA_PROTO_TCP 1
#define SIGMA_PROTO_UDP 2
#define SIGMA_PROTO_RAW 3

/* Sovereign Socket States */
#define SIGMA_SOCK_CLOSED      0
#define SIGMA_SOCK_LISTENING   1
#define SIGMA_SOCK_CONNECTING  2
#define SIGMA_SOCK_ESTABLISHED 3

/* Sovereign Socket Handle */
typedef u32 sigma_sock_t;

struct SigmaSocket {
    u32 protocol;
    u32 state;
    u32 local_ip;
    u16 local_port;
    u32 remote_ip;
    u16 remote_port;
    
    /* Ring Buffers for zero-copy IPC */
    u8* rx_buffer;
    u32 rx_head;
    u32 rx_tail;
    
    u8* tx_buffer;
    u32 tx_head;
    u32 tx_tail;
    
    bool active;
};

#define MAX_SOCKETS 1024
static SigmaSocket socket_table[MAX_SOCKETS];

/* 
 * Create a new Sovereign Socket 
 */
extern "C" sigma_sock_t sigma_net_socket_create(u32 protocol) {
    for (u32 i = 0; i < MAX_SOCKETS; i++) {
        if (!socket_table[i].active) {
            socket_table[i].protocol = protocol;
            socket_table[i].state = SIGMA_SOCK_CLOSED;
            socket_table[i].active = true;
            /* In a full impl, allocate rx/tx buffers via sigma_allocator */
            return i;
        }
    }
    return (sigma_sock_t)-1;
}

/* 
 * Bind socket to local port 
 */
extern "C" int sigma_net_socket_bind(sigma_sock_t sock, u32 ip, u16 port) {
    if (sock >= MAX_SOCKETS || !socket_table[sock].active) return -1;
    socket_table[sock].local_ip = ip;
    socket_table[sock].local_port = port;
    return 0;
}

/* 
 * Initiate connection (TCP) 
 */
extern "C" int sigma_net_socket_connect(sigma_sock_t sock, u32 remote_ip, u16 remote_port) {
    if (sock >= MAX_SOCKETS || !socket_table[sock].active) return -1;
    socket_table[sock].remote_ip = remote_ip;
    socket_table[sock].remote_port = remote_port;
    
    if (socket_table[sock].protocol == SIGMA_PROTO_TCP) {
        socket_table[sock].state = SIGMA_SOCK_CONNECTING;
        /* Route SYN packet via sigma_net_tcp.cpp */
    } else {
        socket_table[sock].state = SIGMA_SOCK_ESTABLISHED;
    }
    
    return 0;
}

/* 
 * Send Data 
 */
extern "C" int sigma_net_socket_send(sigma_sock_t sock, const u8* data, u32 len) {
    if (sock >= MAX_SOCKETS || !socket_table[sock].active) return -1;
    /* Enqueue to tx_buffer and trigger stack TX */
    return len; /* Stubbed success */
}

/* 
 * Receive Data 
 */
extern "C" int sigma_net_socket_recv(sigma_sock_t sock, u8* buffer, u32 max_len) {
    if (sock >= MAX_SOCKETS || !socket_table[sock].active) return -1;
    /* Dequeue from rx_buffer */
    return 0; /* Stubbed */
}

/* 
 * Close Socket 
 */
extern "C" void sigma_net_socket_close(sigma_sock_t sock) {
    if (sock >= MAX_SOCKETS) return;
    socket_table[sock].active = false;
    socket_table[sock].state = SIGMA_SOCK_CLOSED;
}
