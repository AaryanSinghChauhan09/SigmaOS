/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CONSUMER COURT FEE CALCULATOR (S-CCF)
 * =========================================================================
 * Law: Consumer Protection Act, 2019 & Consumer Protection Rules, 2020
 * Principle: Bare-metal execution, zero standard library dependencies.
 * Purpose: Computes ad-valorem filing fees based on dispute valuation slabs.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Tools {
namespace Pro {

class SovereignConsumerCourtFeeCalc : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignConsumerCourtFeeCalc"; }

    static SovereignConsumerCourtFeeCalc& getInstance() {
        static SovereignConsumerCourtFeeCalc instance;
        return instance;
    }

    enum CommissionLevel {
        DISTRICT_COMMISSION = 0, // Up to Rs 50 Lakhs
        STATE_COMMISSION = 1,    // Rs 50 Lakhs to Rs 2 Crores
        NATIONAL_COMMISSION = 2  // Above Rs 2 Crores
    };

    struct CourtFeeResult {
        sigma_u64 official_filing_fee_paise;
        CommissionLevel jurisdiction;
        const char* jurisdiction_name;
    };

    /**
     * Calculates Consumer Court filing fee.
     * statutory Slabs (Consumer Protection Rules 2020):
     * 1. Up to Rs 5 Lakhs: Nil
     * 2. Rs 5 Lakhs to 10 Lakhs: Rs 200 (20,000 paise)
     * 3. Rs 10 Lakhs to 20 Lakhs: Rs 400 (40,000 paise)
     * 4. Rs 20 Lakhs to 50 Lakhs: Rs 1,000 (100,000 paise)
     * 5. State Commission (Rs 50 L to 1 Cr): Rs 2,000 (200,000 paise)
     * 6. State Commission (Rs 1 Cr to 2 Cr): Rs 2,500 (250,000 paise)
     * 7. National Commission (Above Rs 2 Cr): Rs 7,500 (750,000 paise)
     */
    CourtFeeResult calculate(sigma_u64 dispute_value_paise) {
        CourtFeeResult result{};
        
        // Define statutory threshold amounts in paise
        const sigma_u64 FIVE_LAKHS = 50000000;
        const sigma_u64 TEN_LAKHS = 100000000;
        const sigma_u64 TWENTY_LAKHS = 200000000;
        const sigma_u64 FIFTY_LAKHS = 500000000;
        const sigma_u64 ONE_CRORE = 1000000000;
        const sigma_u64 TWO_CRORES = 2000000000;
        
        if (dispute_value_paise <= FIFTY_LAKHS) {
            result.jurisdiction = DISTRICT_COMMISSION;
            result.jurisdiction_name = "District Consumer Disputes Redressal Commission";
            
            if (dispute_value_paise <= FIVE_LAKHS) {
                result.official_filing_fee_paise = 0;
            } else if (dispute_value_paise <= TEN_LAKHS) {
                result.official_filing_fee_paise = 20000; // Rs 200
            } else if (dispute_value_paise <= TWENTY_LAKHS) {
                result.official_filing_fee_paise = 40000; // Rs 400
            } else {
                result.official_filing_fee_paise = 100000; // Rs 1,000
            }
        } else if (dispute_value_paise <= TWO_CRORES) {
            result.jurisdiction = STATE_COMMISSION;
            result.jurisdiction_name = "State Consumer Disputes Redressal Commission";
            
            if (dispute_value_paise <= ONE_CRORE) {
                result.official_filing_fee_paise = 200000; // Rs 2,000
            } else {
                result.official_filing_fee_paise = 250000; // Rs 2,500
            }
        } else {
            result.jurisdiction = NATIONAL_COMMISSION;
            result.jurisdiction_name = "National Consumer Disputes Redressal Commission";
            result.official_filing_fee_paise = 750000; // Rs 7,500
        }
        
        sigma_log_info("[S-CCF] Dispute Valuation: Rs %d.", (int)(dispute_value_paise / 100));
        sigma_log_info("[S-CCF] Filing Jurisdiction: %s", result.jurisdiction_name);
        sigma_log_info("[S-CCF] Statutory Filing Fee: Rs %d.%02d",
            (int)(result.official_filing_fee_paise / 100), (int)(result.official_filing_fee_paise % 100));
            
        return result;
    }

private:
    SovereignConsumerCourtFeeCalc() = default;
};

} // namespace Pro
} // namespace Tools
} // namespace SigmaOS

extern "C" {
    void run_consumer_fee_calc(sigma_u64 claim_value) {
        SigmaOS::Tools::Pro::SovereignConsumerCourtFeeCalc::getInstance().calculate(claim_value);
    }
}
