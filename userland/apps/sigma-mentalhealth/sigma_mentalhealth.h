// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_mentalhealth.h — Psychologists, counselors (MHCA 2017, RCI)
 * Mental Healthcare Act 2017 — almost no software supports it
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* Standardised screening tool scores */
typedef struct {
    sigma_u32 client_id;
    char   tool[16];       /* "PHQ-9", "GAD-7", "PCL-5", "AUDIT"         */
    int    responses[9];   /* max 9 items                                 */
    int    n_items;
    int    total_score;
    char   severity[32];   /* "minimal", "mild", "moderate", "severe"    */
    sigma_u64 date_epoch;
    bool   risk_flag;      /* Columbia Suicide Severity — flag if high   */
} sigma_screening_score_t;

/* Session note (SOAP format) */
typedef struct {
    sigma_u32 client_id;
    sigma_u64 session_epoch;
    char   subjective[1024];  /* Client's reported experience             */
    char   objective[512];    /* Observed behaviour/mental status         */
    char   assessment[512];   /* Clinician's assessment                   */
    char   plan[512];         /* Treatment plan / next steps              */
    char   icd11_code[16];    /* ICD-11 diagnosis code e.g. "6A70"        */
    bool   safety_plan_reviewed;
    bool   consent_obtained;
} sigma_session_note_t;

/* Advance Directive (MHCA 2017 — patient's pre-specified treatment wishes) */
typedef struct {
    char   patient_name[128];
    char   patient_aadhaar[12];
    char   nominated_rep[128]; /* MHCA: Nominated Representative           */
    char   treatment_preferences[1024];
    char   refused_treatments[512];
    sigma_u64 created_epoch;
    char   witness_name[128];
    bool   registered;        /* registered with MHRB                     */
} sigma_advance_directive_t;

int sigma_mh_screening_score(sigma_screening_score_t *s);
int sigma_mh_session_note(const sigma_session_note_t *note);
int sigma_mh_advance_directive(const sigma_advance_directive_t *ad);
int sigma_mh_rci_cpd_log(const char *practitioner_name,
                           const char *activity, sigma_u32 points,
                           sigma_u64 date_epoch);
int sigma_mh_risk_alert(sigma_u32 client_id, const char *risk_level,
                         char *protocol_out, size_t max); /* duty to warn */
