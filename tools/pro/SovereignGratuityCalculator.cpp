/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GRATUITY CALCULATOR (S-GRATUITY)
 * =========================================================================
 * Law: Payment of Gratuity Act, 1972
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes gratuity benefit entitlement for retired or separating employees.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignGratuityCalculator : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignGratuityCalculator"; }

    static SovereignGratuityCalculator& getInstance() {
        static SovereignGratuityCalculator instance;
        return instance;
    }

    struct GratuityResult {
        sigma_u64 statutory_entitlement_paise;
        sigma_u64 raw_calculated_paise;
        bool is_entitled;
        bool is_statutory_cap_exceeded;
    };

    /**
     * Calculates Gratuity entitlement.
     * Minimum continuous service requirement: 5 years (unless deceased or disabled).
     * Formula: (15 * last drawn basic salary & DA * completed years of service) / 26
     * Statutory Limit Cap: Rs 20,000,000 paise (Rs 20 Lakhs).
     */
    GratuityResult calculate(sigma_u64 last_drawn_basic_da_paise, sigma_u32 completed_years_service, bool deceased_or_disabled) {
        GratuityResult result{};
        
        const sigma_u64 STATUTORY_CAP_PAISE = 200000000; // Rs 20,000,000 paise (Rs 20 Lakhs)
        const sigma_u32 MINIMUM_SERVICE_YEARS = 5;
        
        if (completed_years_service < MINIMUM_SERVICE_YEARS && !deceased_or_disabled) {
            result.is_entitled = false;
            result.statutory_entitlement_paise = 0;
            result.raw_calculated_paise = 0;
            result.is_statutory_cap_exceeded = false;
            sigma_log_info("[S-GRATUITY] Employee not entitled: Less than 5 years continuous service.");
            return result;
        }
        
        result.is_entitled = true;
        
        // Compute raw formula: (15 * basic * years) / 26
        result.raw_calculated_paise = (last_drawn_basic_da_paise * 15 * completed_years_service) / 26;
        
        if (result.raw_calculated_paise > STATUTORY_CAP_PAISE) {
            result.statutory_entitlement_paise = STATUTORY_CAP_PAISE;
            result.is_statutory_cap_exceeded = true;
        } else {
            result.statutory_entitlement_paise = result.raw_calculated_paise;
            result.is_statutory_cap_exceeded = false;
        }
        
        sigma_log_info("[S-GRATUITY] Gratuity Calculated: Rs %d.%02d (Statutory Capped: Rs %d.%02d)", 
            (int)(result.raw_calculated_paise / 100), (int)(result.raw_calculated_paise % 100),
            (int)(result.statutory_entitlement_paise / 100), (int)(result.statutory_entitlement_paise % 100));
            
        return result;
    }

private:
    SovereignGratuityCalculator() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_gratuity_calc(sigma_u64 basic_da, sigma_u32 years) {
        SigmaOS::Tools::Pro::SovereignGratuityCalculator::getInstance().calculate(basic_da, years, false);
    }
}
