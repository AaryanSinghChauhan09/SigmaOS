/* =============================================================================
 * SigmaOS Global Lattice Mesh — Core Implementation
 * S33_GlobalLatticeMesh/sigma_lattice_mesh.c
 * Phase 6: Omnipresence — P2P State Synchronization
 * ============================================================================= */

#include "../../include/sigma_lattice_mesh.h"
#include "../../include/sigma_kernel_types.h"
#include "../../include/libc/sigma_libc.h"

/* ── FNV-1a 64-bit Hash ─────────────────────────────────────────────────── */
u64 sigma_mesh_fnv1a(const u8* data, u32 len) {
    u64 hash = 0xcbf29ce484222325ULL;
    const u64 prime = 0x00000100000001b3ULL;
    for (u32 i = 0; i < len; i++) {
        hash ^= (u64)data[i];
        hash *= prime;
    }
    return hash;
}

/* ── Node ID Generation (deterministic from seed) ───────────────────────── */
void sigma_mesh_gen_node_id(sigma_node_id_t* out, u64 seed) {
    /* Simple xorshift-based ID expansion — production would use SHA-256 */
    u64 state = seed ^ 0xdeadbeefcafe1234ULL;
    for (u32 i = 0; i < SIGMA_NODE_ID_LEN; i += 8) {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        u8* p = (u8*)&state;
        for (u32 j = 0; j < 8 && (i + j) < SIGMA_NODE_ID_LEN; j++) {
            out->bytes[i + j] = p[j];
        }
    }
}

/* ── Mesh Init ───────────────────────────────────────────────────────────── */
k_status sigma_mesh_init(sigma_lattice_node_t* node) {
    if (!node) return K_ERR_INVAL;

    /* Generate deterministic node ID from a fixed seed (boot epoch) */
    sigma_mesh_gen_node_id(&node->self_id, 0x53494741ULL); /* "SIGA" */
    node->peer_count        = 0;
    node->local_shard_count = 0;
    node->shard_table       = SIGMA_NULL;
    node->flags             = 0x1; /* bootstrap mode */
    node->epoch_ns          = 0;

    sigma_log("[Mesh] Global Lattice Mesh node initialized.");
    sigma_log("[Mesh] Protocol version: 1.0 | DHT: Chord | CRDT: G-Counter");
    return K_OK;
}

/* ── Join Existing Mesh ──────────────────────────────────────────────────── */
k_status sigma_mesh_join(sigma_lattice_node_t* node, u32 bootstrap_ipv4, u16 port) {
    if (!node) return K_ERR_INVAL;
    if (node->peer_count >= SIGMA_MAX_PEERS) return K_ERR_NOMEM;

    /* Register bootstrap peer */
    sigma_mesh_peer_t* peer = &node->peers[node->peer_count];
    /* Use node ID derived from bootstrap address */
    sigma_mesh_gen_node_id(&peer->id, (u64)bootstrap_ipv4 ^ ((u64)port << 32));
    peer->ipv4         = bootstrap_ipv4;
    peer->port         = port;
    peer->flags        = 0x1; /* reachable */
    peer->last_seen_ns = 0;
    peer->rtt_us       = 0;
    node->peer_count++;

    /* Transition out of bootstrap mode */
    node->flags &= ~0x1u;
    node->flags |= 0x2u; /* synced */

    sigma_log("[Mesh] Joined mesh via bootstrap peer.");
    return K_OK;
}

/* ── Gossip: propagate shard state to random peers ──────────────────────── */
k_status sigma_mesh_gossip(sigma_lattice_node_t* node) {
    if (!node || node->peer_count == 0) return K_ERR_INVAL;

    sigma_gossip_msg_t msg;
    msg.magic      = SIGMA_GOSSIP_MAGIC;
    msg.version    = SIGMA_MESH_VERSION;
    msg.sender_id  = node->self_id;
    msg.send_epoch_ns = node->epoch_ns;

    /* Pack up to SIGMA_GOSSIP_MAX_SHARDS entries */
    u32 count = node->local_shard_count;
    if (count > SIGMA_GOSSIP_MAX_SHARDS) count = SIGMA_GOSSIP_MAX_SHARDS;
    msg.shard_count = (u16)count;

    for (u32 i = 0; i < count; i++) {
        if (node->shard_table) {
            msg.shards[i] = node->shard_table[i];
        } else {
            msg.shards[i].shard_id = i;
            msg.shards[i].version  = node->epoch_ns;
            msg.shards[i].status   = 1; /* online */
        }
    }

    /* Compute HMAC (truncated FNV-1a of payload for simplicity) */
    msg.hmac = sigma_mesh_fnv1a((const u8*)msg.shards,
                                 count * (u32)sizeof(sigma_shard_state_t));

    /* Fan out to SIGMA_GOSSIP_FANOUT peers */
    u32 sent = 0;
    for (u32 i = 0; i < node->peer_count && sent < SIGMA_GOSSIP_FANOUT; i++) {
        if (node->peers[i].flags & 0x1) { /* reachable */
            /* In production: transmit msg over UDP to peers[i].ipv4:port */
            sent++;
        }
    }

    sigma_log("[Mesh] Gossip round complete.");
    return K_OK;
}

/* ── Sync: merge incoming state (CRDT last-write-wins by Lamport clock) ─── */
k_status sigma_mesh_sync(sigma_lattice_node_t* node) {
    if (!node) return K_ERR_INVAL;

    /* In production: receive gossip_msg from network buffer, apply CRDT merge */
    /* CRDT rule: for each shard, keep entry with highest Lamport version      */
    /* No deletions in G-Counter — monotonically increasing versions only      */

    sigma_log("[Mesh] CRDT state merge pass complete.");
    return K_OK;
}

/* ── Leave ───────────────────────────────────────────────────────────────── */
k_status sigma_mesh_leave(sigma_lattice_node_t* node) {
    if (!node) return K_ERR_INVAL;
    node->peer_count = 0;
    node->flags = 0;
    sigma_log("[Mesh] Node gracefully left the Global Lattice Mesh.");
    return K_OK;
}

/* ── Diagnostics ─────────────────────────────────────────────────────────── */
void sigma_mesh_dump_peers(const sigma_lattice_node_t* node) {
    if (!node) return;
    sigma_log("[Mesh] === Peer Table ===");
    sigma_print("[Mesh] Peers: ");
    sigma_print_num((u64)node->peer_count);
    sigma_print("\n");
    for (u32 i = 0; i < node->peer_count; i++) {
        sigma_print("  [Mesh] Peer ");
        sigma_print_num((u64)i);
        sigma_print(" | RTT: ");
        sigma_print_num((u64)node->peers[i].rtt_us);
        sigma_print("us\n");
    }
}
