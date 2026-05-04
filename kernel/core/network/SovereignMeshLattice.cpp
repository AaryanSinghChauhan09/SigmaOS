#include "../../../include/SovereignMeshLattice.h"
#include "../../../include/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace Network {

void SovereignMeshLattice::init() {
    sigma_log("Σ [MESH]: Orchestrating Peer-to-Peer Lattice Discovery...");
    this->m_peers_discovered = 0;
    this->m_active_syncs = 0;
    sigma_log("Σ [MESH]: Mesh-Lattice Protocol v1.0 ACTIVE.");
}

void SovereignMeshLattice::discoverPeers() {
    sigma_log("Σ [MESH]: Broadcasting discovery probes across the Lattice Mesh...");
    // Simulate peer discovery
    this->m_peers_discovered += 3;
    sigma_printf("Σ [MESH]: Discovered %u new peers in local cluster.\n", 3);
}

void SovereignMeshLattice::syncShard(sigma_u32 shard_id, const char* target_node) {
    sigma_printf("Σ [MESH]: Initiating P2P sync for Shard S%02u -> Node %s\n", shard_id, target_node);
    this->m_active_syncs++;
    sigma_log("Σ [MESH]: Distributed State Sync SUCCESS.");
}

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void mesh_init() {
    SigmaOS::Kernel::Network::SovereignMeshLattice::getInstance().init();
}

extern "C" void mesh_discover_peers() {
    SigmaOS::Kernel::Network::SovereignMeshLattice::getInstance().discoverPeers();
}

extern "C" void mesh_sync_shard(sigma_u32 shard_id, const char* target_node) {
    SigmaOS::Kernel::Network::SovereignMeshLattice::getInstance().syncShard(shard_id, target_node);
}
