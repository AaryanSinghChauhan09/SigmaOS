#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Lightweight TCP/IP Stack Prototype
// ---------------------------------------------------------

typedef struct {
    uint8_t dest_mac[6];
    uint8_t src_mac[6];
    uint16_t ethertype;
} eth_header_t;

typedef struct {
    uint8_t version_ihl;
    uint8_t dscp_ecn;
    uint16_t total_length;
    uint16_t id;
    uint16_t flags_fragment_offset;
    uint8_t ttl;
    uint8_t protocol;
    uint16_t header_checksum;
    uint32_t src_ip;
    uint32_t dest_ip;
} ipv4_header_t;

// Minimal packet dispatcher
void network_receive_packet(void* packet_buffer, size_t length) {
    if (length < sizeof(eth_header_t)) return;
    
    eth_header_t* eth = (eth_header_t*)packet_buffer;
    if (eth->ethertype == 0x0800) { // IPv4
        ipv4_header_t* ip = (ipv4_header_t*)((uint8_t*)packet_buffer + sizeof(eth_header_t));
        // Route IP packet to TCP/UDP handlers
        if (ip->protocol == 6) {
            // handle_tcp(ip);
        } else if (ip->protocol == 17) {
            // handle_udp(ip);
        }
    }
}
