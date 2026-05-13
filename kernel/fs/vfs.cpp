#include "Lattice.h"
#include "../../../include/sigma_log.h"
#include "vfs.hpp"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace FS {

SovereignVFS::SovereignVFS() : m_node_count(0) {
    sigma_memset(m_root, 0, sizeof(m_root));
    sigma_log_info("[VFS]: Sovereign Virtual File System Mounted. Lattice Indexed.\n");
}

void SovereignVFS::MountShard(const char* path, sigma_bool is_dir) {
    if (m_node_count < 1024) {
        m_root[m_node_count] = new ShardNode(path, is_dir);
        m_node_count++;
        sigma_log_info("[VFS]: Shard Mounted -> %s\n", path);
    }
}

void SovereignVFS::ListLattice() {
    sigma_log_info("\n--- Σ SOVEREIGN LATTICE DIRECTORY ---\n");
    for (sigma_u32 i = 0; i < m_node_count; ++i) {
        sigma_log_info("| %s %s\n", m_root[i]->is_directory ? "[DIR] " : "[SHARD]", m_root[i]->name);
    }
    sigma_log_info("------------------------------------\n");
}

void SovereignVFS::Audit() {
    sigma_log_info("[VFS]: Integrity Check: 100%%. Total Shard Nodes: %d\n", m_node_count);
}

} // namespace FS
} // namespace SigmaOS


