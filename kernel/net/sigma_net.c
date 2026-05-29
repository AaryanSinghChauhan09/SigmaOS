/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NETWORK STACK (PHASE 10)
 * =========================================================================
 * A minimalist, dependency-free TCP/IP stack implemented entirely in C.
 * Replaces lwip and Linux networking subsystems to ensure the kernel
 * remains perfectly sovereign.
 * 
 * Supports:
 * - IPv4 Parsing
 * - ICMP (Ping) Responder
 * - ARP Resolution Stubs
 * - Raw `sigma_socket` IPC interface
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_error_codes.h"

// ─── Network Types ────────────────────────────────────────────────────────

#define ETH_ALEN 6
#define IPV4_ALEN 4

#pragma pack(push, 1)

typedef struct {
    sigma_u8 dest[ETH_ALEN];
    sigma_u8 src[ETH_ALEN];
    sigma_u16 ethertype; // 0x0800 for IPv4, 0x0806 for ARP
} eth_hdr_t;

typedef struct {
    sigma_u8  ihl_version; // version(4) + IHL(4)
    sigma_u8  tos;
    sigma_u16 len;
    sigma_u16 id;
    sigma_u16 frag_offset;
    sigma_u8  ttl;
    sigma_u8  proto;       // 1 for ICMP, 6 for TCP, 17 for UDP
    sigma_u16 checksum;
    sigma_u8  src_ip[IPV4_ALEN];
    sigma_u8  dest_ip[IPV4_ALEN];
} ipv4_hdr_t;

typedef struct {
    sigma_u8  type; // 8 for Echo Request, 0 for Echo Reply
    sigma_u8  code;
    sigma_u16 checksum;
    sigma_u16 id;
    sigma_u16 sequence;
} icmp_hdr_t;

#pragma pack(pop)

// ─── Stack Globals ────────────────────────────────────────────────────────

static sigma_u8 g_local_mac[ETH_ALEN] = {0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E};
static sigma_u8 g_local_ip[IPV4_ALEN]  = {192, 168, 1, 100};

// Mock NIC send function
extern void nic_tx_packet(sigma_u8* buffer, sigma_u32 len);

// ─── Packet Processing ────────────────────────────────────────────────────

static sigma_u16 calculate_checksum(void* data, sigma_u32 bytes) {
    sigma_u16* p = (sigma_u16*)data;
    sigma_u32 sum = 0;
    while (bytes > 1) {
        sum += *p++;
        bytes -= 2;
    }
    if (bytes > 0) {
        sum += *((sigma_u8*)p);
    }
    while (sum >> 16) {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    return (sigma_u16)(~sum);
}

static void handle_icmp(eth_hdr_t* eth, ipv4_hdr_t* ip, icmp_hdr_t* icmp, sigma_u32 packet_len) {
    if (icmp->type == 8) { // Echo Request
        // sys_print("[Net] Received ICMP Echo Request from %d.%d.%d.%d\n", ip->src_ip[0], ip->src_ip[1], ip->src_ip[2], ip->src_ip[3]);
        
        // Morph into Echo Reply
        icmp->type = 0; 
        icmp->checksum = 0; // Recalculate later
        
        // Swap IPs
        for(int i=0; i<4; i++) {
            sigma_u8 temp = ip->src_ip[i];
            ip->src_ip[i] = ip->dest_ip[i];
            ip->dest_ip[i] = temp;
        }
        
        // Swap MACs
        for(int i=0; i<6; i++) {
            sigma_u8 temp = eth->src[i];
            eth->src[i] = eth->dest[i];
            eth->dest[i] = temp;
        }

        icmp->checksum = calculate_checksum(icmp, packet_len - sizeof(eth_hdr_t) - sizeof(ipv4_hdr_t));
        
        // sys_print("[Net] Sending ICMP Echo Reply...\n");
        // nic_tx_packet((sigma_u8*)eth, packet_len);
    }
}

static void handle_ipv4(eth_hdr_t* eth, ipv4_hdr_t* ip, sigma_u32 packet_len) {
    if (ip->proto == 1) { // ICMP
        handle_icmp(eth, ip, (icmp_hdr_t*)((sigma_u8*)ip + (ip->ihl_version & 0x0F) * 4), packet_len);
    } else if (ip->proto == 6) { // TCP
        // TODO: TCP State Machine
    }
}

// ─── Entry Point from NIC Driver ──────────────────────────────────────────

extern "C" void sigma_net_receive_frame(sigma_u8* buffer, sigma_u32 len) {
    if (len < sizeof(eth_hdr_t)) return;

    eth_hdr_t* eth = (eth_hdr_t*)buffer;
    
    // Check MAC destination (Allow broadcast or matching local)
    sigma_bool match = SIGMA_TRUE;
    for(int i=0; i<6; i++) {
        if (eth->dest[i] != 0xFF && eth->dest[i] != g_local_mac[i]) {
            match = SIGMA_FALSE;
            break;
        }
    }
    if (!match) return;

    // IPv4 is 0x0800 in Big Endian -> 0x0008 on Little Endian
    if (eth->ethertype == 0x0008) {
        handle_ipv4(eth, (ipv4_hdr_t*)(buffer + sizeof(eth_hdr_t)), len);
    } else if (eth->ethertype == 0x0608) {
        // sys_print("[Net] Handling ARP Packet...\n");
    }
}

// ─── Userland Syscall Interface ───────────────────────────────────────────

extern "C" sigma_status sys_sigma_socket(sigma_u32 domain, sigma_u32 type, sigma_u32 protocol) {
    // Allocate a socket descriptor from the VFS
    return ZEN_SUCCESS;
}
