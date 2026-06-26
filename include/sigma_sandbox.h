#ifndef SIGMA_SANDBOX_H
#define SIGMA_SANDBOX_H

#include "sigma_types.h"

// ---------------------------------------------------------
// SigmaOS Sandbox Capabilities
// Defines fine-grained capability isolation rings for processes.
// ---------------------------------------------------------

namespace sigma {
namespace security {

enum class SandboxRing : uint8_t {
    RING_0_KERNEL = 0,
    RING_1_DRIVER = 1,
    RING_2_SERVICE = 2,
    RING_3_USER = 3,
    RING_4_WASM = 4  // Highest restriction, fully isolated
};

struct CapabilityMask {
    bool can_network : 1;
    bool can_fs_write : 1;
    bool can_fs_read : 1;
    bool can_spawn_process : 1;
    bool can_allocate_rwx : 1;
};

struct SovereignSandboxContext {
    SandboxRing ring_level;
    CapabilityMask caps;
    uint64_t max_memory_bytes;
    uint64_t current_memory_bytes;
    uint32_t process_id;
    
    // Check if the process can perform a specific privileged action
    bool check_capability(bool CapabilityMask::* cap_field) const {
        // Ring 0 bypasses checks
        if (ring_level == SandboxRing::RING_0_KERNEL) return true;
        
        return this->caps.*cap_field;
    }
};

} // namespace security
} // namespace sigma

#endif // SIGMA_SANDBOX_H
