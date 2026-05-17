/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN RERA DELAYED INTEREST CALCULATOR (S-RERA)
 * =========================================================================
 * Law: Real Estate (Regulation and Development) Act, 2016 (RERA)
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes delayed possession interest using statutory SBI MCLR + 2% rule.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignRERAPenaltyCalc : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignRERAPenaltyCalc"; }

    static SovereignRERAPenaltyCalc& getInstance() {
        static SovereignRERAPenaltyCalc instance;
        return instance;
    }

    struct RERAResult {
        sigma_u64 total_interest_payable_paise;
        sigma_u64 total_refund_amount_paise;
        sigma_u32 delayed_months;
        sigma_u32 effective_interest_rate_bps; // in Basis Points (1% = 100 bps)
    };

    /**
     * Calculates delay penalty.
     * prescribed statutory interest: SBI Highest MCLR + 2.00% (assumed SBI MCLR = 8.5%, total = 10.5% or 1050 Basis Points)
     */
    RERAResult calculate(sigma_u64 amount_paid_paise, sigma_u32 delayed_days, sigma_u32 sbi_mclr_bps) {
        RERAResult result{};
        
        // Effective Rate = SBI MCLR + 200 Basis Points (2%)
        result.effective_interest_rate_bps = sbi_mclr_bps + 200;
        
        // Convert delayed days to approximate monthly fraction (365 days base per year)
        // Rate of interest is per annum.
        // Formula: Interest = Amount * (Rate_bps / 10000) * (Delayed_days / 365)
        sigma_u64 intermediate = amount_paid_paise * result.effective_interest_rate_bps;
        result.total_interest_payable_paise = (intermediate * delayed_days) / (3650000);
        
        result.total_refund_amount_paise = amount_paid_paise + result.total_interest_payable_paise;
        result.delayed_months = delayed_days / 30;
        
        sigma_log_info("[S-RERA] Delayed possession audit completed for %d days (%d months).", 
            delayed_days, result.delayed_months);
        sigma_log_info("[S-RERA] Effective statutory rate: %d.%02d%%", 
            (int)(result.effective_interest_rate_bps / 100), (int)(result.effective_interest_rate_bps % 100));
        sigma_log_info("[S-RERA] Delayed interest accumulated: Rs %d.%02d", 
            (int)(result.total_interest_payable_paise / 100), (int)(result.total_interest_payable_paise % 100));
            
        return result;
    }

private:
    SovereignRERAPenaltyCalc() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_rera_calc(sigma_u64 paid, sigma_u32 days) {
        // default SBI MCLR as 8.5% (850 bps)
        SigmaOS::Tools::Pro::SovereignRERAPenaltyCalc::getInstance().calculate(paid, days, 850);
    }
}
