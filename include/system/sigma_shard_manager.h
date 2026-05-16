#include "../sigma_types.h"
#ifndef SIGMA_SHARD_MANAGER_H
#define SIGMA_SHARD_MANAGER_H

#include "../sigma_kernel_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignShardManager : public SigmaObject {
public:
    static SovereignShardManager& getInstance();
    
    const char* type_name() const noexcept override { return "SovereignShardManager"; }
    
    void init();
    
    // BACKLOG #32: Hot-Reloading for Cloud Functions (Live Shard Update)
    bool reloadShard(sigma_u32 shard_id, const void* new_bytecode, sigma_usize size);
    
    // BACKLOG #31: Self-healing kernel modules
    void performHealthCheck();
    
    // BACKLOG #34: Fine-Grained Capability Model
    void setCapabilities(sigma_u32 shard_id, sigma_u64 caps);

private:
    SovereignShardManager();
    sigma_u32 m_shard_count;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void shard_manager_init(void);
    bool shard_manager_reload(sigma_u32 id, const void* data, sigma_usize size);
}

#endif
