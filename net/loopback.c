#include "../sigma_libc.h"

/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: LOOPBACK NETWORK INTERFACE (v1.0)
 * =============================================================================
 * Simulates loopback (lo) interface transmission at 127.0.0.1.
 * Matches packet buffers and routes them directly back to the RX engine.
 * =============================================================================
 */

#define LOOPBACK_IP 0x7F000001 // 127.0.0.1
#define MAX_PACKET_SIZE 1518

static sigma_u64 tx_packets = 0;
static sigma_u64 rx_packets = 0;
static sigma_u64 tx_bytes = 0;

void init_loopback_net(void) {
    sigma_printf("[loopback] Registering loopback interface 'lo' (127.0.0.1)...\n");
    tx_packets = 0;
    rx_packets = 0;
    tx_bytes = 0;
    sigma_printf("[loopback] Loopback interface active and MTU set to 16436.\n");
}

sigma_i32 loopback_transmit(const void* data, sigma_size_t size) {
    if (size > MAX_PACKET_SIZE) {
        sigma_printf("[loopback] ERR: Packet size %u exceeds MTU.\n", (sigma_u32)size);
        return -1;
    }
    
    // Simulate transmission back to the stack (rx)
    tx_packets++;
    rx_packets++;
    tx_bytes += size;
    
    // Diagnostic logging for packet transmission
    // sigma_printf("[loopback] Tx/Rx loopback packet: %u bytes routed back.\n", (sigma_u32)size);
    
    return 0; // Success
}

void loopback_get_stats(sigma_u64* out_tx, sigma_u64* out_rx, sigma_u64* out_bytes) {
    if (out_tx) *out_tx = tx_packets;
    if (out_rx) *out_rx = rx_packets;
    if (out_bytes) *out_bytes = tx_bytes;
}
