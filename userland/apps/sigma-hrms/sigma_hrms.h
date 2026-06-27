// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_hrms.h — Human Resource Management System
 *
 * Covers Indian labour law compliance:
 *   - EPF (Employees' Provident Fund) — 12% employee + 12% employer
 *   - ESIC (Employee State Insurance) — 0.75% employee + 3.25% employer
 *   - Professional Tax (state-specific slabs)
 *   - TDS on salary (Section 192, new tax regime FY 2024-25)
 *   - Gratuity (Payment of Gratuity Act 1972)
 *   - Leave management (PL/CL/SL per state labour laws)
 *   - Minimum Wages Act compliance (state-wise)
 *   - Form 16 generation (TDS certificate)
 *   - EPF ECR (Electronic Challan cum Return) upload
 *   - ESIC monthly contribution challan
 *
 * sigma-bus integration:
 *   Salary paid → sigma.Accounts.VoucherPosted (salary expense)
 *   EPF deducted → sigma.Accounts.VoucherPosted (EPF liability)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Employee record ─────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32  id;
    char       name[128];
    char       uan[12];        /* Universal Account Number (EPF)             */
    char       esic_no[17];
    char       pan[11];
    char       aadhaar[12];    /* masked: "XXXX XXXX 1234"                   */
    char       designation[64];
    char       department[64];
    sigma_u64  doj_epoch;      /* Date of joining                            */
    char       bank_account[20];
    char       bank_ifsc[12];
    bool       active;

    /* Salary structure (in paise) */
    sigma_s64  basic_paise;
    sigma_s64  hra_paise;
    sigma_s64  da_paise;        /* Dearness Allowance                        */
    sigma_s64  special_paise;
    sigma_s64  gross_paise;     /* computed: sum of above                    */

    /* Deductions flag */
    bool       epf_applicable;   /* employees < ₹15,000 basic: mandatory   */
    bool       esic_applicable;  /* employees < ₹21,000 gross: mandatory   */
    bool       pt_applicable;    /* Professional Tax (state-wise)           */

    char       state[3];        /* "MH", "DL", "KA" — for PT slabs         */
} sigma_employee_t;

/* ── Monthly payslip ─────────────────────────────────────────────────────── */
typedef struct {
    sigma_u32      employee_id;
    sigma_u32      month;         /* YYYYMM e.g. 202403                     */
    sigma_u32      working_days;
    sigma_u32      present_days;
    sigma_u32      leave_days;
    sigma_u32      lop_days;      /* Loss of Pay                            */

    /* Earnings (paise) */
    sigma_s64      basic_paise;
    sigma_s64      hra_paise;
    sigma_s64      da_paise;
    sigma_s64      special_paise;
    sigma_s64      arrears_paise;
    sigma_s64      gross_paise;

    /* Deductions (paise) */
    sigma_s64      epf_employee_paise;    /* 12% of basic                  */
    sigma_s64      epf_employer_paise;    /* 12% of basic (cost to company)*/
    sigma_s64      esic_employee_paise;   /* 0.75% of gross                */
    sigma_s64      esic_employer_paise;   /* 3.25% of gross                */
    sigma_s64      pt_paise;              /* Professional Tax              */
    sigma_s64      tds_paise;             /* TDS u/s 192                   */
    sigma_s64      loan_paise;            /* Salary advance deduction      */
    sigma_s64      total_deductions_paise;

    /* Net */
    sigma_s64      net_paise;             /* gross - deductions            */

    /* Compliance fields */
    char           epf_ecr_status[16];   /* "uploaded" / "pending"        */
    char           esic_challan[32];
} sigma_payslip_t;

/* ── Leave record ────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_LEAVE_PL  = 1,   /* Privilege Leave (earned)   */
    SIGMA_LEAVE_CL  = 2,   /* Casual Leave               */
    SIGMA_LEAVE_SL  = 3,   /* Sick Leave                 */
    SIGMA_LEAVE_LOP = 4,   /* Loss of Pay                */
    SIGMA_LEAVE_ML  = 5,   /* Maternity Leave            */
    SIGMA_LEAVE_PL2 = 6,   /* Paternity Leave            */
} sigma_leave_type_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Employee management */
int  sigma_hrms_employee_create(const sigma_employee_t *emp);
int  sigma_hrms_employee_update(const sigma_employee_t *emp);
int  sigma_hrms_employee_get(sigma_u32 id, sigma_employee_t *out);

/* Payroll processing */
int  sigma_hrms_process_payroll(sigma_u32 month_yyyymm,
                                 sigma_payslip_t *slips_out,
                                 int max_slips, int *count_out);

/* Calculate one employee's payslip */
int  sigma_hrms_calculate_payslip(sigma_u32 employee_id,
                                   sigma_u32 month_yyyymm,
                                   sigma_payslip_t *out);

/* Generate Form 16 (TDS certificate) for financial year */
int  sigma_hrms_form16(sigma_u32 employee_id, sigma_u32 fy_start_year,
                        char *json_out, size_t max_len);

/* Generate EPF ECR file for upload to EPFO portal */
int  sigma_hrms_epf_ecr(sigma_u32 month_yyyymm,
                          char *ecr_csv_out, size_t max_len);

/* Generate ESIC monthly contribution report */
int  sigma_hrms_esic_challan(sigma_u32 month_yyyymm,
                               char *json_out, size_t max_len);

/* Leave management */
int  sigma_hrms_leave_apply(sigma_u32 employee_id,
                              sigma_leave_type_t type,
                              sigma_u64 from_epoch, sigma_u64 to_epoch,
                              const char *reason);

int  sigma_hrms_leave_balance(sigma_u32 employee_id,
                               sigma_leave_type_t type,
                               sigma_u32 *balance_days_out);

/* Gratuity calculation (Payment of Gratuity Act 1972) */
sigma_s64 sigma_hrms_gratuity(sigma_u32 employee_id,
                               sigma_u64 exit_date_epoch);
