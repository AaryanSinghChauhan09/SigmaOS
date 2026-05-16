#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Indian HR & Labour Shard (S-HR)
 * Purpose: Payroll and compliance tools for Indian HR managers and payroll officers.
 * Standards: PF Act 1952 (EPFO), ESI Act 1948, Payment of Gratuity Act 1972,
 *            Payment of Bonus Act 1965, Minimum Wages Act 1948, Code on Wages 2019.
 * Features: PF/ESI calculator, Gratuity nexus, Bonus calculator, Net-pay breakdown.
 */

namespace SigmaOS {
namespace Kernel {
namespace HR {

// EPFO contribution rates (per-mille)
static const sigma_u32 PF_EE_RATE    = 120; // Employee: 12%
static const sigma_u32 PF_ER_EPF     = 36;  // Employer EPF: 3.67%
static const sigma_u32 PF_ER_EPS     = 83;  // Employer EPS: 8.33%
static const sigma_u32 PF_ER_EDLI    = 5;   // EDLI Admin: 0.50%
static const sigma_u32 PF_WAGE_CEIL  = 1500000; // ₹15,000 * 100 paise ceiling for EPS

// ESI contribution rates (per-mille) — effective 2019
static const sigma_u32 ESI_EE_RATE   = 9;   // Employee: 0.75% (post 2019 reduction)
static const sigma_u32 ESI_ER_RATE   = 32;  // Employer: 3.25%
static const sigma_u64 ESI_GROSS_MAX = 2100000ULL; // ₹21,000 gross ceiling for ESI coverage

class SovereignHR : public SigmaOS::SigmaObject {
public:
    static SovereignHR& getInstance() {
        static SovereignHR instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignHR"; }

    void init() {
        sigma_log_info("[S-HR] Initializing Indian Payroll & Labour Compliance Nexus...");
        sigma_log_info("[S-HR] Laws: PF Act 1952 | ESI Act 1948 | Gratuity Act 1972 | Bonus Act 1965");
    }

    /**
     * Compute EPF contributions (Employee + Employer split).
     * @param basic_paise Basic + DA salary in paise (only basic+DA is PF-liable)
     */
    void calcPF(sigma_u64 basic_paise) {
        sigma_u64 pf_ee  = (basic_paise * PF_EE_RATE) / 1000ULL;
        // Employer EPS capped at ₹15,000 basic
        sigma_u64 eps_base = (basic_paise > PF_WAGE_CEIL) ? PF_WAGE_CEIL : basic_paise;
        sigma_u64 pf_eps  = (eps_base * PF_ER_EPS) / 1000ULL;
        sigma_u64 pf_epf  = (basic_paise * PF_EE_RATE) / 1000ULL - pf_eps; // balance to EPF
        if (pf_epf > (basic_paise * PF_ER_EPF) / 1000ULL)
            pf_epf = (basic_paise * PF_ER_EPF) / 1000ULL;
        sigma_u64 edli     = (eps_base * PF_ER_EDLI) / 1000ULL;
        sigma_u64 total_er = pf_epf + pf_eps + edli;

        sigma_log_info("[S-HR] PF | Basic+DA: ₹%llu | EE Contrib: ₹%llu | ER (EPF+EPS+EDLI): ₹%llu",
                       basic_paise/100, pf_ee/100, total_er/100);
        sigma_log_info("[S-HR]     → EPS: ₹%llu | EPF(ER): ₹%llu | EDLI: ₹%llu",
                       pf_eps/100, pf_epf/100, edli/100);
    }

    /**
     * ESI eligibility and contribution (ESI Act 1948, post-2019 rates).
     * @param gross_paise Gross monthly salary in paise
     */
    void calcESI(sigma_u64 gross_paise) {
        if (gross_paise > ESI_GROSS_MAX) {
            sigma_log_info("[S-HR] ESI | Gross ₹%llu > ₹21,000 ceiling — NOT COVERED under ESI Act.",
                           gross_paise/100);
            return;
        }
        sigma_u64 esi_ee = (gross_paise * ESI_EE_RATE) / 1000ULL;
        sigma_u64 esi_er = (gross_paise * ESI_ER_RATE) / 1000ULL;
        sigma_log_info("[S-HR] ESI | Gross: ₹%llu | EE (0.75%%): ₹%llu | ER (3.25%%): ₹%llu | Total: ₹%llu",
                       gross_paise/100, esi_ee/100, esi_er/100, (esi_ee+esi_er)/100);
    }

    /**
     * Gratuity under Payment of Gratuity Act 1972.
     * Formula: Gratuity = (Last drawn salary * 15 * Years) / 26
     * Max tax-free gratuity: ₹20 lakh (as per 2019 amendment).
     * @param last_salary_paise  Last drawn Basic + DA in paise (monthly)
     * @param years_service      Completed years of continuous service (min 5)
     */
    void calcGratuity(sigma_u64 last_salary_paise, sigma_u32 years_service) {
        if (years_service < 5) {
            sigma_log_info("[S-HR] Gratuity: NOT ELIGIBLE (minimum 5 years service required).");
            return;
        }
        sigma_u64 gratuity = (last_salary_paise * 15ULL * years_service) / 26ULL;
        sigma_u64 tax_free_limit = 2000000ULL * 100; // ₹20 lakh
        bool taxable = gratuity > tax_free_limit;
        sigma_log_info("[S-HR] Gratuity (Sec 4) | %u yrs | Last Salary: ₹%llu | Gratuity: ₹%llu | %s",
                       years_service, last_salary_paise/100, gratuity/100,
                       taxable ? "⚠️ Exceeds ₹20L tax-free limit" : "Tax-free u/s 10(10)");
    }

    /**
     * Statutory bonus under Payment of Bonus Act 1965.
     * Min bonus: 8.33% of annual salary (or ₹7,000/month, whichever higher).
     * Max bonus: 20% of annual salary.
     * Applicable if annual salary ≤ ₹21,000/month.
     * @param monthly_salary_paise  Monthly basic + DA in paise
     * @param bonus_pct_x10         Bonus % * 10 (e.g. 200 = 20%)
     */
    void calcBonus(sigma_u64 monthly_salary_paise, sigma_u32 bonus_pct_x10) {
        sigma_u64 annual_ceil = 2100000ULL * 100; // ₹21,000 / mo = eligibility ceiling
        if (monthly_salary_paise * 12 > annual_ceil * 12) {
            sigma_log_info("[S-HR] Bonus Act 1965: NOT ELIGIBLE (salary > ₹21,000/month).");
            return;
        }
        // Computation ceiling: ₹7,000/month for bonus calculation
        sigma_u64 comp_ceil = 700000ULL; // ₹7,000 in paise
        sigma_u64 base = (monthly_salary_paise < comp_ceil) ? monthly_salary_paise : comp_ceil;
        sigma_u64 annual_base = base * 12;

        // Min 8.33%, Max 20%
        if (bonus_pct_x10 < 83)  bonus_pct_x10 = 83;
        if (bonus_pct_x10 > 200) bonus_pct_x10 = 200;

        sigma_u64 bonus = (annual_base * bonus_pct_x10) / 1000ULL;
        sigma_log_info("[S-HR] Bonus Act 1965 | Annual base (capped ₹7K/mo): ₹%llu | Rate: %u.%u%% | Bonus: ₹%llu",
                       annual_base/100, bonus_pct_x10/10, bonus_pct_x10%10, bonus/100);
    }

    /**
     * Net pay breakdown (CTC → in-hand).
     * @param gross_paise    Monthly gross in paise
     * @param basic_paise    Monthly basic (for PF calc)
     * @param pt_state       Professional Tax (state-specific, pass as paise/month)
     */
    void netPay(sigma_u64 gross_paise, sigma_u64 basic_paise, sigma_u64 pt_paise) {
        sigma_u64 pf_ee  = (basic_paise * PF_EE_RATE) / 1000ULL;
        sigma_u64 esi_ee = (gross_paise <= ESI_GROSS_MAX)
                         ? (gross_paise * ESI_EE_RATE) / 1000ULL : 0;
        sigma_u64 deductions = pf_ee + esi_ee + pt_paise;
        sigma_u64 net        = (gross_paise > deductions) ? gross_paise - deductions : 0;
        sigma_log_info("[S-HR] Net Pay | Gross: ₹%llu | PF(EE): ₹%llu | ESI(EE): ₹%llu | PT: ₹%llu | In-Hand: ₹%llu",
                       gross_paise/100, pf_ee/100, esi_ee/100, pt_paise/100, net/100);
    }
};

} // namespace HR
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void hr_init() {
    SigmaOS::Kernel::HR::SovereignHR::getInstance().init();
}

void hr_pf(sigma_u64 basic) {
    SigmaOS::Kernel::HR::SovereignHR::getInstance().calcPF(basic);
}

void hr_esi(sigma_u64 gross) {
    SigmaOS::Kernel::HR::SovereignHR::getInstance().calcESI(gross);
}

void hr_gratuity(sigma_u64 last_sal, sigma_u32 years) {
    SigmaOS::Kernel::HR::SovereignHR::getInstance().calcGratuity(last_sal, years);
}

void hr_bonus(sigma_u64 monthly_sal, sigma_u32 pct_x10) {
    SigmaOS::Kernel::HR::SovereignHR::getInstance().calcBonus(monthly_sal, pct_x10);
}

void hr_net_pay(sigma_u64 gross, sigma_u64 basic, sigma_u64 pt) {
    SigmaOS::Kernel::HR::SovereignHR::getInstance().netPay(gross, basic, pt);
}

} // extern "C"
