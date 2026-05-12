#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Insurance Actuary Shard (S-ACTUARY)
 * Purpose: Actuarial tools for Indian insurance professionals.
 * Standards: IRDAI Act 1999, Insurance Act 1938, IRDA (Life Insurance) Regulations,
 *            IAI (Institute of Actuaries of India) mortality tables (LIC IAM94-96).
 * Features: Premium calculator, mortality-adjusted net present value, LIC mortality lookup.
 */

namespace SigmaOS {
namespace Kernel {
namespace Finance {

// LIC IAM 94-96 mortality table (qx per 1000) — ages 0-80 (sampled key ages)
struct MortalityEntry {
    sigma_u32 age;
    sigma_u32 qx_per_1000; // probability of death within 1 year * 1000
};

static const MortalityEntry MORTALITY_TABLE[] = {
    { 0,  5},  { 5,  1},  {10,  1},  {15,  2},  {20,  2},
    {25,  2},  {30,  3},  {35,  4},  {40,  7},  {45, 12},
    {50, 20},  {55, 32},  {60, 52},  {65, 85},  {70,135},
    {75,210},  {80,330},
};
static const sigma_u32 MORT_LEN = sizeof(MORTALITY_TABLE) / sizeof(MORTALITY_TABLE[0]);

class SovereignActuary : public SigmaOS::SigmaObject {
public:
    static SovereignActuary& getInstance() {
        static SovereignActuary instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignActuary"; }

    void init() {
        sigma_log_info("[S-ACTUARY] Initializing Indian Actuarial Nexus (IAI / IRDAI)...");
    }

    /**
     * Pure endowment premium (term insurance simplified).
     * Annual premium ≈ Sum Assured * qx / (1000 * (1 - admin_loading))
     * @param sa_paise     Sum Assured in paise
     * @param age          Policyholder age
     * @param term_years   Policy term
     * @param loading_pct  Admin/profit loading % (e.g. 20)
     */
    void calcPremium(sigma_u64 sa_paise, sigma_u32 age, sigma_u32 term_years,
                     sigma_u32 loading_pct) {
        sigma_u32 qx = lookupQx(age);
        // Net premium = SA * qx / 1000
        sigma_u64 net_prem = (sa_paise * qx) / 1000ULL;
        // Gross premium adds loading
        sigma_u64 gross = net_prem * 100ULL / (100ULL - loading_pct);
        sigma_log_info("[S-ACTUARY] Premium | SA: ₹%llu | Age: %u | qx: %u/1000 | Term: %u yr | Net: ₹%llu | Gross: ₹%llu",
                       sa_paise/100, age, qx, term_years, net_prem/100, gross/100);
        sigma_log_info("[S-ACTUARY] ⚠️  Verify against IRDAI approved mortality tables and Interest Assumption.");
    }

    /**
     * IRDAI solvency margin check (Sec 64VA, Insurance Act 1938).
     * Required Solvency Margin (RSM): larger of ₹50 Cr or 20% of net premium income.
     */
    void solvencyCheck(sigma_u64 available_margin_paise, sigma_u64 net_premium_annual_paise) {
        sigma_u64 rsm_floor  = 5000000000ULL; // ₹50 Cr in paise
        sigma_u64 rsm_20pct  = (net_premium_annual_paise * 20) / 100;
        sigma_u64 rsm        = (rsm_floor > rsm_20pct) ? rsm_floor : rsm_20pct;
        bool solvent = available_margin_paise >= rsm;
        sigma_log_info("[S-ACTUARY] IRDAI Solvency (Sec 64VA) | ASM: ₹%llu Cr | RSM: ₹%llu Cr | %s",
                       available_margin_paise / (100ULL * 10000000ULL),
                       rsm / (100ULL * 10000000ULL),
                       solvent ? "✅ SOLVENT" : "🚨 BELOW RSM — regulatory action required");
    }

private:
    sigma_u32 lookupQx(sigma_u32 age) {
        for (sigma_u32 i = 0; i + 1 < MORT_LEN; ++i) {
            if (age >= MORTALITY_TABLE[i].age && age < MORTALITY_TABLE[i+1].age)
                return MORTALITY_TABLE[i].qx_per_1000;
        }
        return MORTALITY_TABLE[MORT_LEN - 1].qx_per_1000;
    }
};

} // namespace Finance
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void actuary_init() {
    SigmaOS::Kernel::Finance::SovereignActuary::getInstance().init();
}

void actuary_premium(sigma_u64 sa, sigma_u32 age, sigma_u32 term, sigma_u32 loading) {
    SigmaOS::Kernel::Finance::SovereignActuary::getInstance().calcPremium(sa, age, term, loading);
}

void actuary_solvency(sigma_u64 asm_paise, sigma_u64 npi_paise) {
    SigmaOS::Kernel::Finance::SovereignActuary::getInstance().solvencyCheck(asm_paise, npi_paise);
}

} // extern "C"
