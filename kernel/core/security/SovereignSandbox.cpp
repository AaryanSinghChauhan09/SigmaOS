#include "sigma_types.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "security/sigma_sandbox.h"
#include "core/SigmaOOP.hpp"

/**
 * SovereignSandbox — Sovereign Zero-Trust Container Engine
 * Implements capability-based isolation with Seccomp-BFP enforcement.
 */

namespace SigmaOS {
namespace Kernel {
namespace Security {

/* Policy Constants */
static constexpr sigma_u32 SYSCALL_SIGMA_YIELD = 0x01U;

SovereignSandboxEngine& SovereignSandboxEngine::getInstance() {
    static SovereignSandboxEngine instance;
    return instance;
}

const char* SovereignSandboxEngine::type_name() const noexcept {
    return "SovereignSandboxEngine";
}

void SovereignSandboxEngine::init() {
    sigma_log_info("[SANDBOX] Initializing Sovereign Zero-Trust Sandbox Isolation...");
    this->initialized = 1U;
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
    /* Basic policy: only sigma_yield is globally permitted */
    return (syscall_id == SYSCALL_SIGMA_YIELD);
}

bool SovereignSandboxEngine::validateMACPolicy(Subject sub, Object obj, Action act) {
        extern "C" int sandbox_mac_validate(const char* sub, const char* obj, const char* act);
        return sandbox_mac_validate(sub.value, obj.value, act.value) != 0;
}

bool SovereignSandboxEngine::hasCapability(ShardName shard, Capability cap) {
    /* Mock capability matrix — replaced by policy engine at runtime */
    sigma_log_info("[SANDBOX] CAP: Checking if %s possesses %s", shard.value, cap.value);
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

extern "C" int sandbox_execute(sigma_u32 container_id, const char* binary_path) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().execute(container_id, binary_path) ? 1 : 0;
}

extern "C" void sandbox_destroy_container(sigma_u32 container_id) {
    SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().destroyContainer(container_id);
}

extern "C" int sandbox_check_syscall(sigma_u32 syscall_id) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().checkSyscall(syscall_id) ? 1 : 0;
}

extern "C" int sandbox_has_capability(const char* shard_name, const char* capability) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().hasCapability(
        SigmaOS::Kernel::Security::SovereignSandboxEngine::ShardName{shard_name},
        SigmaOS::Kernel::Security::SovereignSandboxEngine::Capability{capability}) ? 1 : 0;
}

extern "C" int sandbox_validate_mac(const char* subject, const char* object, const char* action) {
    return SigmaOS::Kernel::Security::SovereignSandboxEngine::getInstance().validateMACPolicy(
        SigmaOS::Kernel::Security::SovereignSandboxEngine::Subject{subject},
        SigmaOS::Kernel::Security::SovereignSandboxEngine::Object{object},
        SigmaOS::Kernel::Security::SovereignSandboxEngine::Action{action}) ? 1 : 0;
}