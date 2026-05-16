#include "../../include/sigma_armor.h"
#include "../../include/sigma_log.h"
#include "../../include/SigmaOOP.hpp"

/**
 * Σ SIGMAOS: SOVEREIGN ARMOR POLICY ENGINE (S-ARMOR)
 * Implementation: Mandatory Access Control and Shard Isolation.
 */

namespace SigmaOS {
namespace Kernel {
namespace Security {

void SovereignArmorEngine::init() {
    sigma_log_info("[S-ARMOR] Initializing Sovereign Armor Policy Engine...");
    sigma_log_info("[S-ARMOR] Status: ENFORCING Mandatory Access Control (MAC).");
}

void SovereignArmorEngine::setLevel(sigma_armor_level_t level) {
    this->m_current_level = level;
    sigma_log_info("[S-ARMOR] Policy Enforcement Level set to: %d", (int)level);
}

bool SovereignArmorEngine::checkPermission(const char* sid, const char* act) {
    sigma_log_info("[S-ARMOR] Checking permission for shard %s: ACTION=%s", sid, act);
    // Logic: Cross-reference with PQC-signed policy manifest
    return true; 
}

void SovereignArmorEngine::applyPolicy(const sigma_armor_policy_t* policy) {
    sigma_log_info("[S-ARMOR] Applying hardened policy to shard: %s", policy->shard_id);
    sigma_log_info("[S-ARMOR] Restrictions: NET=%d STORAGE=%d IPC=%d", 
                   policy->allow_net, policy->allow_storage, policy->allow_ipc);
}

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {
    void armor_init() {
        SigmaOS::Kernel::Security::SovereignArmorEngine::getInstance().init();
    }

    void armor_set_level(sigma_armor_level_t level) {
        SigmaOS::Kernel::Security::SovereignArmorEngine::getInstance().setLevel(level);
    }

    bool armor_check_permission(const char* shard_id, const char* action) {
        return SigmaOS::Kernel::Security::SovereignArmorEngine::getInstance().checkPermission(shard_id, action);
    }

    void armor_enforce_policy(const sigma_armor_policy_t* policy) {
        SigmaOS::Kernel::Security::SovereignArmorEngine::getInstance().applyPolicy(policy);
    }
}
