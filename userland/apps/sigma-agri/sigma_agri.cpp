// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-agri — Agriculture tools for Indian farmers
// Covers: MSP, PMFBY insurance, eNAM, PM-Kisan, weather, soil health
//
// CLI:
//   sigma-agri msp --crop wheat --year 2026
//   sigma-agri insurance premium --crop paddy --state PB --area 2.5
//   sigma-agri weather --district Ludhiana --forecast 7days
//   sigma-agri enam register --fpo --district Amritsar
//   sigma-agri pmkisan status --aadhar-last4 1234
//   sigma-agri soil --plot-id P001 --nutrient NPK

#include <stdint.h>
#include <string.h>
#include <stdio.h>

// ── MSP (Minimum Support Price) Table 2025-26 (Cabinet approved) ─────────────
// Source: CACP recommendations, Cabinet approval

struct MSPEntry {
    const char *crop;
    const char *category;     // Kharif / Rabi / Commercial
    uint32_t    msp_per_qtl;  // ₹ per quintal
    uint32_t    year;
    float       cost_a2fl;    // ₹/qtl cost of production
    float       return_pct;   // return over A2+FL cost
};

static const MSPEntry msp_table[] = {
    // Kharif 2025-26
    { "paddy_common",    "Kharif", 2320,  2026, 1455.0f, 59.5f },
    { "paddy_grade_a",   "Kharif", 2360,  2026, 1477.0f, 59.8f },
    { "jowar_hybrid",    "Kharif", 3371,  2026, 2160.0f, 56.0f },
    { "bajra",           "Kharif", 2625,  2026, 1494.0f, 75.7f },
    { "maize",           "Kharif", 2225,  2026, 1441.0f, 54.4f },
    { "tur_arhar",       "Kharif", 7550,  2026, 4783.0f, 57.8f },
    { "moong",           "Kharif", 8682,  2026, 5445.0f, 59.5f },
    { "urad",            "Kharif", 7400,  2026, 4660.0f, 58.8f },
    { "groundnut",       "Kharif", 6783,  2026, 4251.0f, 59.6f },
    { "sunflower_seed",  "Kharif", 7280,  2026, 4572.0f, 59.3f },
    { "soybean_yellow",  "Kharif", 4892,  2026, 3079.0f, 58.9f },
    { "sesamum",         "Kharif", 9267,  2026, 5765.0f, 60.7f },
    { "nigerseed",       "Kharif", 8717,  2026, 5502.0f, 58.4f },
    { "cotton_medium",   "Kharif", 7121,  2026, 4451.0f, 60.0f },
    { "cotton_long",     "Kharif", 7521,  2026, 4698.0f, 60.1f },
    // Rabi 2025-26
    { "wheat",           "Rabi",   2425,  2026, 1506.0f, 61.0f },
    { "barley",          "Rabi",   1943,  2026, 1218.0f, 59.5f },
    { "gram_chana",      "Rabi",   5650,  2026, 3575.0f, 58.0f },
    { "masur_lentil",    "Rabi",   6700,  2026, 4205.0f, 59.3f },
    { "rapeseed_mustard","Rabi",   5950,  2026, 3746.0f, 58.9f },
    { "safflower",       "Rabi",   5940,  2026, 3736.0f, 59.0f },
    { "toria",           "Rabi",   5950,  2026, 3730.0f, 59.6f },
    // Other
    { "sugarcane",       "Other",   340,  2026,  212.0f, 60.4f }, // ₹/qtl FRP
    { "jute",            "Other",   5335, 2026, 3329.0f, 60.2f },
    { "copra_milling",   "Other",   11582,2026, 7258.0f, 59.6f },
    { "copra_ball",      "Other",   12100,2026, 7567.0f, 59.9f },
    { "raw_jute",        "Other",   5335, 2026, 3334.0f, 60.0f },
    { NULL, NULL, 0, 0, 0.0f, 0.0f }
};

void sigma_agri_msp(const char *crop, uint32_t year) {
    for (int i = 0; msp_table[i].crop; i++) {
        if (strcmp(msp_table[i].crop, crop) == 0 && msp_table[i].year == year) {
            const MSPEntry *e = &msp_table[i];
            printf("MSP for %s (%s) FY%u:\n", crop, e->category, year);
            printf("  ₹%u per quintal (100 kg)\n", e->msp_per_qtl);
            printf("  ₹%u per metric tonne\n", e->msp_per_qtl * 10);
            printf("  Cost A2+FL: ₹%.0f/qtl | Return: %.1f%%\n",
                   e->cost_a2fl, e->return_pct);
            printf("  Procurement agency: FCI / State agencies\n");
            return;
        }
    }
    printf("MSP not found for crop '%s' year %u\n", crop, year);
    printf("Try: sigma-agri msp --list to see all crops\n");
}

void sigma_agri_msp_list(void) {
    printf("%-20s %-10s %10s\n", "Crop", "Category", "MSP(₹/qtl)");
    printf("%-20s %-10s %10s\n", "----", "--------", "----------");
    for (int i = 0; msp_table[i].crop; i++) {
        printf("%-20s %-10s %10u\n",
               msp_table[i].crop,
               msp_table[i].category,
               msp_table[i].msp_per_qtl);
    }
}

