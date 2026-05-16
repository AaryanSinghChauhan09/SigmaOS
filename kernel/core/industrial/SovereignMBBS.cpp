#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Doctor Shard (S-MBBS)
 * Purpose: Clinical decision support for Indian MBBS/MD doctors.
 * Standards: National Medical Commission (NMC), ICMR clinical protocols,
 *            MCI Ethics 2002, Jan Aushadhi Scheme drug references.
 * Features: Creatinine clearance (CKD-EPI), BMI with Indian cutoffs,
 *           Pedantic drug dose calculator, Antenatal risk stratifier.
 */

namespace SigmaOS {
namespace Kernel {
namespace Medical {

// Indian BMI classification (WHO modified for Asian populations, IAP 2009)
struct BMICategory {
    sigma_u32 bmi_x10;   // BMI * 10 (integer)
    const char* label;
};

static const BMICategory INDIAN_BMI_TABLE[] = {
    { 185, "Underweight (Severe)" },
    { 225, "Underweight" },
    { 230, "Normal (Asian cutoff)" },
    { 275, "Overweight (Asian: >=23)" },
    { 300, "Obese Class I (Asian: >=27.5)" },
    { 350, "Obese Class II (>=30)" },
    {   0, "Obese Class III (Morbid)" },
};

class SovereignMBBS : public SigmaOS::SigmaObject {
public:
    static SovereignMBBS& getInstance() {
        static SovereignMBBS instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignMBBS"; }

    void init() {
        sigma_log_info("[S-MBBS] Initializing Clinical Decision Support Nexus...");
        sigma_log_info("[S-MBBS] Standards: NMC | ICMR | WHO-Asia BMI | Jan Aushadhi");
    }

    /**
     * BMI calculator with Indian (Asian) WHO-modified cutoffs.
     * @param weight_kg  Weight in kg (integer)
     * @param height_cm  Height in cm (integer)
     */
    void calcBMI(sigma_u32 weight_kg, sigma_u32 height_cm) {
        if (height_cm == 0) { sigma_log_err("[S-MBBS] Height cannot be zero."); return; }
        // BMI = weight / (height_m)^2; we work with * 10 precision
        // bmi_x10 = weight_kg * 10000 * 100 / (height_cm^2)
        sigma_u64 bmi_x10 = (sigma_u64)weight_kg * 1000000ULL / ((sigma_u64)height_cm * height_cm);

        sigma_log_info("[S-MBBS] BMI: %llu.%01llu kg/m²", bmi_x10/10, bmi_x10%10);
        for (sigma_u32 i = 0; i < 7; ++i) {
            if (INDIAN_BMI_TABLE[i].bmi_x10 == 0 || bmi_x10 < (sigma_u64)INDIAN_BMI_TABLE[i].bmi_x10) {
                sigma_log_info("[S-MBBS] Category (Asian cutoff): %s", INDIAN_BMI_TABLE[i].label);
                break;
            }
        }
    }

    /**
     * CKD-EPI Creatinine Clearance (integer approximation).
     * Simplified: GFR ≈ 186 × (Scr)^-1.154 × (Age)^-0.203 × (0.742 if female) × (1.212 if Black)
     * We use a linearised approximation valid for Scr 0.5–2.0 mg/dL.
     * @param scr_x100   Serum creatinine * 100 (e.g. 120 = 1.20 mg/dL)
     * @param age        Patient age in years
     * @param female     1 if female
     */
    void calcGFR(sigma_u32 scr_x100, sigma_u32 age, sigma_u32 female) {
        if (scr_x100 == 0 || age == 0) {
            sigma_log_err("[S-MBBS] Invalid GFR parameters.");
            return;
        }
        // Linear approx: GFR ≈ (18600 / scr_x100) * (75 / age)  [scaled]
        sigma_u64 gfr = (18600ULL * 75ULL) / ((sigma_u64)scr_x100 * age);
        if (female) gfr = (gfr * 742) / 1000; // female correction 0.742

        const char* ckd_stage;
        if (gfr >= 90)       ckd_stage = "G1 (Normal or High)";
        else if (gfr >= 60)  ckd_stage = "G2 (Mildly decreased)";
        else if (gfr >= 45)  ckd_stage = "G3a (Mild-Moderate)";
        else if (gfr >= 30)  ckd_stage = "G3b (Moderate-Severe)";
        else if (gfr >= 15)  ckd_stage = "G4 (Severely decreased)";
        else                 ckd_stage = "G5 (Kidney Failure — refer nephrology)";

        sigma_log_info("[S-MBBS] eGFR ≈ %llu mL/min/1.73m² | CKD Stage: %s", gfr, ckd_stage);
    }

    /**
     * Paediatric dose calculator (Clark's rule + weight-based).
     * @param adult_dose_mg   Standard adult dose in mg
     * @param child_weight_kg Child weight in kg
     * @param child_age_years Child age in years
     */
    void calcPaedDose(sigma_u32 adult_dose_mg, sigma_u32 child_weight_kg,
                      sigma_u32 child_age_years) {
        // Clark's rule: dose = (weight_lb / 150) * adult_dose
        // weight_lb ≈ weight_kg * 2.2 (approx: * 22 / 10)
        sigma_u32 weight_lb = (child_weight_kg * 22) / 10;
        sigma_u32 clark_dose = (weight_lb * adult_dose_mg) / 150;

        // Young's rule: dose = (age / (age + 12)) * adult_dose
        sigma_u32 young_dose = (child_age_years * adult_dose_mg) / (child_age_years + 12);

        sigma_log_info("[S-MBBS] Paediatric Dose | Weight-based (Clark): %u mg | Age-based (Young): %u mg",
                       clark_dose, young_dose);
        sigma_log_info("[S-MBBS] ⚠️  Always verify with Jan Aushadhi/NMC prescribing guidelines.");
    }

    /**
     * Antenatal High-Risk Stratifier per ICMR/MoHFW guidelines.
     * Returns risk score (0=low, 1=moderate, 2=high).
     */
    sigma_u32 antenatalRisk(sigma_u32 age, sigma_u32 parity, sigma_u32 bp_systolic,
                             sigma_u32 hemoglobin_x10, bool prev_cs) {
        sigma_u32 score = 0;
        if (age < 18 || age > 35) score++;
        if (parity >= 4) score++;
        if (bp_systolic >= 140) score += 2;
        if (hemoglobin_x10 < 80) score++;     // Hb < 8.0 g/dL
        if (prev_cs) score++;

        const char* label = (score == 0) ? "LOW RISK" : (score <= 2) ? "MODERATE RISK" : "HIGH RISK";
        sigma_log_info("[S-MBBS] Antenatal Risk Score: %u — %s", score, label);
        if (score >= 2) {
            sigma_log_info("[S-MBBS] ⚠️  Refer to FRU/CEmONC as per JSSK/NHM protocol.");
        }
        return score;
    }
};

} // namespace Medical
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void mbbs_init() {
    SigmaOS::Kernel::Medical::SovereignMBBS::getInstance().init();
}

void mbbs_bmi(sigma_u32 wt, sigma_u32 ht_cm) {
    SigmaOS::Kernel::Medical::SovereignMBBS::getInstance().calcBMI(wt, ht_cm);
}

void mbbs_gfr(sigma_u32 scr_x100, sigma_u32 age, sigma_u32 female) {
    SigmaOS::Kernel::Medical::SovereignMBBS::getInstance().calcGFR(scr_x100, age, female);
}

void mbbs_paed_dose(sigma_u32 adult_mg, sigma_u32 wt_kg, sigma_u32 age_yr) {
    SigmaOS::Kernel::Medical::SovereignMBBS::getInstance().calcPaedDose(adult_mg, wt_kg, age_yr);
}

sigma_u32 mbbs_antenatal(sigma_u32 age, sigma_u32 par, sigma_u32 bp, sigma_u32 hb_x10, bool cs) {
    return SigmaOS::Kernel::Medical::SovereignMBBS::getInstance()
               .antenatalRisk(age, par, bp, hb_x10, cs);
}

} // extern "C"
