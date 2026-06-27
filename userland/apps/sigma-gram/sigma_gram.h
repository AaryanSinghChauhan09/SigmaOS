// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_gram.h — Panchayat & rural governance tools (sigma-gram)
 *
 * India has 250,000 Gram Panchayats (village governments).
 * Each manages: land records, water supply, MGNREGA wages, BPL cards,
 * birth/death registration, sanitation, CSC services.
 *
 * sigma-gram integrates with:
 *   e-Panchayat (MoPR): national panchayat management system
 *   MGNREGA MIS: wage payment tracking
 *   SVAMITVA: property cards (drones + land records)
 *   PM Awas Yojana: housing scheme beneficiary management
 *   SBM-G (Swachh Bharat Mission Gramin): toilet construction
 *   Jal Jeevan Mission: piped water supply tracking
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Panchayat record ────────────────────────────────────────────────────── */
typedef struct {
    char   lgd_code[12];      /* LGD (Local Government Directory) code     */
    char   panchayat_name[128];
    char   block_name[64];
    char   district[64];
    char   state[3];
    sigma_u32 population;
    sigma_u32 households;
    char   sarpanch_name[128];
    char   sarpanch_phone[16];
    bool   oa_registered;     /* Open Defecation Free status               */
    bool   jjm_covered;       /* Jal Jeevan Mission tap water              */
} sigma_panchayat_t;

/* ── MGNREGA job card ────────────────────────────────────────────────────── */
typedef struct {
    char      job_card_no[20];
    char      head_of_family[128];
    sigma_u32 household_id;
    char      lgd_code[12];
    sigma_u32 days_worked_fy;     /* financial year                        */
    sigma_s64 wages_paid_paise;
    sigma_s64 wages_pending_paise;
    char      work_name[128];     /* name of the work (road, pond, etc.)   */
    sigma_u64 last_payment_epoch;
    bool      aadhaar_seeded;     /* required for DBT payment              */
} sigma_mgnrega_card_t;

/* ── Birth/Death registration ────────────────────────────────────────────── */
typedef struct {
    bool       is_birth;       /* true=birth, false=death                  */
    char       name[128];
    sigma_u64  date_epoch;
    char       place[128];
    char       father_name[128];
    char       mother_name[128];
    char       registration_no[32];
    sigma_u64  registered_epoch;
    char       registered_by[128]; /* panchayat secretary name            */
} sigma_crs_record_t;    /* Civil Registration System                     */

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_gram_panchayat_get(const char *lgd_code, sigma_panchayat_t *out);
int sigma_gram_mgnrega_job_card(const char *job_card_no,
                                 sigma_mgnrega_card_t *out);
int sigma_gram_birth_death_register(const sigma_crs_record_t *rec,
                                     char *cert_no_out, size_t max);
int sigma_gram_pmay_beneficiary(const char *aadhaar_no,
                                 char *status_json_out, size_t max);
int sigma_gram_jjm_status(const char *lgd_code, sigma_u32 *households_covered,
                           sigma_u32 *households_total);
int sigma_gram_svamitva_card(const char *khata_no, const char *district,
                               char *property_json_out, size_t max);
