#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Income Tax Shard (S-TAX)
 * Purpose: Indian income tax computation for individuals, HUF, and firms.
 * Standards: Income Tax Act 1961, AY 2025-26 slabs (New & Old Regime),
 *            Surcharge (Sec 87A), Health & Education Cess (4%).
 */

namespace SigmaOS {
namespace Kernel {
namespace Finance {

struct TaxSlab {
    sigma_u64 from_paise;
    sigma_u64 to_paise;   // 0 = no upper limit
    sigma_u32 rate_permille;
};

// New Regime slabs AY 2025-26 (Budget 2024 — effective FY 2024-25)
static const TaxSlab NEW_REGIME[] = {
    {0,                     300000ULL * 100,   0},   // 0-3L : 0%
    {300000ULL * 100,       700000ULL * 100,   50},  // 3-7L : 5%
    {700000ULL * 100,      1000000ULL * 100,  100},  // 7-10L: 10%
    {1000000ULL * 100,     1200000ULL * 100,  150},  // 10-12L: 15%
    {1200000ULL * 100,     1500000ULL * 100,  200},  // 12-15L: 20%
    {1500000ULL * 100,     0,                 300},  // >15L : 30%
};

// Old Regime slabs AY 2025-26
static const TaxSlab OLD_REGIME[] = {
    {0,                     250000ULL * 100,   0},   // 0-2.5L: 0%
    {250000ULL * 100,       500000ULL * 100,   50},  // 2.5-5L: 5%
    {500000ULL * 100,      1000000ULL * 100,  200},  // 5-10L : 20%
    {1000000ULL * 100,     0,                 300},  // >10L  : 30%
};

static const sigma_u32 NEW_REGIME_LEN = sizeof(NEW_REGIME) / sizeof(NEW_REGIME[0]);
static const sigma_u32 OLD_REGIME_LEN = sizeof(OLD_REGIME) / sizeof(OLD_REGIME[0]);

class SovereignTax : public SigmaOS::SigmaObject {
public:
    static SovereignTax& getInstance() {
        static SovereignTax instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignTax"; }

    void init() {
        sigma_log_info("[S-TAX] Initializing Indian Income Tax Nexus (AY 2025-26)...");
    }

    /**
     * Compute tax liability under New or Old regime.
     * @param income_paise  Gross total income in paise
     * @param use_new       true = New Regime, false = Old Regime
     * @param deductions_p  Section 80C/80D etc. (only applicable for old regime)
     */
    sigma_u64 computeTax(sigma_u64 income_paise, bool use_new, sigma_u64 deductions_p) {
        sigma_u64 taxable = income_paise;
        if (!use_new) {
            taxable = (income_paise > deductions_p) ? income_paise - deductions_p : 0;
        }

        const TaxSlab* slabs = use_new ? NEW_REGIME : OLD_REGIME;
        sigma_u32 len        = use_new ? NEW_REGIME_LEN : OLD_REGIME_LEN;

        sigma_u64 tax = 0;
        for (sigma_u32 i = 0; i < len; ++i) {
            if (taxable <= slabs[i].from_paise) break;
            sigma_u64 upper = (slabs[i].to_paise == 0) ? taxable : slabs[i].to_paise;
            if (taxable < upper) upper = taxable;
            sigma_u64 band  = upper - slabs[i].from_paise;
            tax += (band * slabs[i].rate_permille) / 1000ULL;
        }

        // Sec 87A rebate: full rebate if tax <= ₹25,000 and income <= ₹7L (New) / ₹5L (Old)
        sigma_u64 rebate_limit = use_new ? 700000ULL * 100 : 500000ULL * 100;
        sigma_u64 max_rebate   = 2500000ULL; // ₹25,000 in paise
        if (taxable <= rebate_limit && tax <= max_rebate) {
            sigma_log_info("[S-TAX] ✅ Section 87A rebate applied — Tax = ₹0.");
            tax = 0;
        }

        // Surcharge
        sigma_u64 income_l = taxable / (100ULL * 100000ULL); // in lakhs
        sigma_u64 surcharge = 0;
        if (income_l > 500)       surcharge = (tax * 370) / 1000; // 37%
        else if (income_l > 200)  surcharge = (tax * 250) / 1000; // 25%
        else if (income_l > 100)  surcharge = (tax * 150) / 1000; // 15%
        else if (income_l >  50)  surcharge = (tax * 100) / 1000; // 10%

        sigma_u64 tax_with_sc = tax + surcharge;

        // Health & Education Cess 4%
        sigma_u64 cess = (tax_with_sc * 40) / 1000;
        sigma_u64 total = tax_with_sc + cess;

        sigma_log_info("[S-TAX] Regime: %s | Taxable: ₹%llu | Tax: ₹%llu | Surcharge: ₹%llu | Cess(4%%): ₹%llu | TOTAL: ₹%llu",
                       use_new ? "New" : "Old",
                       taxable/100, tax/100, surcharge/100, cess/100, total/100);
        return total;
    }
};

} // namespace Finance
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void tax_init() {
    SigmaOS::Kernel::Finance::SovereignTax::getInstance().init();
}

sigma_u64 tax_compute(sigma_u64 income_paise, bool new_regime, sigma_u64 deductions_p) {
    return SigmaOS::Kernel::Finance::SovereignTax::getInstance()
               .computeTax(income_paise, new_regime, deductions_p);
}

} // extern "C"
 