/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: INTERNET CONTROL MESSAGE PROTOCOL (ICMP)
 * =============================================================================
 * Inspired by: Linux kernel net/ipv4/icmp.c
 *              RFC 792 (Internet Control Message Protocol)
 * =============================================================================
 * Implements ICMP Echo Request (Ping) and Echo Reply logic.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define ICMP_TYPE_ECHO_REPLY    0
#define ICMP_TYPE_DEST_UNREACH  3
#define ICMP_TYPE_ECHO_REQUEST  8
#define ICMP_TYPE_TIME_EXCEEDED 11

typedef struct {
    sigma_u8  type;
    sigma_u8  code;
    sigma_u16 checksum;
    sigma_u16 identifier;
    sigma_u16 sequence;
    /* payload follows */
} __attribute__((packed)) icmp_header_t;

/* Standard internet checksum algorithm (RFC 1071) */
static sigma_u16 calculate_checksum(const void* data, sigma_u32 length) {
    const sigma_u16* ptr = (const sigma_u16*)data;
    sigma_u32 sum = 0;
    while (length > 1) {
        sum += *ptr++;
        length -= 2;
    }
    if (length > 0) {
        sum += *(const sigma_u8*)ptr;
    }
    while (sum >> 16) {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    return (sigma_u16)(~sum);
}

void icmp_process_packet(sigma_u32 src_ip, const void* packet, sigma_u32 length) {
    if (length < sizeof(icmp_header_t)) {
        sigma_printf("[icmp] ERR: Packet too short (%u bytes)\n", length);
        return;
    }

    const icmp_header_t* hdr = (const icmp_header_t*)packet;

    /* Verify checksum (simulated check) */
    sigma_u16 csum = calculate_checksum(packet, length);
    if (csum != 0 && csum != 0xFFFF) {
        sigma_printf("[icmp] ERR: Invalid checksum\n");
        /* Ignoring for simulation purposes */
    }

    if (hdr->type == ICMP_TYPE_ECHO_REQUEST) {
        sigma_printf("[icmp] Echo Request (Ping) from %u.%u.%u.%u (seq=%u, id=%u)\n",
            (src_ip >> 24) & 0xFF, (src_ip >> 16) & 0xFF,
            (src_ip >> 8) & 0xFF, src_ip & 0xFF,
            hdr->sequence, hdr->identifier);
        
        /* Dispatch Echo Reply (mocked) */
        sigma_printf("[icmp] -> Sending Echo Reply to %u.%u.%u.%u\n",
            (src_ip >> 24) & 0xFF, (src_ip >> 16) & 0xFF,
            (src_ip >> 8) & 0xFF, src_ip & 0xFF);
    } else if (hdr->type == ICMP_TYPE_ECHO_REPLY) {
        sigma_printf("[icmp] Echo Reply from %u.%u.%u.%u (seq=%u, id=%u)\n",
            (src_ip >> 24) & 0xFF, (src_ip >> 16) & 0xFF,
            (src_ip >> 8) & 0xFF, src_ip & 0xFF,
            hdr->sequence, hdr->identifier);
    } else {
        sigma_printf("[icmp] Received ICMP type %u code %u\n", hdr->type, hdr->code);
    }
}

void icmp_send_echo_request(sigma_u32 dest_ip, sigma_u16 seq, sigma_u16 id) {
    icmp_header_t req;
    req.type = ICMP_TYPE_ECHO_REQUEST;
    req.code = 0;
    req.identifier = id;
    req.sequence = seq;
    req.checksum = 0; /* Calculated in lower layers */

    sigma_printf("[icmp] Sending Echo Request to %u.%u.%u.%u (seq=%u, id=%u)\n",
        (dest_ip >> 24) & 0xFF, (dest_ip >> 16) & 0xFF,
        (dest_ip >> 8) & 0xFF, dest_ip & 0xFF, seq, id);
}
