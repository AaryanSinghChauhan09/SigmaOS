// SPDX-License-Identifier: GPL-2.0-only
// sigma_forest.h — SigmaOS Forest & Wildlife Officer App
// Regulator: MoEFCC / CAMPA / Forest Survey of India / NTCA / WII
//            Indian Forest Act 1927 / Forest Rights Act 2006
//            Wildlife Protection Act 1972 / Forest Conservation Act 1980

#pragma once
#include <sigma_indiastack.h>

#define SIGMA_FOREST_MSTRIPES  "https://mstripes.gov.in/api"  // Tiger patrol
#define SIGMA_FOREST_FIRMS     "https://firms.modaps.eosdis.nasa.gov/api" // Fire alerts
#define SIGMA_FOREST_CAMPA     "https://campa.gov.in/api"

typedef enum {
    SIGMA_FOREST_OFFICER_BEAT  = 1,   // Beat Guard / Forest Guard
    SIGMA_FOREST_OFFICER_RANGE = 2,   // Range Forest Officer
    SIGMA_FOREST_OFFICER_DFO   = 3,   // Divisional Forest Officer
    SIGMA_FOREST_OFFICER_CF    = 4,   // Conservator of Forests
    SIGMA_FOREST_OFFICER_CCF   = 5,   // Chief Conservator
    SIGMA_FOREST_OFFICER_PCCF  = 6,   // Principal Chief Conservator
} sigma_forest_officer_rank_t;

typedef enum {
    SIGMA_WPA_SCHEDULE_I   = 1,   // Absolutely protected (Tiger, Elephant, etc.)
    SIGMA_WPA_SCHEDULE_II  = 2,   // Protected
    SIGMA_WPA_SCHEDULE_III = 3,
    SIGMA_WPA_SCHEDULE_IV  = 4,
    SIGMA_WPA_SCHEDULE_V   = 5,   // Vermin
    SIGMA_WPA_SCHEDULE_VI  = 6,   // Plants
} sigma_wpa_schedule_t;

// Forest Rights Claim (FRC) — Forest Rights Act 2006
typedef struct {
    char     claim_id[32];
    char     claimant_name[128];
    char     village_name[64];
    char     gram_sabha_resolution[32]; // Resolution number
    char     district[64];
    char     state[32];
    double   claimed_area_ha;
    char     survey_no[32];
    char     claim_type[32];        // "Individual", "Community"
    char     forest_type[32];       // "Reserved", "Protected", "Village"
    bool     gram_sabha_approved;
    bool     sdlc_approved;         // Sub-Divisional Level Committee
    bool     dlc_approved;          // District Level Committee
    bool     slmc_approved;         // State Level Monitoring Committee
    bool     title_granted;
    char     title_deed_no[32];
    time_t   title_date;
    double   area_granted_ha;
} sigma_forest_frc_t;

// Wildlife patrol log (M-STrIPES format)
typedef struct {
    char     patrol_id[32];
    char     protected_area[64];    // Tiger Reserve / Sanctuary name
    char     range_name[32];
    char     beat_name[32];
    char     patrol_leader[64];
    char     staff[4][64];
    int      staff_count;
    time_t   start_time;
    time_t   end_time;
    double   distance_km;
    double   start_lat;
    double   start_lon;
    // Observations
    struct {
        char   species[64];         // "Panthera tigris", "Elephas maximus"
        sigma_wpa_schedule_t schedule;
        int    count;
        char   sign_type[32];       // "Direct", "Track", "Scat", "Camera trap"
        double lat;
        double lon;
        char   notes[256];
    } observations[16];
    int      observation_count;
    // Violations
    bool     poaching_detected;
    bool     encroachment_detected;
    bool     fire_detected;
    char     violation_details[512];
    bool     fir_filed;
    char     fir_number[32];
} sigma_forest_patrol_t;

// Forest fire record
typedef struct {
    char     fire_id[32];
    char     district[64];
    char     state[32];
    char     forest_division[64];
    double   lat;
    double   lon;
    time_t   detected_at;
    double   area_affected_ha;
    char     fire_cause[64];        // "Anthropogenic", "Lightning", "Escaped field fire"
    char     severity[16];          // "Low", "Medium", "High", "Extreme"
    bool     firms_alert;           // NASA FIRMS satellite detection
    int      firefighters_deployed;
    bool     extinguished;
    time_t   extinguished_at;
    double   timber_loss_inr;
    char     action_report[512];
} sigma_forest_fire_t;

// Forest diversion (FC Act 1980 / Van Sanrakshan & Samvardhan Adhiniyam 2023)
typedef struct {
    char     proposal_id[32];
    char     project_name[128];
    char     user_agency[128];      // Who wants to divert (road/railway/mine)
    double   forest_area_ha;
    char     forest_type[32];
    char     state[32];
    char     district[64];
    double   npv_inr;               // Net Present Value (compensatory levy)
    double   ca_inr;                // Compensatory Afforestation fund
    double   campa_deposit_inr;     // Paid to CAMPA
    char     mef_approval_no[32];   // MoEFCC approval
    bool     fc_clearance_stage1;
    bool     fc_clearance_stage2;
    time_t   stage1_date;
    time_t   stage2_date;
} sigma_forest_diversion_t;

// API
int sigma_forest_frc_submit(sigma_forest_frc_t *claim, char *claim_id_out);
int sigma_forest_frc_status(const char *claim_id, char *status_out);
int sigma_forest_patrol_log(sigma_forest_patrol_t *patrol);
int sigma_forest_fire_report(sigma_forest_fire_t *fire);
int sigma_forest_fire_alert_check(const char *district, const char *state,
                                   bool *active_fires, int *fire_count);
int sigma_forest_diversion_npv_calc(double area_ha, const char *forest_type,
                                     const char *state, double *npv_inr);
int sigma_forest_wpa_species_lookup(const char *species_name,
                                     sigma_wpa_schedule_t *schedule,
                                     char *protection_info_out);
int sigma_forest_cites_permit_check(const char *species,
                                     const char *trade_type,
                                     bool *permit_required,
                                     char *appendix_out);
// CLI: sigma-forest frc claim --village "Rampur" --area 5.2-hectares
//      sigma-forest fire alert --district Bastar --severity high
//      sigma-forest wildlife schedule --species "Bengal Tiger" --schedule 1
//      sigma-forest patrol log --reserve "Corbett" --distance 12km
