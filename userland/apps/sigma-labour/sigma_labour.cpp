// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-labour — Labour law & payroll tools for HR professionals
// Covers: 4 Labour Codes 2020, EPF/ESIC, payroll, compliance calendar
//
// CLI:
//   sigma-labour payroll run --month 2026-06 --employees 50
//   sigma-labour pf ecr generate --month 2026-06
//   sigma-labour wages minimum --state MH --category Unskilled
//   sigma-labour compliance calendar --fy 2026-27
//   sigma-labour gratuity --years 12 --last-salary 45000

#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

// ── Minimum Wages (State-wise, Zone-A) ───────────────────────────────────────

struct MinWage {
    const char *state;
    const char *category;
    uint32_t    daily_wage;    // ₹ per day
    uint32_t    monthly_wage;  // ₹ per month (26 working days)
    const char  *effective_from;
};

static const MinWage min_wages[] = {
    { "MH", "Unskilled",     563,  14638, "2026-01-01" },
    { "MH", "Semi-Skilled",  634,  16484, "2026-01-01" },
    { "MH", "Skilled",       720,  18720, "2026-01-01" },
    { "MH", "Highly-Skilled",816,  21216, "2026-01-01" },
    { "DL", "Unskilled",     793,  20618, "2026-04-01" },
    { "DL", "Skilled",       873,  22698, "2026-04-01" },
    { "DL", "Highly-Skilled",958,  24908, "2026-04-01" },
    { "PB", "Unskilled",     459,  11934, "2026-03-01" },
    { "PB", "Skilled",       567,  14742, "2026-03-01" },
    { "KA", "Unskilled",     746,  19396, "2025-10-01" },
    { "KA", "Skilled",       820,  21320, "2025-10-01" },
    { "TN", "Unskilled",     700,  18200, "2025-10-01" },
    { "TN", "Skilled",       772,  20072, "2025-10-01" },
    { NULL, NULL, 0, 0, NULL }
};

void sigma_labour_min_wages(const char *state, const char *category) {
    printf("Minimum Wages — %s | Category: %s\n", state, category);
    for (int i = 0; min_wages[i].state; i++) {
        if (strcmp(min_wages[i].state, state) == 0 &&
            strcmp(min_wages[i].category, category) == 0) {
            const MinWage *w = &min_wages[i];
            printf("  Daily:      ₹%u\n", w->daily_wage);
            printf("  Monthly:    ₹%u (26 days)\n", w->monthly_wage);
            printf("  Effective:  %s\n", w->effective_from);
            printf("  VDA linked: Yes (variable dearness allowance)\n");
            printf("  Basis: Code on Wages 2019 § 6\n");
            return;
        }
    }
    printf("State '%s' category '%s' not in local database\n", state, category);
    printf("Query: sigma-net get https://labour.gov.in/wages\n");
}

// ── EPF Calculation ───────────────────────────────────────────────────────────

typedef struct {
    uint32_t basic_salary;
    uint32_t employee_contribution;  // 12% of basic (EPF + EPS)
    uint32_t employer_epf;           // 3.67% of basic → EPF
    uint32_t employer_eps;           // 8.33% of basic (max ₹1250) → EPS
    uint32_t employer_edli;          // 0.5% → EDLI insurance
    uint32_t admin_charges;          // 0.5% employer admin
    uint32_t total_employer;
    uint32_t total_monthly;
} EPFCalc;

EPFCalc sigma_labour_pf_calc(uint32_t basic_salary) {
    EPFCalc c = {0};
    c.basic_salary = basic_salary;
    c.employee_contribution = basic_salary * 12 / 100;
    // Employer's 12% split: 8.33% EPS (capped ₹1250) + remaining → EPF
    c.employer_eps = (basic_salary * 833 / 10000);
    if (c.employer_eps > 1250) c.employer_eps = 1250;
    c.employer_epf = (basic_salary * 12 / 100) - c.employer_eps;
    c.employer_edli = basic_salary * 5 / 1000;   // 0.5%
    c.admin_charges = basic_salary * 5 / 1000;    // 0.5%
    c.total_employer = c.employer_epf + c.employer_eps + c.employer_edli + c.admin_charges;
    c.total_monthly  = c.employee_contribution + c.total_employer;
    return c;
}

void sigma_labour_pf_show(uint32_t basic_salary) {
    EPFCalc c = sigma_labour_pf_calc(basic_salary);
    printf("EPF Calculation — Basic: ₹%u/month\n", basic_salary);
    printf("  Employee (12%%):       ₹%u → EPF account\n", c.employee_contribution);
    printf("  Employer EPF (3.67%%): ₹%u → EPF account\n", c.employer_epf);
    printf("  Employer EPS (8.33%%): ₹%u → Pension scheme (max ₹1250)\n", c.employer_eps);
    printf("  EDLI (0.5%%):          ₹%u → Insurance\n", c.employer_edli);
    printf("  Admin (0.5%%):         ₹%u → Admin charges\n", c.admin_charges);
    printf("  ─────────────────────────────\n");
    printf("  Total employer:       ₹%u\n", c.total_employer);
    printf("  Total monthly PF:     ₹%u\n", c.total_monthly);
    printf("  ECR: sigma-labour pf ecr generate\n");
    printf("  Interest rate FY2024-25: 8.25%% p.a.\n");
}

