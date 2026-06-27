// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_wellness.h — Yoga, Ayurveda, gym, spa (CCIM, QCI, AYUSH)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_AYURVEDA = 1, SIGMA_YOGA     = 2,
    SIGMA_NATUROPATHY=3, SIGMA_UNANI   = 4,
    SIGMA_SIDDHA   = 5, SIGMA_HOMEO   = 6,
} sigma_ayush_system_t;

/* Prakriti (Ayurvedic body type assessment) */
typedef struct {
    sigma_u32  patient_id;
    sigma_u32  vata_score;   /* 0-40 */
    sigma_u32  pitta_score;
    sigma_u32  kapha_score;
    char       dominant_prakriti[16]; /* "Vata", "Pitta", "Kapha", "Vata-Pitta" */
    char       ayurvedic_recommendations[1024];
} sigma_prakriti_t;

/* Gym member */
typedef struct {
    sigma_u32  id;
    char       name[128];
    char       phone[16];
    char       membership_type[16]; /* "monthly", "quarterly", "annual"    */
    sigma_s64  fee_paise;
    sigma_u64  start_epoch;
    sigma_u64  expiry_epoch;
    bool       active;
    double     body_fat_pct;
    double     bmi;
} sigma_gym_member_t;

int sigma_wellness_ccim_register(const char *practitioner_name,
                                  sigma_ayush_system_t system,
                                  char *reg_no_out, size_t max);
int sigma_wellness_prakriti_assess(sigma_u32 patient_id,
                                    const sigma_u32 *questionnaire_answers,
                                    int n_answers, sigma_prakriti_t *out);
int sigma_wellness_gym_member_add(const sigma_gym_member_t *m);
int sigma_wellness_gym_fee_receipt(sigma_u32 member_id, char *receipt_json_out, size_t max);
int sigma_wellness_qci_yoga_cert(const char *practitioner_name,
                                  sigma_u32 level, /* 1-5 */
                                  char *cert_no_out, size_t max);
int sigma_wellness_pmjay_ayush(sigma_u32 patient_id, const char *treatment,
                                char *claim_json_out, size_t max);
