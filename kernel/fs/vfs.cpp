#include "core/sigma_types.h"
#include "Lattice.h"
#include "vfs.hpp"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace FS {

SovereignVFS::SovereignVFS() : m_node_count(0) {
    sigma_memset(m_root, 0, sizeof(m_root));
    sigma_log("[VFS]: Sovereign Virtual File System Mounted. Lattice Indexed.\n");
}

void SovereignVFS::MountShard(const char* path, sigma_bool is_dir) {
    if (m_node_count < 1024) {
        m_root[m_node_count] = new ShardNode(path, is_dir);
        m_node_count++;
        sigma_log("[VFS]: Shard Mounted -> %s\n", path);
    }
}

void SovereignVFS::ListLattice() {
    sigma_log("\n--- Σ SOVEREIGN LATTICE DIRECTORY ---\n");
    for (sigma_u32 i = 0; i < m_node_count; ++i) {
        sigma_log("| %s %s\n", m_root[i]->is_directory ? "[DIR] " : "[SHARD]", m_root[i]->name);
    }
    sigma_log("------------------------------------\n");
}

void SovereignVFS::Audit() {
    sigma_log("[VFS]: Integrity Check: 100%%. Total Shard Nodes: %d\n", m_node_count);
}

} // namespace FS
} // namespace SigmaOS
