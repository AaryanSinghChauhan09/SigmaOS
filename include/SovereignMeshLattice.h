#include "core/sigma_types.h"
#ifndef SOVEREIGN_MESH_LATTICE_H
#define SOVEREIGN_MESH_LATTICE_H

#include "core/sigma_kernel_types.h"
#include "sigma_mesh_types.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Network {

class SovereignMeshLattice : public SigmaObject {
public:
    static SovereignMeshLattice& getInstance();
    
    const char* type_name() const noexcept override { return "SovereignMeshLattice"; }
    
    void init();
    void discoverPeers();
    void syncShard(sigma_u32 shard_id, const char* target_node);
    sigma_u32 getPeerCount() const { return m_peers_discovered; }

private:
    SovereignMeshLattice() : m_peers_discovered(0), m_active_syncs(0) {}
    sigma_u32 m_peers_discovered;
    sigma_u32 m_active_syncs;
};

} // namespace Network
} // namespace Kernel
} // namespace SigmaOS

#ifdef __cplusplus
extern "C" {
#endif

void mesh_init(void);
void mesh_discover_peers(void);
void mesh_sync_shard(sigma_u32 shard_id, const char* target_node);

#ifdef __cplusplus
}
#endif

#endif // SOVEREIGN_MESH_LATTICE_H
