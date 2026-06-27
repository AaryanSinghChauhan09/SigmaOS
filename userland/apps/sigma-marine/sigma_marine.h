// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_marine.h — Maritime professionals (DG Shipping, STCW, ISM Code)
 * Merchant Shipping Act 1958, STCW Convention, SOLAS, MARPOL
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Officer rank ────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_RANK_MASTER      = 1,
    SIGMA_RANK_CHIEF_MATE  = 2,
    SIGMA_RANK_SECOND_MATE = 3,
    SIGMA_RANK_THIRD_MATE  = 4,
    SIGMA_RANK_CHIEF_ENG   = 5,
    SIGMA_RANK_SECOND_ENG  = 6,
    SIGMA_RANK_ELECTRO_TECH= 7,
    SIGMA_RANK_RATING      = 8,
} sigma_rank_t;

/* ── Officer certificates ────────────────────────────────────────────────── */
typedef struct {
    char       officer_name[128];
    char       coc_no[32];       /* Certificate of Competency number         */
    sigma_rank_t rank;
    char       coc_issuer[32];   /* "MMD Mumbai", "MMD Chennai"              */
    sigma_u64  coc_expiry_epoch;
    /* STCW certificates */
    sigma_u64  medical_expiry_epoch;  /* ENG1/ML5                           */
    sigma_u64  stsdsd_expiry_epoch;   /* Basic Safety — 5 year refresher    */
    sigma_u64  profpscrb_expiry_epoch;/* Proficiency in Survival Craft       */
    sigma_u64  bf_expiry_epoch;       /* Basic Fire Fighting                 */
    sigma_u64  frb_expiry_epoch;      /* Fast Rescue Boat                    */
    sigma_u64  gmdss_expiry_epoch;    /* GMDSS GOC/ROC                       */
} sigma_officer_certs_t;

/* ── Vessel stability calculation ────────────────────────────────────────── */
typedef struct {
    double displacement_t;   /* vessel displacement in tonnes                */
    double kG;               /* height of centre of gravity (metres)         */
    double kM;               /* metacentric height from keel (from tables)   */
    /* Output */
    double gm;               /* GM = KM - KG (metacentric height)           */
    bool   gm_positive;      /* vessel is stable if GM > 0                  */
    double min_gm_required;  /* IMO/SOLAS minimum (typically 0.15m)         */
    bool   meets_criteria;
} sigma_stability_t;

/* ── Bunker calculation ──────────────────────────────────────────────────── */
typedef struct {
    double distance_nm;        /* nautical miles                              */
    double speed_kts;          /* knots                                       */
    double daily_consumption_mt;/* metric tonnes per day at that speed       */
    double safety_margin_pct;  /* typically 15%                              */
    /* Output */
    double voyage_days;
    double fuel_required_mt;
    double with_margin_mt;
    double bunker_cost_usd;    /* at current USD/MT price                    */
} sigma_bunker_calc_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_marine_stcw_check(const sigma_officer_certs_t *officer,
                              char *expired_certs_json_out, size_t max_len);
int sigma_marine_stability(sigma_stability_t *calc);
int sigma_marine_bunker_calc(sigma_bunker_calc_t *calc);
int sigma_marine_voyage_plan(const char *from_unlocode, const char *to_unlocode,
                               char *waypoints_json_out, size_t max_len);
int sigma_marine_port_dues(const char *unlocode, double grt,
                            const char *vessel_type, sigma_s64 *dues_usd_cents_out);
int sigma_marine_imdg_declaration(const char *class_label, double qty_kg,
                                   const char *un_no, char *declaration_out, size_t max);
