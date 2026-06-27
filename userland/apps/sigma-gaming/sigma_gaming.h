// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_gaming.h — Online gaming developers (IT Online Games Rules 2023)
 * Fantasy sports, GST 28%, TDS 194BA
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    char   platform_name[64];
    bool   skill_based;           /* skill = 28% GST on fee; chance = 28% on full value */
    sigma_s64 entry_fee_paise;
    sigma_s64 prize_pool_paise;
    sigma_s64 gst_paise;
    sigma_s64 tds_paise;          /* Section 194BA: 30% of net winnings > ₹100 */
    sigma_s64 net_winnings_paise;
    sigma_s64 tds_deductible_paise;
} sigma_gaming_calc_t;

/* Cultural compliance check */
typedef struct {
    char   content_description[256];
    bool   religious_sentiment_risk;
    bool   national_symbol_used;    /* Emblems Act 1950                   */
    bool   historical_figure;
    bool   minor_character;         /* POCSO awareness required           */
    char   recommendations[512];
} sigma_gaming_culture_check_t;

int sigma_gaming_gst_calc(sigma_gaming_calc_t *calc);
int sigma_gaming_tds_calc(sigma_s64 gross_winnings_paise,
                           sigma_s64 losses_paise,
                           sigma_s64 *tds_paise_out);
int sigma_gaming_kyc_verify(const char *user_id,
                              const char *aadhaar_no,
                              bool *verified_out);
int sigma_gaming_culture_check(const sigma_gaming_culture_check_t *content);
int sigma_gaming_sro_register(const char *platform_name,
                               char *registration_no_out, size_t max);
