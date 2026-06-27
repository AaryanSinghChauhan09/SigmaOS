// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_sebi.h — SEBI compliance for stock brokers, RIAs, MF distributors
 * Covers: ₹400 lakh crore Indian securities market
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Capital gains types ─────────────────────────────────────────────────── */
typedef enum {
    SIGMA_CG_STCG_EQ   = 1,  /* Short-term: equity (< 1 year) — 20% tax    */
    SIGMA_CG_LTCG_EQ   = 2,  /* Long-term: equity (> 1 year) — 12.5% >₹1L */
    SIGMA_CG_STCG_DEBT = 3,  /* Short-term: debt — slab rate               */
    SIGMA_CG_LTCG_DEBT = 4,  /* Long-term: debt (> 2 years) — 12.5%       */
    SIGMA_CG_STCG_MF   = 5,  /* Mutual fund short-term                     */
    SIGMA_CG_LTCG_MF   = 6,  /* Mutual fund long-term                      */
} sigma_cg_type_t;

/* ── Trade record ────────────────────────────────────────────────────────── */
typedef struct {
    char           symbol[16];
    char           isin[13];
    sigma_u64      buy_date_epoch;
    sigma_u64      sell_date_epoch;
    double         qty;
    sigma_s64      buy_price_paise;
    sigma_s64      sell_price_paise;
    sigma_cg_type_t cg_type;
    sigma_s64      gain_paise;    /* positive = gain, negative = loss      */
    sigma_s64      tax_paise;
} sigma_trade_t;

/* ── SIP calculator ──────────────────────────────────────────────────────── */
typedef struct {
    sigma_s64  monthly_paise;
    double     expected_annual_return_pct;
    sigma_u32  tenure_months;
    /* Output */
    sigma_s64  total_invested_paise;
    sigma_s64  estimated_value_paise;
    double     xirr_pct;
} sigma_sip_calc_t;

/* ── KYC / Risk profile ──────────────────────────────────────────────────── */
typedef enum {
    SIGMA_RISK_CONSERVATIVE  = 1,
    SIGMA_RISK_MODERATE      = 2,
    SIGMA_RISK_AGGRESSIVE    = 3,
} sigma_risk_profile_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_sebi_capital_gains(const sigma_trade_t *trades, int n_trades,
                              sigma_u32 fy_start_year,
                              char *report_json_out, size_t max_len);
int sigma_sebi_sip_calc(sigma_sip_calc_t *calc);
int sigma_sebi_peak_margin_report(const char *client_id, sigma_u64 date_epoch,
                                   char *report_json_out, size_t max_len);
int sigma_sebi_kyc_verify(const char *pan, const char *dob_yyyymmdd,
                           sigma_risk_profile_t *risk_out, char *status_out);
int sigma_sebi_scores_complaint(const char *investor_pan,
                                 const char *broker_sebi_regn,
                                 const char *complaint_text,
                                 char *ticket_id_out, size_t max_len);
