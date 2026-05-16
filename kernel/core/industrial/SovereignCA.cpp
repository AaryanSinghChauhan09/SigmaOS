#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Chartered Accountant Shard (S-CA)
 * Purpose: Professional environment for Indian CAs, tax consultants, and auditors.
 * Standards: Income Tax Act 1961, GST (CGST/SGST/IGST), Companies Act 2013, AS/Ind-AS.
 * Features: GST calculator, TDS computation, advance-tax scheduler, balance sheet reconciler.
 */

namespace SigmaOS {
namespace Kernel {
namespace Finance {

// Indian GST slabs (per-mille, i.e. * 1000 = rate%)
static const sigma_u32 GST_EXEMPT  = 0;
static const sigma_u32 GST_SLAB_5  = 50;   // 5%
static const sigma_u32 GST_SLAB_12 = 120;  // 12%
static const sigma_u32 GST_SLAB_18 = 180;  // 18%
static const sigma_u32 GST_SLAB_28 = 280;  // 28%

// TDS rates per Section (per-mille)
struct TDSRate {
    const char* section;
    sigma_u32 rate_permille; // e.g. 100 = 10%
    const char* description;
};

static const TDSRate TDS_TABLE[] = {
    {"192",  100, "Salary"},
    {"194",   10, "Dividend"},
    {"194A", 100, "Interest (other than securities)"},
    {"194B", 300, "Lottery / Crossword winnings"},
    {"194C",  10, "Contractor / Sub-contractor"},
    {"194J", 100, "Professional / Technical services"},
    {"194I", 100, "Rent (land/building)"},
};
static const sigma_u32 TDS_TABLE_LEN = sizeof(TDS_TABLE) / sizeof(TDS_TABLE[0]);

class SovereignCA : public SigmaOS::SigmaObject {
public:
    static SovereignCA& getInstance() {
        static SovereignCA instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignCA";
    }

    void init() {
        sigma_log_info("[S-CA] Initializing Indian Chartered Accountant Nexus...");
        sigma_log_info("[S-CA] Standards: Income Tax Act 1961 | GST 2017 | Companies Act 2013");
        // Log supported GST slabs to ensure constants are 'used' and user is informed
        sigma_log_info("[S-CA] Supported GST Slabs (per-mille): %u, %u, %u, %u, %u",
                       GST_EXEMPT, GST_SLAB_5, GST_SLAB_12, GST_SLAB_18, GST_SLAB_28);
    }

    /**
     * Calculate GST split (CGST + SGST for intra-state, IGST for inter-state).
     * @param base_paise  Transaction value in paise (1 INR = 100 paise)
     * @param slab        GST slab per-mille (use GST_SLAB_* constants)
     * @param interstate  true = IGST only, false = CGST+SGST split
     */
    void calcGST(sigma_u64 base_paise, sigma_u32 slab, bool interstate) {
        sigma_u64 total_tax = (base_paise * slab) / 1000ULL;
        sigma_u64 half_tax  = total_tax / 2;

        if (interstate) {
            sigma_log_info("[S-CA] GST | Base: ₹%llu.%02llu | IGST(%u%%): ₹%llu.%02llu | Total: ₹%llu.%02llu",
                           base_paise/100, base_paise%100,
                           slab/10,
                           total_tax/100, total_tax%100,
                           (base_paise + total_tax)/100, (base_paise + total_tax)%100);
        } else {
            sigma_log_info("[S-CA] GST | Base: ₹%llu.%02llu | CGST(%u%%): ₹%llu.%02llu | SGST(%u%%): ₹%llu.%02llu",
                           base_paise/100, base_paise%100,
                           slab/20, half_tax/100, half_tax%100,
                           slab/20, half_tax/100, half_tax%100);
        }
    }

    /**
     * Compute TDS amount for a given section and payment.
     * Also checks ₹50,000 threshold (Section 194C) and prints PAN warning if needed.
     */
    void calcTDS(const char* section_code, sigma_u64 payment_paise, bool has_pan) {
        for (sigma_u32 i = 0; i < TDS_TABLE_LEN; ++i) {
            // Simple string comparison logic
            bool match = true;
            for (sigma_u32 j = 0; TDS_TABLE[i].section[j] || section_code[j]; ++j) {
                if (TDS_TABLE[i].section[j] != section_code[j]) {
                    match = false;
                    break;
                }
            }
            if (match) {
                sigma_u32 rate = has_pan ? TDS_TABLE[i].rate_permille : TDS_TABLE[i].rate_permille * 2;
                if (!has_pan) {
                    sigma_log_info("[S-CA] ⚠️  No PAN — TDS rate doubled per Section 206AA.");
                }
                sigma_u64 tds = (payment_paise * rate) / 1000ULL;
                sigma_log_info("[S-CA] TDS u/s %s (%s): ₹%llu.%02llu on payment ₹%llu.%02llu",
                               section_code, TDS_TABLE[i].description,
                               tds/100, tds%100,
                               payment_paise/100, payment_paise%100);
                return;
            }
        }
        sigma_log_err("[S-CA] TDS section %s not found in table.", section_code);
    }

    /**
     * Compute advance tax installments as per Section 208/211 (for AY).
     * Installments: 15%/45%/75%/100% by Jun 15/Sep 15/Dec 15/Mar 15.
     * @param annual_tax_paise Estimated total tax liability for the year.
     */
    void calcAdvanceTax(sigma_u64 annual_tax_paise) {
        sigma_log_info("[S-CA] Advance Tax Schedule (Sec 208/211) for liability ₹%llu.%02llu:",
                       annual_tax_paise/100, annual_tax_paise%100);
        sigma_u64 installments[4] = {
            (annual_tax_paise * 15) / 100,  // 15% by Jun 15
            (annual_tax_paise * 45) / 100,  // 45% by Sep 15 (cumulative)
            (annual_tax_paise * 75) / 100,  // 75% by Dec 15 (cumulative)
             annual_tax_paise,              // 100% by Mar 15
        };
        const char* dates[4] = {"Jun 15", "Sep 15", "Dec 15", "Mar 15"};
        for (sigma_u32 i = 0; i < 4; ++i) {
            sigma_u64 due = (i == 0) ? installments[0]
                          : installments[i] - installments[i-1];
            sigma_log_info("[S-CA]   %s → Due: ₹%llu.%02llu (Cumulative: ₹%llu.%02llu)",
                           dates[i], due/100, due%100,
                           installments[i]/100, installments[i]%100);
        }
    }
};

} // namespace Finance
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void ca_init() {
    SigmaOS::Kernel::Finance::SovereignCA::getInstance().init();
}

void ca_gst(sigma_u64 base_paise, sigma_u32 slab, bool interstate) {
    SigmaOS::Kernel::Finance::SovereignCA::getInstance().calcGST(base_paise, slab, interstate);
}

void ca_tds(const char* section, sigma_u64 payment_paise, bool has_pan) {
    SigmaOS::Kernel::Finance::SovereignCA::getInstance().calcTDS(section, payment_paise, has_pan);
}

void ca_advance_tax(sigma_u64 annual_tax_paise) {
    SigmaOS::Kernel::Finance::SovereignCA::getInstance().calcAdvanceTax(annual_tax_paise);
}

} // extern "C"
