/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN RTI COMPLIANCE TRACKER (S-RTI)
 * =========================================================================
 * Law: Right to Information (RTI) Act, 2005
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Tracks statutory timelines, filing fees, and delayed penalty accruals.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignRTIComplianceCalc : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignRTIComplianceCalc"; }

    static SovereignRTIComplianceCalc& getInstance() {
        static SovereignRTIComplianceCalc instance;
        return instance;
    }

    struct RTIComplianceResult {
        sigma_u32 filing_fee_paise;
        sigma_u32 statutory_limit_days;
        sigma_u32 delayed_days;
        sigma_u32 penalty_amount_rupees;
        bool is_timely;
        bool is_life_or_liberty_case;
    };

    /**
     * Calculates RTI parameters.
     * C1: Standard fee Rs 10 (1000 paise). Free for Below Poverty Line (BPL) applicants.
     * C2: Timelines: 30 days standard, 35 days if filed via Assistant PIO, 48 hours (2 days) if life/liberty concern.
     * C3: Penalty (Section 20(1)): Rs 250 per day up to Rs 25,000 maximum for default delay.
     */
    RTIComplianceResult audit(bool is_bpl, bool is_life_liberty, bool via_apio, sigma_u32 actual_resolution_days) {
        RTIComplianceResult result{};
        result.is_life_or_liberty_case = is_life_liberty;
        
        // Fee computation
        result.filing_fee_paise = is_bpl ? 0 : 1000;
        
        // Timeline limit
        if (is_life_liberty) {
            result.statutory_limit_days = 2; // 48 Hours
        } else if (via_apio) {
            result.statutory_limit_days = 35;
        } else {
            result.statutory_limit_days = 30;
        }
        
        // Delay and penalty
        if (actual_resolution_days > result.statutory_limit_days) {
            result.delayed_days = actual_resolution_days - result.statutory_limit_days;
            result.is_timely = false;
            
            // Penalty = Rs 250 * delayed days (capped at Rs 25,000)
            sigma_u32 calculated_penalty = result.delayed_days * 250;
            result.penalty_amount_rupees = (calculated_penalty > 25000) ? 25000 : calculated_penalty;
            
            sigma_log_error("[S-RTI] COMPLIANCE VIOLATION: Delay of %d days. Statutory penalty of Rs %d accrued.",
                result.delayed_days, result.penalty_amount_rupees);
        } else {
            result.delayed_days = 0;
            result.penalty_amount_rupees = 0;
            result.is_timely = true;
            sigma_log_info("[S-RTI] Timeline check: PASSED. Completed in %d days.", actual_resolution_days);
        }
        
        return result;
    }

private:
    SovereignRTIComplianceCalc() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_rti_calc(sigma_u8 is_bpl, sigma_u8 is_life, sigma_u32 days) {
        SigmaOS::Tools::Pro::SovereignRTIComplianceCalc::getInstance().audit(is_bpl != 0, is_life != 0, false, days);
    }
}
