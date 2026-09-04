// SPDX-License-Identifier: MIT
// SigmaOS Custom TCP/IP Stack Kernel Implementation
// Clean-room TCP/IP implementation for the SigmaOS microkernel
#include <stddef.h>
#include <stdint.h>
#include <string.h>

// ============================================================================
// Core Network Data Structures
// ============================================================================

typedef struct {
    uint8_t a, b, c, d;
} ipv4_addr_t;

typedef struct {
    uint8_t addr[6];
} mac_addr_t;

typedef struct {
    mac_addr_t dest_mac;
    mac_addr_t src_mac;
    uint16_t ethertype;
    uint8_t *payload;
    uint16_t payload_len;
} ethernet_frame_t;

typedef struct {
    uint8_t version_ihl;
    uint8_t dscp_ecn;
    uint16_t total_length;
    uint16_t identification;
    uint16_t flags_fragment_offset;
    uint8_t ttl;
    uint8_t protocol;
    uint16_t checksum;
    ipv4_addr_t src_ip;
    ipv4_addr_t dest_ip;
    uint8_t *options;
    uint8_t *payload;
    uint16_t payload_len;
} ipv4_packet_t;

typedef struct {
    uint16_t src_port;
    uint16_t dest_port;
    uint32_t sequence_number;
    uint32_t ack_number;
    uint8_t data_offset;
    uint8_t flags;
    uint16_t window_size;
    uint16_t checksum;
    uint16_t urgent_pointer;
    uint8_t *options;
    uint8_t *payload;
    uint16_t payload_len;
} tcp_segment_t;

typedef struct {
    uint16_t src_port;
    uint16_t dest_port;
    uint16_t length;
    uint16_t checksum;
    uint8_t *payload;
    uint16_t payload_len;
} udp_segment_t;

// TCP Connection State Machine
typedef enum {
    TCP_CLOSED,
    TCP_LISTEN,
    TCP_SYN_SENT,
    TCP_SYN_RECEIVED,
    TCP_ESTABLISHED,
    TCP_FIN_WAIT_1,
    TCP_FIN_WAIT_2,
    TCP_CLOSE_WAIT,
    TCP_CLOSING,
    TCP_LAST_ACK,
    TCP_TIME_WAIT,
} tcp_state_t;

typedef struct tcp_connection {
    uint32_t local_ip;
    uint32_t remote_ip;
    uint16_t local_port;
    uint16_t remote_port;
    tcp_state_t state;
    uint32_t seq;
    uint32_t ack;
    uint32_t window_size;
    struct tcp_connection *next;
} tcp_connection_t;

// Global network stack state
static tcp_connection_t *tcp_connections = NULL;
static uint32_t interface_ip = 0;
static mac_addr_t interface_mac = {{0}};

// ============================================================================
// Checksum Computation
// ============================================================================

