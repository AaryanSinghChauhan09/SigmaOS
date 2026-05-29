/*
 * =========================================================================
 * Σ SIGMAOS KERNEL: NETWORK INTERNAL DEFINITIONS (Phase 16)
 * =========================================================================
 * Internal structures for the sovereign TCP/IP stack.
 * =========================================================================
 */

#ifndef SIGMA_NET_INTERNAL_H
#define SIGMA_NET_INTERNAL_H

#include "../sigma_kernel_types.h"
#include "../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ---- Endianness Helpers (assuming x86_64 LE) ---- */
static inline sigma_u16 htons(sigma_u16 v) {
    return (v >> 8) | (v << 8);
}
#define ntohs(v) htons(v)

static inline sigma_u32 htonl(sigma_u32 v) {
    return ((v & 0xFF) << 24) | ((v & 0xFF00) << 8) |
           ((v & 0xFF0000) >> 8) | ((v >> 24) & 0xFF);
}
#define ntohl(v) htonl(v)

/* ---- Ethernet ---- */
#define ETHERTYPE_IPv4 0x0800
#define ETHERTYPE_ARP  0x0806

typedef struct __attribute__((packed)) {
    sigma_u8  dest_mac[6];
    sigma_u8  src_mac[6];
    sigma_u16 ethertype;
    sigma_u8  payload[];
} sigma_eth_hdr_t;

/* ---- IPv4 ---- */
typedef struct __attribute__((packed)) {
    sigma_u8  ihl : 4;
    sigma_u8  version : 4;
    sigma_u8  tos;
    sigma_u16 total_len;
    sigma_u16 id;
    sigma_u16 frag_offset;
    sigma_u8  ttl;
    sigma_u8  protocol;
    sigma_u16 checksum;
    sigma_u32 src_ip;
    sigma_u32 dst_ip;
    sigma_u8  payload[];
} sigma_ipv4_hdr_t;

/* ---- TCP ---- */
typedef struct __attribute__((packed)) {
    sigma_u16 src_port;
    sigma_u16 dst_port;
    sigma_u32 seq_num;
    sigma_u32 ack_num;
    sigma_u16 flags;      /* Includes data offset (4 bits), reserved (3 bits), flags (9 bits) */
    sigma_u16 window;
    sigma_u16 checksum;
    sigma_u16 urgent_ptr;
    sigma_u8  payload[];
} sigma_tcp_hdr_t;

/* TCP Pseudo Header for Checksum */
typedef struct __attribute__((packed)) {
    sigma_u32 src_ip;
    sigma_u32 dst_ip;
    sigma_u8  reserved;
    sigma_u8  protocol;
    sigma_u16 tcp_length;
} sigma_tcp_pseudo_hdr_t;

/* ---- Core Functions ---- */

sigma_u16 sigma_inet_cksum(const void* data, sigma_size_t len);

void sigma_eth_receive(const void* frame, sigma_size_t len);
void sigma_eth_send(const sigma_u8 dest_mac[6], sigma_u16 ethertype, const void* payload, sigma_size_t len);

void sigma_ipv4_receive(const void* packet, sigma_size_t len);
void sigma_ipv4_send(sigma_u32 dst_ip, sigma_u8 protocol, const void* payload, sigma_size_t len);

void sigma_tcp_process_packet(sigma_u32 src_ip, sigma_u16 src_port, sigma_u32 dst_ip, sigma_u16 dst_port, sigma_u16 flags, sigma_u32 seq, sigma_u32 ack, const void* payload, sigma_size_t payload_len);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NET_INTERNAL_H */
