#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Indian Professional Tools (S-IN-TOOLS)
 * Implementation: Tax, Finance, and Legal calculators based on Indian Standards.
 * Compliance: GST Act, Income Tax Act (FY 2024-25), BNS 2023.
 */

namespace SigmaOS {
namespace Kernel {
namespace Professional {

class SovereignIndianTools : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignIndianTools> {
    friend class SigmaOS::SigmaSingleton<SovereignIndianTools>;
public:
    const char* type_name() const noexcept override { return "SovereignIndianTools"; }

    // --- GST Calculator (Indian Standards) ---
    struct GSTResult {
        sigma_u64 cgst;
        sigma_u64 sgst;
        sigma_u64 igst;
        sigma_u64 total_tax;
    };

    GSTResult calculateGST(sigma_u64 base_amount, sigma_u32 rate_pct, bool is_interstate) {
        GSTResult res = {0, 0, 0, 0};
        sigma_u64 total_gst = (base_amount * rate_pct) / 100;
        
        if (is_interstate) {
            res.igst = total_gst;
        } else {
            res.cgst = total_gst / 2;
            res.sgst = total_gst / 2;
        }
        res.total_tax = total_gst;
        
        sigma_log_info("[S-IN-TOOLS] GST Calc: Base %llu, Rate %u%%, Inter-state: %d", base_amount, rate_pct, is_interstate);
        sigma_log_info("[S-IN-TOOLS] Result: CGST %llu, SGST %llu, IGST %llu", res.cgst, res.sgst, res.igst);
        
        return res;
    }

    // --- Income Tax Calculator (FY 2024-25 New Regime) ---
    sigma_u64 calculateIncomeTaxNewRegime(sigma_u64 annual_income) {
        sigma_u64 tax = 0;
        sigma_u64 taxable = annual_income;

        // Standard Deduction (FY 24-25)
        if (taxable > 75000) taxable -= 75000;
        else taxable = 0;

        // Rebate under 87A: No tax if taxable income <= 7,00,000 (New Regime)
        if (taxable <= 700000) return 0;

        // Slabs (FY 2024-25 New Regime)
        // 0-3L: NIL
        // 3-6L: 5%
        // 6-9L: 10%
        // 9-12L: 15%
        // 12-15L: 20%
        // Above 15L: 30%

        if (taxable > 300000) {
            sigma_u64 slab = (taxable > 600000 ? 300000 : taxable - 300000);
            tax += (slab * 5) / 100;
        }
        if (taxable > 600000) {
            sigma_u64 slab = (taxable > 900000 ? 300000 : taxable - 600000);
            tax += (slab * 10) / 100;
        }
        if (taxable > 900000) {
            sigma_u64 slab = (taxable > 1200000 ? 300000 : taxable - 900000);
            tax += (slab * 15) / 100;
        }
        if (taxable > 1200000) {
            sigma_u64 slab = (taxable > 1500000 ? 300000 : taxable - 1200000);
            tax += (slab * 20) / 100;
        }
        if (taxable > 1500000) {
            tax += ((taxable - 1500000) * 30) / 100;
        }

        // Add 4% Health & Education Cess
        tax = (tax * 104) / 100;

        sigma_log_info("[S-IN-TOOLS] Income Tax (New Regime): Income %llu -> Tax %llu", annual_income, tax);
        return tax;
    }

    // --- BNS (Bharatiya Nyaya Sanhita) Legal Mapping ---
    const char* lookupBNSSection(sigma_u32 ipc_section) {
        sigma_log_info("[S-IN-TOOLS] Legal Audit: Mapping IPC %u to BNS 2023...", ipc_section);
        
        switch (ipc_section) {
            case 302: return "BNS Section 101 (Murder)";
            case 378: return "BNS Section 301 (Theft)";
            case 420: return "BNS Section 316 (Cheating)";
            case 124: return "BNS Section 150 (Acts endangering sovereignty/integrity)";
            default:  return "BNS Mapping Pending (Industrial Shard Update Required)";
        }
    }

private:
    SovereignIndianTools() = default;
};

} // namespace Professional
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void in_tools_calc_gst(sigma_u64 amount, sigma_u32 rate, int interstate) {
        SigmaOS::Kernel::Professional::SovereignIndianTools::getInstance().calculateGST(amount, rate, interstate != 0);
    }
    void in_tools_calc_tax(sigma_u64 income) {
        SigmaOS::Kernel::Professional::SovereignIndianTools::getInstance().calculateIncomeTaxNewRegime(income);
    }
    const char* in_tools_lookup_bns(sigma_u32 ipc) {
        return SigmaOS::Kernel::Professional::SovereignIndianTools::getInstance().lookupBNSSection(ipc);
    }
}
 