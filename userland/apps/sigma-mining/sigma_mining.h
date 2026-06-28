// SPDX-License-Identifier: GPL-2.0-only
// sigma_mining.h — SigmaOS Mining Professional App
// Regulator: DGMS (Directorate General of Mines Safety) / IBM / MMDR Act 2015
//            Mines Act 1952 / PESO (Explosives) / Atomic Minerals Directorate

#pragma once
#include <sigma_indiastack.h>

#define SIGMA_MINING_DGMS_API  "https://dgms.gov.in/api"
#define SIGMA_MINING_IBM_API   "https://ibm.nic.in/api"
#define SIGMA_MINING_CPCB_AQI  "https://cpcb.nic.in/aqi/api"

typedef enum {
    SIGMA_MINE_TYPE_COAL         = 1,
    SIGMA_MINE_TYPE_IRON_ORE     = 2,
    SIGMA_MINE_TYPE_LIMESTONE    = 3,
    SIGMA_MINE_TYPE_BAUXITE      = 4,
    SIGMA_MINE_TYPE_COPPER       = 5,
    SIGMA_MINE_TYPE_GOLD         = 6,
    SIGMA_MINE_TYPE_GRANITE      = 7,
    SIGMA_MINE_TYPE_SAND         = 8,
    SIGMA_MINE_TYPE_QUARRY       = 9,
    SIGMA_MINE_TYPE_URANIUM      = 10,  // AMD regulated
} sigma_mine_mineral_t;

typedef enum {
    SIGMA_MINE_METHOD_OPENCAST   = 1,
    SIGMA_MINE_METHOD_UNDERGROUND = 2,
    SIGMA_MINE_METHOD_MIXED      = 3,
} sigma_mine_method_t;

typedef struct {
    char     mine_id[16];           // DGMS mine code
    char     mine_name[128];
    sigma_mine_mineral_t mineral;
    sigma_mine_method_t method;
    char     district[64];
    char     state[32];
    char     lessee_name[128];
    char     lessee_pan[12];
    char     mlno[32];              // Mining Lease Number
    time_t   ml_from;
    time_t   ml_to;
    double   ml_area_ha;
    char     manager_cert_no[32];   // Mine Manager Certificate (First Class)
    char     competent_person[128];
    bool     environmental_clearance;
    char     ec_no[32];
    bool     forest_clearance;
    char     fc_no[32];
    double   production_capacity_mt; // MT per annum
    int      workers_count;
} sigma_mine_t;

// Production record (DGMS Form III / IBM monthly return)
typedef struct {
    char     mine_id[16];
    char     month[8];             // YYYY-MM
    double   ore_extracted_mt;
    double   ore_dispatched_mt;
    double   overburden_mt;        // OB removal (for opencast)
    double   stripping_ratio;      // OB:Ore ratio
    double   average_grade;        // Fe%, CaO%, etc.
    double   royalty_inr;          // State royalty
    double   dmf_inr;              // District Mineral Foundation (10/30% of royalty)
    double   nmet_inr;             // National Mineral Exploration Trust (2% of royalty)
    bool     ibm_return_filed;
    char     ibm_ack[32];
} sigma_mine_production_t;

// Accident/incident report — MANDATORY within 2 hours to DGMS (Form I)
typedef struct {
    char     report_id[32];
    char     mine_id[16];
    time_t   accident_time;
    time_t   report_time;
    char     location_in_mine[128];
    char     accident_description[512];
    int      fatal_count;
    int      serious_injury_count;
    int      minor_injury_count;
    char     injured_person_name[128];
    char     injured_person_id[32];
    char     cause[256];
    char     immediate_action[256];
    char     dgms_inspector_notified[64];
    bool     dgms_form_i_filed;
    char     dgms_ack[32];
    bool     police_fir_filed;
    char     fir_number[32];
} sigma_mine_accident_t;

// Blasting register (PESO regulation — mandatory log every shot)
typedef struct {
    char     register_id[32];
    char     mine_id[16];
    time_t   blast_time;
    char     location[128];
    char     shot_firer_cert[32];   // DGMS shot firer certificate
    char     shot_firer_name[64];
    double   explosive_used_kg;
    char     explosive_type[32];    // "ANFO", "Slurry", "Emulsion"
    int      detonator_count;
    char     detonator_type[32];    // "Electric", "Non-electric", "Electronic"
    double   burden_m;
    double   spacing_m;
    int      holes_charged;
    bool     danger_zone_cleared;   // Safety perimeter cleared before blast
    char     guard_names[4][64];    // Names of guards at perimeter
    double   ground_vibration_mms;  // Peak particle velocity measurement
    double   air_blast_db;          // Air overpressure
    bool     cpcb_limits_met;       // PPV < 10mm/s (residential), <12.5 (industrial)
} sigma_mine_blast_register_t;

// HEMM — Heavy Earth Moving Machinery maintenance log
typedef struct {
    char     equipment_id[32];
    char     equipment_type[64];   // "Dumper 100T", "Hydraulic Shovel", "Dozer"
    char     make_model[64];
    time_t   maintenance_date;
    char     maintenance_type[32]; // "Scheduled", "Breakdown", "Preventive"
    char     work_done[512];
    double   hours_at_service;
    double   next_service_hours;
    char     mechanic_name[64];
    bool     fit_for_operation;    // Sign-off by Mine Manager
    char     remarks[256];
} sigma_mine_hemm_log_t;

// Environmental monitoring
typedef struct {
    char     mine_id[16];
    time_t   monitored_at;
    double   pm10_ug_m3;           // CPCB limit: 100 µg/m³
    double   pm25_ug_m3;           // CPCB limit: 60 µg/m³
    double   noise_db;             // Limit: 75 dB(A) day, 70 night
    double   vibration_mms;        // Peak Particle Velocity
    double   effluent_tss_mg_l;    // Mine water discharge quality
    bool     meets_cpcb_norms;
    char     monitoring_agency[64];
    char     esc_compliance_report[256]; // Environmental Statutory Compliance
} sigma_mine_env_monitoring_t;

// API
int sigma_mine_register(sigma_mine_t *mine);
int sigma_mine_production_record(sigma_mine_production_t *prod);
int sigma_mine_accident_report(sigma_mine_accident_t *accident);
bool sigma_mine_accident_in_time(const sigma_mine_accident_t *accident);
int sigma_mine_blast_log(sigma_mine_blast_register_t *blast);
int sigma_mine_hemm_log(sigma_mine_hemm_log_t *hemm);
int sigma_mine_env_report(sigma_mine_env_monitoring_t *env);
int sigma_mine_ibm_monthly_return(const char *mine_id, const char *month,
                                   char *ack_out);
int sigma_mine_royalty_calculate(sigma_mine_mineral_t mineral,
                                  double quantity_mt, const char *state,
                                  double *royalty, double *dmf, double *nmet);
// CLI: sigma-mining accident report --type fatal --date today --location "Level 3"
//      sigma-mining dispatch challan --mineral iron-ore --quantity 500MT
//      sigma-mining blast log --explosive ANFO --qty 150kg --holes 12
//      sigma-mining dgms inspection checklist --mine-type underground
