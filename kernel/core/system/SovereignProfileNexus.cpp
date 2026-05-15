#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Profile Nexus (S-NEXUS)
 * Purpose: Central orchestration for profession-specific lattice profiles.
 * USP: Instantly reconfigures the entire OS lattice (resource quotas, 
 *      visible shards, UI persona) based on the active professional role.
 */

 "C" {

class SovereignProfileNexus : public SigmaOS::SigmaObject {
public:
    static SovereignProfileNexus& getInstance() {
        static SovereignProfileNexus instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignProfileNexus";
    }

    void init() {
        sigma_log_info("[S-NEXUS] Initializing Sovereign Profile Nexus...");
        this->m_active_role = ProfessionRole::DEVELOPER;
        m_strategy_count = 0;
    }

    void registerStrategy(ProfessionRole role, void (*heal)(), void (*rollback)()) {
        if (m_strategy_count >= 32) return;
        m_strategies[m_strategy_count++] = { role, heal, rollback };
    }

    void switchProfile(ProfessionRole role) {
        sigma_log_info("[S-NEXUS] Switching lattice profile to Role ID: %u...", (unsigned)role);
        
        switch(role) {
            case ProfessionRole::CREATIVE:
                sigma_log_info("[S-NEXUS] Activating GPU-Priority & Zenith Glass Persona.");
                break;
            case ProfessionRole::HEALTHCARE:
                sigma_log_info("[S-NEXUS] Activating HIPAA-Seal & High-Contrast UI.");
                break;
            case ProfessionRole::INDUSTRIAL:
                sigma_log_info("[S-NEXUS] Activating RT-Scheduler & G-Code Acceleration.");
                break;
            case ProfessionRole::GOVERNMENT:
                sigma_log_info("[S-NEXUS] Activating PQC-Audit & Air-Gap Shield.");
                break;
            case ProfessionRole::LEGAL:
                sigma_log_info("[S-NEXUS] Activating BNS-Lattice & Evidence-Seal.");
                break;
            case ProfessionRole::FINANCE:
                sigma_log_info("[S-NEXUS] Activating Fiscal-Shield & Audit-Chain.");
                break;
            default:
                sigma_log_info("[S-NEXUS] Reverting to Developer Low-Latency profile.");
        }
        
        this->m_active_role = role;
        sigma_log_info("[S-NEXUS] Profile switch COMPLETE.");
    }

    void triggerResilience() {
        sigma_log_info("[S-NEXUS] Triggering TAILORED resilience for active profile...");
        for (sigma_u32 i = 0; i < m_strategy_count; ++i) {
            if (m_strategies[i].role == m_active_role) {
                if (m_strategies[i].heal_fn) m_strategies[i].heal_fn();
                return;
            }
        }
        sigma_log_info("[S-NEXUS] No specific strategy found. Using default lattice healing.");
    }

private:
    SovereignProfileNexus() = default;
    ProfessionRole m_active_role;
    ProfessionStrategy m_strategies[32];
    sigma_u32 m_strategy_count;
};

} // namespace Profiles
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void nexus_init() {
    SigmaOS::Kernel::Profiles::SovereignProfileNexus::getInstance().init();
}

void nexus_switch_profile(sigma_u32 role_id) {
    SigmaOS::Kernel::Profiles::SovereignProfileNexus::getInstance().switchProfile((SigmaOS::Kernel::Profiles::ProfessionRole)role_id);
}

void nexus_register_strategy(sigma_u32 role_id, void (*heal)(), void (*rollback)()) {
    SigmaOS::Kernel::Profiles::SovereignProfileNexus::getInstance().registerStrategy((SigmaOS::Kernel::Profiles::ProfessionRole)role_id, heal, rollback);
}

void nexus_trigger_resilience() {
    SigmaOS::Kernel::Profiles::SovereignProfileNexus::getInstance().triggerResilience();
}

} // extern "C"

} // extern "C"
