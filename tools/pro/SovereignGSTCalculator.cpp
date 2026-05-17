/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PRO TOOL - SovereignGSTCalculator
 * =========================================================================
 * REGULATORY CONTEXT: GST Act / Income Tax Act Compliance (Indian Standards)
 * Principle: Bare-metal execution, zero standard library dependencies.
 * =========================================================================
 */
#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace ProTools {

class SovereignGSTCalculator {
public:
    void init() {
        sigma_log_info("[SovereignGST] Sovereign GST Calculator (Indian Standards) initialized.");
        sigma_log_info("[SovereignGST] Enforcing CGST Act 2017 & Integrated GST Act 2017.");
    }
    
    // Core engine computing CGST, SGST, and IGST without high-level floating points
    // Amounts represented in paise (1 INR = 100 paise) to maintain fixed-point precision
    sigma_u32 compute_gst(sigma_u64 base_amount_paise, sigma_u32 rate_percent, sigma_bool is_interstate,
                          sigma_u64* out_cgst_paise, sigma_u64* out_sgst_paise, sigma_u64* out_igst_paise,
                          sigma_u64* out_total_paise) {
        
        // Enforce valid Indian GST slab rates (0%, 3%, 5%, 12%, 18%, 28%)
        if (rate_percent != 0 && rate_percent != 3 && rate_percent != 5 && 
            rate_percent != 12 && rate_percent != 18 && rate_percent != 28) {
            sigma_log_error("[SovereignGST] Non-standard GST rate percent specified: %u%%", rate_percent);
            return SIGMA_ERROR;
        }

        sigma_u64 total_gst = (base_amount_paise * rate_percent) / 100u;

        if (is_interstate) {
            *out_igst_paise = total_gst;
            *out_cgst_paise = 0;
            *out_sgst_paise = 0;
        } else {
            *out_igst_paise = 0;
            *out_cgst_paise = total_gst / 2;
            *out_sgst_paise = total_gst - (*out_cgst_paise); // Prevent rounding losses
        }

        *out_total_paise = base_amount_paise + total_gst;
        
        sigma_log_info("[SovereignGST] Base: %llu INR | Rate: %u%% | IGST: %llu | CGST: %llu | SGST: %llu | Total: %llu",
                       base_amount_paise / 100, rate_percent, *out_igst_paise / 100, 
                       *out_cgst_paise / 100, *out_sgst_paise / 100, *out_total_paise / 100);

        return SIGMA_OK;
    }
};

} // namespace ProTools
} // namespace SigmaOS

extern "C" {
    void gst_init() {
        SigmaOS::ProTools::SovereignGSTCalculator calc;
        calc.init();
    }
    
    sigma_u32 gst_calculate(sigma_u64 base_inr, sigma_u32 rate, sigma_u8 is_interstate,
                            sigma_u64* cgst, sigma_u64* sgst, sigma_u64* igst, sigma_u64* total) {
        SigmaOS::ProTools::SovereignGSTCalculator calc;
        return calc.compute_gst(base_inr * 100, rate, is_interstate != 0, cgst, sgst, igst, total);
    }
}
