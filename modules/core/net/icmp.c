#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS ICMP & ARP Protocol Stubs (Phase 2)
// ---------------------------------------------------------

typedef struct {
    uint8_t type;
    uint8_t code;
    uint16_t checksum;
    uint16_t identifier;
    uint16_t sequence;
} icmp_header_t;

#define ICMP_ECHO_REQUEST 8
#define ICMP_ECHO_REPLY   0

// 1. ICMP Ping implementation
int icmp_send_echo_request(uint32_t dest_ip, uint16_t seq_num) {
    icmp_header_t hdr;
    hdr.type = ICMP_ECHO_REQUEST;
    hdr.code = 0;
    hdr.identifier = 0x1234;
    hdr.sequence = seq_num;
    hdr.checksum = 0; // Would calculate actual checksum
    
    // Inject packet into IP layer
    // ...
    return 0; // Success
}

void icmp_handle_packet(const void* packet, int len) {
    icmp_header_t* hdr = (icmp_header_t*)packet;
    if (hdr->type == ICMP_ECHO_REQUEST) {
        // Construct and send ECHO REPLY
    }
}

// ---------------------------------------------------------
// ARP (Address Resolution Protocol)
// ---------------------------------------------------------

typedef struct {
    uint32_t ip_addr;
    uint8_t  mac_addr[6];
} arp_entry_t;

#define ARP_CACHE_SIZE 128
static arp_entry_t arp_table[ARP_CACHE_SIZE];
static int arp_count = 0;

void arp_resolve(uint32_t ip_addr) {
    // Check cache
    for (int i = 0; i < arp_count; i++) {
        if (arp_table[i].ip_addr == ip_addr) return; // Found
    }
    
    // Send ARP broadcast request to Link Layer
    // ...
}
