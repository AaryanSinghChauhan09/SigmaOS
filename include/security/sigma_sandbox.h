/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SANDBOX CONTAINER (S-SANDBOX)
 * =========================================================================
 * Mission: Isolated, zero-trust execution environments for all applications.
 * =========================================================================
 */

#ifndef SIGMA_SANDBOX_H
#define SIGMA_SANDBOX_H

#include "core/sigma_types.h"

typedef struct {
    sigma_u32 container_id;
    bool      network_access;
    bool      fs_access;
    sigma_u32 memory_limit;
} sigma_sandbox_config_t;

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Security {

class SigmaOS::Kernel::Security::SovereignSandboxEngine {
public:
    static SigmaOS::Kernel::Security::SovereignSandboxEngine& getInstance() {
        static SigmaOS::Kernel::Security::SovereignSandboxEngine instance;
        return instance;
    }

    void init();
    sigma_u32 createContainer(const sigma_sandbox_config_t* config);
    bool execute(sigma_u32 container_id, const char* binary_path);
    void destroyContainer(sigma_u32 container_id);

    bool checkSyscall(sigma_u32 syscall_id);
    bool hasCapability(const char* shard_name, const char* capability);

private:
    SigmaOS::Kernel::Security::SovereignSandboxEngine() : next_container_id(1), initialized(0) {}
    
    sigma_u32 next_container_id;
    sigma_u32 initialized;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* --- Sandbox Primitives --- */
void      sandbox_init(void);
sigma_u32 sandbox_create_container(const sigma_sandbox_config_t* config);
bool      sandbox_execute(sigma_u32 container_id, const char* binary_path);
void      sandbox_destroy_container(sigma_u32 container_id);

bool      sandbox_check_syscall(sigma_u32 syscall_id);
bool      sandbox_has_capability(const char* shard_name, const char* capability);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SANDBOX_H */
