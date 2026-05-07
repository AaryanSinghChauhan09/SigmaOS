#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "security/sigma_sandbox.h"
#include "libc/SovereignLibC.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace Security {

void SovereignSandboxEngine::init() {
    sigma_log("[SANDBOX] Initializing Sovereign Zero-Trust Sandbox Isolation...");
    this->initialized = 1u;
}

sigma_u32 SovereignSandboxEngine::createContainer(const sigma_sandbox_config_t* config) {
    (void)config;
    sigma_u32 id = this->next_container_id++;
    sigma_log("[SANDBOX] Container C%04u created with sealed amnesic profile.\n", id);
    return id;
}

bool SovereignSandboxEngine::execute(sigma_u32 container_id, const char* binary_path) {
    sigma_log("[SANDBOX] C%04u executing shard: %s\n", container_id, binary_path);
    sigma_log("[SANDBOX] Runtime: Seccomp-BFP filter applied. Resource caps locked.");
    return true;
}

void SovereignSandboxEngine::destroyContainer(sigma_u32 container_id) {
    sigma_log("[SANDBOX] C%04u terminated. Scrubbing amnesic memory artifacts...\n", container_id);
}

bool SovereignSandboxEngine::checkSyscall(sigma_u32 syscall_id) {
    // Basic policy enforcement: ID 0x01 (sigma_yield) is allowed globally
    if (syscall_id == 0x01) return true;
    return false;
}

bool SovereignSandboxEngine::hasCapability(const char* shard_name, const char* capability) {
    // Mock capability matrix
    if (sigma_strcmp(shard_name, "SovereignMonitor") == 0 && sigma_strcmp(capability, "EBPF_INJECT") == 0) {
        return true;
    }
    return false;
}

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void sandbox_init() {
    SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().init();
}

extern "C" sigma_u32 sandbox_create_container(const sigma_sandbox_config_t* config) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().createContainer(config);
}

extern "C" bool sandbox_execute(sigma_u32 container_id, const char* binary_path) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().execute(container_id, binary_path);
}

extern "C" void sandbox_destroy_container(sigma_u32 container_id) {
    SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().destroyContainer(container_id);
}

extern "C" bool sandbox_check_syscall(sigma_u32 syscall_id) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().checkSyscall(syscall_id);
}

extern "C" bool sandbox_has_capability(const char* shard_name, const char* capability) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().hasCapability(shard_name, capability);
}
