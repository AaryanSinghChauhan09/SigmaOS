#ifndef SIGMA_PROTO_H
#define SIGMA_PROTO_H

#include <stdint.h>

/* =========================================================================
 * SIGMA OS: SOVEREIGN NETWORK PROTOCOLS (S07)
 * Native, zero-dependency implementations of high-performance networking.
 * ========================================================================= */

typedef struct {
    uint8_t  dest_mac[6];
    uint8_t  src_mac[6];
    uint16_t type;
} __attribute__((packed)) sigma_eth_header_t;

typedef struct {
    uint8_t  version_ihl;
    uint8_t  type_of_service;
    uint16_t total_length;
    uint16_t identification;
    uint16_t fragment_offset;
    uint8_t  time_to_live;
    uint8_t  protocol;
    uint16_t checksum;
    uint32_t src_ip;
    uint32_t dest_ip;
} __attribute__((packed)) sigma_ip_header_t;

typedef struct {
    uint16_t src_port;
    uint16_t dest_port;
    uint32_t sequence_number;
    uint32_t ack_number;
    uint16_t flags;
    uint16_t window_size;
    uint16_t checksum;
    uint16_t urgent_pointer;
} __attribute__((packed)) sigma_tcp_header_t;

void sigma_net_init(void);
void sigma_net_handle_packet(void* packet, uint32_t size);

#endif
