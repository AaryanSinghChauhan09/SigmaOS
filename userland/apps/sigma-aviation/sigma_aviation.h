// SPDX-License-Identifier: GPL-2.0-only
// sigma_aviation.h — SigmaOS Aviation Professional App
// Regulator: DGCA / AAI / BAS / BCAS / ICAO / IATA
// Purpose  : Pilot logbook, DGCA license tracking, weather briefing,
//            flight planning, W&B, AME maintenance tracking, ATC ops.

#pragma once
#include <sigma_indiastack.h>

#define SIGMA_AVIA_DGCA_API    "https://dgca.gov.in/api"
#define SIGMA_AVIA_AAI_NOTAM   "https://notamweb.aai.aero/api"
#define SIGMA_AVIA_IMD_METAR   "https://aviationweather.gov/api/data/metar"

typedef enum {
    SIGMA_AVIA_LIC_PPL   = 1,   // Private Pilot Licence
    SIGMA_AVIA_LIC_CPL   = 2,   // Commercial Pilot Licence
    SIGMA_AVIA_LIC_ATPL  = 3,   // Airline Transport Pilot Licence
    SIGMA_AVIA_LIC_AME   = 4,   // Aircraft Maintenance Engineer
    SIGMA_AVIA_LIC_ATC   = 5,   // Air Traffic Controller
    SIGMA_AVIA_LIC_FTI   = 6,   // Flight Test Instructor
    SIGMA_AVIA_LIC_CFI   = 7,   // Chief Flight Instructor
} sigma_avia_license_type_t;

// Pilot licence and currency
typedef struct {
    char     licence_no[32];       // DGCA format: CPL/1234/2025
    sigma_avia_license_type_t type;
    char     pilot_name[128];
    char     pan[12];
    // Medical
    char     medical_class[4];     // "1", "2", "LAPL"
    time_t   medical_expiry;
    // Type ratings
    char     type_ratings[8][16];  // Aircraft types: "B737", "A320", etc.
    int      type_rating_count;
    // FRMS
    double   duty_hours_last_24h;
    double   duty_hours_last_7d;
    double   duty_hours_last_28d;
    double   flight_hours_last_90d; // Currency requirement: ≥3 landings in 90 days
    bool     night_current;
    bool     instrument_current;    // IFR: 6 approaches in 6 months
    // Totals
    double   total_flight_hours;
    double   total_pic_hours;
    double   total_instrument_hours;
    double   total_night_hours;
    time_t   licence_expiry;
    bool     valid;
} sigma_avia_pilot_t;

// Flight log entry
typedef struct {
    char     date[12];             // YYYY-MM-DD
    char     aircraft_reg[12];     // VT-ABC
    char     aircraft_type[16];    // B737-800
    char     from_icao[5];         // VIDP (Delhi)
    char     to_icao[5];           // VABB (Mumbai)
    double   block_off;            // Off-block time (decimal hours from midnight)
    double   block_on;
    double   total_time;           // Block-to-block
    double   flight_time;          // Airborne time
    double   pic_time;
    double   sic_time;
    double   instrument_time;
    double   night_time;
    int      landings_day;
    int      landings_night;
    bool     ifr;
    char     captain_name[64];
    char     remarks[256];
} sigma_avia_logbook_entry_t;

// Weather briefing
typedef struct {
    char     icao[5];
    char     metar[512];           // Raw METAR string
    char     taf[1024];            // Raw TAF string
    char     sigmet[512];
    char     pirep[512];           // Pilot report
    int      wind_direction;       // Degrees
    int      wind_speed_kt;
    int      visibility_m;
    int      ceiling_ft;           // Cloud base
    bool     ifr_conditions;       // Visibility < 3SM or ceiling < 1000ft
    bool     mvfr;                 // Marginal VFR
    bool     vfr;
    double   qnh_hpa;              // Altimeter setting
    int      temperature_c;
    int      dewpoint_c;
    time_t   obs_time;
} sigma_avia_weather_t;

// Weight and Balance
typedef struct {
    char     aircraft_type[16];
    double   basic_empty_weight_kg;
    double   basic_empty_arm_m;    // Centre of gravity arm
    double   basic_empty_moment;   // kg·m
    double   fuel_kg;
    double   fuel_arm_m;
    double   payload_kg;
    double   payload_arm_m;
    double   total_weight_kg;
    double   cg_position_m;
    double   mtow_kg;              // Maximum take-off weight
    double   fw_cg_limit_m;        // Forward CG limit
    double   aft_cg_limit_m;       // Aft CG limit
    bool     within_limits;
} sigma_avia_wb_t;

// AME — Maintenance record
typedef struct {
    char     task_ref[32];         // AD/SB/Scheduled task reference
    char     aircraft_reg[12];
    char     task_description[256];
    char     ame_licence_no[32];   // DGCA AME licence
    char     ame_name[128];
    time_t   performed_date;
    double   aircraft_hours;       // Total hours at task performance
    int      aircraft_cycles;
    double   next_due_hours;       // Next due at this many hours
    int      next_due_cycles;
    time_t   next_due_date;
    bool     rts_issued;           // Release to Service certificate
    char     rts_no[32];
    char     came_reference[32];   // CAME section reference
} sigma_avia_maintenance_t;

// NOTAM
typedef struct {
    char     notam_id[32];         // e.g. A0123/26
    char     fir[5];               // VIDP — Delhi FIR
    char     series;               // A, B, C...
    time_t   from_time;
    time_t   to_time;
    int      lower_limit_ft;       // SFC = 0
    int      upper_limit_ft;
    double   lat;
    double   lon;
    double   radius_nm;
    char     message[1024];
    bool     critical;             // Affects planned route
} sigma_avia_notam_t;

// API
int sigma_avia_pilot_currency_check(const sigma_avia_pilot_t *pilot,
                                     bool *current, char *issues_out);
int sigma_avia_logbook_add(sigma_avia_logbook_entry_t *entry);
int sigma_avia_logbook_totals(const char *licence_no,
                               double *total_hrs, double *pic_hrs,
                               double *instrument_hrs, double *night_hrs);
int sigma_avia_weather_fetch(const char *icao, sigma_avia_weather_t *out);
int sigma_avia_route_weather(const char *from_icao, const char *to_icao,
                              sigma_avia_weather_t *departure,
                              sigma_avia_weather_t *destination,
                              sigma_avia_weather_t *alternate);
int sigma_avia_wb_calculate(sigma_avia_wb_t *wb);
int sigma_avia_notam_fetch(const char *icao, time_t from, time_t to,
                            sigma_avia_notam_t *notams, int *count);
int sigma_avia_sdrd_file(const char *aircraft_reg,
                          const char *defect_description,
                          const char *rectification);
int sigma_avia_frms_check(const sigma_avia_pilot_t *pilot,
                           double planned_duty_hrs,
                           bool *fatigue_risk, char *warning_out);
// CLI: sigma-aviation hours log --aircraft VT-ABC --hours 6.5
//      sigma-aviation weather briefing --from DEL --to BOM
//      sigma-aviation dgca license check --validity
//      sigma-aviation wb calculate --aircraft B737 --payload 15000
