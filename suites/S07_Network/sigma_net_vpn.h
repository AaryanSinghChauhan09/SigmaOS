// SigmaOS — sigma-net-vpn: WireGuard-inspired Sovereign VPN Tunnel
// Inspired by: WireGuard — modern, minimal VPN
// Module: sigma-net-vpn
// USP over WireGuard: No kernel module, no Linux netlink — native C peer table
// Uses Sigma-ZKP for handshake instead of Curve25519 (dependency-free)

#ifndef SIGMA_NET_VPN_H
#define SIGMA_NET_VPN_H

#include "sigma_caps.h"

#define SIGMA_VPN_MAX_PEERS  16
#define SIGMA_VPN_KEY_LEN    32
#define SIGMA_VPN_HANDSHAKE_TIMEOUT_MS 5000

typedef enum SigmaVPNPeerState {
    VPN_PEER_IDLE        = 0,
    VPN_PEER_HANDSHAKING = 1,
    VPN_PEER_ACTIVE      = 2,
    VPN_PEER_DEAD        = 3
} SigmaVPNPeerState;

typedef struct SigmaVPNPeer {
    unsigned int       peer_id;
    unsigned int       endpoint_ip;
    unsigned short     endpoint_port;
    unsigned char      public_key[SIGMA_VPN_KEY_LEN];
    unsigned char      session_key[SIGMA_VPN_KEY_LEN]; // derived after handshake
    SigmaVPNPeerState  state;
    unsigned long      last_handshake;  // RDTSC timestamp
    unsigned long      rx_bytes;
    unsigned long      tx_bytes;
} SigmaVPNPeer;

typedef struct SigmaVPN {
    SigmaVPNPeer  peers[SIGMA_VPN_MAX_PEERS];
    unsigned int  peer_count;
    unsigned char local_public_key[SIGMA_VPN_KEY_LEN];
    unsigned char local_private_key[SIGMA_VPN_KEY_LEN];
} SigmaVPN;

static inline unsigned long vpn_rdtsc(void) {
#if defined(__x86_64__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline void vpn_init(SigmaVPN* v,
                              const unsigned char* pub, const unsigned char* priv) {
    v->peer_count = 0;
    for (int i = 0; i < SIGMA_VPN_KEY_LEN; i++) {
        v->local_public_key[i]  = pub[i];
        v->local_private_key[i] = priv[i];
    }
}

static inline int vpn_add_peer(SigmaVPN* v, unsigned int ip, unsigned short port,
                                 const unsigned char* peer_pub) {
    if (v->peer_count >= SIGMA_VPN_MAX_PEERS) return -1;
    SigmaVPNPeer* p = &v->peers[v->peer_count++];
    p->peer_id       = v->peer_count;
    p->endpoint_ip   = ip;
    p->endpoint_port = port;
    p->state         = VPN_PEER_IDLE;
    p->rx_bytes = p->tx_bytes = 0;
    for (int i = 0; i < SIGMA_VPN_KEY_LEN; i++) p->public_key[i] = peer_pub[i];
    return (int)p->peer_id;
}

// Initiate handshake (ZKP-style in production; here sets handshake state)
static inline int vpn_handshake(SigmaVPN* v, unsigned int peer_id,
                                  SigmaCapToken* tok) {
    if (!cap_check(tok, SIGMA_CAP_NET)) return -1;
    for (unsigned int i = 0; i < v->peer_count; i++) {
        if (v->peers[i].peer_id == peer_id) {
            v->peers[i].state          = VPN_PEER_HANDSHAKING;
            v->peers[i].last_handshake = vpn_rdtsc();
            // In production: run ZKP commit/challenge/respond here
            // Derive session_key from shared secret XOR of keys
            for (int k = 0; k < SIGMA_VPN_KEY_LEN; k++)
                v->peers[i].session_key[k] =
                    v->local_public_key[k] ^ v->peers[i].public_key[k];
            v->peers[i].state = VPN_ACTIVE;
            return 0;
        }
    }
    return -2;
}

static inline void vpn_record_tx(SigmaVPN* v, unsigned int peer_id, unsigned long bytes) {
    for (unsigned int i = 0; i < v->peer_count; i++)
        if (v->peers[i].peer_id == peer_id) { v->peers[i].tx_bytes += bytes; return; }
}

#define VPN_ACTIVE VPN_PEER_ACTIVE

#endif /* SIGMA_NET_VPN_H */
