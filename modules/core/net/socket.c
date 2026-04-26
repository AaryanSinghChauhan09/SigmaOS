#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Shard-Level Socket API & Firewall (Phase 2)
// ---------------------------------------------------------

#define MAX_SOCKETS 256

typedef enum {
    SOCK_STREAM = 1, // TCP
    SOCK_DGRAM  = 2, // UDP
    SOCK_RAW    = 3  // ICMP/Raw
} socket_type_t;

typedef struct {
    int socket_id;
    int shard_owner_id;
    socket_type_t type;
    uint32_t local_ip;
    uint16_t local_port;
    uint32_t remote_ip;
    uint16_t remote_port;
    int state; // 0: Closed, 1: Bound, 2: Listening, 3: Connected
} sigma_socket_t;

static sigma_socket_t socket_table[MAX_SOCKETS];

// External dependency on Isolation Shard
extern int isolation_check_capability(int shard_id, uint32_t capability);

void socket_init() {
    for (int i = 0; i < MAX_SOCKETS; i++) {
        socket_table[i].state = 0;
    }
}

// Firewall rule evaluation
static int firewall_check_traffic(uint32_t src_ip, uint32_t dest_ip, uint16_t port) {
    // Basic Shard-Level Firewall: Block external traffic by default unless explicitly allowed
    // In reality, this would check a routing/firewall rules table
    return 1; // Allow for now
}

// Socket API: socket()
int sigma_socket(int shard_id, socket_type_t type) {
    // 1. Isolation Enforcement: Check if shard has NETWORK_ACCESS capability (Mask: 1)
    if (!isolation_check_capability(shard_id, 1)) {
        return -1; // Access Denied
    }
    
    // 2. Allocate socket
    for (int i = 0; i < MAX_SOCKETS; i++) {
        if (socket_table[i].state == 0) {
            socket_table[i].socket_id = i;
            socket_table[i].shard_owner_id = shard_id;
            socket_table[i].type = type;
            socket_table[i].state = 1; // Created
            return i;
        }
    }
    return -1; // Out of sockets
}

// Socket API: bind()
int sigma_bind(int shard_id, int sockfd, uint32_t ip, uint16_t port) {
    if (sockfd < 0 || sockfd >= MAX_SOCKETS || socket_table[sockfd].shard_owner_id != shard_id) {
        return -1;
    }
    socket_table[sockfd].local_ip = ip;
    socket_table[sockfd].local_port = port;
    socket_table[sockfd].state = 1; // Bound
    return 0;
}

// Socket API: send()
int sigma_send(int shard_id, int sockfd, const void* buffer, int len) {
    if (sockfd < 0 || sockfd >= MAX_SOCKETS || socket_table[sockfd].shard_owner_id != shard_id) {
        return -1;
    }
    
    if (!firewall_check_traffic(socket_table[sockfd].local_ip, socket_table[sockfd].remote_ip, socket_table[sockfd].remote_port)) {
        return -1; // Blocked by firewall
    }
    
    // Proceed to TCP/UDP stack transmission
    // ...
    return len; // Mock success
}
