/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GRATUITY TAX EXEMPTION CALCULATOR (S-GTX)
 * =========================================================================
 * Law: Section 10(10) of the Indian Income Tax Act, 1961
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes taxable and tax-exempt portions of gratuity payments.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignGratuityTaxCalc : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignGratuityTaxCalc"; }

    static SovereignGratuityTaxCalc& getInstance() {
        static SovereignGratuityTaxCalc instance;
        return instance;
    }

    struct TaxExemptionResult {
        sigma_u64 exempt_amount_paise;
        sigma_u64 taxable_amount_paise;
        bool is_fully_exempt;
    };

    /**
     * Calculates tax exemption.
     * C1: Government employees: Gratuity is fully tax-exempt.
     * C2: Covered under Gratuity Act 1972: Exempt amount is the least of:
     *     a) Rs 20,00,000 (20 Lakhs limit)
     *     b) Actual gratuity received
     *     c) 15 days salary per year of service: (15 * last basic & DA * years) / 26
     * C3: Not covered under Gratuity Act: Exempt is least of:
     *     a) Rs 20,00,000
     *     b) Actual gratuity
     *     c) Half month's average salary per year of service: (15 * avg salary * years) / 30
     */
    TaxExemptionResult calculate(bool is_govt_employee, bool is_covered_under_act, sigma_u64 gratuity_received_paise, sigma_u64 last_salary_paise, sigma_u32 service_years) {
        TaxExemptionResult result{};
        
        if (is_govt_employee) {
            result.exempt_amount_paise = gratuity_received_paise;
            result.taxable_amount_paise = 0;
            result.is_fully_exempt = true;
            sigma_log_info("[S-GTX] Govt Employee Gratuity: Fully tax-exempt.");
            return result;
        }
        
        result.is_fully_exempt = false;
        const sigma_u64 EXEMPTION_LIMIT_PAISE = 200000000; // Rs 20 Lakhs
        
        sigma_u64 formula_limit = 0;
        if (is_covered_under_act) {
            formula_limit = (last_salary_paise * 15 * service_years) / 26;
        } else {
            formula_limit = (last_salary_paise * 15 * service_years) / 30; // Half month average salary
        }
        
        // Exemption is the minimum of actual received, statutory limit, and formula result
        sigma_u64 least_exempt = gratuity_received_paise;
        if (EXEMPTION_LIMIT_PAISE < least_exempt) {
            least_exempt = EXEMPTION_LIMIT_PAISE;
        }
        if (formula_limit < least_exempt) {
            least_exempt = formula_limit;
        }
        
        result.exempt_amount_paise = least_exempt;
        result.taxable_amount_paise = gratuity_received_paise - least_exempt;
        
        sigma_log_info("[S-GTX] Gratuity Received: Rs %d.%02d", (int)(gratuity_received_paise / 100), (int)(gratuity_received_paise % 100));
        sigma_log_info("[S-GTX] Tax Exempt: Rs %d.%02d (Taxable Portion: Rs %d.%02d)",
            (int)(result.exempt_amount_paise / 100), (int)(result.exempt_amount_paise % 100),
            (int)(result.taxable_amount_paise / 100), (int)(result.taxable_amount_paise % 100));
            
        return result;
    }

private:
    SovereignGratuityTaxCalc() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_gratuity_tax_calc(sigma_u8 govt, sigma_u8 covered, sigma_u64 received, sigma_u64 salary, sigma_u32 years) {
        SigmaOS::Tools::Pro::SovereignGratuityTaxCalc::getInstance().calculate(govt != 0, covered != 0, received, salary, years);
    }
}
