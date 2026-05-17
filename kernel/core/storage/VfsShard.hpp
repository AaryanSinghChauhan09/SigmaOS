#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_hal.h"
/* =========================================================================
 * Σ SIGMAOS: VFS SHARD (v2.0 - OOP SOVEREIGNTY)
 * =========================================================================
 */

#ifndef VFS_SHARD_HPP
#define VFS_SHARD_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class VfsNode : public SigmaOS::SigmaObject {
protected:
    char m_name[128];
    sigma_u32 m_size;

public:
    VfsNode(const char* name) : m_size(0) {
        sigma_strncpy(m_name, name, 127);
    }

    const char* type_name() const noexcept override { return "VfsNode"; }
    const char* GetName() const { return m_name; }

    virtual sigma_u32 Read(sigma_u32 offset, sigma_u32 size, sigma_u8* buffer) = 0;
    virtual sigma_u32 Write(sigma_u32 offset, sigma_u32 size, sigma_u8* buffer) = 0;
};

class SovereignVFS : public SigmaOS::SigmaObject {
private:
    SigmaVector<VfsNode*> m_nodes;

public:
    const char* type_name() const noexcept override { return "SovereignVFS"; }

    void RegisterNode(VfsNode* node) {
        m_nodes.push_back(node);
        sigma_log("[VFS]: Registered Node: %s\n", node->GetName());
    }

    VfsNode* FindNode(const char* name) {
        for (sigma_usize i = 0; i < m_nodes.size(); i++) {
            if (sigma_strcmp(m_nodes[i]->GetName(), name) == 0) {
                return m_nodes[i];
            }
        }
        return 0;
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

 