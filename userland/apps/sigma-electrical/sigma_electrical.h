// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_electrical.h — Electrical engineers & contractors
 * IE Rules 1956, CEA Technical Standards 2010, IS 732, IS 3043, IS 3961
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Load calculation result ─────────────────────────────────────────────── */
typedef struct {
    double connected_kw;
    double demand_factor;     /* 0.0–1.0                                     */
    double diversity_factor;  /* 0.0–1.0                                     */
    double maximum_demand_kw; /* connected × demand / diversity              */
    double power_factor;      /* typical 0.8 lag for industrial               */
    double kva;               /* kW / pf                                     */
    double current_amps;      /* at 415V 3-phase                             */
} sigma_load_calc_t;

/* ── Cable sizing result (IS 3961) ──────────────────────────────────────── */
typedef struct {
    double current_amps;
    double length_m;
    double voltage_drop_pct_limit;  /* typically 3% for lighting, 5% for power */
    double supply_voltage;          /* 230V single-phase, 415V three-phase    */
    /* Output */
    double recommended_sqmm;        /* cable cross-section                   */
    double actual_voltage_drop_pct;
    const char *cable_type;         /* "XLPE", "PVC", "Armoured"             */
    bool   within_voltage_drop;
} sigma_cable_size_t;

/* ── DG set sizing ───────────────────────────────────────────────────────── */
typedef struct {
    double critical_load_kw;    /* loads that must run during power cut      */
    double power_factor;
    double derating_factor;     /* 0.8 for altitude >1000m or temp >40°C    */
    /* Output */
    double kva_required;
    double recommended_kva;     /* next standard size                        */
    const char *fuel_type;      /* "diesel", "LNG"                           */
} sigma_dg_sizing_t;

/* ── Net metering application ────────────────────────────────────────────── */
typedef struct {
    char   state[3];          /* "MH", "DL", "KA" — DISCOM varies by state  */
    double solar_capacity_kw;
    char   consumer_no[32];
    char   discom_name[64];
    bool   three_phase;
    /* Output */
    char   form_name[32];     /* MSEDCL, BSES, KSEB — different forms       */
    char   instructions[512];
    double subsidy_pct;       /* PM Surya Ghar subsidy                      */
} sigma_net_meter_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_electrical_load_calc(sigma_load_calc_t *calc);
int sigma_electrical_cable_size(sigma_cable_size_t *sizing);
int sigma_electrical_dg_size(sigma_dg_sizing_t *sizing);
int sigma_electrical_net_meter_apply(sigma_net_meter_t *app);
int sigma_electrical_short_circuit_kA(double supply_kva, double impedance_pct,
                                       double *isc_kA_out);
int sigma_electrical_earth_resistance(double resistivity_ohm_m, double rod_len_m,
                                       double rod_dia_m, double *resistance_out);