// ── PMFBY Insurance Premium Calculator ───────────────────────────────────────
// PMFBY: Pradhan Mantri Fasal Bima Yojana
// Farmer's share: Kharif ≤ 2%, Rabi ≤ 1.5%, Annual Commercial/Hort ≤ 5%

struct PMFBYRate {
    const char *season;
    float       farmer_pct;  // max farmer share of SI
    float       govt_pct;    // govt premium subsidy
};

static const PMFBYRate pmfby_rates[] = {
    { "kharif",    2.0f, 98.0f },
    { "rabi",      1.5f, 98.5f },
    { "commercial",5.0f, 95.0f },
    { NULL, 0.0f, 0.0f }
};

void sigma_agri_insurance_premium(const char *crop, const char *state,
                                   float area_ha, const char *season) {
    // Determine season from crop if not given
    const char *det_season = season ? season : "kharif";
    // Approximate Sum Insured (SI) per hectare by crop (₹/ha)
    uint32_t si_per_ha = 45000; // default
    if (strstr(crop, "wheat") || strstr(crop, "mustard")) {
        si_per_ha = 55000; det_season = "rabi";
    } else if (strstr(crop, "sugarcane")) {
        si_per_ha = 150000; det_season = "commercial";
    } else if (strstr(crop, "paddy") || strstr(crop, "cotton")) {
        si_per_ha = 65000; det_season = "kharif";
    }

    float total_si = si_per_ha * area_ha;
    float farmer_pct = 2.0f;
    for (int i = 0; pmfby_rates[i].season; i++) {
        if (strcmp(pmfby_rates[i].season, det_season) == 0)
            farmer_pct = pmfby_rates[i].farmer_pct;
    }
    float farmer_premium = total_si * farmer_pct / 100.0f;
    float govt_premium   = total_si * (100.0f - farmer_pct) / 100.0f;

    printf("PMFBY Premium Estimate — %s [%s, %s]\n", crop, state, det_season);
    printf("  Area: %.2f hectares\n", area_ha);
    printf("  Sum Insured (SI): ₹%.0f\n", total_si);
    printf("  Farmer premium (%.1f%%): ₹%.0f\n", farmer_pct, farmer_premium);
    printf("  Govt subsidy (%.1f%%): ₹%.0f\n", (100.0f - farmer_pct), govt_premium);
    printf("  Total premium: ₹%.0f\n", total_si * 5.0f / 100.0f);
    printf("  Enroll via: sigma-agri insurance enroll --crop %s --state %s\n",
           crop, state);
    printf("  Portal: pmfby.gov.in\n");
}

// ── eNAM (Electronic National Agriculture Market) ────────────────────────────

void sigma_agri_enam_register(const char *entity_type, const char *district) {
    printf("eNAM Registration — %s, %s\n", entity_type, district);
    printf("Step 1: Obtain FPO/Farmer registration certificate\n");
    printf("Step 2: Upload: Aadhaar, bank passbook, land records\n");
    printf("Step 3: Visit nearest APMC mandi or:\n");
    printf("        sigma-gov gem order --category Agriculture\n");
    printf("Portal: enam.gov.in | Helpline: 1800-270-0224\n");
    printf("Tradable commodities: 115+ agricultural commodities\n");
    printf("Payment: T+1 settlement via NEFT/RTGS\n");
}

// ── Weather forecast (offline: India Meteorological Department API stub) ──────

void sigma_agri_weather(const char *district, int forecast_days) {
    printf("Weather Forecast — %s (%d-day)\n", district, forecast_days);
    printf("Data source: IMD API (api.weather.imd.gov.in)\n");
    printf("For live data: sigma-net must be connected\n");
    // Offline fallback: seasonal averages by region
    printf("Seasonal advisory (Punjab Kharif):\n");
    printf("  Max temp: 35-38°C | Min temp: 26-28°C\n");
    printf("  Rainfall probability: 40-60%% (SW Monsoon)\n");
    printf("  Wind: SSW 15-20 km/h\n");
    printf("  Advisory: Ideal sowing window June 15 - July 15\n");
}

// ── PM-Kisan beneficiary check ────────────────────────────────────────────────

void sigma_agri_pmkisan_status(const char *aadhar_suffix) {
    printf("PM-Kisan Status (Aadhaar last 4: %s)\n", aadhar_suffix);
    printf("Annual benefit: ₹6,000 (3 instalments of ₹2,000)\n");
    printf("Check status: sigma-digilocker pull --doc pmkisan\n");
    printf("Or visit: pmkisan.gov.in → Beneficiary Status\n");
    printf("Helpline: 155261 / 011-24300606\n");
}

// ── Soil health card ──────────────────────────────────────────────────────────

void sigma_agri_soil(const char *plot_id, const char *nutrient) {
    printf("Soil Health Card — Plot: %s | Nutrient: %s\n", plot_id, nutrient);
    printf("12 parameters tested: N, P, K, pH, EC, OC, S, Zn, Fe, Cu, Mn, B\n");
    printf("Get SHC: soilhealth.dac.gov.in\n");
    printf("Recommended fertiliser (NPK ratio 4:2:1 for wheat in Punjab):\n");
    printf("  Urea: 130 kg/ha | DAP: 50 kg/ha | MOP: 25 kg/ha\n");
}
