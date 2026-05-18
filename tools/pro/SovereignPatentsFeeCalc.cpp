/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PATENT FILING & FEE CALCULATOR (S-PATENT)
 * =========================================================================
 * Law: Indian Patents Act, 1970 & Patents (Amendment) Rules, 2024
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes patent filing, examination, and renewal fees dynamically.
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignPatentsFeeCalc : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignPatentsFeeCalc"; }

    static SovereignPatentsFeeCalc& getInstance() {
        static SovereignPatentsFeeCalc instance;
        return instance;
    }

    enum EntityType {
        NATURAL_PERSON = 0,
        STARTUP = 1,
        SMALL_ENTITY = 2,
        LARGE_ENTITY = 3
    };

    struct PatentFeeResult {
        sigma_u64 official_filing_fee_paise;
        sigma_u64 official_examination_fee_paise;
        sigma_u64 early_publication_fee_paise;
        sigma_u64 total_statutory_fees_paise;
        bool is_e_filing;
    };

    /**
     * Calculates Patent statutory fees.
     * C1: Natural Persons/Startups/MSMEs receive an 80% discount over Large Entities.
     * C2: Physical filing incurs a 10% surcharge compared to e-Filing.
     */
    PatentFeeResult calculate(EntityType entity, bool request_examination, bool request_early_publication, bool physical_filing) {
        PatentFeeResult result{};
        result.is_e_filing = !physical_filing;
        
        // Base Fees (for Large Entity in e-filing mode)
        sigma_u64 base_filing = 800000;       // Rs 8,000
        sigma_u64 base_exam = 2000000;       // Rs 20,000
        sigma_u64 base_early_pub = 1250000;  // Rs 12,500
        
        // Entity Discount (Natural Person, Startup, Small Entity get 80% discount, i.e., pay 20%)
        sigma_u64 scale_numerator = 100;
        if (entity == NATURAL_PERSON || entity == STARTUP || entity == SMALL_ENTITY) {
            scale_numerator = 20; // 80% concession
        }
        
        result.official_filing_fee_paise = (base_filing * scale_numerator) / 100;
        
        if (request_examination) {
            result.official_examination_fee_paise = (base_exam * scale_numerator) / 100;
        } else {
            result.official_examination_fee_paise = 0;
        }
        
        if (request_early_publication) {
            result.early_publication_fee_paise = (base_early_pub * scale_numerator) / 100;
        } else {
            result.early_publication_fee_paise = 0;
        }
        
        // Physical Filing Surcharge (+10% on top of total)
        if (physical_filing) {
            result.official_filing_fee_paise = (result.official_filing_fee_paise * 110) / 100;
            result.official_examination_fee_paise = (result.official_examination_fee_paise * 110) / 100;
            result.early_publication_fee_paise = (result.early_publication_fee_paise * 110) / 100;
        }
        
        result.total_statutory_fees_paise = result.official_filing_fee_paise + 
                                             result.official_examination_fee_paise + 
                                             result.early_publication_fee_paise;
                                             
        sigma_log_info("[S-PATENT] Statutory Patent Fee audited successfully.");
        sigma_log_info("[S-PATENT] Total Filing Fee: Rs %d.%02d (Total Fees: Rs %d.%02d)",
            (int)(result.official_filing_fee_paise / 100), (int)(result.official_filing_fee_paise % 100),
            (int)(result.total_statutory_fees_paise / 100), (int)(result.total_statutory_fees_paise % 100));
            
        return result;
    }

private:
    SovereignPatentsFeeCalc() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_patent_calc(sigma_u32 entity_code, sigma_u8 physical) {
        SigmaOS::Tools::Pro::SovereignPatentsFeeCalc::getInstance().calculate(
            (SigmaOS::Tools::Pro::SovereignPatentsFeeCalc::EntityType)entity_code, true, true, physical != 0);
    }
}
