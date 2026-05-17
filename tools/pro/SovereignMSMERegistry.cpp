/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PRO TOOL - SovereignMSMERegistry
 * =========================================================================
 * REGULATORY CONTEXT: MSMED Act / Ministry of MSME Rules (Indian Standards)
 * Principle: Bare-metal execution, zero standard library dependencies.
 * =========================================================================
 */
#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace ProTools {

enum class MSMEType : sigma_u8 {
    NOT_MSME = 0,
    MICRO    = 1,
    SMALL    = 2,
    MEDIUM   = 3,
};

class SovereignMSMERegistry {
public:
    void init() {
        sigma_log_info("[SovereignMSME] MSME Categorization & Registry Engine (MSMED Act 2020 Compliant) initialized.");
    }

    // Classifies MSME status based on Investment and Turnover in INR (amounts in Lakhs to fit inside sigma_u32)
    // 1 Crore = 100 Lakhs
    MSMEType classify_enterprise(sigma_u32 investment_lakhs, sigma_u32 turnover_lakhs, const char** out_type_name) {
        
        // Limits under Indian MSMED Act (Revised 2020):
        // Micro:  Investment <= 1 Crore (100 Lakhs)   AND Turnover <= 5 Crore (500 Lakhs)
        // Small:  Investment <= 10 Crore (1000 Lakhs)  AND Turnover <= 50 Crore (5000 Lakhs)
        // Medium: Investment <= 50 Crore (5000 Lakhs)  AND Turnover <= 250 Crore (25000 Lakhs)

        if (investment_lakhs <= 100 && turnover_lakhs <= 500) {
            *out_type_name = "MICRO";
            return MSMEType::MICRO;
        } 
        else if (investment_lakhs <= 1000 && turnover_lakhs <= 5000) {
            *out_type_name = "SMALL";
            return MSMEType::SMALL;
        }
        else if (investment_lakhs <= 5000 && turnover_lakhs <= 25000) {
            *out_type_name = "MEDIUM";
            return MSMEType::MEDIUM;
        }
        else {
            *out_type_name = "LARGE / NOT ELIGIBLE FOR MSME";
            return MSMEType::NOT_MSME;
        }
    }
};

} // namespace ProTools
} // namespace SigmaOS

extern "C" {
    void msme_init() {
        SigmaOS::ProTools::SovereignMSMERegistry registry;
        registry.init();
    }

    sigma_u8 msme_classify(sigma_u32 investment, sigma_u32 turnover, const char** type_str) {
        SigmaOS::ProTools::SovereignMSMERegistry registry;
        return (sigma_u8)registry.classify_enterprise(investment, turnover, type_str);
    }
}
