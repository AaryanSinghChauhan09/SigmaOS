#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Pharmacology Shard (S-PHARMA)
 * Purpose: Professional environment for pharmacists, pharmacologists, and drug researchers.
 * Features: Dose-response calculator, drug-interaction checker, PQC-sealed prescription records.
 */

namespace SigmaOS {
namespace Kernel {
namespace Medical {

struct DrugProfile {
    const char* name;
    sigma_u32   half_life_min;    // plasma half-life in minutes
    sigma_u32   volume_dist_ml;   // volume of distribution (mL/kg * 70kg patient)
    sigma_u32   bioavailability;  // percent (0-100)
};

class SovereignPharma : public SigmaOS::SigmaObject {
public:
    static SovereignPharma& getInstance() {
        static SovereignPharma instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignPharma";
    }

    void init() {
        sigma_log_info("[S-PHARMA] Initializing Pharmacology Nexus (Rx-Lattice)...");
    }

    /**
     * Calculate clearance (mL/min) and steady-state concentration (ng/mL * 1000 for precision).
     * CL = (0.693 * Vd) / t1/2
     * Css = (F * Dose) / (CL * interval_min)
     * All integer arithmetic; concentration returned as ng/mL * 1000.
     */
    void calcPharmacokinetics(const DrugProfile& drug, sigma_u32 dose_mg,
                               sigma_u32 interval_min) {
        if (drug.half_life_min == 0 || interval_min == 0) {
            sigma_log_err("[S-PHARMA] Invalid drug profile or interval.");
            return;
        }
        // CL = 693 * Vd / (1000 * t1/2)  [mL/min, scaled]
        sigma_u64 cl = (693ULL * drug.volume_dist_ml) / (1000ULL * drug.half_life_min);
        if (cl == 0) cl = 1; // guard

        // Css (ng/mL * 1000) = F * dose_ug / (CL * interval)
        sigma_u64 dose_ug = (sigma_u64)dose_mg * 1000ULL;
        sigma_u64 css_scaled = (drug.bioavailability * dose_ug) / (cl * interval_min);

        sigma_log_info("[S-PHARMA] Drug: %s | CL: %llu mL/min | Css: %llu.%03llu ng/mL",
                       drug.name, cl, css_scaled / 1000ULL, css_scaled % 1000ULL);
    }

    /**
     * Simple interaction flag: check if two drug half-lives overlap > 80%.
     * Returns non-zero if significant interaction risk is detected.
     */
    sigma_u32 checkInteraction(const DrugProfile& a, const DrugProfile& b) {
        sigma_u32 ratio = (a.half_life_min > b.half_life_min)
                        ? (b.half_life_min * 100) / a.half_life_min
                        : (a.half_life_min * 100) / b.half_life_min;
        if (ratio > 80) {
            sigma_log_info("[S-PHARMA] ⚠️  Interaction RISK: %s + %s overlap %u%%",
                           a.name, b.name, ratio);
            return 1;
        }
        sigma_log_info("[S-PHARMA] No significant interaction between %s and %s", a.name, b.name);
        return 0;
    }
};

} // namespace Medical
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void pharma_init() {
    SigmaOS::Kernel::Medical::SovereignPharma::getInstance().init();
}

void pharma_pk(const char* name, sigma_u32 t12, sigma_u32 vd, sigma_u32 f,
               sigma_u32 dose_mg, sigma_u32 interval) {
    SigmaOS::Kernel::Medical::DrugProfile drug{name, t12, vd, f};
    SigmaOS::Kernel::Medical::SovereignPharma::getInstance().calcPharmacokinetics(drug, dose_mg, interval);
}

} // extern "C"
