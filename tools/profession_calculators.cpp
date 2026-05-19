/*
 * SigmaOS: Profession-Based Calculators
 * Zero-dependency implementations for GST, Income Tax, Court Fees, and BIS Standards.
 */
#include "../include/core/sigma_kernel_types.h"
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

        // Indian Payment of Gratuity Act 1972 Calculator
        sigma_u64 calculate_gratuity(sigma_u64 monthly_basic, sigma_u32 years_of_service) {
            if (years_of_service < 5) return 0; // Gratuity requires 5 years minimum
            return (monthly_basic * years_of_service * 15) / 26;
        }

        // Indian Employees' Provident Funds (EPF) Calculator
        void calculate_epf(sigma_u64 monthly_basic, sigma_u64* employee_share, sigma_u64* employer_share) {
            // Salary ceiling is standard ₹15,000 for mandatory contributions
            sigma_u64 effective_salary = (monthly_basic > 15000) ? 15000 : monthly_basic;
            *employee_share = (effective_salary * 12) / 100;
            *employer_share = (effective_salary * 12) / 100;
        }

        // Indian Companies Act 2013 - Board Meeting Quorum Calculator
        sigma_u32 calculate_board_quorum(sigma_u32 total_directors) {
            sigma_u32 third = total_directors / 3;
            if (total_directors % 3 != 0) third += 1;
            return (third > 2) ? third : 2;
        }

        // Indian Companies Act 2013 - Corporate Social Responsibility (CSR) Minimum Spend Calculator
        sigma_u64 calculate_csr_minimum_spend(sigma_u64 avg_net_profit_3_years) {
            return (avg_net_profit_3_years * 2) / 100; // 2% of average net profit
        }

        // Indian RERA (Real Estate Regulation & Development Act) Delay Interest Calculator
        sigma_u64 calculate_rera_delay_interest(sigma_u64 principal_amount, sigma_u32 months_delayed, sigma_u32 sbi_mclr_basis_points) {
            // Formula: MCLR + 2% per annum simple interest
            sigma_u32 rate_bp = sbi_mclr_basis_points + 200; // + 2.00%
            sigma_u64 annual_rate_scaled = rate_bp; // rate in basis points (e.g. 1025 for 10.25%)
            sigma_u64 interest = (principal_amount * annual_rate_scaled * months_delayed) / (12 * 10000);
            return interest;
        }
    };
}

extern "C" {

typedef struct {
    sigma_u64 base_amount;
    sigma_u64 cgst;
    sigma_u64 sgst;
    sigma_u64 igst;
    sigma_u64 total_amount;
} c_gst_result_t;

c_gst_result_t c_calculate_gst(sigma_u64 base_amount, sigma_u32 rate_percent, int is_interstate) {
    SigmaOS::ProfessionTools tools;
    auto res = tools.calculate_gst(base_amount, rate_percent, is_interstate != 0);
    c_gst_result_t out;
    out.base_amount = res.base_amount;
    out.cgst = res.cgst;
    out.sgst = res.sgst;
    out.igst = res.igst;
    out.total_amount = res.total_amount;
    return out;
}

sigma_u64 c_calculate_income_tax(sigma_u64 annual_income) {
    SigmaOS::ProfessionTools tools;
    return tools.calculate_income_tax(annual_income);
}

sigma_u64 c_calculate_court_fees(sigma_u64 claim_amount) {
    SigmaOS::ProfessionTools tools;
    return tools.calculate_court_fees(claim_amount);
}

int c_verify_bis_standards(const char* material_type, sigma_u32 yield_strength_mpa) {
    SigmaOS::ProfessionTools tools;
    return tools.verify_bis_standards(material_type, yield_strength_mpa) ? 1 : 0;
}

sigma_u64 c_calculate_gratuity(sigma_u64 monthly_basic, sigma_u32 years_of_service) {
    SigmaOS::ProfessionTools tools;
    return tools.calculate_gratuity(monthly_basic, years_of_service);
}

void c_calculate_epf(sigma_u64 monthly_basic, sigma_u64* employee_share, sigma_u64* employer_share) {
    SigmaOS::ProfessionTools tools;
    tools.calculate_epf(monthly_basic, employee_share, employer_share);
}

sigma_u32 c_calculate_board_quorum(sigma_u32 total_directors) {
    SigmaOS::ProfessionTools tools;
    return tools.calculate_board_quorum(total_directors);
}

sigma_u64 c_calculate_csr_minimum_spend(sigma_u64 avg_net_profit_3_years) {
    SigmaOS::ProfessionTools tools;
    return tools.calculate_csr_minimum_spend(avg_net_profit_3_years);
}

sigma_u64 c_calculate_rera_delay_interest(sigma_u64 principal_amount, sigma_u32 months_delayed, sigma_u32 sbi_mclr_basis_points) {
    SigmaOS::ProfessionTools tools;
    return tools.calculate_rera_delay_interest(principal_amount, months_delayed, sbi_mclr_basis_points);
}

}

