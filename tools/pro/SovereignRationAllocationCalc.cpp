/*
 * =========================================================================
 * Σ SIGMAOS: PUBLIC DISTRIBUTION RATION ALLOCATION CALCULATOR (S-PDS)
 * =========================================================================
 * Law: National Food Security Act, 2013 (NFSA)
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes food grain entitlement and subsidized cost.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignRationAllocationCalc : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignRationAllocationCalc"; }

    static SovereignRationAllocationCalc& getInstance() {
        static SovereignRationAllocationCalc instance;
        return instance;
    }

    enum HouseholdCategory {
        PRIORITY_HOUSEHOLD = 0, // PHH (5 kg / person)
        ANTYODAYA_ANNA_YOJANA = 1 // AAY (35 kg / household)
    };

    struct RationReport {
        sigma_u32 wheat_kg;
        sigma_u32 rice_kg;
        sigma_u32 coarse_grain_kg;
        sigma_u32 total_kg;
        sigma_u64 total_cost_paise;
    };

    /**
     * Calculates entitlement under NFSA.
     * C1: PHH - 5kg per person. AAY - 35kg per household.
     * C2: Subsidized prices: Rice @ Rs 3/kg (300 paise), Wheat @ Rs 2/kg (200 paise), Coarse grains @ Rs 1/kg (100 paise).
     * C3: Allocation mix: 50% Rice, 40% Wheat, 10% Coarse grains.
     */
    RationReport calculate(HouseholdCategory category, sigma_u32 family_members) {
        RationReport report{};
        
        sigma_u32 total_quota_kg = 0;
        if (category == ANTYODAYA_ANNA_YOJANA) {
            total_quota_kg = 35; // Fixed per household
        } else {
            total_quota_kg = family_members * 5; // 5 kg per member
        }
        
        // Split quota: 50% Rice, 40% Wheat, 10% Coarse grains
        report.rice_kg = total_quota_kg / 2;
        report.wheat_kg = (total_quota_kg * 4) / 10;
        report.coarse_grain_kg = total_quota_kg - (report.rice_kg + report.wheat_kg);
        report.total_kg = total_quota_kg;
        
        // Compute Cost in Paise
        report.total_cost_paise = (report.rice_kg * 300) + 
                                  (report.wheat_kg * 200) + 
                                  (report.coarse_grain_kg * 100);
                                  
        sigma_log_info("[S-PDS] Entitlement Audited: Wheat: %d kg, Rice: %d kg, Coarse: %d kg.",
            report.wheat_kg, report.rice_kg, report.coarse_grain_kg);
        sigma_log_info("[S-PDS] Total Cost: Rs %d.%02d",
            (int)(report.total_cost_paise / 100), (int)(report.total_cost_paise % 100));
            
        return report;
    }

private:
    SovereignRationAllocationCalc() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_pds_ration_calc(sigma_u32 cat, sigma_u32 members) {
        SigmaOS::Tools::Pro::SovereignRationAllocationCalc::getInstance().calculate(
            (SigmaOS::Tools::Pro::SovereignRationAllocationCalc::HouseholdCategory)cat, members);
    }
}
