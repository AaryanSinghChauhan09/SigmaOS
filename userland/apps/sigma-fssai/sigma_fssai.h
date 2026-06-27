// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_fssai.h — Food Safety & Standards Authority of India compliance
 * 7.5 million food businesses in India
 * License types: Basic (<₹12L), State (₹12L-₹20Cr), Central (>₹20Cr)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_FSSAI_BASIC   = 1,  /* Turnover < ₹12L, single state             */
    SIGMA_FSSAI_STATE   = 2,  /* ₹12L – ₹20Cr                              */
    SIGMA_FSSAI_CENTRAL = 3,  /* >₹20Cr or multi-state or importer          */
} sigma_fssai_license_type_t;

typedef struct {
    char   license_no[15];   /* 14-digit FSSAI license number               */
    sigma_fssai_license_type_t type;
    char   business_name[128];
    char   address[256];
    sigma_u64 issue_epoch;
    sigma_u64 expiry_epoch;
    bool   active;
} sigma_fssai_license_t;

/* ── HACCP Critical Control Point ───────────────────────────────────────── */
typedef struct {
    char     zone[64];        /* "Kitchen", "Cold Storage", "Dispatch"       */
    double   temp_celsius;
    sigma_u64 logged_epoch;
    bool     within_limit;   /* FSSAI: cold storage ≤ 4°C, hot hold ≥ 60°C */
    char     action_taken[256];
} sigma_haccp_log_t;

/* ── Menu item with allergen info (FSSAI 2024 mandatory) ────────────────── */
typedef struct {
    char   name[128];
    char   allergens[12][32]; /* gluten, milk, eggs, peanuts, etc.          */
    int    n_allergens;
    double gst_rate;          /* restaurants: 5% (non-AC), 12% (AC/liquor) */
    sigma_s64 price_paise;
    bool   veg;
} sigma_menu_item_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
sigma_fssai_license_type_t sigma_fssai_license_type_for_turnover(sigma_s64 annual_turnover_paise);
int sigma_fssai_license_check(sigma_s64 annual_turnover_paise, sigma_fssai_license_t *out);
int sigma_fssai_haccp_log(const sigma_haccp_log_t *entry);
int sigma_fssai_hygiene_audit(const char *establishment_id, char *report_json_out, size_t max);
int sigma_fssai_allergen_declare(const sigma_menu_item_t *item, char *label_out, size_t max);
int sigma_fssai_recall_report(const char *product_name, const char *batch, const char *reason);
