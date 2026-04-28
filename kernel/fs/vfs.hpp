#ifndef VFS_HPP
#define VFS_HPP

#include "../../include/sigma_types.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace FS {

/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN FILE NODE (Encapsulation)
 * =========================================================================
 */
class ShardNode : public SigmaObject {
public:
    const char* name;
    sigma_size_t size;
    sigma_u8* data;
    sigma_bool is_directory;

    ShardNode(const char* _name, sigma_bool _is_dir) 
        : name(_name), size(0), data((sigma_u8*)SIGMA_NULL), is_directory(_is_dir) {}

    const char* type_name() const noexcept override { return "ShardNode"; }
};

/*
 * =========================================================================
 * SOVEREIGN VFS (Sovereign File System Principles)
 * =========================================================================
 */
class SovereignVFS : public SigmaObject {
private:
    ShardNode* m_root[1024];
    sigma_u32 m_node_count;

public:
    SovereignVFS();
    const char* type_name() const noexcept override { return "SovereignVFS"; }

    void MountShard(const char* path, sigma_bool is_dir);
    void ListLattice();
    void Audit();
};

} // namespace FS
} // namespace SigmaOS

#endif
