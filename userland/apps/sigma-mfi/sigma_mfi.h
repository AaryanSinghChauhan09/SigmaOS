// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_mfi.h — Microfinance & cooperatives (RBI NBFC-MFI, Chit Funds Act)
 * Reaching India's 250 million unbanked
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* JLG (Joint Liability Group) */
typedef struct {
    char       group_id[16];
    char       members[5][128];
    sigma_u64  formation_epoch;
    char       centre_name[64];
    sigma_s64  loan_per_member_paise;
    double     interest_rate_pct;   /* RBI ceiling for MFIs               */
    sigma_u64  disbursement_epoch;
    sigma_s64  emi_paise;
    int        tenure_weeks;
    bool       third_lender_check_done; /* max 3 MFIs per borrower        */
} sigma_jlg_t;

/* Chit fund auction */
typedef struct {
    char       chit_id[16];
    sigma_u32  n_subscribers;
    sigma_s64  monthly_contribution_paise;
    sigma_u32  current_month;       /* 1 to n_subscribers                  */
    sigma_s64  prize_pool_paise;    /* n_subscribers × monthly             */
    sigma_s64  foreman_commission_paise; /* max 5%                        */
    sigma_s64  prizi_paise;         /* won by lucky subscriber            */
    char       winner_name[128];
} sigma_chit_auction_t;

int sigma_mfi_jlg_create(const sigma_jlg_t *jlg);
int sigma_mfi_jlg_meeting_log(const char *group_id, sigma_u64 date_epoch,
                               int present, int total,
                               const char *notes);
int sigma_mfi_three_lender_check(const char *borrower_aadhaar,
                                  bool *eligible_out, int *current_lenders_out);
int sigma_chit_auction(sigma_chit_auction_t *auction);
int sigma_chit_dividend(const char *chit_id, sigma_u32 month,
                         sigma_s64 *dividend_per_subscriber_paise_out);
int sigma_mfi_pacs_kcc(const char *farmer_aadhaar,
                        sigma_s64 *kcc_limit_paise_out);
