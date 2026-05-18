/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN INDUSTRIAL DISPUTE COMPLIANCE AUDITOR (S-IDA)
 * =========================================================================
 * Law: Industrial Disputes Act, 1947
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Audits Strike and Lockout notices in public utilities under Section 22.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignIndustrialDisputeArbitrator : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignIndustrialDisputeArbitrator"; }

    static SovereignIndustrialDisputeArbitrator& getInstance() {
        static SovereignIndustrialDisputeArbitrator instance;
        return instance;
    }

    struct ComplianceReport {
        bool is_strike_legal;
        bool has_violated_six_weeks_rule;
        bool has_violated_fourteen_days_rule;
        bool was_conciliation_pending;
    };

    /**
     * Audits strike notice.
     * statutory requirements: Section 22 (Public Utility Service):
     * C1: Strike notice must be given at least 6 weeks before strike.
     * C2: Strike cannot be held within 14 days of giving notice.
     * C3: Strike cannot be held during conciliation proceedings and 7 days after completion.
     */
    ComplianceReport audit(sigma_u32 days_since_notice_given, sigma_u32 days_before_expiry_of_notice, bool is_conciliation_active) {
        ComplianceReport report{};
        
        report.has_violated_six_weeks_rule = false;
        report.has_violated_fourteen_days_rule = false;
        report.was_conciliation_pending = is_conciliation_active;
        
        // 6 Weeks = 42 days
        if (days_since_notice_given > 42) {
            report.has_violated_six_weeks_rule = true;
        }
        
        // Notice must be given within 6 weeks, but not within 14 days
        if (days_since_notice_given < 14) {
            report.has_violated_fourteen_days_rule = true;
        }
        
        // Final strike legality
        report.is_strike_legal = !report.has_violated_fourteen_days_rule &&
                                 !is_conciliation_active &&
                                 (days_before_expiry_of_notice > 0);
                                 
        if (!report.is_strike_legal) {
            sigma_log_error("[S-IDA] ILLEGAL STRIKE DETERMINED: Notice constraints or active conciliation violated.");
        } else {
            sigma_log_info("[S-IDA] Strike notice compliance: LEGAL. All Section 22 parameters met.");
        }
        
        return report;
    }

private:
    SovereignIndustrialDisputeArbitrator() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_industrial_arbitration_audit(sigma_u32 days_since, sigma_u32 days_expiry, sigma_u8 conciliation) {
        SigmaOS::Tools::Pro::SovereignIndustrialDisputeArbitrator::getInstance().audit(days_since, days_expiry, conciliation != 0);
    }
}
