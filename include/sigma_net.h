/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NETWORKING STACK (SILICON-DIRECT)
 * =========================================================================
 * Mission: Zero-buffer, shard-mapped packet processing.
 * =========================================================================
 */

#ifndef SIGMA_NET_H
#define SIGMA_NET_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u8 mac[6];
    sigma_u32 ip;
    uint32_t bound_shard_id;
} sigma_net_interface_t;

typedef struct {
    uint32_t src_ip;
    uint32_t dst_ip;
    uint16_t src_port;
    uint16_t dst_port;
    uint32_t shard_payload_id;
} sigma_packet_t;

/* --- Networking Primitives --- */
void net_init(void);
void net_process_packet(sigma_packet_t* pkt);
bool net_transmit_shard(uint32_t target_ip, uint32_t shard_id);

/* --- Sovereign Stack (ZBPA) Primitives --- */
void     netstack_init(void);
void     netstack_process_packet(const void* buffer, sigma_u32 size);
void     netstack_send_packet(const void* buffer, sigma_u32 size);
sigma_u32 netstack_is_link_active(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NET_H */
