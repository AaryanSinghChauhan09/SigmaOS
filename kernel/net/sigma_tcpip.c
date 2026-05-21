#include "sigma_kernel_types.h"
#include "sigma_slab.h"

// Basic TCP/IP Stub Implementation

typedef struct {
    uint8_t version_ihl;
    uint8_t dscp_ecn;
    uint16_t total_length;
    uint16_t identification;
    uint16_t flags_fragment;
    uint8_t ttl;
    uint8_t protocol;
    uint16_t header_checksum;
    uint32_t src_ip;
    uint32_t dst_ip;
} __attribute__((packed)) ipv4_header_t;

typedef struct {
    uint16_t src_port;
    uint16_t dst_port;
    uint32_t seq_num;
    uint32_t ack_num;
    uint8_t data_offset_res;
    uint8_t flags;
    uint16_t window_size;
    uint16_t checksum;
    uint16_t urgent_ptr;
} __attribute__((packed)) tcp_header_t;

extern void kprintf(const char* fmt, ...);

void init_tcp_ip(void) {
    // kprintf is just a placeholder here.
    // In actual implementation, we register protocol handlers
    // for IPv4 (0x0800) and ARP (0x0806) to the ethernet layer.
}

void sigma_tcp_rx(void* packet, size_t len) {
    (void)packet; (void)len;
    // Handle incoming TCP segments
}

void sigma_tcp_tx(uint32_t dst_ip, uint16_t src_port, uint16_t dst_port, void* data, size_t len) {
    (void)dst_ip; (void)src_port; (void)dst_port; (void)data; (void)len;
    // Construct TCP header, then IPv4 header, then queue to net_device
}
