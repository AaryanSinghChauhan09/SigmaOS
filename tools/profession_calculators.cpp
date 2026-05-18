/*
 * SigmaOS: Profession-Based Calculators
 * Zero-dependency implementations for GST, Income Tax, Court Fees, and BIS Standards.
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
    class ProfessionTools {
    public:
        struct GSTResult {
            sigma_u64 base_amount;
            sigma_u64 cgst;
            sigma_u64 sgst;
            sigma_u64 igst;
            sigma_u64 total_amount;
        };

        // Indian GST Act 2017 Calculator
        GSTResult calculate_gst(sigma_u64 base_amount, sigma_u32 rate_percent, bool is_interstate) {
            GSTResult result{};
            result.base_amount = base_amount;
            sigma_u64 total_tax = (base_amount * rate_percent) / 100;
            
            if (is_interstate) {
                result.igst = total_tax;
                result.cgst = 0;
                result.sgst = 0;
            } else {
                result.igst = 0;
                result.cgst = total_tax / 2;
                result.sgst = total_tax / 2;
            }
            result.total_amount = base_amount + total_tax;
            return result;
        }

        // Indian Income Tax Act 1961 Calculator (New Tax Regime Slabs)
        sigma_u64 calculate_income_tax(sigma_u64 annual_income) {
            sigma_u64 tax = 0;
            
            if (annual_income <= 300000) {
                tax = 0;
            } else if (annual_income <= 600000) {
                tax = ((annual_income - 300000) * 5) / 100;
            } else if (annual_income <= 900000) {
                tax = 15000 + ((annual_income - 600000) * 10) / 100;
            } else if (annual_income <= 1200000) {
                tax = 45000 + ((annual_income - 900000) * 15) / 100;
            } else if (annual_income <= 1500000) {
                tax = 90000 + ((annual_income - 1200000) * 20) / 100;
            } else {
                tax = 150000 + ((annual_income - 1500000) * 30) / 100;
            }
            
            return tax; 
        }

        // Court Fees Act Ad-Valorem Calculator
        sigma_u64 calculate_court_fees(sigma_u64 claim_amount) {
            sigma_u64 fee = 0;
            
            if (claim_amount <= 50000) {
                fee = (claim_amount * 25) / 1000; // 2.5%
            } else if (claim_amount <= 200000) {
                fee = 1250 + ((claim_amount - 50000) * 50) / 1000; // 5%
            } else {
                fee = 8750 + ((claim_amount - 200000) * 75) / 1000; // 7.5%
            }
            
            return fee;
        }

        // BIS Structural & Materials Standards Verification (e.g. IS 800:2007)
        bool verify_bis_standards(const char* material_type, sigma_u32 yield_strength_mpa) {
            if (sigma_strcmp(material_type, "Fe410") == 0) {
                // IS 2062: Structural steel yield must be >= 250 MPa
                return yield_strength_mpa >= 250;
            } else if (sigma_strcmp(material_type, "Fe500D") == 0) {
                // IS 1786: Reinforcement bars yield must be >= 500 MPa
                return yield_strength_mpa >= 500;
            }
            return false;
        }
    };
}

