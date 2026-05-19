/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: USER DATAGRAM PROTOCOL (UDP)
 * =============================================================================
 * Inspired by: Linux kernel net/ipv4/udp.c
 *              RFC 768 (User Datagram Protocol)
 * =============================================================================
 * Implements connectionless, unreliable datagram transmission and multiplexing.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define UDP_MAX_PORTS 65536
#define UDP_MAX_SOCKETS 128

typedef struct {
    sigma_u16 src_port;
    sigma_u16 dst_port;
    sigma_u16 length;
    sigma_u16 checksum;
    /* payload follows */
} __attribute__((packed)) udp_header_t;

typedef void (*udp_recv_cb_t)(sigma_u32 src_ip, sigma_u16 src_port, const void* data, sigma_u16 len);

typedef struct {
    sigma_u16       local_port;
    udp_recv_cb_t   recv_cb;
    sigma_bool      active;
} udp_socket_t;

static udp_socket_t udp_sockets[UDP_MAX_SOCKETS];

void udp_init(void) {
    sigma_memset(udp_sockets, 0, sizeof(udp_sockets));
    sigma_printf("[udp] User Datagram Protocol initialized\n");
}

int udp_bind(sigma_u16 port, udp_recv_cb_t cb) {
    if (port == 0) return -1;
    
    /* Check if already bound */
    for (sigma_u32 i = 0; i < UDP_MAX_SOCKETS; i++) {
        if (udp_sockets[i].active && udp_sockets[i].local_port == port) {
            sigma_printf("[udp] ERR: Port %u already bound\n", port);
            return -1;
        }
    }
    
    for (sigma_u32 i = 0; i < UDP_MAX_SOCKETS; i++) {
        if (!udp_sockets[i].active) {
            udp_sockets[i].local_port = port;
            udp_sockets[i].recv_cb    = cb;
            udp_sockets[i].active     = SIGMA_TRUE;
            sigma_printf("[udp] Bound to port %u\n", port);
            return 0;
        }
    }
    sigma_printf("[udp] ERR: Max sockets reached\n");
    return -1;
}

void udp_unbind(sigma_u16 port) {
    for (sigma_u32 i = 0; i < UDP_MAX_SOCKETS; i++) {
        if (udp_sockets[i].active && udp_sockets[i].local_port == port) {
            udp_sockets[i].active = SIGMA_FALSE;
            sigma_printf("[udp] Unbound port %u\n", port);
            return;
        }
    }
}

void udp_process_packet(sigma_u32 src_ip, const void* packet, sigma_u32 length) {
    if (length < sizeof(udp_header_t)) {
        sigma_printf("[udp] ERR: Packet too short (%u bytes)\n", length);
        return;
    }
    
    const udp_header_t* hdr = (const udp_header_t*)packet;
    
    /* Convert endianness (simulated - assuming big endian network byte order) */
    sigma_u16 src_port = (hdr->src_port >> 8) | (hdr->src_port << 8);
    sigma_u16 dst_port = (hdr->dst_port >> 8) | (hdr->dst_port << 8);
    sigma_u16 udp_len  = (hdr->length >> 8) | (hdr->length << 8);
    
    if (udp_len > length) {
        sigma_printf("[udp] ERR: Length mismatch (hdr %u, actual %u)\n", udp_len, length);
        return;
    }
    
    sigma_printf("[udp] Received packet %u -> %u (%u bytes)\n", src_port, dst_port, udp_len);
    
    /* Dispatch to bound socket */
    for (sigma_u32 i = 0; i < UDP_MAX_SOCKETS; i++) {
        if (udp_sockets[i].active && udp_sockets[i].local_port == dst_port) {
            if (udp_sockets[i].recv_cb) {
                const sigma_u8* payload = (const sigma_u8*)packet + sizeof(udp_header_t);
                udp_sockets[i].recv_cb(src_ip, src_port, payload, udp_len - sizeof(udp_header_t));
            }
            return;
        }
    }
    
    sigma_printf("[udp] No socket listening on port %u (Dropping)\n", dst_port);
}

void udp_send(sigma_u32 dst_ip, sigma_u16 src_port, sigma_u16 dst_port, const void* data, sigma_u16 len) {
    sigma_printf("[udp] Sending %u bytes: %u -> %u to %u.%u.%u.%u\n",
                 len, src_port, dst_port,
                 (dst_ip >> 24) & 0xFF, (dst_ip >> 16) & 0xFF,
                 (dst_ip >> 8) & 0xFF, dst_ip & 0xFF);
                 
    /* In a real kernel, this would construct the UDP/IP header and hand off to the Link Layer */
}