// ── ESIC Contribution ─────────────────────────────────────────────────────────

void sigma_labour_esic_calc(uint32_t gross_salary) {
    if (gross_salary > 21000) {
        printf("ESIC Not applicable (gross > ₹21,000)\n");
        return;
    }
    uint32_t employer_cont = gross_salary * 325 / 10000; // 3.25%
    uint32_t employee_cont = gross_salary * 75  / 10000; // 0.75%
    printf("ESIC Contribution — Gross: ₹%u\n", gross_salary);
    printf("  Employee (0.75%%): ₹%u\n", employee_cont);
    printf("  Employer (3.25%%): ₹%u\n", employer_cont);
    printf("  Total:             ₹%u\n", employer_cont + employee_cont);
    printf("  Eligibility: Gross ≤ ₹21,000 | Factories/Shops with ≥10 employees\n");
    printf("  Benefits: Medical, sickness, maternity, disablement\n");
    printf("  Filing: Form 5 (half-yearly) via sigma-labour esic return\n");
}

// ── Gratuity Calculator (Payment of Gratuity Act 1972) ───────────────────────

void sigma_labour_gratuity(uint32_t years_service, uint32_t last_basic_da) {
    // Gratuity = (last salary × 15/26) × years of service
    // Capped at ₹20 lakh (tax-free under Income Tax Act)
    uint64_t gratuity = (uint64_t)last_basic_da * 15 / 26 * years_service;
    uint64_t cap = 2000000ULL; // ₹20 lakh
    int capped = gratuity > cap;
    if (capped) gratuity = cap;

    printf("Gratuity Calculation (Gratuity Act 1972)\n");
    printf("  Years of service:  %u\n", years_service);
    printf("  Last basic+DA:     ₹%u/month\n", last_basic_da);
    printf("  Formula:           Salary × 15/26 × Years\n");
    printf("  Gratuity payable:  ₹%llu%s\n",
           (unsigned long long)gratuity, capped ? " (CAPPED at ₹20L)" : "");
    if (years_service < 5) {
        printf("  Note: Gratuity requires minimum 5 years of continuous service\n");
        printf("  Exception: Death/disability — no minimum service required\n");
    }
    printf("  Tax: Exempt up to ₹20L (u/s 10(10) Income Tax Act)\n");
}

// ── Compliance Calendar ───────────────────────────────────────────────────────

void sigma_labour_compliance_calendar(const char *fy) {
    printf("Labour Law Compliance Calendar — FY %s\n\n", fy);
    printf("MONTHLY DEADLINES:\n");
    printf("  7th:   TDS deposit (salary deductions)\n");
    printf("  15th:  ESI contribution deposit\n");
    printf("  15th:  PF contribution deposit (ECR filing)\n");
    printf("  Last:  Professional Tax (state-specific)\n\n");
    printf("QUARTERLY:\n");
    printf("  Jul 15: TDS return Q1 (Form 24Q)\n");
    printf("  Oct 15: TDS return Q2 (Form 24Q)\n");
    printf("  Jan 15: TDS return Q3 (Form 24Q)\n");
    printf("  May 31: TDS return Q4 (Form 24Q)\n\n");
    printf("ANNUAL:\n");
    printf("  Apr 30: PF annual return (Form 3A/6A)\n");
    printf("  Nov 30: Payment of Bonus (within 8 months of year-end)\n");
    printf("  Dec 31: Leave encashment\n");
    printf("  Mar 31: Form 16 issuance\n\n");
    printf("HALF-YEARLY:\n");
    printf("  Apr 11: ESIC return H2 (Form 5)\n");
    printf("  Oct 11: ESIC return H1 (Form 5)\n\n");
    printf("Penalty tracker: sigma-labour compliance penalty --late\n");
}

// ── Payroll Run ───────────────────────────────────────────────────────────────

void sigma_labour_payroll_run(const char *month, uint32_t employee_count) {
    printf("Payroll Run — %s | %u employees\n", month, employee_count);
    printf("Step 1: Import attendance (sigma-labour attendance import)\n");
    printf("Step 2: Calculate gross salary (Basic + HRA + DA + allowances)\n");
    printf("Step 3: Compute statutory deductions:\n");
    printf("         PF (12%% employee) + ESIC (0.75%%) + PT + TDS\n");
    printf("Step 4: Net salary = Gross − Deductions\n");
    printf("Step 5: Generate payslips (PDF via sigma-labour payslip --month %s)\n", month);
    printf("Step 6: Bank transfer file (NEFT batch) → sigma-upi batch\n");
    printf("Step 7: Generate ECR for PF portal\n");
    printf("Status: Ready to process %u salaries\n", employee_count);
    printf("Export: sigma-labour payroll export --format excel --month %s\n", month);
}
