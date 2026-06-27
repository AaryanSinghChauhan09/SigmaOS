// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_pharma.h — Pharmaceutical professionals (CDSCO, NDPS, Schedule H)
 * India is world's pharmacy — ₹4.2 lakh crore industry
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_DRUG_SCHEDULE_H   = 1,  /* Prescription-only                     */
    SIGMA_DRUG_SCHEDULE_H1  = 2,  /* High-risk prescription                */
    SIGMA_DRUG_SCHEDULE_X   = 3,  /* Psychotropic/narcotic (NDPS)          */
    SIGMA_DRUG_OTC          = 4,  /* Over-the-counter                      */
    SIGMA_DRUG_AYUSH        = 5,  /* Ayurvedic/herbal                      */
} sigma_drug_schedule_t;

/* Mandatory drug register entry (Schedule H/H1/X) */
typedef struct {
    char   drug_name[64];
    char   manufacturer[64];
    char   batch_no[32];
    sigma_u64 expiry_epoch;
    double qty_received;
    double qty_dispensed;
    char   rx_no[32];          /* prescription number                       */
    char   patient_name[128];
    char   doctor_name[128];
    sigma_u64 dispensed_epoch;
    sigma_drug_schedule_t schedule;
} sigma_drug_log_t;

/* Drug interaction check */
typedef struct {
    char   drug_a[64];
    char   drug_b[64];
    char   severity[16];       /* "minor", "moderate", "major"             */
    char   mechanism[256];
    char   recommendation[256];
} sigma_drug_interaction_t;

/* Pediatric dose calculator */
typedef struct {
    char   drug_name[64];
    double weight_kg;
    double age_years;
    double dose_mg_per_kg;
    /* Output */
    double total_dose_mg;
    char   frequency[32];
    char   route[16];
    double max_dose_mg;        /* do not exceed                            */
} sigma_pediatric_dose_t;

int sigma_pharma_drug_log(const sigma_drug_log_t *entry);
int sigma_pharma_expiry_check(sigma_u32 days_threshold,
                               sigma_drug_log_t *expiring_out,
                               int max, int *count_out);
int sigma_pharma_interaction_check(const char *drug_a, const char *drug_b,
                                    sigma_drug_interaction_t *out);
int sigma_pharma_pediatric_dose(sigma_pediatric_dose_t *calc);
int sigma_pharma_ndps_log(const sigma_drug_log_t *entry); /* narcotic register */
int sigma_pharma_batch_recall(const char *drug_name, const char *batch_no,
                               const char *reason);
