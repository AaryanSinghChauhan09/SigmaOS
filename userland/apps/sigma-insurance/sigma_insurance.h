// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_insurance.h — Insurance professionals (IRDAI compliance)
 * Covers: Life, Health, Motor, Property, PMJJBY/PMSBY govt schemes
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_INS_LIFE_TERM       = 1,
    SIGMA_INS_LIFE_ULIP       = 2,
    SIGMA_INS_LIFE_ENDOWMENT  = 3,
    SIGMA_INS_HEALTH_INDIV    = 4,
    SIGMA_INS_HEALTH_FLOATER  = 5,
    SIGMA_INS_MOTOR_OD        = 6,
    SIGMA_INS_MOTOR_TP        = 7,
    SIGMA_INS_PMJJBY          = 8,  /* ₹2L life cover @ ₹436/year          */
    SIGMA_INS_PMSBY           = 9,  /* ₹2L accident @ ₹20/year             */
    SIGMA_INS_CROP_PMFBY      = 10, /* Pradhan Mantri Fasal Bima Yojana     */
} sigma_ins_type_t;

typedef struct {
    char           policy_no[32];
    sigma_ins_type_t type;
    char           insured_name[128];
    char           insured_pan[11];
    sigma_s64      sum_assured_paise;
    sigma_s64      annual_premium_paise;
    sigma_u64      inception_epoch;
    sigma_u64      expiry_epoch;
    char           nominee_name[128];
    char           agent_code[16];
    bool           active;
    sigma_u64      next_renewal_epoch;
} sigma_policy_t;

typedef struct {
    char           claim_no[32];
    char           policy_no[32];
    sigma_ins_type_t type;
    sigma_u64      date_of_loss_epoch;
    char           cause_of_loss[256];
    sigma_s64      claimed_amount_paise;
    sigma_s64      settled_amount_paise;
    char           status[32];    /* "registered","survey","settled","repudiated" */
    char           surveyor[128];
} sigma_claim_t;

/* ── Premium calculator ──────────────────────────────────────────────────── */
typedef struct {
    sigma_ins_type_t type;
    sigma_u32        age;
    sigma_s64        sum_assured_paise;
    sigma_u32        term_years;
    bool             non_smoker;
    /* Output */
    sigma_s64        annual_premium_paise;
    sigma_s64        gst_18pct_paise;
    sigma_s64        total_paise;
} sigma_premium_calc_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_ins_premium_calc(sigma_premium_calc_t *calc);
int sigma_ins_policy_create(const sigma_policy_t *pol);
int sigma_ins_claim_register(const sigma_claim_t *claim);
int sigma_ins_renewal_reminders(sigma_u32 days_ahead,
                                 sigma_policy_t *out, int max, int *count);
int sigma_ins_agent_commission(const char *agent_code,
                                sigma_u32 month_yyyymm,
                                sigma_s64 *commission_paise_out);
