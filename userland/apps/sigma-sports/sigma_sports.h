// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_sports.h — Sports professionals (SAI, NADA, National Sports Policy)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef struct {
    char   athlete_name[128];
    char   sport[64];
    char   noc_code[4];        /* "IND"                                   */
    char   sai_id[16];
    bool   tops_registered;    /* Target Olympic Podium Scheme            */
    bool   khelo_india;
    sigma_s64 annual_grant_paise;
    char   coach_name[128];
} sigma_athlete_t;

/* ACWR — Acute:Chronic Workload Ratio (sports science) */
typedef struct {
    double acute_load;         /* last 7 days                             */
    double chronic_load;       /* last 28 days rolling average            */
    double acwr;               /* acute/chronic — 0.8-1.3 = safe zone    */
    char   risk_zone[16];      /* "safe", "danger", "overtraining"       */
} sigma_acwr_t;

/* NADA prohibited substance check */
typedef struct {
    char   substance[64];
    bool   prohibited_in_competition;
    bool   prohibited_out_of_competition;
    bool   prohibited_in_specific_sports;
    char   class[32];          /* "S1 Anabolic", "S3 Beta-2-agonists"   */
    bool   tue_eligible;       /* Therapeutic Use Exemption available    */
} sigma_nada_check_t;

int sigma_sports_athlete_register(const sigma_athlete_t *athlete);
int sigma_sports_acwr(const double *daily_loads, int n_days, sigma_acwr_t *out);
int sigma_sports_nada_check(const char *substance, sigma_nada_check_t *out);
int sigma_sports_tue_apply(const char *athlete_name, const char *substance,
                            const char *medical_condition,
                            char *application_id_out, size_t max);
int sigma_sports_tops_apply(const char *athlete_name, const char *sport,
                             char *status_out, size_t max);
