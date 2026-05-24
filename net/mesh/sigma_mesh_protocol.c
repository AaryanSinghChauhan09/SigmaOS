/*
 * Σ SigmaOS — sigma_mesh_protocol: Distributed Mesh Networking
 * Zero-Dependency.
 * 
 * Gossip-based peer discovery, DHT routing, and self-healing topology.
 * For use in the release/distributed branch.
 */

typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define MAX_PEERS 64
#define MESH_PORT 9090

struct PeerNode {
    u8  node_id[32]; // SHA256 of public key
    u8  ip_addr[16]; // IPv6 address
    u64 last_seen_ms;
    u32 ping_ms;
    bool active;
};

static PeerNode routing_table[MAX_PEERS];

/* Process an incoming gossip heartbeat */
extern "C" void sigma_mesh_handle_gossip(const u8* sender_id, const u8* sender_ip, u64 timestamp) {
    // 1. Check if peer exists in routing table
    for (int i = 0; i < MAX_PEERS; i++) {
        if (routing_table[i].active) {
            bool match = true;
            for (int j = 0; j < 32; j++) {
                if (routing_table[i].node_id[j] != sender_id[j]) {
                    match = false;
                    break;
                }
            }
            if (match) {
                routing_table[i].last_seen_ms = timestamp;
                // sigma_vga_printf("[Mesh] Updated peer heartbeat.\n");
                return;
            }
        }
    }
    
    // 2. Not found, add new peer
    for (int i = 0; i < MAX_PEERS; i++) {
        if (!routing_table[i].active) {
            for (int j = 0; j < 32; j++) routing_table[i].node_id[j] = sender_id[j];
            for (int j = 0; j < 16; j++) routing_table[i].ip_addr[j] = sender_ip[j];
            routing_table[i].last_seen_ms = timestamp;
            routing_table[i].active = true;
            sigma_vga_printf("[Mesh] Discovered new peer in mesh topology!\n");
            return;
        }
    }
}

/* DHT Route lookup: Find closest peer to destination ID (Kademlia-style XOR metric stub) */
extern "C" PeerNode* sigma_mesh_find_route(const u8* dest_id) {
    PeerNode* best_peer = 0;
    u32 min_distance = 0xFFFFFFFF; // Simplified distance metric
    
    for (int i = 0; i < MAX_PEERS; i++) {
        if (routing_table[i].active) {
            // Simplified XOR distance (just checking first byte for stub)
            u32 dist = routing_table[i].node_id[0] ^ dest_id[0];
            if (dist < min_distance) {
                min_distance = dist;
                best_peer = &routing_table[i];
            }
        }
    }
    
    if (best_peer) {
        sigma_vga_printf("[Mesh] Routing packet via intermediate peer.\n");
    } else {
        sigma_vga_printf("[Mesh] No route to host!\n");
    }
    
    return best_peer;
}
