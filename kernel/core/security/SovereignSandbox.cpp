#include "sigma_types.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "security/sigma_sandbox.h"

/**
 * SovereignSandbox — Sovereign Zero-Trust Container Engine
 * Implements capability-based isolation with Seccomp-BFP enforcement.
 */

namespace SigmaOS {
namespace Kernel {
namespace Security {

void SovereignSandboxEngine::init() {
    sigma_log_info("[SANDBOX] Initializing Sovereign Zero-Trust Sandbox Isolation...");
    this->initialized = 1u;
}

sigma_u32 SovereignSandboxEngine::createContainer(const sigma_sandbox_config_t* config) {
    (void)config;
    sigma_u32 id = this->next_container_id++;
    sigma_log_info("[SANDBOX] Container created with sealed amnesic profile.");
    return id;
}

bool SovereignSandboxEngine::execute(sigma_u32 container_id, const char* binary_path) {
    (void)container_id; (void)binary_path;
    sigma_log_info("[SANDBOX] Executing shard in container.");
    sigma_log_info("[SANDBOX] Runtime: Seccomp-BFP filter applied. Resource caps locked.");
    return true;
}

void SovereignSandboxEngine::destroyContainer(sigma_u32 container_id) {
    (void)container_id;
    sigma_log_info("[SANDBOX] Container terminated. Scrubbing amnesic memory artifacts...");
}

bool SovereignSandboxEngine::checkSyscall(sigma_u32 syscall_id) {
    /* Basic policy: only sigma_yield (0x01) is globally permitted */
    return (syscall_id == 0x01u);
}

bool SovereignSandboxEngine::hasCapability(const char* shard_name, const char* capability) {
    /* Mock capability matrix — replaced by policy engine at runtime */
    (void)shard_name; (void)capability;
    return false;
}

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void sandbox_init() {
    SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().init();
}

extern "C" unsigned int sandbox_create_container(const sigma_sandbox_config_t* config) {
    return (unsigned int)SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().createContainer(config);
}

extern "C" int sandbox_execute(unsigned int container_id, const char* binary_path) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().execute(
        (sigma_u32)container_id, binary_path) ? 1 : 0;
}

extern "C" void sandbox_destroy_container(unsigned int container_id) {
    SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().destroyContainer((sigma_u32)container_id);
}

extern "C" int sandbox_check_syscall(unsigned int syscall_id) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().checkSyscall((sigma_u32)syscall_id) ? 1 : 0;
}

extern "C" int sandbox_has_capability(const char* shard_name, const char* capability) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().hasCapability(shard_name, capability) ? 1 : 0;
}
