/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-UDP-STACK (v1.0)
 * =============================================================================
 * Principles: Zero-Abstract Communication Onion.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct {
    u16 src_port;
    u16 dst_port;
    u16 len;
    u16 checksum;
} udp_hdr_t;

typedef struct {
    u8  version_ihl;
    u8  tos;
    u16 len;
    u16 id;
    u16 flags_frag;
    u8  ttl;
    u8  proto;
    u16 checksum;
    u32 src_ip;
    u32 dst_ip;
} ip_hdr_t;

extern void e1000_send_packet(void* data, u16 len);
extern void sigma_memcpy(void* dest, const void* src, u64 n);

void net_send_udp(u32 dst_ip, u16 dst_port, void* data, u16 len) {
    u8 packet[1500];
    
    udp_hdr_t udp = { .src_port = 8080, .dst_port = dst_port, .len = (len + 8) };
    ip_hdr_t  ip  = { .src_ip = 0x0100007F, .dst_ip = dst_ip, .proto = 17, .len = (len + 28) };

    /* Construct the Onion Packet */
    sigma_memcpy(packet + 34, &udp, 8);
    sigma_memcpy(packet + 14, &ip, 20);
    sigma_memcpy(packet + 42, data, len);

    e1000_send_packet(packet, len + 42);
}
