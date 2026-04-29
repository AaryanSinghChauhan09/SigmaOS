/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-UDP-STACK (v1.0)
 * =============================================================================
 * Principles: Zero-Abstract Communication Onion.
 * =============================================================================
 */
#include "sigma_kernel_types.h"

typedef struct {
    sigma_u16 src_port;
    sigma_u16 dst_port;
    sigma_u16 len;
    sigma_u16 checksum;
} udp_hdr_t;

typedef struct {
    sigma_u8  version_ihl;
    sigma_u8  tos;
    sigma_u16 len;
    sigma_u16 id;
    sigma_u16 flags_frag;
    sigma_u8  ttl;
    sigma_u8  proto;
    sigma_u16 checksum;
    sigma_u32 src_ip;
    sigma_u32 dst_ip;
} ip_hdr_t;

extern void e1000_send_packet(void* data, sigma_u16 len);
extern void sigma_memcpy(void* dest, const void* src, sigma_u64 n);

void net_send_udp(sigma_u32 dst_ip, sigma_u16 dst_port, void* data, sigma_u16 len) {
    sigma_u8 packet[1500];
    
    udp_hdr_t udp = { .src_port = 8080, .dst_port = dst_port, .len = (len + 8) };
    ip_hdr_t  ip  = { .src_ip = 0x0100007F, .dst_ip = dst_ip, .proto = 17, .len = (len + 28) };

    /* Construct the Onion Packet */
    sigma_memcpy(packet + 34, &udp, 8);
    sigma_memcpy(packet + 14, &ip, 20);
    sigma_memcpy(packet + 42, data, len);

    e1000_send_packet(packet, len + 42);
}
