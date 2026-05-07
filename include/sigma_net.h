#ifndef SIGMA_NET_H
#define SIGMA_NET_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 interface_id;
    sigma_u32 mtu;
    sigma_u32 flags;
} sigma_net_config_t;

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
/* --- Networking Primitives --- */
void net_init(const sigma_net_config_t* config);
void net_process_packet(sigma_packet_t* pkt);
bool net_transmit_shard(uint32_t target_ip, uint32_t shard_id);

#ifdef __cplusplus
}

class SovereignNetStackEngine {
public:
    static SovereignNetStackEngine& getInstance() {
        static SovereignNetStackEngine instance;
        return instance;
    }

    void init(const sigma_net_config_t* config);
    void sendPacket(const void* data, sigma_u32 len);
    void receivePacket(void* buffer, sigma_u32* len);
    void reportStats() const;

private:
    SovereignNetStackEngine() : packets_sent(0), packets_received(0), initialized(0), firewall_enabled(true) {}
    
    sigma_net_config_t config;
    sigma_u32          packets_sent;
    sigma_u32          packets_received;
    sigma_u32          initialized;
    bool               firewall_enabled;
};
#endif

#endif /* SIGMA_NET_H */
