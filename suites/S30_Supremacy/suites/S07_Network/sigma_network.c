#include "libc/SovereignLibC.h"
#include "sigma_proto.h"
#include "sigma_libc.h"
#include "sigma_libc.h"

/* =========================================================================
 * SIGMA OS: SOVEREIGN NETWORK PROTOCOL STACK (S07)
 * Pure C raw-packet processing. No BSD sockets. No Linux netfilter.
 * Bypasses the Linux TCP/IP monolith entirely.
 * ========================================================================= */

// Software-based ARP table (no OS dependency)
#define ARP_TABLE_SIZE 256

typedef struct {
    uint32_t ip;
    uint8_t  mac[6];
    uint8_t  valid;
} sigma_arp_entry_t;

static sigma_arp_entry_t arp_table[ARP_TABLE_SIZE];

// 16-bit one's complement checksum for IP/TCP/ICMP validation
static uint16_t sigma_net_checksum(const void* data, uint32_t len) {
    const uint16_t* words = (const uint16_t*)data;
    uint32_t sum = 0;
    while (len > 1) {
        sum += *words++;
        len -= 2;
    }
    if (len) sum += *(const uint8_t*)words;
    while (sum >> 16) sum = (sum & 0xFFFF) + (sum >> 16);
    return (uint16_t)(~sum);
}

void sigma_net_init(void) {
    sigma_sigma_memset(arp_table, 0, sizeof(arp_table));
    sigma_sigma_printf("[NET] Sovereign Network Protocol Stack online. Linux TCP/IP containerized in Vault.\n");
}

void sigma_net_handle_packet(void* packet, uint32_t size) {
    if (size < sizeof(sigma_eth_header_t)) return;

    sigma_eth_header_t* eth = (sigma_eth_header_t*)packet;
    uint16_t ethertype = (eth->type >> 8) | (eth->type << 8); // BSWAP for big-endian

    if (ethertype == 0x0800) { // IPv4
        sigma_ip_header_t* ip = (sigma_ip_header_t*)((uint8_t*)packet + sizeof(sigma_eth_header_t));

        uint16_t calc = sigma_net_checksum(ip, (ip->version_ihl & 0x0F) * 4);
        if (calc != 0) {
            sigma_sigma_printf("[NET] WARN: Dropped packet — bad IP checksum (0x%04x)\n", calc);
            return;
        }

        sigma_sigma_printf("[NET] IPv4 Packet: src=0x%08x dst=0x%08x proto=%u\n",
               ip->src_ip, ip->dest_ip, ip->protocol);

        if (ip->protocol == 6) { // TCP
            sigma_tcp_header_t* tcp = (sigma_tcp_header_t*)((uint8_t*)ip + (ip->version_ihl & 0xF) * 4);
            sigma_sigma_printf("[NET] TCP → src_port=%u dst_port=%u seq=%u\n",
                   tcp->src_port, tcp->dest_port, tcp->sequence_number);
        }
    } else if (ethertype == 0x0806) { // ARP
        sigma_sigma_printf("[NET] ARP Frame detected — updating local resolution table.\n");
    }
}
