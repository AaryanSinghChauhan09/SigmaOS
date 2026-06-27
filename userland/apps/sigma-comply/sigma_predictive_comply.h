// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_predictive_comply.h — Predictive compliance engine
 *
 * Laws in India change constantly (BNS 2024, GST council meetings,
 * SEBI circulars, RBI master directions, FSSAI notifications).
 *
 * Traditional software: wait for update, user manually updates.
 * SigmaOS: monitor legislation → detect changes → auto-update OS modules.
 *
 * Sources monitored:
 *   egazette.nic.in    — Official Gazette of India (all central laws)
 *   gst.gov.in         — GST Council notifications + rate changes
 *   sebi.gov.in        — SEBI circulars (capital markets)
 *   rbi.org.in         — RBI master directions + circulars
 *   mca.gov.in         — Companies Act amendments (ROC filings)
 *   icai.org           — ICAI standards (accounting + auditing)
 *   epfindia.gov.in    — EPFO circulars (EPF rate changes)
 *   labour.gov.in      — Labour Code notifications
 *
 * When a change is detected:
 *   1. sigma-comply downloads and parses the notification
 *   2. Determines which sigma-* modules are affected
 *   3. Generates a compliance patch (delta to rates/forms/logic)
 *   4. Shows user a plain-language summary: "GST rate on X changed from Y% to Z%"
 *   5. User approves → sigma-comply applies the update
 *   6. Affected apps (sigma-accounts, sigma-legal, etc.) hot-reload
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Law category ────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_LAW_GST            = 1,
    SIGMA_LAW_INCOME_TAX     = 2,
    SIGMA_LAW_COMPANIES_ACT  = 3,
    SIGMA_LAW_SEBI           = 4,
    SIGMA_LAW_RBI            = 5,
    SIGMA_LAW_LABOUR         = 6,
    SIGMA_LAW_FSSAI          = 7,
    SIGMA_LAW_DGCA           = 8,
    SIGMA_LAW_EPF_ESIC       = 9,
    SIGMA_LAW_CRIMINAL_BNS   = 10,
    SIGMA_LAW_OTHER          = 99,
} sigma_law_category_t;

/* ── Compliance change notification ──────────────────────────────────────── */
typedef struct {
    sigma_u32           id;
    sigma_law_category_t category;
    char                title[256];
    char                gazette_no[32];    /* e.g. "GSR 487(E)"             */
    sigma_u64           effective_epoch;   /* when it takes effect          */
    char                summary[1024];     /* plain English explanation     */
    char                affected_modules[8][32]; /* e.g. "sigma-accounts"  */
    int                 n_affected;
    bool                breaking_change;   /* requires user action          */
    bool                auto_patchable;    /* can be applied automatically  */
    char                patch_id[32];      /* sigma-pkg patch identifier    */
    sigma_u64           detected_epoch;
    bool                applied;
} sigma_comply_change_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Start monitoring all registered law sources. */
int sigma_comply_start_monitoring(void);

/* Get pending changes not yet applied. */
int sigma_comply_pending(sigma_comply_change_t *out, int max, int *count_out);

/* Apply a compliance change (patches the relevant OS module). */
int sigma_comply_apply(sigma_u32 change_id);

/* Apply all auto-patchable changes without user confirmation. */
int sigma_comply_apply_all_auto(int *applied_count_out);

/* Get compliance calendar: upcoming regulatory deadlines. */
typedef struct {
    sigma_u64  deadline_epoch;
    char       description[256];  /* "GSTR-1 due for Jan 2025"            */
    char       module[32];
    bool       completed;
} sigma_comply_deadline_t;

int sigma_comply_calendar(sigma_comply_deadline_t *out, int max, int *count_out);

/* Check if current software is compliant as of today. */
typedef struct {
    bool  fully_compliant;
    int   gaps;
    char  gap_descriptions[8][256];
} sigma_comply_status_t;

int sigma_comply_status(sigma_comply_status_t *out);