static uint16_t calculate_checksum(uint8_t *data, uint16_t len) {
    uint32_t sum = 0;
    uint16_t *ptr = (uint16_t *)data;
    
    while (len > 1) {
        sum += *ptr++;
        len -= 2;
    }
    
    if (len) {
        sum += *(uint8_t *)ptr;
    }
    
    while (sum >> 16) {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    
    return ~sum;
}

static uint16_t calculate_tcp_checksum(ipv4_packet_t *ip_pkt, tcp_segment_t *tcp_seg) {
    uint32_t sum = 0;
    uint8_t *data = (uint8_t *)tcp_seg;
    
    // Pseudo-header: src IP, dst IP, protocol, length
    sum += ((ip_pkt->src_ip.a << 8) | ip_pkt->src_ip.b);
    sum += ((ip_pkt->src_ip.c << 8) | ip_pkt->src_ip.d);
    sum += ((ip_pkt->dest_ip.a << 8) | ip_pkt->dest_ip.b);
    sum += ((ip_pkt->dest_ip.c << 8) | ip_pkt->dest_ip.d);
    sum += 6; // TCP protocol number
    sum += tcp_seg->payload_len + 20; // TCP header + payload
    
    // TCP header + payload
    uint16_t len = tcp_seg->payload_len + 20;
    uint16_t *ptr = (uint16_t *)data;
    while (len > 1) {
        sum += *ptr++;
        len -= 2;
    }
    
    while (sum >> 16) {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    
    return ~sum;
}

// ============================================================================
// TCP Connection Management
// ============================================================================

static tcp_connection_t* find_connection(uint32_t local_ip, uint32_t remote_ip,
                                          uint16_t local_port, uint16_t remote_port) {
    tcp_connection_t *conn = tcp_connections;
    while (conn) {
        if (conn->local_ip == local_ip && conn->remote_ip == remote_ip &&
            conn->local_port == local_port && conn->remote_port == remote_port) {
            return conn;
        }
        conn = conn->next;
    }
    return NULL;
}

static tcp_connection_t* create_connection(uint32_t local_ip, uint32_t remote_ip,
                                            uint16_t local_port, uint16_t remote_port) {
    tcp_connection_t *conn = (tcp_connection_t *)malloc(sizeof(tcp_connection_t));
    if (!conn) return NULL;
    
    conn->local_ip = local_ip;
    conn->remote_ip = remote_ip;
    conn->local_port = local_port;
    conn->remote_port = remote_port;
    conn->state = TCP_CLOSED;
    conn->seq = 0x12345678; // Initial sequence number (should be random)
    conn->ack = 0;
    conn->window_size = 65536;
    conn->next = tcp_connections;
    tcp_connections = conn;
    
    return conn;
}

// ============================================================================
// Packet Processing
// ============================================================================

void sigma_process_ipv4_packet(ipv4_packet_t *pkt) {
    switch (pkt->protocol) {
        case 6: // TCP
            // TCP processing will be handled by the Rust layer
            break;
        case 17: // UDP
            // UDP processing will be handled by the Rust layer
            break;
        case 1: // ICMP
            // ICMP processing for ping, etc.
            break;
        default:
            break;
    }
}

void sigma_tcp_handle_syn(ipv4_packet_t *ip_pkt, tcp_segment_t *tcp_seg) {
    // SYN received - move to SYN_RECEIVED state
    tcp_connection_t *conn = find_connection(ip_pkt->dest_ip.a << 24 | ip_pkt->dest_ip.b << 16 | 
                                              ip_pkt->dest_ip.c << 8 | ip_pkt->dest_ip.d,
                                              ip_pkt->src_ip.a << 24 | ip_pkt->src_ip.b << 16 |
                                              ip_pkt->src_ip.c << 8 | ip_pkt->src_ip.d,
                                              tcp_seg->dest_port, tcp_seg->src_port);
    if (!conn) {
        conn = create_connection(ip_pkt->dest_ip.a << 24 | ip_pkt->dest_ip.b << 16 | 
                                 ip_pkt->dest_ip.c << 8 | ip_pkt->dest_ip.d,
                                 ip_pkt->src_ip.a << 24 | ip_pkt->src_ip.b << 16 |
                                 ip_pkt->src_ip.c << 8 | ip_pkt->src_ip.d,
                                 tcp_seg->dest_port, tcp_seg->src_port);
    }
    if (conn) {
        conn->state = TCP_SYN_RECEIVED;
        conn->ack = tcp_seg->sequence_number + 1;
    }
}

void sigma_tcp_handle_ack(ipv4_packet_t *ip_pkt, tcp_segment_t *tcp_seg) {
    // ACK processing
    tcp_connection_t *conn = find_connection(ip_pkt->dest_ip.a << 24 | ip_pkt->dest_ip.b << 16 | 
                                              ip_pkt->dest_ip.c << 8 | ip_pkt->dest_ip.d,
                                              ip_pkt->src_ip.a << 24 | ip_pkt->src_ip.b << 16 |
                                              ip_pkt->src_ip.c << 8 | ip_pkt->src_ip.d,
                                              tcp_seg->dest_port, tcp_seg->src_port);
    if (conn) {
        if (conn->state == TCP_SYN_RECEIVED) {
            conn->state = TCP_ESTABLISHED;
        }
    }
}

// ============================================================================
// Initialization & Interface
// ============================================================================

void sigma_tcp_init() {
    tcp_connections = NULL;
    interface_ip = 0;
    memset(&interface_mac, 0, sizeof(mac_addr_t));
}

void sigma_network_set_interface_mac(uint8_t a, uint8_t b, uint8_t c, uint8_t d, uint8_t e, uint8_t f) {
    interface_mac.addr[0] = a;
    interface_mac.addr[1] = b;
    interface_mac.addr[2] = c;
    interface_mac.addr[3] = d;
    interface_mac.addr[4] = e;
    interface_mac.addr[5] = f;
}

void sigma_network_set_interface_ip(uint8_t a, uint8_t b, uint8_t c, uint8_t d) {
    interface_ip = (a << 24) | (b << 16) | (c << 8) | d;
}

void sigma_network_transmit_packet(uint8_t *packet_data, uint16_t packet_len) {
    // This function is called from Rust layer to physically transmit a packet
    // Implementation depends on network device driver (NIC)
}

void sigma_network_receive_packet(uint8_t *packet_data, uint16_t packet_len) {
    // This function is called from Rust layer when a packet is received from NIC
    // Parse and route the packet appropriately
}
