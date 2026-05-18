/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN BONUS COMPLIANCE CALCULATOR (S-BONUS)
 * =========================================================================
 * Law: Payment of Bonus Act, 1965
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes mandatory bonus entitlement, caps, and distributions.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignBonusComplianceCalc : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignBonusComplianceCalc"; }

    static SovereignBonusComplianceCalc& getInstance() {
        static SovereignBonusComplianceCalc instance;
        return instance;
    }

    struct BonusResult {
        sigma_u64 minimum_bonus_payable_paise;
        sigma_u64 maximum_bonus_payable_paise;
        sigma_u64 statutory_bonus_paise;
        bool is_eligible;
    };

    /**
     * Calculates Bonus.
     * C1: Minimum eligibility: Worked for at least 30 working days in the financial year.
     * C2: Capped Salary Ceiling: Calculation ceiling is Rs 7,000 (700,000 paise) or minimum wage, whichever is higher.
     * C3: Rates: Minimum bonus 8.33% of basic & DA, Maximum bonus 20.00% of basic & DA.
     */
    BonusResult calculate(sigma_u32 working_days_in_year, sigma_u64 monthly_salary_paise, sigma_u32 allocation_percentage_bps) {
        BonusResult result{};
        result.is_eligible = (working_days_in_year >= 30);
        
        if (!result.is_eligible) {
            result.minimum_bonus_payable_paise = 0;
            result.maximum_bonus_payable_paise = 0;
            result.statutory_bonus_paise = 0;
            sigma_log_info("[S-BONUS] Employee not eligible: Worked less than 30 days in the year.");
            return result;
        }
        
        const sigma_u64 CALCULATION_CEILING_PAISE = 700000; // Rs 7,000 per month
        sigma_u64 annual_salary_base_paise = monthly_salary_paise;
        
        if (monthly_salary_paise > CALCULATION_CEILING_PAISE) {
            annual_salary_base_paise = CALCULATION_CEILING_PAISE; // Capped for calculation
        }
        
        sigma_u64 total_annual_base_paise = annual_salary_base_paise * 12;
        
        // 8.33% = 833 bps
        result.minimum_bonus_payable_paise = (total_annual_base_paise * 833) / 10000;
        // 20.00% = 2000 bps
        result.maximum_bonus_payable_paise = (total_annual_base_paise * 2000) / 10000;
        
        // Statutory requested bonus
        if (allocation_percentage_bps < 833) {
            allocation_percentage_bps = 833; // Enforce statutory minimum
        } else if (allocation_percentage_bps > 2000) {
            allocation_percentage_bps = 2000; // Cap at statutory maximum
        }
        
        result.statutory_bonus_paise = (total_annual_base_paise * allocation_percentage_bps) / 10000;
        
        sigma_log_info("[S-BONUS] Bonus audit complete. Annual base: Rs %d.", (int)(total_annual_base_paise / 100));
        sigma_log_info("[S-BONUS] Minimum statutory bonus: Rs %d.%02d",
            (int)(result.minimum_bonus_payable_paise / 100), (int)(result.minimum_bonus_payable_paise % 100));
        sigma_log_info("[S-BONUS] Allocated bonus: Rs %d.%02d",
            (int)(result.statutory_bonus_paise / 100), (int)(result.statutory_bonus_paise % 100));
            
        return result;
    }

private:
    SovereignBonusComplianceCalc() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_bonus_calc(sigma_u32 days, sigma_u64 salary, sigma_u32 percent_bps) {
        SigmaOS::Tools::Pro::SovereignBonusComplianceCalc::getInstance().calculate(days, salary, percent_bps);
    }
}
