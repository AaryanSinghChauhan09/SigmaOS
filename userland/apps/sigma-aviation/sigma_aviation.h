// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_aviation.h — Aviation professionals (DGCA compliance, flight ops)
 * DGCA CARs, ICAO standards, STCW, ISM Code
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Pilot license ───────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_PIL_ATPL = 1,  /* Airline Transport Pilot License  */
    SIGMA_PIL_CPL  = 2,  /* Commercial Pilot License          */
    SIGMA_PIL_PPL  = 3,  /* Private Pilot License             */
    SIGMA_PIL_SPL  = 4,  /* Student Pilot License             */
} sigma_pil_type_t;

typedef struct {
    char           pilot_name[128];
    char           license_no[32];
    sigma_pil_type_t type;
    char           aircraft_type_ratings[8][32]; /* A320, B737, ATR72 etc. */
    int            n_type_ratings;
    sigma_u64      medical_class1_expiry;
    sigma_u64      license_expiry;
    double         total_hours;              /* total flight hours           */
    double         hours_last_90_days;       /* currency check               */
    bool           instrument_current;       /* 6 approaches in last 6 months*/
} sigma_pilot_t;

/* ── Flight log entry ────────────────────────────────────────────────────── */
typedef struct {
    sigma_u64  date_epoch;
    char       aircraft_reg[8];    /* "VT-ABC"                               */
    char       aircraft_type[16];  /* "A320"                                 */
    char       from_icao[5];       /* "VIDP" (Delhi)                        */
    char       to_icao[5];         /* "VABB" (Mumbai)                       */
    double     block_hours;        /* block-to-block time                    */
    double     flight_hours;
    double     night_hours;
    double     ifr_hours;
    char       role[8];            /* "PIC", "SIC", "SI"                    */
    char       remarks[256];
} sigma_flight_log_t;

/* ── Weather briefing ────────────────────────────────────────────────────── */
typedef struct {
    char  icao[5];
    char  metar[512];   /* raw METAR string                                   */
    char  taf[1024];    /* Terminal Aerodrome Forecast                        */
    char  sigmet[512];  /* SIGMET for the route                               */
    char  notam[2048];  /* relevant NOTAMs                                    */
    bool  ifr_conditions;
    bool  sigmet_active;
} sigma_wx_brief_t;

/* ── API ─────────────────────────────────────────────────────────────────── */
int sigma_aviation_log_hours(const sigma_flight_log_t *entry);
int sigma_aviation_pilot_currency(const char *license_no,
                                   sigma_pilot_t *out);
int sigma_aviation_wx_brief(const char *from_icao, const char *to_icao,
                              sigma_wx_brief_t *out);
int sigma_aviation_weight_balance(const char *aircraft_type,
                                   double pax_weight_kg, double cargo_kg,
                                   double fuel_kg,
                                   double *cg_out, bool *within_envelope);
int sigma_aviation_fuel_calc(const char *from_icao, const char *to_icao,
                               double wind_component_kts,
                               double *fuel_required_kg, double *diversion_kg);
