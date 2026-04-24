#include "sigma_libc.h"
#include "sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Mesh Networking Module
// Peer-to-peer sovereign network formation
// ---------------------------------------------------------

#define MAX_PEERS    64
#define MAX_ROUTES   128
#define NODE_ID_LEN  16

typedef struct {
    uint8_t  node_id[NODE_ID_LEN];  // Unique 128-bit node identity
    uint32_t ip_addr;               // Last known IP
    uint16_t port;
    uint32_t last_seen_tick;
    uint8_t  public_key[32];        // Ed25519 public key for zero-trust
    uint8_t  alive;
} mesh_peer_t;

typedef struct {
    uint8_t  dest_node[NODE_ID_LEN];
    uint8_t  via_node[NODE_ID_LEN]; // Next hop
    uint32_t metric;                // Hop count / latency score
} mesh_route_t;

static mesh_peer_t peers[MAX_PEERS];
static mesh_route_t routes[MAX_ROUTES];
static uint32_t peer_count = 0;
static uint32_t route_count = 0;
static uint8_t  my_node_id[NODE_ID_LEN];

void mesh_init(const uint8_t* node_id) {
    memcpy(my_node_id, node_id, NODE_ID_LEN);
    peer_count = 0;
    route_count = 0;
}

// Register a discovered peer (via broadcast or discovery beacon)
int mesh_add_peer(const uint8_t* node_id, uint32_t ip, uint16_t port, const uint8_t* pubkey) {
    if (peer_count >= MAX_PEERS) return -1;
    mesh_peer_t* p = &peers[peer_count++];
    memcpy(p->node_id, node_id, NODE_ID_LEN);
    p->ip_addr = ip;
    p->port = port;
    p->alive = 1;
    p->last_seen_tick = 0;
    memcpy(p->public_key, pubkey, 32);
    return 0;
}

// Add a routing entry
int mesh_add_route(const uint8_t* dest, const uint8_t* via, uint32_t metric) {
    if (route_count >= MAX_ROUTES) return -1;
    mesh_route_t* r = &routes[route_count++];
    memcpy(r->dest_node, dest, NODE_ID_LEN);
    memcpy(r->via_node, via, NODE_ID_LEN);
    r->metric = metric;
    return 0;
}

// Route a packet to a destination node (Dijkstra-stub — returns next hop)
const mesh_peer_t* mesh_route_to(const uint8_t* dest_node) {
    uint32_t best_metric = UINT32_MAX;
    const uint8_t* best_via = NULL;

    for (uint32_t i = 0; i < route_count; i++) {
        if (memcmp(routes[i].dest_node, dest_node, NODE_ID_LEN) == 0) {
            if (routes[i].metric < best_metric) {
                best_metric = routes[i].metric;
                best_via = routes[i].via_node;
            }
        }
    }
    if (!best_via) return NULL;

    for (uint32_t i = 0; i < peer_count; i++) {
        if (memcmp(peers[i].node_id, best_via, NODE_ID_LEN) == 0)
            return &peers[i];
    }
    return NULL;
}
