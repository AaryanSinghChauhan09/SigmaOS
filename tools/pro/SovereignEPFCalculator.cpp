/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN EPF CALCULATOR (S-EPF)
 * =========================================================================
 * Law: Employees' Provident Funds and Miscellaneous Provisions Act, 1952
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes employee and employer monthly EPF & EPS contributions.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignEPFCalculator : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignEPFCalculator"; }

    static SovereignEPFCalculator& getInstance() {
        static SovereignEPFCalculator instance;
        return instance;
    }

    struct EPFResult {
        sigma_u64 employee_share_paise;
        sigma_u64 employer_epf_share_paise;
        sigma_u64 employer_eps_share_paise;
        sigma_u64 total_monthly_accumulation_paise;
        bool is_statutory_limit_capped;
    };

    /**
     * Calculates EPF contribution.
     * Statutory wage ceiling: Rs 15,000 per month (1,500,000 paise).
     * Employee Share: 12% of basic + DA.
     * Employer Share: 8.33% goes to EPS (capped at wage ceiling), remaining 3.67% goes to EPF.
     */
    EPFResult calculate(sigma_u64 monthly_basic_da_paise, bool voluntary_excess_contribution) {
        EPFResult result{};
        
        const sigma_u64 STATUTORY_CEILING_PAISE = 1500000; // Rs 15,000
        sigma_u64 eps_wage_base = monthly_basic_da_paise;
        
        if (monthly_basic_da_paise > STATUTORY_CEILING_PAISE) {
            eps_wage_base = STATUTORY_CEILING_PAISE;
            result.is_statutory_limit_capped = true;
        } else {
            result.is_statutory_limit_capped = false;
        }

        // Employee EPF Contribution (12%)
        result.employee_share_paise = (monthly_basic_da_paise * 12) / 100;
        
        // Employer EPS Contribution (8.33% of capped wage)
        // Fixed-point division precision offset
        result.employer_eps_share_paise = (eps_wage_base * 833) / 10000;
        
        // Employer EPF Contribution (12% of basic minus the EPS portion)
        // If voluntary EPF is active, calculate on actual base salary, else on capped base
        sigma_u64 employer_base = voluntary_excess_contribution ? monthly_basic_da_paise : eps_wage_base;
        sigma_u64 total_employer_twelve = (employer_base * 12) / 100;
        
        if (total_employer_twelve >= result.employer_eps_share_paise) {
            result.employer_epf_share_paise = total_employer_twelve - result.employer_eps_share_paise;
        } else {
            result.employer_epf_share_paise = 0;
        }
        
        result.total_monthly_accumulation_paise = result.employee_share_paise + result.employer_epf_share_paise;
        
        sigma_log_info("[S-EPF] Employee Contribution: Rs %d.%02d", 
            (int)(result.employee_share_paise / 100), (int)(result.employee_share_paise % 100));
        sigma_log_info("[S-EPF] Employer EPF Share: Rs %d.%02d", 
            (int)(result.employer_epf_share_paise / 100), (int)(result.employer_epf_share_paise % 100));
        sigma_log_info("[S-EPF] Employer EPS Share: Rs %d.%02d", 
            (int)(result.employer_eps_share_paise / 100), (int)(result.employer_eps_share_paise % 100));
            
        return result;
    }

private:
    SovereignEPFCalculator() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_epf_calc(sigma_u64 salary_paise) {
        SigmaOS::Tools::Pro::SovereignEPFCalculator::getInstance().calculate(salary_paise, false);
    }
}
