/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN SANDBOX CONTAINER (S-SANDBOX)
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
    /* Qubes-OS-inspired compartmentalization flags */
    bool      strict_isolation; /* If true: no IPC to other shards, kernel calls filtered via seccomp-style allowlist */
    bool      device_access;    /* If false: all hardware access (DMA, MMIO) is blocked at the HAL boundary */
} sigma_sandbox_config_t;

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignSandboxEngine {
public:
    static SovereignSandboxEngine& getInstance() {
        static SovereignSandboxEngine instance;
        return instance;
    }

    const char* type_name() const noexcept { return "SovereignSandboxEngine"; }

    void init();
    sigma_u32 createContainer(const sigma_sandbox_config_t* config);
    bool execute(sigma_u32 container_id, const char* binary_path);
    void destroyContainer(sigma_u32 container_id);
    bool checkSyscall(sigma_u32 syscall_id);
    bool hasCapability(const char* shard_name, const char* capability);
    bool validateMACPolicy(const char* sub, const char* obj, const char* act);

private:
    SovereignSandboxEngine() : next_container_id(1U), initialized(0U) {}
    sigma_u32 next_container_id;
    sigma_u32 initialized;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
#endif /* __cplusplus */

#ifdef __cplusplus
extern "C" {
#endif

void      sandbox_init(void);
sigma_u32 sandbox_create_container(const sigma_sandbox_config_t* config);
int       sandbox_execute(sigma_u32 container_id, const char* binary_path);
void      sandbox_destroy_container(sigma_u32 container_id);
int       sandbox_check_syscall(sigma_u32 syscall_id);
int       sandbox_has_capability(const char* shard_name, const char* capability);
int       sandbox_validate_mac(const char* subject, const char* object, const char* action);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SANDBOX_H */
