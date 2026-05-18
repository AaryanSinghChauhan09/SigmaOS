/* =============================================================================
 * SigmaOS Global Lattice Mesh — S33 (Phase 6: Omnipresence)
 * =============================================================================
 * P2P state synchronization across multiple SigmaOS instances.
 * Architecture: Chord DHT overlay with CRDTs for conflict-free state merging.
 * Each Lattice Node maintains:
 *   - A unique Node ID (SHA-256 of MAC + boot epoch)
 *   - A partial keyspace of the global shard state table
 *   - A gossip fan-out of 3 peers for low-latency propagation
 * ============================================================================= */

#ifndef SIGMA_GLOBAL_LATTICE_MESH_H
#define SIGMA_GLOBAL_LATTICE_MESH_H

#include "sigma_kernel_types.h"

/* ── Node Identity ────────────────────────────────────────────────────────── */
#define SIGMA_NODE_ID_LEN   32u      /* SHA-256 length in bytes              */
#define SIGMA_MAX_PEERS     64u      /* Maximum known peers per node         */
#define SIGMA_GOSSIP_FANOUT 3u       /* Peers to forward each state update   */
#define SIGMA_MESH_VERSION  0x0100u  /* Protocol version: 1.0                */

typedef struct sigma_node_id {
    u8 bytes[SIGMA_NODE_ID_LEN];
} sigma_node_id_t;

/* ── Shard State Entry (CRDT G-Counter per shard) ────────────────────────── */
typedef struct sigma_shard_state {
    u32 shard_id;
    u64 version;          /* Lamport timestamp                               */
    u64 checksum;         /* FNV-1a hash of shard memory region              */
    u8  status;           /* 0=offline 1=online 2=degraded 3=evolving        */
    u8  _pad[7];
} sigma_shard_state_t;

/* ── Peer Descriptor ─────────────────────────────────────────────────────── */
typedef struct sigma_mesh_peer {
    sigma_node_id_t  id;
    u32              ipv4;          /* Network byte order                    */
    u16              port;
    u16              flags;         /* 0x1=reachable 0x2=trusted 0x4=relay   */
    u64              last_seen_ns;  /* Nanosecond epoch timestamp            */
    u32              rtt_us;        /* Round-trip time in microseconds       */
    u8               _pad[4];
} sigma_mesh_peer_t;

/* ── Gossip Message ──────────────────────────────────────────────────────── */
#define SIGMA_GOSSIP_MAGIC  0x53474D53u  /* "SGMS" */
#define SIGMA_GOSSIP_MAX_SHARDS 32u

typedef struct sigma_gossip_msg {
    u32                  magic;         /* SIGMA_GOSSIP_MAGIC                */
    u16                  version;       /* SIGMA_MESH_VERSION                */
    u16                  shard_count;
    sigma_node_id_t      sender_id;
    u64                  send_epoch_ns;
    sigma_shard_state_t  shards[SIGMA_GOSSIP_MAX_SHARDS];
    u64                  hmac;          /* Truncated HMAC-SHA256 of payload  */
} sigma_gossip_msg_t;

/* ── Mesh Node (local instance) ─────────────────────────────────────────── */
typedef struct sigma_lattice_node {
    sigma_node_id_t    self_id;
    sigma_mesh_peer_t  peers[SIGMA_MAX_PEERS];
    u32                peer_count;
    u32                local_shard_count;
    sigma_shard_state_t* shard_table;    /* Pointer to kernel shard registry */
    u64                epoch_ns;
    u32                flags;            /* 0x1=bootstrap 0x2=synced         */
    u8                 _pad[4];
} sigma_lattice_node_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
k_status sigma_mesh_init(sigma_lattice_node_t* node);
k_status sigma_mesh_join(sigma_lattice_node_t* node, u32 bootstrap_ipv4, u16 port);
k_status sigma_mesh_sync(sigma_lattice_node_t* node);
k_status sigma_mesh_gossip(sigma_lattice_node_t* node);
k_status sigma_mesh_leave(sigma_lattice_node_t* node);

void sigma_mesh_dump_peers(const sigma_lattice_node_t* node);
u64  sigma_mesh_fnv1a(const u8* data, u32 len);
void sigma_mesh_gen_node_id(sigma_node_id_t* out, u64 seed);

#endif /* SIGMA_GLOBAL_LATTICE_MESH_H */
