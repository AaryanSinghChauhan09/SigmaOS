#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Agriculture Shard (S-AGRI)
 * Purpose: Tools for Indian farmers, agronomists, and agri‑consultants.
 * Standards: Indian Council of Agricultural Research (ICAR) guidelines, MSP 2024, FPO regulations.
 * Features: Crop‑yield estimator, Kharif‑Rabi sowing window checker, Soil‑NPK balance calculator.
 */

namespace SigmaOS {
namespace Kernel {
namespace Agriculture {

// Simple integer‑based crop yield estimator (kg per hectare)
struct CropYield {
    const char* crop;           // e.g. "Wheat"
    sigma_u32 expected_kg_per_ha; // Expected yield in kg/ha
};

static const CropYield CROP_YIELD_TABLE[] = {
    {"Wheat", 3400},
    {"Rice",  4200},
    {"Maize", 3000},
    {"Cotton", 2500},
    {"Sugarcane", 8000},
};
static const sigma_u32 CROP_YIELD_LEN = sizeof(CROP_YIELD_TABLE) / sizeof(CROP_YIELD_TABLE[0]);

// Soil NPK balance checker (mg/kg)
struct SoilNPK {
    sigma_u32 nitrogen;
    sigma_u32 phosphorus;
    sigma_u32 potassium;
    
    // Recommended minimum per ICAR guidelines
    static constexpr sigma_u32 MIN_N = 280; // kg/ha equivalent mapped to units
    static constexpr sigma_u32 MIN_P = 20;
    static constexpr sigma_u32 MIN_K = 120;
};

class SovereignAgri : public SigmaOS::SigmaObject {
public:
    static SovereignAgri& getInstance() {
        static SovereignAgri instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignAgri"; }

    void init() {
        sigma_log_info("[S-AGRI] Initializing Indian Agriculture Nexus...");
        sigma_log_info("[S-AGRI] Standards: ICAR Guidelines | MSP 2024 | FPO Regulations");
    }

    /**
     * Compare actual yield against ICAR expected yields.
     */
    void checkYield(const char* crop, sigma_u32 actual_kg_ha) {
        for (sigma_u32 i = 0; i < CROP_YIELD_LEN; ++i) {
            bool match = true;
            for (sigma_u32 j = 0; CROP_YIELD_TABLE[i].crop[j] || crop[j]; ++j) {
                if (CROP_YIELD_TABLE[i].crop[j] != crop[j]) { match = false; break; }
            }
            if (match) {
                sigma_i32 variance = (sigma_i32)actual_kg_ha - (sigma_i32)CROP_YIELD_TABLE[i].expected_kg_per_ha;
                sigma_log_info("[S-AGRI] Yield Audit | Crop: %s | Actual: %u kg/ha | Expected: %u kg/ha | Variance: %d",
                               crop, actual_kg_ha, CROP_YIELD_TABLE[i].expected_kg_per_ha, variance);
                return;
            }
        }
        sigma_log_err("[S-AGRI] Crop '%s' not in yield database.", crop);
    }

    /**
     * Check Soil NPK levels against ICAR recommended minimums.
     */
    void checkSoilHealth(sigma_u32 n, sigma_u32 p, sigma_u32 k) {
        bool n_ok = n >= SoilNPK::MIN_N;
        bool p_ok = p >= SoilNPK::MIN_P;
        bool k_ok = k >= SoilNPK::MIN_K;

        sigma_log_info("[S-AGRI] Soil Health Report:");
        sigma_log_info("[S-AGRI]   Nitrogen: %u (%s)", n, n_ok ? "OPTIMAL" : "DEFICIENT - Add Urea/Compost");
        sigma_log_info("[S-AGRI]   Phosphorus: %u (%s)", p, p_ok ? "OPTIMAL" : "DEFICIENT - Add DAP");
        sigma_log_info("[S-AGRI]   Potassium: %u (%s)", k, k_ok ? "OPTIMAL" : "DEFICIENT - Add MOP");
    }

    /**
     * Determine sowing window based on crop type (Kharif/Rabi/Zaid).
     */
    void sowingWindow(const char* crop) {
        // Simple mapping
        const char* window = "Unknown";
        if (isMatch(crop, "Rice") || isMatch(crop, "Cotton") || isMatch(crop, "Maize")) {
            window = "Kharif (June - July)";
        } else if (isMatch(crop, "Wheat") || isMatch(crop, "Mustard") || isMatch(crop, "Gram")) {
            window = "Rabi (October - December)";
        } else if (isMatch(crop, "Sugarcane")) {
            window = "Perennial / Multi-season";
        }

        sigma_log_info("[S-AGRI] Sowing Recommendation | Crop: %s | Window: %s", crop, window);
    }

private:
    bool isMatch(const char* s1, const char* s2) {
        for (sigma_u32 i = 0; s1[i] || s2[i]; ++i) {
            if (s1[i] != s2[i]) return false;
        }
        return true;
    }
};

} // namespace Agriculture
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void agri_init() {
    SigmaOS::Kernel::Agriculture::SovereignAgri::getInstance().init();
}

void agri_check_yield(const char* crop, sigma_u32 actual) {
    SigmaOS::Kernel::Agriculture::SovereignAgri::getInstance().checkYield(crop, actual);
}

void agri_check_soil(sigma_u32 n, sigma_u32 p, sigma_u32 k) {
    SigmaOS::Kernel::Agriculture::SovereignAgri::getInstance().checkSoilHealth(n, p, k);
}

void agri_sowing_window(const char* crop) {
    SigmaOS::Kernel::Agriculture::SovereignAgri::getInstance().sowingWindow(crop);
}

} // extern "C"
