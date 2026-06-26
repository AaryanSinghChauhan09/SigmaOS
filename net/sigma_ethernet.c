/*
 * =========================================================================
 * Σ SIGMAOS KERNEL: ETHERNET LAYER (Phase 16)
 * =========================================================================
 */

#include "../include/net/sigma_net_internal.h"
#include "../include/sigma_zenithd_log.h"

static sigma_u8 local_mac[6] = {0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E}; /* Mock MAC */

void sigma_eth_receive(const void* frame, sigma_size_t len) {
    if (len < sizeof(sigma_eth_hdr_t)) {
        ZENITH_WARN("net_eth", "Dropped undersized Ethernet frame");
        return;
    }

    const sigma_eth_hdr_t* hdr = (const sigma_eth_hdr_t*)frame;
    sigma_u16 ethertype = ntohs(hdr->ethertype);

    if (ethertype == ETHERTYPE_IPv4) {
        sigma_ipv4_receive(hdr->payload, len - sizeof(sigma_eth_hdr_t));
    } else if (ethertype == ETHERTYPE_ARP) {
        ZENITH_DEBUG("net_eth", "ARP packet received");
        /* TODO: Route to ARP module */
    } else {
        ZENITH_DEBUG("net_eth", "Unknown EtherType received");
    }
}

void sigma_eth_send(const sigma_u8 dest_mac[6], sigma_u16 ethertype, const void* payload, sigma_size_t len) {
    /* In a real implementation, this allocates a buffer, prepends the eth header, 
       and calls the NIC driver's tx ring buffer function. */
    sigma_u8 buffer[1514];
    if (len + sizeof(sigma_eth_hdr_t) > sizeof(buffer)) {
        ZENITH_ERROR(0, "net_eth", "Frame too large to send");
        return;
    }

    sigma_eth_hdr_t* hdr = (sigma_eth_hdr_t*)buffer;
    sigma_memcpy(hdr->dest_mac, dest_mac, 6);
    sigma_memcpy(hdr->src_mac, local_mac, 6);
    hdr->ethertype = htons(ethertype);
    sigma_memcpy(hdr->payload, payload, len);

    ZENITH_TRACE("net_eth", "Ethernet frame sent to NIC");
    /* sigma_nic_transmit(buffer, len + sizeof(sigma_eth_hdr_t)); */
}
