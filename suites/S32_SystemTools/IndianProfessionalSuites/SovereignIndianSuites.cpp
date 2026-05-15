#include "include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "include/SovereignLibC.h"

/* =========================================================================
 * SIGMAOS: INDIAN PROFESSIONAL SUITES (S-INDIA) v1.0
 * Compliant with: GST Act 2017, Income Tax Act 1961, Companies Act 2013
 * Target Professions: Accountant, Doctor, Lawyer, Engineer, Farmer, Teacher
 * ========================================================================= */

/* ---- GST CALCULATOR (as per GST Act 2017) ---- */
struct GSTResult {
    sigma_f64 base_amount;
    sigma_f64 cgst;
    sigma_f64 sgst;
    sigma_f64 igst;       /* for inter-state */
    sigma_f64 cess;
    sigma_f64 total;
};

class SovereignGSTEngine {
public:
    /* Intra-state supply */
    GSTResult calculateIntraState(sigma_f64 amount, sigma_f64 rate_pct) {
        GSTResult r;
        r.base_amount = amount;
        r.cgst  = (amount * rate_pct / 2.0) / 100.0;
        r.sgst  = r.cgst;
        r.igst  = 0.0;
        r.cess  = 0.0;
        r.total = amount + r.cgst + r.sgst;
        sigma_log_info("[GST] Base: %.2f | CGST %.2f%%: %.2f | SGST %.2f%%: %.2f | Total: %.2f INR\n",
            r.base_amount, rate_pct/2, r.cgst, rate_pct/2, r.sgst, r.total);
        return r;
    }
    /* Inter-state supply (IGST) */
    GSTResult calculateInterState(sigma_f64 amount, sigma_f64 rate_pct) {
        GSTResult r;
        r.base_amount = amount;
        r.cgst  = 0.0;
        r.sgst  = 0.0;
        r.igst  = (amount * rate_pct) / 100.0;
        r.cess  = 0.0;
        r.total = amount + r.igst;
        sigma_log_info("[GST/IGST] Base: %.2f | IGST %.2f%%: %.2f | Total: %.2f INR\n",
            r.base_amount, rate_pct, r.igst, r.total);
        return r;
    }
    /* Reverse charge mechanism */
    void reverseCharge(sigma_f64 amount, sigma_f64 rate_pct) {
        sigma_f64 tax = (amount * rate_pct) / 100.0;
        sigma_log_info("[GST/RCM] Reverse Charge on %.2f @ %.2f%% = %.2f INR (payable by recipient)\n",
            amount, rate_pct, tax);
    }
};

/* ---- INCOME TAX ENGINE (FY 2024-25, New Regime per Section 115BAC) ---- */
struct ITaxResult {
    sigma_f64 gross_income;
    sigma_f64 standard_deduction;
    sigma_f64 taxable_income;
    sigma_f64 base_tax;
    sigma_f64 surcharge;
    sigma_f64 cess;        /* 4% Health & Education Cess */
    sigma_f64 total_tax;
    sigma_f64 effective_rate_pct;
};

class SovereignIncomeTaxEngine {
public:
    /* New Tax Regime (Section 115BAC, AY 2025-26) */
    ITaxResult calculateNewRegime(sigma_f64 gross_income) {
        ITaxResult r;
        r.gross_income       = gross_income;
        r.standard_deduction = 75000.0; /* Budget 2024 enhanced std deduction */
        r.taxable_income     = gross_income - r.standard_deduction;
        if (r.taxable_income < 0) r.taxable_income = 0;

        /* Slabs: 0-3L: 0%, 3-7L: 5%, 7-10L: 10%, 10-12L: 15%, 12-15L: 20%, >15L: 30% */
        r.base_tax = 0.0;
        sigma_f64 ti = r.taxable_income;
        if      (ti <= 300000)  r.base_tax = 0;
        else if (ti <= 700000)  r.base_tax = (ti - 300000) * 0.05;
        else if (ti <= 1000000) r.base_tax = 20000 + (ti - 700000) * 0.10;
        else if (ti <= 1200000) r.base_tax = 50000 + (ti - 1000000) * 0.15;
        else if (ti <= 1500000) r.base_tax = 80000 + (ti - 1200000) * 0.20;
        else                    r.base_tax = 140000 + (ti - 1500000) * 0.30;

        /* Rebate u/s 87A: full rebate if taxable income <= 7L */
        if (r.taxable_income <= 700000) r.base_tax = 0;

        /* Surcharge */
        r.surcharge = 0;
        if      (gross_income > 50000000)  r.surcharge = r.base_tax * 0.37;
        else if (gross_income > 20000000)  r.surcharge = r.base_tax * 0.25;
        else if (gross_income > 10000000)  r.surcharge = r.base_tax * 0.15;
        else if (gross_income > 5000000)   r.surcharge = r.base_tax * 0.10;

        r.cess       = (r.base_tax + r.surcharge) * 0.04;
        r.total_tax  = r.base_tax + r.surcharge + r.cess;
        r.effective_rate_pct = gross_income > 0 ? (r.total_tax / gross_income) * 100.0 : 0.0;

        sigma_log_info("[TAX/NEW] Gross: %.2f | Std Dedn: %.2f | Taxable: %.2f\n",
            r.gross_income, r.standard_deduction, r.taxable_income);
        sigma_log_info("[TAX/NEW] Tax: %.2f | Surcharge: %.2f | Cess: %.2f | TOTAL: %.2f INR (%.2f%%)\n",
            r.base_tax, r.surcharge, r.cess, r.total_tax, r.effective_rate_pct);
        return r;
    }

