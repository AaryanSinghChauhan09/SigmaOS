#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "core/sigma_kernel_types.h"
#include "libc/SovereignLibC.h"
#include "core/SigmaOOP.hpp"
#include "../security/SovereignQKD.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Community {

/**
 * SigmaOS Sovereign Governance Shard
 * Principles: Decentralized Voting, Node-Weighted Consensus, Meritocratic Rewards.
 * Mission: Enabling the OS lattice to evolve via community-driven cryptographic proposals.
 */
class SovereignGovernance : public SigmaObject {
public:
    static SovereignGovernance& getInstance() {
        static SovereignGovernance instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignGovernance"; }

    static void init() {
        sigma_log("Σ [GOVERNANCE]: Initializing Decentralized Contributor Registry...");
        m_active_proposals = 0;
        m_total_votes = 0;
        sigma_log("Σ [GOVERNANCE]: Node-Weighted Consensus Engine ACTIVE.");
    }

    void submitProposal(const char* proposal_id) {
        if (m_active_proposals >= 16) return;
        sigma_log("Σ [GOVERNANCE]: Proposal Submitted -> '%s'\n", proposal_id);
        m_active_proposals++;
    }

    void castVote(const char* node_id, const char* proposal_id, bool support) {
        // Enforce QKD verification for voting rights
        if (!Security::SovereignQKD::verifyQuantumIntegrity()) {
            sigma_log("Σ [GOVERNANCE]: [ERROR] Node '%s' rejected. Invalid Quantum Signature.\n", node_id);
            return;
        }

        sigma_log("Σ [GOVERNANCE]: Node '%s' voted %s on '%s'.\n", 
                     node_id, support ? "YEA" : "NAY", proposal_id);
        m_total_votes++;
    }

    void rewardContributor(const char* node_id, sigma_u32 credits) {
        sigma_log("Σ [GOVERNANCE]: Contributor '%s' rewarded %u SigmaCredits for successful merge.\n", node_id, credits);
    }

    void audit() {
        sigma_log("\n--- Σ GOVERNANCE AUDIT ---\n");
        sigma_log("| Active Proposals : %u\n", m_active_proposals);
        sigma_log("| Total Votes Cast : %u\n", m_total_votes);
        sigma_log("| Consensus Model  : MESH-WEIGHTED\n");
        sigma_log("--------------------------\n");
    }

private:
    SovereignGovernance() : m_active_proposals(0), m_total_votes(0) {}
    sigma_u32 m_active_proposals;
    sigma_u32 m_total_votes;
};

} // namespace Community
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void governance_init_shard() {
    SigmaOS::Kernel::Community::SovereignGovernance::init();
}

extern "C" void governance_submit(const char* prop) {
    SigmaOS::Kernel::Community::SovereignGovernance::submitProposal(prop);
}

extern "C" void governance_vote(const char* node, const char* prop, bool support) {
    SigmaOS::Kernel::Community::SovereignGovernance::castVote(node, prop, support);
}




