// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_legal.h — Legal professionals tool (India-specific, BNS/BNSS/BSA 2023)
 *
 * Covers the three new Indian criminal laws (July 1, 2024):
 *   BNS  2023 — Bharatiya Nyaya Sanhita       (replaced IPC 1860)
 *   BNSS 2023 — Bharatiya Nagarik Suraksha Sanhita (replaced CrPC 1973)
 *   BSA  2023 — Bharatiya Sakshya Adhiniyam   (replaced Evidence Act 1872)
 *
 * Also covers: CPC 1908, Companies Act 2013, GST Acts, IT Act + DPDP 2023,
 *              IBC 2016, RERA 2016, POCSO, Constitution + 1000+ central acts.
 */
#include <stdbool.h>

/* ── Law lookup ───────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_LAW_BNS_2023  = 0,  /* Bharatiya Nyaya Sanhita                  */
    SIGMA_LAW_BNSS_2023 = 1,  /* Bharatiya Nagarik Suraksha Sanhita       */
    SIGMA_LAW_BSA_2023  = 2,  /* Bharatiya Sakshya Adhiniyam              */
    SIGMA_LAW_IPC_1860  = 3,  /* Indian Penal Code (superseded by BNS)    */
    SIGMA_LAW_CRPC_1973 = 4,  /* CrPC (superseded by BNSS)                */
    SIGMA_LAW_COMPANIES = 5,  /* Companies Act 2013                       */
    SIGMA_LAW_GST       = 6,  /* CGST + IGST + SGST Acts                  */
    SIGMA_LAW_IT_DPDP   = 7,  /* IT Act 2000 + DPDP Act 2023              */
    SIGMA_LAW_IBC       = 8,  /* Insolvency & Bankruptcy Code 2016        */
    SIGMA_LAW_RERA      = 9,  /* Real Estate (Regulation) Act 2016        */
    SIGMA_LAW_POCSO     = 10, /* Protection of Children from Sexual Offences */
    SIGMA_LAW_LIMITATION= 11, /* Limitation Act 1963                      */
    SIGMA_LAW_CPC       = 12, /* Civil Procedure Code 1908                */
} sigma_law_id_t;

typedef struct {
    char section[16];     /* e.g. "302" or "103(2)"                       */
    char title[256];      /* short title of the provision                 */
    char text[4096];      /* full text of the section                     */
    char punishment[512]; /* punishment clause                            */
    char cross_refs[512]; /* related sections in other acts               */
} sigma_legal_section_t;

/* Search a law by section number */
int sigma_legal_lookup(sigma_law_id_t law, const char* section,
                        sigma_legal_section_t* out);

/* Full-text search across all loaded laws */
int sigma_legal_search(const char* query, sigma_legal_section_t* out, int max);

/* ── Case management ─────────────────────────────────────────────────────── */
typedef struct {
    char cnr_number[32];   /* eCourts CNR: DLNT01-123456-2025             */
    char court[128];
    char parties[512];
    char next_hearing[32]; /* ISO date                                    */
    char stage[64];
    char judge[128];
} sigma_case_t;

/* Fetch case status from eCourts API */
int sigma_ecourts_status(const char* cnr, sigma_case_t* out);

/* ── Deadline calculator (Limitation Act 1963) ───────────────────────────── */
typedef struct {
    char cause_of_action[256];
    char date_of_cause[12];    /* ISO date                                */
    int  limitation_years;
    char last_filing_date[12]; /* computed                                */
    char court[128];           /* appropriate court for the cause         */
    char article[16];          /* Article of Limitation Act               */
} sigma_limitation_t;

int sigma_limitation_calculate(const char* cause_description,
                                 const char* date_of_cause,
                                 sigma_limitation_t* out);

/* ── RERA compliance ─────────────────────────────────────────────────────── */
int sigma_rera_check_state(const char* state, const char* project_name,
                             char* status_out, int out_len);