    /* TDS Calculator (Section 192) */
    void calculateTDS(sigma_f64 annual_salary, sigma_u32 months_remaining) {
        if (months_remaining == 0) return;
        ITaxResult r = calculateNewRegime(annual_salary);
        sigma_f64 monthly_tds = r.total_tax / 12.0;
        sigma_log_info("[TDS/192] Annual Tax: %.2f | Monthly TDS: %.2f INR\n", r.total_tax, monthly_tds);
    }

    /* Advance Tax Calculator (Sections 207-219) */
    void calculateAdvanceTax(sigma_f64 annual_income) {
        ITaxResult r = calculateNewRegime(annual_income);
        sigma_log_info("[ADV-TAX] June 15 (15%%): %.2f | Sep 15 (45%%): %.2f | Dec 15 (75%%): %.2f | Mar 15 (100%%): %.2f\n",
            r.total_tax*0.15, r.total_tax*0.45, r.total_tax*0.75, r.total_tax);
    }
};

/* ---- EMI CALCULATOR (Banking Regulation Act 1949) ---- */
class SovereignEMIEngine {
public:
    void calculateEMI(sigma_f64 principal, sigma_f64 annual_rate_pct, sigma_u32 months) {
        if (months == 0 || annual_rate_pct < 0) return;
        sigma_f64 r = annual_rate_pct / (12.0 * 100.0);
        sigma_f64 emi;
        if (r == 0.0) {
            emi = principal / months;
        } else {
            /* EMI = P * r * (1+r)^n / ((1+r)^n - 1) */
            sigma_f64 pow_factor = 1.0;
            for (sigma_u32 i = 0; i < months; ++i) pow_factor *= (1.0 + r);
            emi = principal * r * pow_factor / (pow_factor - 1.0);
        }
        sigma_f64 total_payment = emi * months;
        sigma_f64 total_interest = total_payment - principal;
        sigma_log_info("[EMI] Principal: %.2f | Rate: %.2f%% pa | Tenure: %u months\n",
            principal, annual_rate_pct, months);
        sigma_log_info("[EMI] Monthly EMI: %.2f | Total: %.2f | Interest: %.2f INR\n",
            emi, total_payment, total_interest);
    }
};

/* ---- PROVIDENT FUND CALCULATOR (EPF Act 1952) ---- */
class SovereignPFEngine {
public:
    void calculateEPF(sigma_f64 basic_salary, sigma_u32 years) {
        /* Employee: 12% of basic, Employer: 12% (3.67% EPF + 8.33% EPS) */
        sigma_f64 employee_pf  = basic_salary * 0.12;
        sigma_f64 employer_epf = basic_salary * 0.0367;
        sigma_f64 eps          = basic_salary * 0.0833;
        sigma_f64 monthly_total = employee_pf + employer_epf;
        sigma_f64 yearly_total  = monthly_total * 12;
        sigma_f64 corpus_estimate = yearly_total * years * 1.085; /* ~8.5% EPF interest */
        sigma_log_info("[EPF] Basic: %.2f | Employee: %.2f | Employer EPF: %.2f | EPS: %.2f\n",
            basic_salary, employee_pf, employer_epf, eps);
        sigma_log_info("[EPF] Monthly Deduction: %.2f | Corpus (%u yrs @8.5%%): %.2f INR\n",
            monthly_total, years, corpus_estimate);
    }
};

/* ---- C BRIDGE ---- */
static SovereignGSTEngine       gst_engine;
static SovereignIncomeTaxEngine tax_engine;
static SovereignEMIEngine       emi_engine;
static SovereignPFEngine        pf_engine;

extern "C" {
    void sigma_gst_intra(sigma_f64 amount, sigma_f64 rate)   { gst_engine.calculateIntraState(amount, rate); }
    void sigma_gst_inter(sigma_f64 amount, sigma_f64 rate)   { gst_engine.calculateInterState(amount, rate); }
    void sigma_income_tax(sigma_f64 gross)                   { tax_engine.calculateNewRegime(gross); }
    void sigma_tds(sigma_f64 salary, sigma_u32 months)       { tax_engine.calculateTDS(salary, months); }
    void sigma_advance_tax(sigma_f64 income)                 { tax_engine.calculateAdvanceTax(income); }
    void sigma_emi(sigma_f64 p, sigma_f64 r, sigma_u32 n)    { emi_engine.calculateEMI(p, r, n); }
    void sigma_epf(sigma_f64 basic, sigma_u32 years)         { pf_engine.calculateEPF(basic, years); }
}
