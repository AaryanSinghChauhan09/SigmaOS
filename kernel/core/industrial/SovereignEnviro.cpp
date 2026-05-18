#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian Environmental Engineer Shard (S-ENVIRO)
 * Purpose: Environmental compliance tools for Indian environmental engineers and officers.
 * Standards: Environment Protection Act 1986, Water (Prevention & Control) Act 1974,
 *            Air (Prevention & Control) Act 1981, CPCB emission norms, EIA Notification 2006.
 * Features: Ambient air quality index (AQI), effluent dilution calculator, EIA category checker.
 */

namespace SigmaOS {
namespace Kernel {
namespace Industrial {

// CPCB National Ambient Air Quality Standards (NAAQS 2009)
struct NAAQS {
    const char* pollutant;
    sigma_u32 annual_ug_m3;  // Annual standard µg/m³
    sigma_u32 daily_ug_m3;   // 24-hr standard µg/m³
};

static const NAAQS NAAQS_TABLE[] = {
    {"PM2.5",   40,  60},
    {"PM10",    60, 100},
    {"NO2",     40,  80},
    {"SO2",     50,  80},
    {"CO_mg",    2,   4},   // mg/m³
    {"Ozone",  100, 180},
    {"Lead",     0,   1},   // µg/m³
};
static const sigma_u32 NAAQS_LEN = sizeof(NAAQS_TABLE) / sizeof(NAAQS_TABLE[0]);

// CPCB AQI breakpoints for PM2.5 (µg/m³, 24-hr)
struct AQIBreak {
    sigma_u32 pm25_lo;
    sigma_u32 pm25_hi;
    sigma_u32 aqi_lo;
    sigma_u32 aqi_hi;
    const char* category;
};
static const AQIBreak AQI_TABLE[] = {
    { 0,  30,   0,  50, "Good"},
    {31,  60,  51, 100, "Satisfactory"},
    {61,  90, 101, 200, "Moderate"},
    {91, 120, 201, 300, "Poor"},
    {121,250, 301, 400, "Very Poor"},
    {251,500, 401, 500, "Severe"},
};
static const sigma_u32 AQI_LEN = sizeof(AQI_TABLE) / sizeof(AQI_TABLE[0]);

class SovereignEnviro : public SigmaOS::SigmaObject {
public:
    static SovereignEnviro& getInstance() {
        static SovereignEnviro instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignEnviro"; }

    void init() {
        sigma_log_info("[S-ENVIRO] Initializing Environmental Compliance Nexus...");
        sigma_log_info("[S-ENVIRO] Standards: EPA 1986 | Air/Water Acts | CPCB NAAQS 2009 | EIA 2006");
    }

    /**
     * CPCB AQI calculator for PM2.5 (linear interpolation in integer arithmetic).
     * @param pm25_ug  PM2.5 concentration in µg/m³ (24-hr average)
     */
    sigma_u32 calcAQI(sigma_u32 pm25_ug) {
        for (sigma_u32 i = 0; i < AQI_LEN; ++i) {
            if (pm25_ug >= AQI_TABLE[i].pm25_lo && pm25_ug <= AQI_TABLE[i].pm25_hi) {
                // Linear interpolation: AQI = (AQI_hi-AQI_lo)/(PM_hi-PM_lo) * (PM-PM_lo) + AQI_lo
                sigma_u32 range_pm  = AQI_TABLE[i].pm25_hi - AQI_TABLE[i].pm25_lo;
                sigma_u32 range_aqi = AQI_TABLE[i].aqi_hi  - AQI_TABLE[i].aqi_lo;
                sigma_u32 aqi = AQI_TABLE[i].aqi_lo
                              + (range_pm > 0
                                 ? (range_aqi * (pm25_ug - AQI_TABLE[i].pm25_lo)) / range_pm
                                 : 0);
                sigma_log_info("[S-ENVIRO] CPCB AQI | PM2.5: %u µg/m³ | AQI: %u | Category: %s",
                               pm25_ug, aqi, AQI_TABLE[i].category);
                return aqi;
            }
        }
        sigma_log_err("[S-ENVIRO] PM2.5 value out of AQI range.");
        return 999;
    }

    /**
     * NAAQS compliance checker for a given pollutant and measured concentration.
     */
    void checkNAAQS(const char* pollutant, sigma_u32 daily_ug) {
        for (sigma_u32 i = 0; i < NAAQS_LEN; ++i) {
            bool m = true;
            for (sigma_u32 j = 0; NAAQS_TABLE[i].pollutant[j] || pollutant[j]; ++j) {
                if (NAAQS_TABLE[i].pollutant[j] != pollutant[j]) { m = false; break; }
            }
            if (!m) continue;
            bool ok = daily_ug <= NAAQS_TABLE[i].daily_ug_m3;
            sigma_log_info("[S-ENVIRO] NAAQS 2009 | %s: %u µg/m³ (limit %u µg/m³) | %s",
                           pollutant, daily_ug, NAAQS_TABLE[i].daily_ug_m3,
                           ok ? "✅ COMPLIANT" : "🚫 VIOLATION — source control required");
            return;
        }
        sigma_log_err("[S-ENVIRO] Pollutant '%s' not in NAAQS table.", pollutant);
    }

    /**
     * EIA Notification 2006 — project category A or B.
     * Category A (central clearance) if: thermal power ≥25MW, mining ≥50ha, 
     * large area industries. Category B: all others.
     * @param capacity_mw      Capacity in MW (for power plants, 0 if not applicable)
     * @param area_ha          Project area in hectares
     * @param in_sensitive_zone true if near eco-sensitive zone/CRZ
     */
    void eiaCategory(sigma_u32 capacity_mw, sigma_u32 area_ha, bool in_sensitive_zone) {
        bool cat_a = (capacity_mw >= 25) || (area_ha >= 50) || in_sensitive_zone;
        sigma_log_info("[S-ENVIRO] EIA 2006 | Capacity: %u MW | Area: %u ha | Eco-zone: %s | Category: %s",
                       capacity_mw, area_ha, in_sensitive_zone ? "Yes" : "No",
                       cat_a ? "A (MoEF&CC clearance required)"
                             : "B (State SEIAA clearance)");
    }
};

} // namespace Industrial
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void enviro_init() {
    SigmaOS::Kernel::Industrial::SovereignEnviro::getInstance().init();
}

sigma_u32 enviro_aqi(sigma_u32 pm25) {
    return SigmaOS::Kernel::Industrial::SovereignEnviro::getInstance().calcAQI(pm25);
}

void enviro_naaqs(const char* p, sigma_u32 conc) {
    SigmaOS::Kernel::Industrial::SovereignEnviro::getInstance().checkNAAQS(p, conc);
}

void enviro_eia(sigma_u32 mw, sigma_u32 ha, bool sensitive) {
    SigmaOS::Kernel::Industrial::SovereignEnviro::getInstance().eiaCategory(mw, ha, sensitive);
}

} // extern "C"
 