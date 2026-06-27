// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_dental.h — Dental professionals (DCI, CGHS, BMW Rules 2016)
 * 3 lakh+ dentists in India
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── FDI tooth notation (international two-digit system) ─────────────────── */
/* Quadrant 1: 11-18 (upper right), Quadrant 2: 21-28 (upper left)          */
/* Quadrant 3: 31-38 (lower left),  Quadrant 4: 41-48 (lower right)         */
/* Deciduous: 51-55, 61-65, 71-75, 81-85                                     */
typedef sigma_u8 sigma_tooth_fdi_t;   /* e.g. 16 = upper right first molar  */

/* ── Dental procedure ────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_DENTAL_EXTRACTION     = 1,
    SIGMA_DENTAL_FILLING        = 2,
    SIGMA_DENTAL_RCT            = 3,   /* Root Canal Treatment               */
    SIGMA_DENTAL_CROWN_PFM      = 4,   /* Porcelain-Fused-to-Metal           */
    SIGMA_DENTAL_CROWN_CERAMIC  = 5,
    SIGMA_DENTAL_SCALING        = 6,
    SIGMA_DENTAL_IMPLANT        = 7,
    SIGMA_DENTAL_DENTURE_FULL   = 8,
    SIGMA_DENTAL_DENTURE_PARTIAL= 9,
    SIGMA_DENTAL_ORTHODONTICS   = 10,
    SIGMA_DENTAL_BLEACHING      = 11,
} sigma_dental_proc_t;

/* ── Dental chart entry ──────────────────────────────────────────────────── */
typedef struct {
    sigma_tooth_fdi_t tooth;
    sigma_dental_proc_t procedure;
    char   surface[8];        /* "M", "D", "O", "B", "L", "MOD" etc.       */
    sigma_u64 date_epoch;
    char   notes[256];
    bool   completed;
    sigma_s64 fee_paise;
    sigma_s64 cghs_rate_paise; /* CGHS empanelled rate for this procedure   */
} sigma_dental_chart_entry_t;

/* ── Autoclave sterilisation log ────────────────────────────────────────── */
typedef struct {
    sigma_u32  cycle_no;
    sigma_u64  cycle_start_epoch;
    double     temp_celsius;    /* 121°C (gravity) or 134°C (flash)         */
    double     pressure_bar;
    sigma_u32  duration_min;
    bool       biological_indicator_pass;  /* mandatory weekly BI test      */
    char       load_description[256];
    char       operator_name[128];
} sigma_autoclave_log_t;

/* ── Biomedical waste log (BMW Rules 2016) ──────────────────────────────── */
typedef struct {
    sigma_u64  date_epoch;
    double     yellow_bag_kg;  /* anatomical/pathological                   */
    double     red_bag_kg;     /* contaminated recyclable                   */
    double     blue_bag_kg;    /* glass/metallic sharps                     */
    double     white_bag_kg;   /* sharp-translucent                         */
    char       agency_name[128];  /* authorised waste collection agency     */
    char       manifest_no[32];
} sigma_bmw_log_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_dental_chart_add(sigma_u32 patient_id, const sigma_dental_chart_entry_t *entry);
int sigma_dental_cghs_rate(sigma_dental_proc_t proc, sigma_s64 *rate_paise_out);
int sigma_dental_autoclave_log(const sigma_autoclave_log_t *log);
int sigma_dental_bmw_log(const sigma_bmw_log_t *log);
int sigma_dental_pmjay_claim(sigma_u32 patient_id, sigma_dental_proc_t proc,
                               char *claim_json_out, size_t max_len);
int sigma_dental_gst_rate(sigma_dental_proc_t proc, double *rate_out,
                            bool *is_service); /* prosthetics = 5% goods; services = exempt */
