/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PFRDA MEMBER REGISTRY & NPS CALCULATOR (S-NPS)
 * =========================================================================
 * Law: Pension Fund Regulatory and Development Authority (PFRDA) Act, 2013
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes NPS Tier I / II contributions and Section 80CCD tax benefits.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignPFRDAMemberRegistry : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignPFRDAMemberRegistry"; }

    static SovereignPFRDAMemberRegistry& getInstance() {
        static SovereignPFRDAMemberRegistry instance;
        return instance;
    }

    struct NPSResult {
        sigma_u64 total_contribution_paise;
        sigma_u64 sec_80ccd_1_deduction_paise; // Capped at 10% of salary
        sigma_u64 sec_80ccd_1b_deduction_paise; // Capped at Rs 50,000 (5,000,000 paise)
        sigma_u64 total_tax_deduction_eligible_paise;
    };

    /**
     * Calculates NPS statutory tax exemptions.
     * C1: Section 80CCD(1): Employee contribution up to 10% of salary (Basic + DA) is exempt.
     * C2: Section 80CCD(1B): Additional exclusive exemption of up to Rs 50,000 (5,000,000 paise) is allowed for Tier-I.
     */
    NPSResult calculate(sigma_u64 annual_salary_basic_da_paise, sigma_u64 tier_1_investment_paise, sigma_u64 tier_2_investment_paise) {
        NPSResult result{};
        result.total_contribution_paise = tier_1_investment_paise + tier_2_investment_paise;
        
        // Section 80CCD(1) - 10% of basic + DA cap
        sigma_u64 ten_percent_cap = annual_salary_basic_da_paise / 10;
        result.sec_80ccd_1_deduction_paise = (tier_1_investment_paise < ten_percent_cap) ? tier_1_investment_paise : ten_percent_cap;
        
        // Section 80CCD(1B) - Extra Rs 50,000 deduction on remaining Tier-I investment
        sigma_u64 remaining_tier_1 = 0;
        if (tier_1_investment_paise > result.sec_80ccd_1_deduction_paise) {
            remaining_tier_1 = tier_1_investment_paise - result.sec_80ccd_1_deduction_paise;
        }
        
        const sigma_u64 EXTRA_DEDUCTION_LIMIT_PAISE = 5000000; // Rs 50,000
        result.sec_80ccd_1b_deduction_paise = (remaining_tier_1 < EXTRA_DEDUCTION_LIMIT_PAISE) ? remaining_tier_1 : EXTRA_DEDUCTION_LIMIT_PAISE;
        
        // Total deduction
        result.total_tax_deduction_eligible_paise = result.sec_80ccd_1_deduction_paise + result.sec_80ccd_1b_deduction_paise;
        
        sigma_log_info("[S-NPS] NPS Investment registered successfully.");
        sigma_log_info("[S-NPS] Section 80CCD(1) Deduction: Rs %d.", (int)(result.sec_80ccd_1_deduction_paise / 100));
        sigma_log_info("[S-NPS] Section 80CCD(1B) Deduction: Rs %d.", (int)(result.sec_80ccd_1b_deduction_paise / 100));
        sigma_log_info("[S-NPS] Total NPS Exemption: Rs %d.%02d",
            (int)(result.total_tax_deduction_eligible_paise / 100), (int)(result.total_tax_deduction_eligible_paise % 100));
            
        return result;
    }

private:
    SovereignPFRDAMemberRegistry() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_nps_calc(sigma_u64 annual_salary, sigma_u64 tier1) {
        SigmaOS::Tools::Pro::SovereignPFRDAMemberRegistry::getInstance().calculate(annual_salary, tier1, 0);
    }
}
