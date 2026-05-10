#include "../include/sigma_log.h"
#include "../include/hal/sigma_hal.h"
#include "../include/libc/SovereignLibC.h"
#include "../include/core/sigma_types.h"

/**
 * SigmaOS Sovereign Capability Vault
 * Implements token-based access control (No Root model).
 * Architecture: Capability-Based Addressing (CBA).
 */

namespace SigmaOS {
namespace Kernel {
namespace Security {

enum class CapabilityType : sigma_u32 {
    RESOURCE_NONE    = 0,
    RESOURCE_MEMORY  = 1 << 0,
    RESOURCE_NETWORK = 1 << 1,
    RESOURCE_DRIVER  = 1 << 2,
    RESOURCE_IPC     = 1 << 3,
    RESOURCE_SYSTEM  = 1 << 4
};

typedef sigma_u64 sigma_capability_t;

class SovereignCapabilityVault {
public:
    static SovereignCapabilityVault& getInstance() {
        static SovereignCapabilityVault instance;
        return instance;
    }

    static void init() {
        sigma_log("Σ [CAP-VAULT]: Initializing Capability-Based Access Control (CBAC)...");
        this->tokens_issued = 0;
        this->initialized = true;
    }

    sigma_capability_t issueToken(sigma_u32 resource_id, sigma_u32 permissions) {
        sigma_capability_t token = ((sigma_capability_t)resource_id << 32) | permissions;
        this->tokens_issued++;
        sigma_log("Σ [CAP-VAULT]: Issued Token 0x%llX for Resource %u\n", token, resource_id);
        return token;
    }

    bool validate(sigma_capability_t token, sigma_u32 resource_id, sigma_u32 required_perm) {
        sigma_u32 res = (sigma_u32)(token >> 32);
        sigma_u32 perm = (sigma_u32)(token & 0xFFFFFFFFu);
        
        if (res == resource_id && (perm & required_perm) == required_perm) {
            return true;
        }
        return false;
    }

private:
    SovereignCapabilityVault() : tokens_issued(0), initialized(false) {}
    sigma_u64 tokens_issued;
    bool initialized;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void cap_vault_init() {
    SigmaOS::Kernel::Security::SovereignCapabilityVault::init();
}

extern "C" sigma_u64 cap_vault_issue(sigma_u32 res, sigma_u32 perm) {
    return SigmaOS::Kernel::Security::SovereignCapabilityVault::issueToken(res, perm);
}

extern "C" int cap_vault_validate(sigma_u64 token, sigma_u32 res, sigma_u32 perm) {
    return SigmaOS::Kernel::Security::SovereignCapabilityVault::validate(token, res, perm) ? 1 : 0;
}




