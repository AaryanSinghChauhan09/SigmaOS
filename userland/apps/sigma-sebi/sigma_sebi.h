// SPDX-License-Identifier: GPL-2.0-only
// sigma_sebi.h — SigmaOS SEBI / Capital Markets Professional App
// Regulator: SEBI / AMFI / NSDL / CDSL / BSE / NSE / MCX

#pragma once
#include <sigma_indiastack.h>

typedef enum {
    SIGMA_SEBI_REG_STOCK_BROKER = 1,
    SIGMA_SEBI_REG_DP           = 2,  // Depository Participant
    SIGMA_SEBI_REG_IA           = 3,  // Investment Adviser
    SIGMA_SEBI_REG_RA           = 4,  // Research Analyst
    SIGMA_SEBI_REG_MF_DIST      = 5,  // Mutual Fund Distributor (AMFI ARN)
    SIGMA_SEBI_REG_PMS          = 6,  // Portfolio Manager
    SIGMA_SEBI_REG_AIF          = 7,  // Alternative Investment Fund
} sigma_sebi_reg_type_t;

// Broker tools
typedef struct {
    char   sebi_reg_no[32];       // INZ000XXXXXX format
    char   pan[12];
    bool   kyc_verified;
    bool   ipv_done;              // In-Person Verification (annual)
    time_t ipv_date;
    time_t ipv_due;               // Annual renewal
    double peak_margin_utilised;  // SEBI peak margin rule
    double available_margin;
    double collateral_value;
    bool   client_fund_segregated; // Mandatory: client ≠ broker pool
} sigma_sebi_broker_t;

// Investment Adviser (IA) — SEBI (IA) Regulations 2013
typedef struct {
    char   ia_reg_no[32];         // INA000XXXXXX
    char   nism_cert[32];         // NISM-Series-X-A certificate
    time_t nism_expiry;           // 3-year renewal
    bool   fee_only;              // Flat fee or AUM — NOT commission (SEBI mandate)
    double fee_annual_flat;       // If flat fee
    double fee_aum_pct;           // If AUM-based (max 2.5% per SEBI)
    bool   client_agreement_signed;
    bool   risk_profile_done;
    bool   suitability_assessment_done;
    char   advice_register_path[256]; // Mandatory log of all advice given
} sigma_sebi_ia_t;

// Capital gains calculation
typedef struct {
    char   asset[64];             // Stock/MF name
    char   isin[14];
    char   asset_type[16];        // "equity", "debt_mf", "etf", "reit"
    double qty;
    double purchase_price;
    double sale_price;
    time_t purchase_date;
    time_t sale_date;
    bool   is_stcg;               // Short term (< 12 months for equity)
    double stcg_tax_pct;          // 20% (post Jul 2024 budget)
    double ltcg_tax_pct;          // 12.5% (post Jul 2024 budget)
    double ltcg_exemption_limit;  // ₹1.25 lakh
    double gain_amount;
    double tax_amount;
    bool   grandfathered;         // Pre-2018 purchase (31-Jan-18 cost basis)
    double grandfathered_cost;
} sigma_sebi_capital_gain_t;

// Mutual Fund Distributor
typedef struct {
    char   amfi_arn[16];          // ARN-XXXXXX
    char   euin[16];              // Employee Unique Identification Number
    time_t arn_expiry;
    bool   cpd_done;              // Continuing Professional Development
    double aum_total;             // Total AUM managed
    double sip_book;              // Monthly SIP book value
    double commission_trail;      // Monthly trail commission
    double commission_upfront;    // Upfront (SEBI has reduced this to near zero)
    bool   clawback_risk;         // If > 30-day redemption after upfront
} sigma_sebi_mfd_t;

// SCORES complaint
typedef struct {
    char   scores_reg_no[32];     // SEBI SCORES registration
    char   complaint_id[32];
    char   complainant_pan[12];
    char   complaint_text[1024];
    char   entity_name[128];
    char   entity_sebi_reg[32];
    time_t filed_date;
    char   status[32];            // "PENDING", "RESOLVED", "CLOSED"
} sigma_sebi_scores_t;

int sigma_sebi_peak_margin_report(const char *client_id, time_t date,
                                   double *margin_used, double *shortfall);
int sigma_sebi_gains_calculate(sigma_sebi_capital_gain_t *gains, int count,
                                double *total_stcg, double *total_ltcg,
                                double *total_tax);
int sigma_sebi_kyc_verify(const char *pan, const char *dob,
                           bool *verified);
int sigma_sebi_scores_file(sigma_sebi_scores_t *complaint,
                            char *complaint_id_out);
int sigma_sebi_nism_check(const char *cert_no, time_t *expiry,
                           bool *valid);
// CLI: sigma-sebi margin peak-report --client CLIENT001 --date today
//      sigma-sebi gains calculate --fy 2026-27 --demat-statement demat.pdf
//      sigma-sebi kyc verify --pan ABCDE1234F --dob 1990-01-15
