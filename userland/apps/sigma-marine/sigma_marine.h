// SPDX-License-Identifier: GPL-2.0-only
// sigma_marine.h — SigmaOS Maritime Professional App
// Regulator: DG Shipping / MMD / IMO / STCW / ISM Code / MARPOL

#pragma once
#include <sigma_indiastack.h>

#define SIGMA_MARINE_DGS_API   "https://dgshipping.gov.in/api"
#define SIGMA_MARINE_INDSAR    "https://indsar.gov.in/api"

typedef enum {
    SIGMA_MARINE_RANK_MASTER      = 1,
    SIGMA_MARINE_RANK_CHIEF_MATE  = 2,
    SIGMA_MARINE_RANK_OFFICER     = 3,
    SIGMA_MARINE_RANK_CHIEF_ENG   = 4,
    SIGMA_MARINE_RANK_SECOND_ENG  = 5,
    SIGMA_MARINE_RANK_ENGINEER    = 6,
    SIGMA_MARINE_RANK_ELECTRO     = 7,  // Electro-Technical Officer
    SIGMA_MARINE_RANK_RATING      = 8,  // AB, OS, Wiper, etc.
} sigma_marine_rank_t;

// Officer certificate tracking
typedef struct {
    char     coc_no[32];            // Certificate of Competency number
    sigma_marine_rank_t rank;
    char     officer_name[128];
    char     indos_no[16];          // INDoS — Indian seafarer ID
    char     cdc_no[16];            // Continuous Discharge Certificate
    // STCW certificates
    struct {
        char   cert_name[64];       // "STCW Basic Safety", "ARPA", "GMDSS-GOC" etc.
        char   cert_no[32];
        time_t issue_date;
        time_t expiry_date;
        bool   valid;
    } stcw_certs[16];
    int      stcw_cert_count;
    // Medical
    char     medical_cert_no[32];   // ENG1 (foreign) or ML5 (Indian)
    time_t   medical_expiry;
    // Yellow fever / vaccination
    bool     yellow_fever_vaccinated;
    time_t   yellow_fever_expiry;
    // Service record
    double   sea_service_months_total;
    double   sea_service_months_on_rank;
    time_t   coc_expiry;
    bool     revalidation_required;
    time_t   revalidation_due;
} sigma_marine_officer_t;

// Voyage planning
typedef struct {
    char     voyage_no[32];
    char     vessel_name[32];
    char     imo_no[10];
    char     from_port_unlocode[6]; // UN/LOCODE e.g. INBOM
    char     to_port_unlocode[6];
    double   distance_nm;
    double   speed_kts;
    double   eta_hours;
    double   fuel_consumption_mt;   // Based on speed+distance
    double   rob_fuel_mt;           // Remaining on Board
    double   required_fuel_mt;
    double   margin_fuel_mt;        // Safety margin (5% min)
    bool     sufficient_fuel;
    char     weather_routing[128];  // Via waypoint
    char     dgps_accuracy[16];
    bool     iceberg_risk;          // If high latitudes
    bool     piracy_risk;           // BMP5 High Risk Area
} sigma_marine_voyage_t;

// Stability calculation (simplified)
typedef struct {
    char     vessel_name[32];
    double   displacement_mt;
    double   kg_m;                  // Vertical centre of gravity from keel
    double   kM_m;                  // Transverse metacentre height (from tables)
    double   gm_m;                  // GM = KM - KG (must be > 0.15m for min stability)
    double   gm_min_required_m;     // IMO minimum
    bool     stable;
    double   trim_m;                // Forward/aft difference in draft
    double   list_deg;              // Port/starboard list
    bool     trim_acceptable;
    // GZ curve check points
    double   gz_30deg;              // GZ at 30° (must be ≥ 0.2m)
    double   gz_max_deg;            // Angle of maximum GZ (must be ≥ 25°)
    double   area_0_30;             // Righting lever area 0-30° (≥ 0.055 m·rad)
} sigma_marine_stability_t;

// Bunker record
typedef struct {
    char     vessel_name[32];
    char     port_unlocode[6];
    time_t   bunkering_date;
    char     fuel_type[16];         // HFO, VLSFO (0.5%), LSMGO, LNG
    double   quantity_received_mt;
    double   density_at_15c;        // For volume-to-mass conversion
    double   viscosity_cst;
    char     bdn_no[32];            // Bunker Delivery Note number
    char     supplier_name[64];
    double   price_per_mt_usd;
    double   total_cost_usd;
    char     sulphur_content[8];    // "0.5%", "0.1%", "3.5%"
    bool     marpol_annex_vi_compliant; // Sulphur content ≤ 0.5% global cap
} sigma_marine_bunker_t;

// GMDSS log
typedef struct {
    char     vessel_name[32];
    char     call_sign[8];
    char     mmsi[10];              // Maritime Mobile Service Identity
    time_t   watch_time;
    char     watch_officer[64];
    bool     dst_watch_maintained;  // DSC watch on CH70 (VHF) and 2187.5 kHz (MF)
    bool     epirb_tested_monthly;
    bool     sart_tested_weekly;
    bool     navtex_functioning;
    char     weather_bulletin[256]; // Weather received
    char     navigational_warning[256];
    char     remarks[512];
} sigma_marine_gmdss_log_t;

// Port dues calculator
typedef struct {
    char     port_name[64];
    char     vessel_type[32];       // "Container", "Bulk", "Tanker", "Passenger"
    double   grt;                   // Gross Register Tonnage
    double   nrt;                   // Net Register Tonnage
    int      port_days;
    double   berth_hire_per_day;    // ₹ per day
    double   pilotage_charges;
    double   tug_charges;
    double   garbage_fee;
    double   waste_reception_fee;   // MARPOL Annex V
    double   total_port_dues;
    char     currency[4];           // "INR" or "USD"
} sigma_marine_port_dues_t;

// API
int sigma_marine_officer_stcw_check(const sigma_marine_officer_t *officer,
                                     bool *compliant, char *expired_certs_out);
int sigma_marine_voyage_plan(sigma_marine_voyage_t *voyage);
int sigma_marine_stability_check(sigma_marine_stability_t *stab);
int sigma_marine_bunker_record(sigma_marine_bunker_t *bunker);
int sigma_marine_gmdss_log(sigma_marine_gmdss_log_t *log);
int sigma_marine_port_dues_calculate(sigma_marine_port_dues_t *dues);
int sigma_marine_coc_revalidation_due(const sigma_marine_officer_t *officer,
                                       int *days_remaining, bool *urgent);
// CLI: sigma-marine stcw check --rank "Chief Officer" --expiry-check
//      sigma-marine stability --gm 1.5 --kG 7.2 --displacement 5000T
//      sigma-marine voyage plan --from INBOM --to SGSIN
//      sigma-marine bunker record --fuel VLSFO --qty 500MT
