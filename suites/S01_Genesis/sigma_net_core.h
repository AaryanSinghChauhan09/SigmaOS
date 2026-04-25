// SigmaOS — sigma-net-core: Network Stack Core
// Modularised from: SovereignNetMesh.c + net.c
// Single responsibility: raw ethernet frame send/receive primitives

#ifndef SIGMA_NET_CORE_H
#define SIGMA_NET_CORE_H

#define SIGMA_ETH_MTU       1500
#define SIGMA_ETH_HDR_LEN   14
#define SIGMA_ETH_ETYPE_IP  0x0800
#define SIGMA_ETH_ETYPE_ARP 0x0806

typedef struct SigmaEthHeader {
    unsigned char dst_mac[6];
    unsigned char src_mac[6];
    unsigned short etype;     // big-endian
} SigmaEthHeader;

typedef struct SigmaEthFrame {
    SigmaEthHeader hdr;
    unsigned char  payload[SIGMA_ETH_MTU];
    unsigned int   payload_len;
} SigmaEthFrame;

typedef struct SigmaNetStats {
    unsigned long tx_frames;
    unsigned long rx_frames;
    unsigned long tx_bytes;
    unsigned long rx_bytes;
    unsigned long rx_drops;
} SigmaNetStats;

static inline void eth_frame_init(SigmaEthFrame* f,
                                   const unsigned char* dst,
                                   const unsigned char* src,
                                   unsigned short etype) {
    for (int i = 0; i < 6; i++) { f->hdr.dst_mac[i] = dst[i]; f->hdr.src_mac[i] = src[i]; }
    // Convert to big-endian manually
    f->hdr.etype = (unsigned short)((etype >> 8) | (etype << 8));
    f->payload_len = 0;
}

static inline int eth_frame_set_payload(SigmaEthFrame* f,
                                          const unsigned char* data,
                                          unsigned int len) {
    if (len > SIGMA_ETH_MTU) return -1;
    for (unsigned int i = 0; i < len; i++) f->payload[i] = data[i];
    f->payload_len = len;
    return 0;
}

// Compute frame checksum (FNV-1a over header + payload)
static inline unsigned int eth_frame_checksum(const SigmaEthFrame* f) {
    unsigned int h = 2166136261U;
    const unsigned char* p = (const unsigned char*)&f->hdr;
    for (int i = 0; i < SIGMA_ETH_HDR_LEN; i++) { h ^= p[i]; h *= 16777619U; }
    for (unsigned int i = 0; i < f->payload_len; i++) { h ^= f->payload[i]; h *= 16777619U; }
    return h;
}

static inline void net_stats_record_tx(SigmaNetStats* s, unsigned int len) {
    s->tx_frames++; s->tx_bytes += len;
}
static inline void net_stats_record_rx(SigmaNetStats* s, unsigned int len) {
    s->rx_frames++; s->rx_bytes += len;
}
static inline void net_stats_record_drop(SigmaNetStats* s) { s->rx_drops++; }

#endif /* SIGMA_NET_CORE_H */
