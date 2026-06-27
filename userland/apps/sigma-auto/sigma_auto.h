// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_auto.h — Connected vehicle & EV management (sigma-auto)
 *
 * India's FAME-II scheme subsidises 1 million EVs.
 * SigmaOS in a vehicle = sovereign connected car OS.
 *
 * Features:
 *   - OBD-II / CAN bus interface (read engine codes, live telemetry)
 *   - VAHAN integration (RC, insurance, fitness certificate)
 *   - FAME-II EV subsidy claim management
 *   - FastTag / NETC (national electronic toll collection) balance
 *   - Real-time NavIC-based GPS (no Google Maps)
 *   - Fleet management (sigma-fleet integration for taxi/trucking)
 *   - BS-VI emission norms compliance checker
 *   - Battery health monitor for EVs (SOC, SOH, cycle count)
 *   - Driving behaviour analytics (for insurance telematics)
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Vehicle record ──────────────────────────────────────────────────────── */
typedef struct {
    char   registration_no[16];  /* "MH02AB1234"                           */
    char   chassis_no[18];       /* VIN                                     */
    char   engine_no[16];
    char   owner_name[128];
    char   owner_aadhaar[12];
    char   vehicle_class[32];    /* "LMV", "HGV", "2W", "EV"              */
    char   fuel_type[16];        /* "petrol", "diesel", "cng", "electric"  */
    char   manufacturer[64];
    char   model[64];
    sigma_u32 year;
    double engine_cc;
    double battery_kwh;          /* EVs only                               */
    sigma_u64 rc_expiry_epoch;
    sigma_u64 insurance_expiry_epoch;
    sigma_u64 fitness_expiry_epoch;
    sigma_u64 pollution_expiry_epoch;  /* PUCC certificate                 */
} sigma_vehicle_t;

/* ── Live OBD-II telemetry ───────────────────────────────────────────────── */
typedef struct {
    double   speed_kmh;
    double   rpm;
    double   throttle_pct;
    double   engine_temp_c;
    double   fuel_level_pct;
    double   battery_soc_pct;    /* EV: state of charge                    */
    double   battery_soh_pct;    /* EV: state of health (degradation)      */
    double   odometer_km;
    char     dtc_codes[16][8];   /* OBD-II fault codes e.g. "P0300"       */
    int      n_dtc;
    bool     mil_on;             /* Malfunction Indicator Lamp (Check Engine) */
} sigma_obd_t;

/* ── FAME-II EV subsidy ──────────────────────────────────────────────────── */
typedef struct {
    char   vehicle_reg[16];
    char   oem_name[64];
    char   model[64];
    double battery_kwh;
    sigma_s64 subsidy_paise;     /* ₹10,000/KWh for 2W, higher for buses  */
    char   claim_status[32];     /* "eligible","claimed","disbursed"       */
    char   claim_ref[32];
} sigma_fame2_t;

/* ── Driving behaviour ───────────────────────────────────────────────────── */
typedef struct {
    double   distance_km;
    double   avg_speed_kmh;
    double   max_speed_kmh;
    int      harsh_braking_count;
    int      harsh_acceleration_count;
    int      over_speed_count;    /* >80 kmh city, >120 kmh highway        */
    double   fuel_efficiency_kmpl;
    double   co2_emission_gkm;
    sigma_u32 safety_score;       /* 0-100 (for telematics insurance)      */
} sigma_driving_behaviour_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_auto_vehicle_verify(const char *reg_no, sigma_vehicle_t *out);
int sigma_auto_obd_read(sigma_obd_t *out);
int sigma_auto_fasttag_balance(const char *vehicle_reg,
                                sigma_s64 *balance_paise_out);
int sigma_auto_fame2_check(const char *chassis_no, sigma_fame2_t *out);
int sigma_auto_driving_score(const sigma_obd_t *trip_data, sigma_u32 n_samples,
                               sigma_driving_behaviour_t *out);
int sigma_auto_emission_check(const sigma_obd_t *obd,
                               const char *vehicle_class,
                               bool *bsvi_compliant);
int sigma_auto_fleet_register(const char *vehicle_reg,
                               const char *fleet_group);
