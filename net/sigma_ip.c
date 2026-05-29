/*
 * =========================================================================
 * Σ SIGMAOS KERNEL: IPv4 LAYER (Phase 16)
 * =========================================================================
 */

#include "../include/net/sigma_net_internal.h"
#include "../include/sigma_zenithd_log.h"

sigma_u16 sigma_inet_cksum(const void* data, sigma_size_t len) {
    const sigma_u16* p = (const sigma_u16*)data;
    sigma_u32 sum = 0;

    while (len > 1) {
        sum += *p++;
        len -= 2;
    }
    if (len > 0) {
        sum += *(const sigma_u8*)p;
    }

    while (sum >> 16) {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    return (sigma_u16)(~sum);
}

void sigma_ipv4_receive(const void* packet, sigma_size_t len) {
    if (len < sizeof(sigma_ipv4_hdr_t)) {
        ZENITH_WARN("net_ipv4", "Dropped undersized IPv4 packet");
        return;
    }

    const sigma_ipv4_hdr_t* hdr = (const sigma_ipv4_hdr_t*)packet;
    
    if (hdr->version != 4) {
        ZENITH_WARN("net_ipv4", "Dropped non-IPv4 packet in IPv4 handler");
        return;
    }

    /* Verify header checksum */
    sigma_u32 header_len = hdr->ihl * 4;
    if (sigma_inet_cksum(hdr, header_len) != 0) {
        ZENITH_WARN("net_ipv4", "Dropped IPv4 packet with bad checksum");
        return;
    }

    sigma_u16 total_len = ntohs(hdr->total_len);
    if (total_len > len) {
        ZENITH_WARN("net_ipv4", "Dropped truncated IPv4 packet");
        return;
    }

    if (hdr->protocol == 6) { /* TCP */
        const sigma_tcp_hdr_t* tcp_hdr = (const sigma_tcp_hdr_t*)(hdr->payload);
        /* Note: Minimal extraction, real TCP needs full checksum validation including pseudo header */
        sigma_tcp_process_packet(ntohl(hdr->src_ip), ntohs(tcp_hdr->src_port),
                                 ntohl(hdr->dst_ip), ntohs(tcp_hdr->dst_port),
                                 ntohs(tcp_hdr->flags), ntohl(tcp_hdr->seq_num), ntohl(tcp_hdr->ack_num),
                                 tcp_hdr->payload, total_len - header_len - ((ntohs(tcp_hdr->flags) >> 12) * 4));
    } else {
        ZENITH_TRACE("net_ipv4", "Unsupported IP protocol");
    }
}

void sigma_ipv4_send(sigma_u32 dst_ip, sigma_u8 protocol, const void* payload, sigma_size_t len) {
    /* Construct IPv4 Header and route (ARP lookup etc) */
    sigma_u8 dest_mac[6] = {0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF}; /* Broadcast for now */
    
    sigma_u8 buffer[1500];
    sigma_ipv4_hdr_t* hdr = (sigma_ipv4_hdr_t*)buffer;
    
    hdr->version = 4;
    hdr->ihl = 5;
    hdr->tos = 0;
    hdr->total_len = htons(sizeof(sigma_ipv4_hdr_t) + len);
    hdr->id = htons(1); /* Static for now */
    hdr->frag_offset = 0;
    hdr->ttl = 64;
    hdr->protocol = protocol;
    hdr->src_ip = htonl(0x00000000); /* 0.0.0.0 */
    hdr->dst_ip = htonl(dst_ip);
    hdr->checksum = 0;
    hdr->checksum = sigma_inet_cksum(hdr, sizeof(sigma_ipv4_hdr_t));
    
    sigma_memcpy(hdr->payload, payload, len);
    
    sigma_eth_send(dest_mac, ETHERTYPE_IPv4, buffer, sizeof(sigma_ipv4_hdr_t) + len);
}
