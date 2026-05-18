#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "libc/SovereignLibC.h"

// Professional Indian Standards Calculator
class SovereignIndianCalc {
public:
    void calculateGST(double amount, double rate_percent) {
        double gst_amount = (amount * rate_percent) / 100.0;
        double total_amount = amount + gst_amount;
        sigma_log_info("[CALC/GST] Base Amount: %.2f INR\n", amount);
        sigma_log_info("[CALC/GST] CGST (%.2f%%): %.2f INR\n", rate_percent/2, gst_amount/2);
        sigma_log_info("[CALC/GST] SGST (%.2f%%): %.2f INR\n", rate_percent/2, gst_amount/2);
        sigma_log_info("[CALC/GST] Total Payable: %.2f INR\n", total_amount);
    }

    void calculateIncomeTax(double annual_income) {
        // Simplified New Tax Regime (FY 2023-24)
        double tax = 0;
        if (annual_income <= 300000) tax = 0;
        else if (annual_income <= 600000) tax = (annual_income - 300000) * 0.05;
        else if (annual_income <= 900000) tax = 15000 + (annual_income - 600000) * 0.10;
        else if (annual_income <= 1200000) tax = 45000 + (annual_income - 900000) * 0.15;
        else if (annual_income <= 1500000) tax = 90000 + (annual_income - 1200000) * 0.20;
        else tax = 150000 + (annual_income - 1500000) * 0.30;
        
        sigma_log_info("[CALC/TAX] Annual Income: %.2f INR\n", annual_income);
        sigma_log_info("[CALC/TAX] Income Tax Liability (New Regime): %.2f INR\n", tax);
        sigma_log_info("[CALC/TAX] Cess (4%%): %.2f INR\n", tax * 0.04);
        sigma_log_info("[CALC/TAX] Total Tax: %.2f INR\n", tax * 1.04);
    }
};

extern "C" void run_indian_calc_tools() {
    SovereignIndianCalc calc;
    calc.calculateGST(10000.0, 18.0);
    calc.calculateIncomeTax(1250000.0);
}
