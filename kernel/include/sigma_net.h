#ifndef SIGMA_NET_H
#define SIGMA_NET_H

#include "sigma_types.h"

/**
 * @file sigma_net.h
 * @brief Sovereign P2P Networking Stack (Lattice-Link)
 * 
 * Instead of centralized TCP/IP, SigmaOS prioritizes P2P mesh networking
 * via encrypted shards and DHT-based discovery.
 */

typedef struct {
    sigma_u8 peer_id[32]; // Ed25519 or PQC-equivalent
    sigma_u32 trust_score;
    sigma_u32 latency_ms;
} sigma_peer_t;

/**
 * @brief Send a packet over the Sovereign Mesh.
 * @param target_peer_id The 256-bit ID of the target shard.
 * @param buffer Data to send.
 * @param size Size of the data.
 */
extern "C" sigma_s32 sigma_net_mesh_send(const sigma_u8* target_peer_id, const void* buffer, sigma_size_t size);

/**
 * @brief Poll for incoming mesh packets.
 */
extern "C" sigma_s32 sigma_net_mesh_poll(void* buffer, sigma_size_t max_size, sigma_u8* out_peer_id);

/**
 * @brief Discover nearby Sovereign nodes using DHT.
 */
extern "C" sigma_u32 sigma_net_mesh_discover(sigma_peer_t* out_peers, sigma_u32 max_peers);

#endif // SIGMA_NET_H
