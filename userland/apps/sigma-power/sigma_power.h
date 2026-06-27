// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_power.h — Power sector professionals (CERC, SERCs, Electricity Act 2003)
 * ₹6 lakh crore electricity sector
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* Solar project DPR */
typedef struct {
    double   capacity_kw;
    double   lat, lon;
    double   tilt_degrees;
    char     state[3];
    /* Calculated */
    double   ghi_kwh_m2_day;     /* Global Horizontal Irradiation        */
    double   cuf_pct;            /* Capacity Utilisation Factor          */
    double   annual_gen_kwh;     /* P90 estimate                         */
    double   plf_pct;
    sigma_s64 project_cost_paise;
    sigma_s64 tariff_per_kwh_paise;
    sigma_s64 annual_revenue_paise;
    double   payback_years;
} sigma_solar_dpr_t;

/* Renewable Purchase Obligation check */
typedef struct {
    char   state[3];
    char   category[32];         /* "industrial", "commercial", "HT"     */
    double annual_consumption_kwh;
    double rpo_solar_pct;        /* state-specific                       */
    double rpo_non_solar_pct;
    double rpo_total_pct;
    double rpo_kwh_required;
    double rpo_kwh_met;
    bool   compliant;
    sigma_s64 penalty_paise;     /* if non-compliant                     */
} sigma_rpo_check_t;

/* AT&C (Aggregate Technical & Commercial) loss */
typedef struct {
    char   discom[64];
    double units_input_mwh;
    double units_billed_mwh;
    double units_collected_mwh;
    double technical_loss_pct;
    double commercial_loss_pct;
    double atc_loss_pct;
    double target_atc_pct;       /* RDSS target                          */
    bool   within_target;
} sigma_atc_loss_t;

int sigma_power_solar_dpr(sigma_solar_dpr_t *dpr);
int sigma_power_rpo_check(sigma_rpo_check_t *rpo);
int sigma_power_atc_calc(sigma_atc_loss_t *atc);
int sigma_power_rec_balance(const char *generator_id,
                              sigma_s64 *rec_balance_out);
int sigma_power_open_access_apply(const char *state, double load_kw,
                                   const char *source,
                                   char *application_json_out, size_t max);
