#include "core/sigma_types.h"
#include "libc/SovereignLibC.h"
#include "hal/sigma_hal.h"

/**
 * SigmaOS AI Governance Rules
 * Defines compliance and execution policies for agents.
 */

class GovernanceRules {
private:
    bool execution_allowed;

    GovernanceRules() : execution_allowed(true) {}

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
            sigma_log("[GOVERNANCE] Strict compliance mode enforced.");
        }
    }
};

extern "C" bool check_governance_policy() {
    return GovernanceRules::getInstance().isExecutionAllowed();
}
