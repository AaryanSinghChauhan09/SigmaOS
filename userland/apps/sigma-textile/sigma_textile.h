// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_textile.h — Textile & fashion professionals
 * Textile Consumer Protection Rules 2023, GOTS, GI Tags, PM Vishwakarma
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_TEXTILE_COTTON   = 1,
    SIGMA_TEXTILE_SILK     = 2,
    SIGMA_TEXTILE_WOOL     = 3,
    SIGMA_TEXTILE_POLYESTER= 4,
    SIGMA_TEXTILE_LINEN    = 5,
    SIGMA_TEXTILE_JUTE     = 6,
    SIGMA_TEXTILE_KHADI    = 7,
    SIGMA_TEXTILE_BLEND    = 8,
} sigma_fiber_t;

/* ── Garment production order ────────────────────────────────────────────── */
typedef struct {
    sigma_u32  order_id;
    char       style_no[32];
    char       buyer_name[128];
    sigma_fiber_t fabric_type;
    char       color[32];
    sigma_u32  size_xs, size_s, size_m, size_l, size_xl, size_xxl;
    sigma_u32  total_pcs;
    sigma_u64  delivery_epoch;
    double     fabric_gsm;
    double     fabric_consumption_per_pc_m; /* marker efficiency output  */
    double     total_fabric_required_m;
    sigma_s64  cost_per_pc_paise;
    /* GST */
    double     gst_rate;    /* 5% if ≤₹1000/pc, 12% if >₹1000 MRP        */
    sigma_s64  gst_per_pc_paise;
} sigma_garment_order_t;

/* ── GI Tag application ──────────────────────────────────────────────────── */
typedef struct {
    char   product_name[64];    /* "Banarasi Saree", "Kanchipuram Silk"    */
    char   applicant_name[128];
    char   state[3];
    char   description[1024];
    char   gi_class[8];         /* NICE classification                    */
    bool   registered;
    char   gi_no[16];
    sigma_u64 registration_epoch;
} sigma_gi_tag_t;

/* ── Mandatory label (Textile Rules 2023) ────────────────────────────────── */
typedef struct {
    char   product_name[128];
    char   fiber_content[256]; /* "65% Polyester, 35% Cotton"             */
    char   country_of_origin[32]; /* "Made in India"                      */
    char   care_instructions[256]; /* ISO 3758 symbols + text             */
    char   manufacturer_name[128];
    char   manufacturer_address[256];
    char   gstin[16];
    sigma_s64 mrp_paise;
    char   hsn[8];
} sigma_textile_label_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_textile_fabric_consumption(sigma_garment_order_t *order);
int sigma_textile_gst_rate(sigma_s64 mrp_paise, sigma_fiber_t fiber,
                            double *rate_out);
int sigma_textile_label_generate(const sigma_textile_label_t *label,
                                  char *pdf_path_out, size_t max_len);
int sigma_textile_gi_apply(const sigma_gi_tag_t *gi);
int sigma_textile_pm_vishwakarma(const char *artisan_name,
                                  const char *craft_type,
                                  char *application_json_out, size_t max_len);
int sigma_textile_rosl_calc(double fob_value_usd, sigma_fiber_t fiber,
                              double *rosl_pct_out, sigma_s64 *rebate_inr_paise);
