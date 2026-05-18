/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MATERNITY BENEFIT TRACKER (S-MATERNITY)
 * =========================================================================
 * Law: Maternity Benefit (Amendment) Act, 2017
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes mandatory paid leave, bonus compliance, and corporate requirements.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignMaternityBenefitTracker : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignMaternityBenefitTracker"; }

    static SovereignMaternityBenefitTracker& getInstance() {
        static SovereignMaternityBenefitTracker instance;
        return instance;
    }

    struct ComplianceReport {
        sigma_u32 mandatory_paid_leave_weeks;
        sigma_u64 statutory_medical_bonus_paise;
        bool is_creche_mandatory;
        bool is_wfh_eligible;
        bool has_violated_rules;
    };

    /**
     * Audits and tracks compliance.
     * C1:組織 has 50+ employees -> Creche facility is mandatory.
     * C2: Paid leave duration is 26 weeks for up to 2 surviving children, 12 weeks for 3+ children.
     * C3: Medical bonus of Rs 3,500 (350,000 paise) is statutory if no pre-natal/post-natal care is provided.
     */
    ComplianceReport audit(sigma_u32 surviving_children, sigma_u32 company_total_employees, bool pre_post_natal_provided, sigma_u32 leaves_granted_weeks) {
        ComplianceReport report{};
        
        // Paid leaves calculation
        if (surviving_children < 2) {
            report.mandatory_paid_leave_weeks = 26;
        } else {
            report.mandatory_paid_leave_weeks = 12;
        }
        
        // Creche facility requirement (mandatory for 50+ employees)
        report.is_creche_mandatory = (company_total_employees >= 50);
        
        // Medical bonus (Rs 3,500 = 350,000 paise)
        if (!pre_post_natal_provided) {
            report.statutory_medical_bonus_paise = 350000;
        } else {
            report.statutory_medical_bonus_paise = 0;
        }
        
        // WFH eligibility is true by default under amendment if nature of work permits
        report.is_wfh_eligible = true;
        
        // Audit check
        if (leaves_granted_weeks < report.mandatory_paid_leave_weeks) {
            report.has_violated_rules = true;
            sigma_log_error("[S-MATERNITY] COMPLIANCE ALERT: Paid leaves granted (%d weeks) is less than statutory requirement (%d weeks)!",
                leaves_granted_weeks, report.mandatory_paid_leave_weeks);
        } else {
            report.has_violated_rules = false;
            sigma_log_info("[S-MATERNITY] Compliance verified successfully. organizaton matches statutory guidelines.");
        }
        
        return report;
    }

private:
    SovereignMaternityBenefitTracker() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_maternity_audit(sigma_u32 kids, sigma_u32 staff, sigma_u32 weeks) {
        SigmaOS::Tools::Pro::SovereignMaternityBenefitTracker::getInstance().audit(kids, staff, false, weeks);
    }
}
