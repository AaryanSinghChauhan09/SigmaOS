#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Civil Engineering Shard (S-IS)
 * Purpose: Structural design calculators for Indian civil engineers and structural consultants.
 * Standards: IS 456:2000 (RCC), IS 875:1987 (Loads), IS 1893:2016 (Seismic),
 *            IS 800:2007 (Steel), NBC 2016 (National Building Code).
 * Features: Wind load (IS 875-3), Seismic base shear (IS 1893), RCC beam design check (IS 456).
 */

namespace SigmaOS {
namespace Kernel {
namespace Engineering {

// Seismic Zone factors per IS 1893:2016 Table 3
struct SeismicZone {
    sigma_u32 zone;     // II, III, IV, V
    sigma_u32 Z_x1000; // Zone factor Z * 1000 (Z: 0.10, 0.16, 0.24, 0.36)
};

static const SeismicZone SEISMIC_ZONES[] = {
    {2, 100},  // Zone II:  Z = 0.10
    {3, 160},  // Zone III: Z = 0.16
    {4, 240},  // Zone IV:  Z = 0.24
    {5, 360},  // Zone V:   Z = 0.36
};

// Basic Wind Speed Vb (km/h) for key Indian cities (IS 875-3 Fig 1)
struct WindCity {
    const char* city;
    sigma_u32 vb_kmh;
};

static const WindCity WIND_TABLE[] = {
    {"Mumbai",        44},
    {"Delhi",         47},
    {"Chennai",       50},
    {"Kolkata",       50},
    {"Bangalore",     33},
    {"Hyderabad",     44},
    {"Ahmedabad",     39},
    {"Pune",          39},
    {"Jaipur",        47},
    {"Bhopal",        47},
};
static const sigma_u32 WIND_TABLE_LEN = sizeof(WIND_TABLE) / sizeof(WIND_TABLE[0]);

class SovereignISCode : public SigmaOS::SigmaObject {
public:
    static SovereignISCode& getInstance() {
        static SovereignISCode instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignISCode"; }

    void init() {
        sigma_log_info("[S-IS] Initializing Indian Standards Engineering Nexus...");
        sigma_log_info("[S-IS] Standards: IS 456:2000 | IS 875:1987 | IS 1893:2016 | NBC 2016");
    }

    /**
     * Design wind pressure per IS 875 Part 3.
     * pz = 0.6 * Vz^2   (N/m²)
     * Vz = Vb * k1 * k2 * k3  — here k1=1.0, k3=1.0 for risk/terrain factor simplicity
     * @param city      City name matching WIND_TABLE
     * @param k2_x100   k2 factor * 100 (terrain factor, e.g. 98 = 0.98)
     * @param height_m  Structure height in metres (used only for logging)
     */
    void calcWindLoad(const char* city, sigma_u32 k2_x100, sigma_u32 height_m) {
        sigma_u32 vb = 0;
        for (sigma_u32 i = 0; i < WIND_TABLE_LEN; ++i) {
            bool m = true;
            for (sigma_u32 j = 0; WIND_TABLE[i].city[j] || city[j]; ++j) {
                if (WIND_TABLE[i].city[j] != city[j]) { m = false; break; }
            }
            if (m) { vb = WIND_TABLE[i].vb_kmh; break; }
        }
        if (vb == 0) {
            sigma_log_err("[S-IS] City '%s' not in IS 875-3 wind table.", city);
            return;
        }
        // Vz = Vb * k2 (simplified); pz = 0.6 * Vz^2 N/m²
        sigma_u64 vz_x100  = (sigma_u64)vb * k2_x100;          // Vz * 100
        sigma_u64 pz       = (6ULL * vz_x100 * vz_x100) / (10ULL * 10000ULL); // 0.6 * Vz^2
        sigma_log_info("[S-IS] IS 875-3 Wind Load | City: %s | Vb: %u km/h | Vz: %u.%02u km/h | pz: %llu N/m² | Height: %um",
                       city, vb, vz_x100/100, vz_x100%100, pz, height_m);
    }

    /**
     * Seismic base shear per IS 1893:2016 Cl 7.6.1
     * VB = Ah * W   where Ah = (Z/2) * (Sa/g) / (R/I)
     * Simplified for T < 0.4s: Sa/g = 2.5 (hard soil, zone-specific)
     * @param zone_no     Seismic zone (2,3,4,5)
     * @param weight_kn   Seismic weight W in kN
     * @param R           Response reduction factor (e.g. 5 for SMRF)
     * @param I_x10       Importance factor * 10 (e.g. 10 = 1.0, 15 = 1.5)
     */
    void calcBaseShear(sigma_u32 zone_no, sigma_u32 weight_kn, sigma_u32 R, sigma_u32 I_x10) {
        sigma_u32 Z_x1000 = 0;
        for (sigma_u32 i = 0; i < 4; ++i) {
            if (SEISMIC_ZONES[i].zone == zone_no) { Z_x1000 = SEISMIC_ZONES[i].Z_x1000; break; }
        }
        if (Z_x1000 == 0 || R == 0 || I_x10 == 0) {
            sigma_log_err("[S-IS] Invalid seismic parameters."); return;
        }
        // Ah = (Z/2) * 2.5 / (R/I) = Z * 2.5 * I / (2 * R)
        // Ah_x10000 = Z_x1000 * 25 * I_x10 / (2 * R * 10)
        sigma_u64 Ah_x10000 = ((sigma_u64)Z_x1000 * 25ULL * I_x10) / ((sigma_u64)2 * R * 10);
        sigma_u64 VB_kn     = (Ah_x10000 * weight_kn) / 10000ULL;
        sigma_log_info("[S-IS] IS 1893:2016 Base Shear | Zone %u | W: %u kN | R: %u | I: %u.%u | Ah: 0.%04llu | VB: %llu kN",
                       zone_no, weight_kn, R, I_x10/10, I_x10%10, Ah_x10000, VB_kn);
    }

    /**
     * IS 456:2000 Minimum steel check for RCC beam (Cl 26.5.1.1).
     * Ast_min = 0.85 * b * d / fy
     * @param b_mm   Beam width in mm
     * @param d_mm   Effective depth in mm
     * @param fy_mpa Steel yield strength (Fe415=415, Fe500=500)
     */
    void rcBeamMinSteel(sigma_u32 b_mm, sigma_u32 d_mm, sigma_u32 fy_mpa) {
        // Ast_min = 85 * b * d / (100 * fy)  [mm²]
        sigma_u64 ast_min = (85ULL * b_mm * d_mm) / (100ULL * fy_mpa);
        sigma_log_info("[S-IS] IS 456:2000 Min Steel | b=%umm, d=%umm, fy=%uMPa | Ast_min: %llu mm²",
                       b_mm, d_mm, fy_mpa, ast_min);
    }
};

} // namespace Engineering
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void is_init() {
    SigmaOS::Kernel::Engineering::SovereignISCode::getInstance().init();
}

void is_wind(const char* city, sigma_u32 k2, sigma_u32 ht) {
    SigmaOS::Kernel::Engineering::SovereignISCode::getInstance().calcWindLoad(city, k2, ht);
}

void is_seismic(sigma_u32 zone, sigma_u32 w, sigma_u32 R, sigma_u32 I) {
    SigmaOS::Kernel::Engineering::SovereignISCode::getInstance().calcBaseShear(zone, w, R, I);
}

void is_rc_beam(sigma_u32 b, sigma_u32 d, sigma_u32 fy) {
    SigmaOS::Kernel::Engineering::SovereignISCode::getInstance().rcBeamMinSteel(b, d, fy);
}

} // extern "C"
