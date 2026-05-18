/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TDS CALCULATOR (S-TDS)
 * =========================================================================
 * Law: Indian Income Tax Act, 1961
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes TDS rate and deduction amount for diverse payments.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignTDSCalculator : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignTDSCalculator"; }

    static SovereignTDSCalculator& getInstance() {
        static SovereignTDSCalculator instance;
        return instance;
    }

    enum PaymentSection {
        SEC_194C = 0, // Contractors (1% Indiv, 2% Co)
        SEC_194J = 1, // Professional / Technical Fees (10%)
        SEC_194I = 2, // Rent (2% Plant/Machy, 10% Land/Bldg)
        SEC_194H = 3  // Commission / Brokerage (5%)
    };

    struct TDSResult {
        sigma_u64 tds_deducted_paise;
        sigma_u64 net_payable_paise;
        sigma_u32 rate_basis_points; // 1% = 100 bps
        bool is_pan_penalty_applied;
    };

    /**
     * Calculates TDS.
     * C1: If no PAN (Personal Account Number) is provided, TDS is charged at a flat penalty rate of 20.00% (2000 bps)
     * C2: Threshold limits: 194C (Rs 30k single / 100k aggregate), 194J (Rs 30k), 194H (Rs 15k).
     */
    TDSResult calculate(PaymentSection section, sigma_u64 gross_amount_paise, bool has_pan, bool is_individual_contractor, bool is_rent_plant_machy) {
        TDSResult result{};
        result.is_pan_penalty_applied = false;
        
        // Threshold limits check (in paise)
        sigma_u64 threshold = 0;
        sigma_u32 base_rate_bps = 0;
        
        switch (section) {
            case SEC_194C:
                threshold = 3000000; // Single bill threshold Rs 30,000
                base_rate_bps = is_individual_contractor ? 100 : 200; // 1% or 2%
                break;
            case SEC_194J:
                threshold = 3000000; // Rs 30,000 threshold
                base_rate_bps = 1000; // 10%
                break;
            case SEC_194I:
                threshold = 24000000; // Rs 2,40,000 threshold
                base_rate_bps = is_rent_plant_machy ? 200 : 1000; // 2% or 10%
                break;
            case SEC_194H:
                threshold = 1500000; // Rs 15,000 threshold
                base_rate_bps = 500; // 5%
                break;
        }
        
        if (gross_amount_paise < threshold) {
            result.tds_deducted_paise = 0;
            result.net_payable_paise = gross_amount_paise;
            result.rate_basis_points = 0;
            sigma_log_info("[S-TDS] Transaction below statutory threshold of Rs %d.", (int)(threshold / 100));
            return result;
        }
        
        if (!has_pan) {
            result.rate_basis_points = 2000; // 20% flat penalty rate
            result.is_pan_penalty_applied = true;
            sigma_log_error("[S-TDS] COMPLIANCE WARNING: No PAN provided! flat penalty rate 20% applied.");
        } else {
            result.rate_basis_points = base_rate_bps;
        }
        
        result.tds_deducted_paise = (gross_amount_paise * result.rate_basis_points) / 10000;
        result.net_payable_paise = gross_amount_paise - result.tds_deducted_paise;
        
        sigma_log_info("[S-TDS] Gross Payment: Rs %d.%02d", (int)(gross_amount_paise / 100), (int)(gross_amount_paise % 100));
        sigma_log_info("[S-TDS] TDS Deducted: Rs %d.%02d (Rate: %d.%02d%%)", 
            (int)(result.tds_deducted_paise / 100), (int)(result.tds_deducted_paise % 100),
            (int)(result.rate_basis_points / 100), (int)(result.rate_basis_points % 100));
            
        return result;
    }

private:
    SovereignTDSCalculator() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_tds_calc(sigma_u32 section, sigma_u64 gross, sigma_u8 has_pan) {
        SigmaOS::Tools::Pro::SovereignTDSCalculator::getInstance().calculate(
            (SigmaOS::Tools::Pro::SovereignTDSCalculator::PaymentSection)section, gross, has_pan != 0, true, false);
    }
}
