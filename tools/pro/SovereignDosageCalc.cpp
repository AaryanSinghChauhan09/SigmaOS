/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PRO TOOL - SovereignDosageCalc
 * =========================================================================
 * REGULATORY CONTEXT: CDSCO Pharmacopoeia / Drug Dosage Guidelines (Indian Standards)
 * Principle: Bare-metal execution, zero standard library dependencies.
 * =========================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace ProTools {

struct DrugProfile {
    char        name[32];
    sigma_u32   dose_mg_per_kg; // dose per day per kg body weight
    sigma_u32   doses_per_day;  // frequency (e.g. TDS = 3, BD = 2)
    sigma_u32   suspension_mg_per_5ml; // suspension concentration (mg / 5ml)
};

class SovereignDosageCalc {
public:
    void init() {
        sigma_log_info("[SovereignDosage] Clinical Drug Dosage Calculator (CDSCO Compliant) initialized.");
    }

    sigma_u32 calculate_dose(sigma_u32 patient_weight_kg, const char* drug_name, 
                             sigma_u32* out_daily_mg, sigma_u32* out_single_mg, sigma_u32* out_single_ml_scaled) {
        
        DrugProfile profiles[3] = {
            {"Paracetamol", 60, 4, 120},  // 60mg/kg/day, divided in 4 doses, 120mg/5ml suspension
            {"Ibuprofen", 30, 3, 100},    // 30mg/kg/day, divided in 3 doses, 100mg/5ml suspension
            {"Amoxicillin", 45, 3, 250}   // 45mg/kg/day, divided in 3 doses, 250mg/5ml suspension
        };

        DrugProfile selected = {"", 0, 0, 0};
        for (int i = 0; i < 3; i++) {
            if (sigma_strcmp(profiles[i].name, drug_name) == 0) {
                selected = profiles[i];
                break;
            }
        }

        if (selected.dose_mg_per_kg == 0) {
            sigma_log_error("[SovereignDosage] Drug profile not found in CDSCO registry: %s", drug_name);
            return SIGMA_ERROR;
        }

        sigma_u32 daily_mg = patient_weight_kg * selected.dose_mg_per_kg;
        sigma_u32 single_mg = daily_mg / selected.doses_per_day;
        
        // ML calculation scaled by 100 to avoid floating points (e.g. 5.50 ml is represented as 550)
        sigma_u32 single_ml_scaled = (single_mg * 5 * 100) / selected.suspension_mg_per_5ml;

        *out_daily_mg = daily_mg;
        *out_single_mg = single_mg;
        *out_single_ml_scaled = single_ml_scaled;

        sigma_log_info("[SovereignDosage] Patient: %ukg | Drug: %s | Daily: %umg | Single: %umg | Dose Vol: %u.%02u ml",
                       patient_weight_kg, drug_name, daily_mg, single_mg, 
                       single_ml_scaled / 100, single_ml_scaled % 100);

        return SIGMA_OK;
    }
};

} // namespace ProTools
} // namespace SigmaOS

extern "C" {
    void dosage_init() {
        SigmaOS::ProTools::SovereignDosageCalc calc;
        calc.init();
    }

    sigma_u32 dosage_calculate(sigma_u32 weight, const char* drug, 
                               sigma_u32* daily_mg, sigma_u32* single_mg, sigma_u32* single_ml_scaled) {
        SigmaOS::ProTools::SovereignDosageCalc calc;
        return calc.calculate_dose(weight, drug, daily_mg, single_mg, single_ml_scaled);
    }
}
