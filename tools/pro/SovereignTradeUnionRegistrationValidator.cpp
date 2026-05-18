/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TRADE UNION REGISTRATION VALIDATOR (S-TUR)
 * =========================================================================
 * Law: Trade Unions Act, 1926 (Section 9A)
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Audits registration eligibility based on minimum membership.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignTradeUnionRegistrationValidator : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignTradeUnionRegistrationValidator"; }

    static SovereignTradeUnionRegistrationValidator& getInstance() {
        static SovereignTradeUnionRegistrationValidator instance;
        return instance;
    }

    struct ComplianceReport {
        bool is_eligible_for_registration;
        sigma_u32 required_minimum_workers;
        sigma_u32 actual_workers;
        bool violates_absolute_minimum;
    };

    /**
     * Audits Trade Union membership eligibility.
     * statutory requirement: Section 9A:
     * C1: Must have at least 10% of total workers or 100 workers, whichever is less, engaged in the establishment.
     * C2: Minimum of 7 workers (absolute minimum) is mandatory under all conditions.
     */
    ComplianceReport audit(sigma_u32 total_establishment_workers, sigma_u32 active_union_members) {
        ComplianceReport report{};
        report.actual_workers = active_union_members;
        report.violates_absolute_minimum = (active_union_members < 7);
        
        // Compute 10% of workers or 100 workers, whichever is less
        sigma_u32 ten_percent = total_establishment_workers / 10;
        if (ten_percent == 0) ten_percent = 1; // Safeguard division
        
        sigma_u32 statutory_limit = (ten_percent < 100) ? ten_percent : 100;
        report.required_minimum_workers = statutory_limit;
        
        if (report.violates_absolute_minimum) {
            report.is_eligible_for_registration = false;
            sigma_log_error("[S-TUR] REGISTRATION FAILED: Absolute minimum requirement of 7 members not met.");
        } else if (active_union_members < report.required_minimum_workers) {
            report.is_eligible_for_registration = false;
            sigma_log_error("[S-TUR] REGISTRATION FAILED: Under Section 9A, 10%% of workers or 100 workers (%d required, got %d) is necessary.",
                report.required_minimum_workers, active_union_members);
        } else {
            report.is_eligible_for_registration = true;
            sigma_log_info("[S-TUR] Trade Union Registration Eligibility: VALID. All statutory parameters satisfied.");
        }
        
        return report;
    }

private:
    SovereignTradeUnionRegistrationValidator() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_trade_union_validation(sigma_u32 total_workers, sigma_u32 union_members) {
        SigmaOS::Tools::Pro::SovereignTradeUnionRegistrationValidator::getInstance().audit(total_workers, union_members);
    }
}
