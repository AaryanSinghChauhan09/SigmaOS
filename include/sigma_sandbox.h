/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SANDBOX CONTAINER (S-SANDBOX)
 * =========================================================================
 * Mission: Isolated, zero-trust execution environments for all applications.
 * =========================================================================
 */

#ifndef SIGMA_SANDBOX_H
#define SIGMA_SANDBOX_H

#include "sigma_types.h"

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

class SovereignSandboxEngine {
public:
    static SovereignSandboxEngine& getInstance();

    void init();
    sigma_u32 createContainer(const sigma_sandbox_config_t* config);
    bool execute(sigma_u32 container_id, const char* binary_path);
    void destroyContainer(sigma_u32 container_id);

private:
    SovereignSandboxEngine() : next_container_id(1), initialized(0) {}
    
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

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SANDBOX_H */
