#include "core/sigma_types.h"
#include "libc/SovereignLibC.h"
#include "hal/sigma_hal.h"
#include "sigma_log.h"
#include "core/context/manager.hpp"

/**
 * SigmaOS AI Governance Rules
 * Defines compliance and execution policies for agents.
 */

class GovernanceRules {
private:
    bool execution_allowed;

    GovernanceRules() : execution_allowed(true) {
        SigmaOS::Kernel::Context::ContextManager::getInstance().registerModule("agent.policy", this);
    }

public:
    static GovernanceRules& getInstance() {
        static GovernanceRules instance;
        return instance;
    }

    bool isExecutionAllowed() {
        return execution_allowed;
    }

    void enforceComplianceMode(bool strict) {
        if (strict) {
            sigma_log("[GOVERNANCE] Strict compliance mode enforced.\n");
        }
    }
};

extern "C" bool check_governance_policy() {
    GovernanceRules* policyModule = (GovernanceRules*) SigmaOS::Kernel::Context::ContextManager::getInstance().resolve("agent.policy");
    if (!policyModule) {
        policyModule = &GovernanceRules::getInstance();
    }
    return policyModule->isExecutionAllowed();
}

} // extern "C"
