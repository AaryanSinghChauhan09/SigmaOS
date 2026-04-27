/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: ZERO-COPY NETWORK BUFFER (v1.0)
 * =============================================================================
 * Principles: Zero-Allocation Packet Processing & Silicon-Direct IO.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

#define NET_BUF_SIZE 2048
#define MAX_PACKETS  128

typedef struct NetPacket {
    u8      data[NET_BUF_SIZE];
    usize   len;
    bool_t  in_use;
} net_packet_t;

static net_packet_t packet_pool[MAX_PACKETS];

void net_buf_init() {
    sigma_memset(packet_pool, 0, sizeof(packet_pool));
}

/* Fast O(1) buffer acquisition */
net_packet_t* net_buf_acquire() {
    for (u32 i = 0; i < MAX_PACKETS; i++) {
        if (!packet_pool[i].in_use) {
            packet_pool[i].in_use = TRUE;
            return &packet_pool[i];
        }
    }
    return NULL;
}

void net_buf_release(net_packet_t* pkt) {
    pkt->in_use = FALSE;
}
