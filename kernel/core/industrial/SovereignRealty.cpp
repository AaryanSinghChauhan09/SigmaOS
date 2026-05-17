#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Real Estate Shard (S-REALTY)
 * Purpose: Professional environment for Indian real estate agents, developers, and valuers.
 * Standards: RERA 2016, Stamp Duty (state-wise), Registration Act 1908, Circle Rates.
 * Features: Stamp duty / registration charge calculator, EMI nexus, RERA compliance check.
 */

namespace SigmaOS {
namespace Kernel {
namespace Finance {

// State-wise stamp duty rates (per-mille of property value).
// Urban area rates as per typical 2024-25 values.
struct StampDutyRate {
    const char* state;
    sigma_u32 male_permille;
    sigma_u32 female_permille;
    sigma_u32 joint_permille;
    sigma_u32 reg_fee_permille; // registration fee
};

static const StampDutyRate STAMP_TABLE[] = {
    {"Maharashtra",     50, 50, 50, 10},  // 5% + 1% reg
    {"Delhi",           60, 40, 50, 10},  // 6% male / 4% female / 5% joint + 1%
    {"Karnataka",       55, 55, 55, 10},
    {"Tamil_Nadu",      70, 70, 70, 10},
    {"Uttar_Pradesh",   70, 60, 65, 10},
    {"West_Bengal",     70, 70, 70, 10},
    {"Gujarat",         45, 45, 45, 10},
    {"Rajasthan",       60, 50, 55, 10},
    {"Telangana",       50, 50, 50, 5},
    {"Kerala",          60, 60, 60, 20},
};
static const sigma_u32 STAMP_TABLE_LEN = sizeof(STAMP_TABLE) / sizeof(STAMP_TABLE[0]);

class SovereignRealty : public SigmaOS::SigmaObject {
public:
    static SovereignRealty& getInstance() {
        static SovereignRealty instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignRealty";
    }

    void init() {
        sigma_log_info("[S-REALTY] Initializing Indian Real Estate Nexus...");
        sigma_log_info("[S-REALTY] Standards: RERA 2016 | Stamp Duty Act | Registration Act 1908");
    }

    /**
     * Calculate stamp duty and registration charges for a property.
     * @param state_name  Matches STAMP_TABLE
     * @param value_paise Property value in paise
     * @param gender      0 = male, 1 = female, 2 = joint
     */
    void calcStampDuty(const char* state_name, sigma_u64 value_paise, sigma_u32 gender) {
        for (sigma_u32 i = 0; i < STAMP_TABLE_LEN; ++i) {
            const StampDutyRate& r = STAMP_TABLE[i];
            // Simple string comparison
            bool match = true;
            for (sigma_u32 j = 0; r.state[j] || state_name[j]; ++j) {
                if (r.state[j] != state_name[j]) { match = false; break; }
            }
            if (!match) continue;

            sigma_u32 rate = (gender == 1) ? r.female_permille
                           : (gender == 2) ? r.joint_permille
                           : r.male_permille;

            sigma_u64 stamp  = (value_paise * rate) / 1000ULL;
            sigma_u64 regfee = (value_paise * r.reg_fee_permille) / 1000ULL;
            sigma_u64 total  = stamp + regfee;

            sigma_log_info("[S-REALTY] State: %s | Property: ₹%llu.%02llu",
                           r.state, value_paise/100, value_paise%100);
            sigma_log_info("[S-REALTY]   Stamp Duty (%u%%): ₹%llu.%02llu",
                           rate/10, stamp/100, stamp%100);
            sigma_log_info("[S-REALTY]   Reg Fee   (%u%%): ₹%llu.%02llu",
                           r.reg_fee_permille/10, regfee/100, regfee%100);
            sigma_log_info("[S-REALTY]   TOTAL OUTGO:       ₹%llu.%02llu",
                           total/100, total%100);
            return;
        }
        sigma_log_err("[S-REALTY] State '%s' not found in stamp duty table.", state_name);
    }

    /**
     * EMI calculator: EMI = P * r * (1+r)^n / ((1+r)^n - 1)
     * Uses integer arithmetic; EMI returned in paise.
     * @param principal_paise  Loan amount in paise
     * @param annual_rate_bp   Annual interest rate in basis points (e.g. 850 = 8.50%)
     * @param tenure_months    Loan tenure in months
     */
    sigma_u64 calcEMI(sigma_u64 principal_paise, sigma_u32 annual_rate_bp,
                      sigma_u32 tenure_months) {
        if (tenure_months == 0 || annual_rate_bp == 0) {
            sigma_log_err("[S-REALTY] Invalid EMI parameters.");
            return 0;
        }
        // Monthly rate in micro-units: r_micro = annual_rate_bp * 10^6 / (12 * 10000)
        sigma_u64 r_micro = (sigma_u64)annual_rate_bp * 1000000ULL / 120000ULL;

        // (1+r)^n using fixed-point iteration (micro units, base 10^6)
        sigma_u64 base_micro = 1000000ULL + r_micro;
        sigma_u64 pow_micro  = 1000000ULL;
        for (sigma_u32 i = 0; i < tenure_months; ++i) {
            pow_micro = (pow_micro * base_micro) / 1000000ULL;
        }
        // EMI = P * r * (1+r)^n / ((1+r)^n - 1)
        sigma_u64 num = (principal_paise * r_micro / 1000000ULL) * pow_micro / 1000000ULL;
        sigma_u64 den = pow_micro - 1000000ULL;
        if (den == 0) { sigma_log_err("[S-REALTY] EMI denominator zero."); return 0; }

        sigma_u64 emi = (num * 1000000ULL) / den;
        sigma_log_info("[S-REALTY] EMI | Principal: ₹%llu | Rate: %u.%02u%% p.a. | Tenure: %u mo | EMI: ₹%llu.%02llu",
                       principal_paise/100, annual_rate_bp/100, annual_rate_bp%100,
                       tenure_months, emi/100, emi%100);
        return emi;
    }

    /**
     * RERA compliance check: verifies project is within 500-unit or < 500 sq m exemption.
     * Returns 1 if RERA registration is MANDATORY.
     */
    sigma_u32 reraCheck(sigma_u32 units, sigma_u32 plot_sqm) {
        bool mandatory = (units > 8) || (plot_sqm > 500);
        sigma_log_info("[S-REALTY] RERA Check | Units: %u | Plot: %u m² | Registration: %s",
                       units, plot_sqm, mandatory ? "MANDATORY (RERA 2016 Sec 3)" : "EXEMPT");
        return mandatory ? 1 : 0;
    }
};

} // namespace Finance
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void realty_init() {
    SigmaOS::Kernel::Finance::SovereignRealty::getInstance().init();
}

void realty_stamp(const char* state, sigma_u64 val_paise, sigma_u32 gender) {
    SigmaOS::Kernel::Finance::SovereignRealty::getInstance().calcStampDuty(state, val_paise, gender);
}

sigma_u64 realty_emi(sigma_u64 principal, sigma_u32 rate_bp, sigma_u32 months) {
    return SigmaOS::Kernel::Finance::SovereignRealty::getInstance().calcEMI(principal, rate_bp, months);
}

sigma_u32 realty_rera_check(sigma_u32 units, sigma_u32 sqm) {
    return SigmaOS::Kernel::Finance::SovereignRealty::getInstance().reraCheck(units, sqm);
}

} // extern "C"
 