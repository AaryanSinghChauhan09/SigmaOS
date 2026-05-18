#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Dietitian / Nutritionist Shard (S-DIET)
 * Purpose: Clinical nutrition tools for registered dietitians and nutritionists.
 * Standards: FSSAI (Food Safety & Standards Act 2006), ICMR Dietary Reference Values 2020,
 *            NIN (National Institute of Nutrition) Recommended Dietary Allowances,
 *            Eat Right India (FSSAI), POSHAN Abhiyaan norms.
 * Features: BMR calculator, RDA lookup, Macro calculator, Food label compliance checker.
 */

namespace SigmaOS {
namespace Kernel {
namespace Medical {

// ICMR-NIN 2020 RDA for Indian adults (sedentary male, 20-39 years)
struct RDAEntry {
    const char* nutrient;
    sigma_u32 rda_value;   // in standard units (mg, µg, or kcal)
    const char* unit;
};

static const RDAEntry ICMR_RDA[] = {
    {"Energy_kcal",    2110,  "kcal/day (sedentary male)"},
    {"Protein_g",        60,  "g/day"},
    {"Fat_g",            30,  "g/day"},
    {"Carb_g",          310,  "g/day"},
    {"Calcium_mg",      800,  "mg/day"},
    {"Iron_mg",          17,  "mg/day"},
    {"VitaminC_mg",      40,  "mg/day"},
    {"VitaminA_ug",     600,  "µg retinol eq/day"},
    {"Zinc_mg",          10,  "mg/day"},
    {"Folate_ug",       200,  "µg/day"},
    {"VitaminB12_ug",    1,   "µg/day"},
    {"Fibre_g",          30,  "g/day"},
};
static const sigma_u32 ICMR_LEN = sizeof(ICMR_RDA) / sizeof(ICMR_RDA[0]);

class SovereignDiet : public SigmaOS::SigmaObject {
public:
    static SovereignDiet& getInstance() {
        static SovereignDiet instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignDiet"; }

    void init() {
        sigma_log_info("[S-DIET] Initializing Indian Clinical Nutrition Nexus...");
        sigma_log_info("[S-DIET] Standards: ICMR-NIN RDA 2020 | FSSAI 2006 | POSHAN Abhiyaan");
    }

    /**
     * Mifflin-St Jeor BMR (integer approximation, kilocalories/day).
     * Male:   BMR = 10*W + 6.25*H - 5*A + 5
     * Female: BMR = 10*W + 6.25*H - 5*A - 161
     * @param weight_kg  Body weight in kg
     * @param height_cm  Height in cm
     * @param age_years  Age in years
     * @param female     1 if female
     */
    sigma_u32 calcBMR(sigma_u32 weight_kg, sigma_u32 height_cm,
                      sigma_u32 age_years, sigma_u32 female) {
        // Multiply by 4 to avoid fractions from 6.25, then divide at end
        sigma_u32 bmr_x4 = 40 * weight_kg + 25 * height_cm - 20 * age_years;
        bmr_x4 += female ? 0 : 20;          // +5 male, then divide by 4
        sigma_u32 correction = female ? 161 * 4 : 0;
        sigma_u32 bmr = (bmr_x4 - correction) / 4;
        sigma_log_info("[S-DIET] BMR (Mifflin-St Jeor) | %ukg, %ucm, %uyr, %s | BMR: %u kcal/day",
                       weight_kg, height_cm, age_years, female ? "Female" : "Male", bmr);
        return bmr;
    }

    /**
     * Macro split calculator as per ICMR 2020 (55-60% carb, 15-20% protein, 20-30% fat).
     * @param tdee_kcal  Total Daily Energy Expenditure in kcal
     */
    void calcMacros(sigma_u32 tdee_kcal) {
        sigma_u32 carb_kcal    = (tdee_kcal * 58) / 100;  // 58% mid-point
        sigma_u32 protein_kcal = (tdee_kcal * 17) / 100;  // 17% mid-point
        sigma_u32 fat_kcal     = tdee_kcal - carb_kcal - protein_kcal;

        // 1g carb/protein = 4 kcal; 1g fat = 9 kcal
        sigma_u32 carb_g    = carb_kcal    / 4;
        sigma_u32 protein_g = protein_kcal / 4;
        sigma_u32 fat_g     = fat_kcal     / 9;

        sigma_log_info("[S-DIET] ICMR Macro Split | TDEE: %u kcal | Carb: %ug | Protein: %ug | Fat: %ug",
                       tdee_kcal, carb_g, protein_g, fat_g);
    }

    /**
     * FSSAI food label compliance — checks if per-100g values are within label claim tolerances.
     * FSSAI FSS (Labelling) 2011 allows ±20% for most nutrients.
     * @param declared  Declared value per 100g
     * @param actual    Actual tested value per 100g
     */
    void fssaiLabelCheck(const char* nutrient, sigma_u32 declared, sigma_u32 actual) {
        // tolerance ±20% of declared
        sigma_u32 tol_low  = (declared * 80) / 100;
        sigma_u32 tol_high = (declared * 120) / 100;
        bool ok = (actual >= tol_low && actual <= tol_high);
        sigma_log_info("[S-DIET] FSSAI Label Check | %s | Declared: %u | Actual: %u | Range: %u-%u | %s",
                       nutrient, declared, actual, tol_low, tol_high,
                       ok ? "✅ COMPLIANT (FSS Labelling 2011)" : "🚫 NON-COMPLIANT — re-label required");
    }

    /**
     * ICMR-NIN RDA lookup.
     */
    void rda(const char* nutrient) {
        for (sigma_u32 i = 0; i < ICMR_LEN; ++i) {
            bool m = true;
            for (sigma_u32 j = 0; ICMR_RDA[i].nutrient[j] || nutrient[j]; ++j) {
                if (ICMR_RDA[i].nutrient[j] != nutrient[j]) { m = false; break; }
            }
            if (!m) continue;
            sigma_log_info("[S-DIET] ICMR-NIN RDA 2020 | %s: %u %s",
                           nutrient, ICMR_RDA[i].rda_value, ICMR_RDA[i].unit);
            return;
        }
        sigma_log_err("[S-DIET] Nutrient '%s' not in ICMR RDA table.", nutrient);
    }
};

} // namespace Medical
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void diet_init() {
    SigmaOS::Kernel::Medical::SovereignDiet::getInstance().init();
}

sigma_u32 diet_bmr(sigma_u32 wt, sigma_u32 ht, sigma_u32 age, sigma_u32 female) {
    return SigmaOS::Kernel::Medical::SovereignDiet::getInstance().calcBMR(wt, ht, age, female);
}

void diet_macros(sigma_u32 tdee) {
    SigmaOS::Kernel::Medical::SovereignDiet::getInstance().calcMacros(tdee);
}

void diet_fssai(const char* nutr, sigma_u32 decl, sigma_u32 actual) {
    SigmaOS::Kernel::Medical::SovereignDiet::getInstance().fssaiLabelCheck(nutr, decl, actual);
}

void diet_rda(const char* nutr) {
    SigmaOS::Kernel::Medical::SovereignDiet::getInstance().rda(nutr);
}

} // extern "C"
 