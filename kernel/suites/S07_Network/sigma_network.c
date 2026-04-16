#include "../../include/sigma_proto.h"
#include <stdio.h>

/* =========================================================================
 * SIGMA OS: SOVEREIGN NETWORK STACK (S07)
 * Directly handles raw hardware frames without Linux socket overhead.
 * ========================================================================= */

void sigma_net_init(void) {
    printf("[NET] Initializing Sovereign Network Protocol Stack (SNPS)...\n");
}

void sigma_net_handle_packet(void* packet, uint32_t size) {
    sigma_eth_header_t* eth = (sigma_eth_header_t*)packet;
    
    // Process only IP traffic (0x0800)
    if (eth->type == 0x0008) { // Note: Endianness check usually needed
        sigma_ip_header_t* ip = (sigma_ip_header_t*)(packet + sizeof(sigma_eth_header_t));
        printf("[NET] IP Packet Detected: From 0x%x to 0x%x\n", ip->src_ip, ip->dest_ip);
    }
}
