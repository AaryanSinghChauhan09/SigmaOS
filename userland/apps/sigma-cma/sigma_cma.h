// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_cma.h — Cost & Management Accountants (ICMAI, Companies Act §148)
 * 5 lakh+ CMAs — invisible in existing software
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* Cost audit threshold check */
typedef struct {
    sigma_s64  annual_turnover_paise;
    char       industry[32];         /* "pharma", "cement", "steel", etc.  */
    bool       cost_audit_required;
    char       applicable_products[256];
    char       form_required[8];     /* "CRA-1", "CRA-2", "CRA-3"         */
    char       due_date[32];
} sigma_cost_audit_check_t;

/* CMA Data for bank loan (project finance) */
typedef struct {
    char       project_name[128];
    sigma_s64  loan_amount_paise;
    sigma_u32  tenure_years;
    double     interest_rate_pct;
    /* Projected financials (3-5 years) */
    sigma_s64  projected_revenue[5];  /* paise */
    sigma_s64  projected_ebitda[5];
    sigma_s64  projected_pat[5];      /* Profit After Tax */
    double     dscr[5];               /* Debt Service Coverage Ratio      */
    double     bep_pct;               /* Break-even point as % of capacity */
    char       bank_name[64];
} sigma_cma_data_t;

/* Cost variance analysis */
typedef struct {
    char       product[64];
    sigma_s64  standard_cost_paise;
    sigma_s64  actual_cost_paise;
    sigma_u32  units;
    sigma_s64  total_variance_paise;  /* negative = favorable              */
    char       variance_type[32];     /* "material", "labour", "overhead"  */
    char       cause[256];
} sigma_cost_variance_t;

int sigma_cma_audit_threshold(sigma_s64 turnover_paise, const char *industry,
                               sigma_cost_audit_check_t *out);
int sigma_cma_data_generate(const sigma_cma_data_t *data,
                              char *report_json_out, size_t max_len);
int sigma_cma_variance_analysis(const sigma_cost_variance_t *v,
                                 char *report_json_out, size_t max_len);
int sigma_cma_abc_costing(const char *product_code, double units,
                           sigma_s64 *overhead_paise_out);
int sigma_cma_bep(sigma_s64 fixed_cost_paise, sigma_s64 contribution_per_unit,
                   double *bep_units_out, double *bep_revenue_paise_out);
