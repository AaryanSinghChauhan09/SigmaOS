// SigmaOS — sigma-netmesh-topology: Mesh Topology Discovery
// Modularised from: SovereignNetMesh.c
// USP: Autonomous discovery of peer nodes over Layer 2 broadcasts.

#ifndef SIGMA_NETMESH_TOPOLOGY_HPP
#define SIGMA_NETMESH_TOPOLOGY_HPP

namespace sigma {
namespace netmesh {

struct MeshPeer {
    unsigned char mac[6];
    unsigned int  ip;
    unsigned int  last_seen_rdtsc;
    unsigned int  signal_strength;
};

class TopologyManager {
private:
    MeshPeer active_peers[64];
    unsigned int peer_count;

public:
    TopologyManager() : peer_count(0) {}

    // Process an incoming heartbeat from a peer
    void register_heartbeat(unsigned char peer_mac[6], unsigned int ip, unsigned int signal) {
        for (unsigned int i = 0; i < peer_count; ++i) {
            bool match = true;
            for (int j = 0; j < 6; j++) {
                if (active_peers[i].mac[j] != peer_mac[j]) { match = false; break; }
            }
            if (match) {
                active_peers[i].last_seen_rdtsc = 0; // Mock update
                active_peers[i].signal_strength = signal;
                return;
            }
        }
        
        if (peer_count < 64) {
            MeshPeer* p = &active_peers[peer_count++];
            for (int j = 0; j < 6; j++) p->mac[j] = peer_mac[j];
            p->ip = ip;
            p->signal_strength = signal;
        }
    }

    void prune_stale_peers(unsigned int timeout_rdtsc) {
        (void)timeout_rdtsc;
        // Evict peers missing heartbeats
    }
};

} // namespace netmesh
} // namespace sigma

#endif /* SIGMA_NETMESH_TOPOLOGY_HPP */
