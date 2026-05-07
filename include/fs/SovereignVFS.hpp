#ifndef SOVEREIGN_VFS_HPP
#define SOVEREIGN_VFS_HPP

#include "core/sigma_types.h"

namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignDistributedVFS {
public:
    static SovereignDistributedVFS& getInstance();
    void init();
    void mountDistributedNode(const char* node_address);
    sigma_u32 open(const char* filepath, sigma_u32 flags);
    sigma_u32 read(sigma_u32 fd, void* buffer, sigma_u32 size);
    sigma_u32 write(sigma_u32 fd, const void* buffer, sigma_u32 size);
    void close(sigma_u32 fd);
    void writeReplicatedFile(const char* filepath, const char* data);
    void atomicSync();

private:
    SovereignDistributedVFS();
    char m_shard_nodes[8][32];
    sigma_u32 m_active_shards;
    sigma_u32 m_files_tracked;
    sigma_u32 m_system_vector_clock;
    sigma_u32 m_drift_correction_ms;
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void vfs_init();
    void vfs_mount_node(const char* node_address);
    void vfs_write_file(const char* filepath, const char* data);
    sigma_u32 vfs_open(const char* path, sigma_u32 flags);
    sigma_u32 vfs_read(sigma_u32 fd, void* buf, sigma_u32 sz);
    sigma_u32 vfs_write(sigma_u32 fd, const void* buf, sigma_u32 sz);
    void vfs_close(sigma_u32 fd);
}

#endif
