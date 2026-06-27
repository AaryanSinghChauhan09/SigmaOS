// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_ca.h — Chartered Accountant & tax compliance tools (India)
 *
 * Covers: GST (GSTR-1, 3B, 9, 9C), Income Tax (ITR-1 to ITR-7),
 *         TDS (24Q, 26Q, 27Q), Companies Act 2013, Ind AS standards.
 * Regulatory bodies: CBIC (GST), CBDT (IT), MCA (Companies), ICAI (CA).
 */
#include <stdint.h>
#include <stdbool.h>

/* ── GST ──────────────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_GST_GSTR1  = 1,   /* outward supplies (monthly/quarterly) */
    SIGMA_GST_GSTR3B = 2,   /* summary return (monthly)             */
    SIGMA_GST_GSTR9  = 3,   /* annual return                        */
    SIGMA_GST_GSTR9C = 4,   /* reconciliation + audit               */
} sigma_gst_return_t;

typedef struct {
    char gstin[16];          /* 15-char GSTIN                        */
    char legal_name[256];
    char period[8];          /* "2026-06" for June 2026              */
    sigma_gst_return_t type;
    int64_t outward_taxable; /* in paise (avoid float rounding)      */
    int64_t igst_payable;
    int64_t cgst_payable;
    int64_t sgst_payable;
    int64_t itc_available;
    bool    filed;
    char    arn[24];         /* Acknowledgment Reference Number      */
} sigma_gst_return_data_t;

/* Compute GST return — fills amounts, validates GSTIN */
int sigma_gst_compute(sigma_gst_return_data_t* data);

/* File return via GSTN API (requires DSC/EVC) */
int sigma_gst_file(const sigma_gst_return_data_t* data,
                    const char* dsc_path);

/* Reconcile GSTR-2A/2B vs purchase register */
int sigma_gst_reconcile_2a(const char* gstin, const char* period,
                             const char* purchase_csv,
                             char* mismatch_out, int out_len);

/* ── e-Invoice (mandatory for B2B > ₹5 Cr turnover) ─────────────────────── */
typedef struct {
    char irn[64];     /* Invoice Reference Number (64-char hash)  */
    char qr_code[512];/* base64 QR code data                       */
    char ack_no[20];
    char ack_date[20];
} sigma_einvoice_t;

int sigma_einvoice_generate(const char* gstin,
                              const char* invoice_json,
                              sigma_einvoice_t* out);

/* ── Income Tax ───────────────────────────────────────────────────────────── */
typedef struct {
    char pan[11];
    char ay[8];          /* Assessment Year: "2026-27"               */
    int  itr_form;       /* 1..7                                     */
    int64_t gross_income;/* in paise                                 */
    int64_t deductions;  /* 80C, 80D, etc.                           */
    int64_t tax_payable;
    int64_t tds_deducted;
    int64_t advance_tax;
    int64_t refund_due;
} sigma_itr_data_t;

int sigma_itr_compute(sigma_itr_data_t* itr);
int sigma_itr_fetch_26as(const char* pan, const char* ay,
                          char* json_out, int out_len);

/* ── TDS ──────────────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_TDS_24Q = 1,  /* salary (Section 192)     */
    SIGMA_TDS_26Q = 2,  /* non-salary (all others)  */
    SIGMA_TDS_27Q = 3,  /* NRI payments             */
} sigma_tds_form_t;

int sigma_tds_calculate(sigma_tds_form_t form, int64_t payment_paise,
                          const char* section, char pan[11],
                          int64_t* tds_out);

/* ── Capital Gains ───────────────────────────────────────────────────────── */
typedef struct {
    char   asset_type[64]; /* "equity", "real_estate", "debt_mf"    */
    char   purchase_date[12];
    char   sale_date[12];
    int64_t purchase_price; /* paise                                */
    int64_t sale_price;
    int64_t indexation_cost;/* computed from CII tables              */
    bool   is_ltcg;
    int64_t tax_at_rate;   /* 10% LTCG equity (112A), 20% others   */
} sigma_capital_gains_t;

int sigma_cg_compute(sigma_capital_gains_t* cg);
