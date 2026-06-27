// SPDX-License-Identifier: GPL-2.0-only
// sigma_urbanplanning.h — SigmaOS Urban Planning & Infrastructure App
// Regulator: RERA / MoHUA / AMRUT / Smart Cities Mission / Municipal Corporations
// Purpose : Building plan approval, FSI/FAR calculation, AMRUT project tracking,
//           Smart City portal integration, land use compliance, RERA project status

#pragma once
#include <sigma_indiastack.h>
#include <sigma_bus.h>

// ---------------------------------------------------------------------------
// Regulatory Compliance Constants
// ---------------------------------------------------------------------------
#define SIGMA_UP_RERA_ACT_YEAR       2016  // Real Estate (Regulation & Development) Act
#define SIGMA_UP_AMRUT_VERSION       "2.0"  // Atal Mission for Rejuvenation & Urban Transformation
#define SIGMA_UP_SMART_CITY_API      "https://smartcities.gov.in/api/v2"
#define SIGMA_UP_RERA_API            "https://rera.gov.in/api"
#define SIGMA_UP_ULBS_API            "https://ulbservices.gov.in/api"  // Urban Local Body Services

// ---------------------------------------------------------------------------
// Building Plan & FSI Structures
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_UP_ZONE_RESIDENTIAL    = 1,
    SIGMA_UP_ZONE_COMMERCIAL     = 2,
    SIGMA_UP_ZONE_INDUSTRIAL     = 3,
    SIGMA_UP_ZONE_MIXED_USE      = 4,
    SIGMA_UP_ZONE_GREEN          = 5,
    SIGMA_UP_ZONE_HERITAGE       = 6,
    SIGMA_UP_ZONE_SPECIAL        = 7,   // SEZ, DMIC corridor, etc.
} sigma_up_zone_type_t;

typedef enum {
    SIGMA_UP_APPROVAL_PENDING    = 0,
    SIGMA_UP_APPROVAL_APPROVED   = 1,
    SIGMA_UP_APPROVAL_REJECTED   = 2,
    SIGMA_UP_APPROVAL_REVOKED    = 3,
    SIGMA_UP_APPROVAL_PARTIAL    = 4,   // Conditional approval
} sigma_up_approval_status_t;

typedef struct {
    double   plot_area_sqm;          // Plot area in sq meters
    double   built_up_area_sqm;      // Proposed built-up area
    double   fsi_permissible;        // Floor Space Index (FSI) as per zone
    double   fsi_used;               // Calculated FSI = built_up / plot
    double   far_permissible;        // Floor Area Ratio (same as FSI in some states)
    double   ground_coverage_pct;    // Ground coverage percentage
    double   setback_front_m;        // Front setback in meters
    double   setback_rear_m;         // Rear setback in meters
    double   setback_side_m;         // Side setback in meters
    int      floors_proposed;        // Number of floors
    double   height_m;               // Building height in meters
    bool     fire_noc_required;      // Mandatory if > 15m or > 500 sqm
    bool     environment_clearance;  // EC mandatory if > 20,000 sqm
    bool     heritage_zone;          // Additional ASI/state clearance needed
} sigma_up_fsi_calc_t;

typedef struct {
    char     application_id[32];
    char     plot_number[64];
    char     survey_number[64];
    char     village_taluka[128];
    char     district[64];
    char     state[32];
    sigma_up_zone_type_t zone;
    sigma_up_fsi_calc_t  fsi;
    char     architect_reg_no[32];   // Council of Architecture registration
    char     structural_eng_reg[32]; // Structural engineer license
    sigma_up_approval_status_t status;
    char     approval_number[32];
    char     commencement_cert[32];  // Commencement Certificate number
    char     occupancy_cert[32];     // Occupancy Certificate (OC) number
    time_t   application_date;
    time_t   approval_date;
    time_t   validity_expiry;        // Building plan approvals expire (typically 3 years)
} sigma_up_building_plan_t;

// ---------------------------------------------------------------------------
// RERA Project Registration
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_UP_RERA_RESIDENTIAL   = 1,
    SIGMA_UP_RERA_COMMERCIAL    = 2,
    SIGMA_UP_RERA_MIXED         = 3,
    SIGMA_UP_RERA_PLOTTED       = 4,
} sigma_up_rera_project_type_t;

typedef struct {
    char     rera_reg_number[32];     // Format: state/RERA/R/YEAR/000000
    char     project_name[128];
    char     promoter_name[128];
    char     promoter_pan[12];
    char     promoter_gstin[16];
    sigma_up_rera_project_type_t type;
    double   land_area_sqm;
    int      total_units;
    int      units_sold;
    int      units_unsold;
    double   total_consideration_cr;  // Total project cost in Crores
    double   escrow_balance_cr;       // 70% of collections in RERA escrow
    time_t   possession_date;         // Promised possession date
    time_t   completion_date;         // Actual/expected completion
    bool     qr_code_displayed;       // Mandatory RERA QR on site board
    double   completion_pct;          // Completion percentage (quarterly update)
} sigma_up_rera_project_t;

// ---------------------------------------------------------------------------
// AMRUT / Smart City Project Tracking
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_UP_INFRA_WATER        = 1,   // Water supply & sewerage
    SIGMA_UP_INFRA_TRANSPORT    = 2,   // Urban transport
    SIGMA_UP_INFRA_GREEN        = 3,   // Parks & open spaces
    SIGMA_UP_INFRA_STORM        = 4,   // Storm water drainage
    SIGMA_UP_INFRA_DIGITAL      = 5,   // ICCC / Smart solutions
    SIGMA_UP_INFRA_HOUSING      = 6,   // Affordable housing (PMAY-U)
} sigma_up_infra_type_t;

typedef struct {
    char     project_id[32];
    char     project_name[128];
    char     city_name[64];
    char     state[32];
    sigma_up_infra_type_t type;
    double   approved_cost_cr;
    double   central_share_cr;
    double   state_share_cr;
    double   ulb_share_cr;
    double   expenditure_cr;
    double   progress_pct;
    time_t   sanction_date;
    time_t   target_completion;
    char     agency[128];           // Implementing agency
    bool     geo_tagged;            // PFMS geo-tagging compliance
} sigma_up_amrut_project_t;

// ---------------------------------------------------------------------------
// Land Use & Town Planning
// ---------------------------------------------------------------------------

typedef struct {
    char     survey_no[64];
    char     khasra_no[64];        // Revenue land record number
    double   area_sqm;
    sigma_up_zone_type_t current_zone;
    sigma_up_zone_type_t proposed_zone;
    bool     conversion_pending;
    char     nlc_status[32];       // Non-Lapsable Central Pool
    double   guideline_value_sqm;  // DLC/Jantri/Circle rate per sqm
    double   market_value_sqm;
    char     mutation_no[32];      // Dakhil-Kharij / Mutation number
    bool     encumbrance_clear;
    char     encumbrance_cert[64]; // EC number
} sigma_up_land_record_t;

// ---------------------------------------------------------------------------
// Swachh Bharat / SBM-Urban Compliance
// ---------------------------------------------------------------------------

typedef struct {
    char     ulb_code[16];
    char     ward_name[64];
    int      household_coverage_pct;
    int      odf_status;            // 0=Not ODF, 1=ODF, 2=ODF+, 3=ODF++
    double   waste_processed_pct;   // % of waste with scientific processing
    bool     wet_dry_segregation;
    int      star_rating;           // Swachh Survekshan star rating (1-7)
    time_t   last_survey_date;
    double   grievances_resolved_pct;
} sigma_up_sbm_compliance_t;

// ---------------------------------------------------------------------------
// API Functions
// ---------------------------------------------------------------------------

// FSI Calculation
int sigma_up_calculate_fsi(sigma_up_fsi_calc_t *calc);
bool sigma_up_fsi_compliant(const sigma_up_fsi_calc_t *calc);
const char *sigma_up_fsi_violation_reason(const sigma_up_fsi_calc_t *calc);

// Building Plan Approval
int sigma_up_submit_building_plan(sigma_up_building_plan_t *plan);
int sigma_up_check_approval_status(const char *application_id,
                                    sigma_up_approval_status_t *status);
int sigma_up_download_approved_plan(const char *application_id,
                                     const char *output_path);

// RERA
int sigma_up_rera_register_project(sigma_up_rera_project_t *project);
int sigma_up_rera_file_quarterly_update(const char *reg_number,
                                         double completion_pct,
                                         double escrow_balance_cr);
int sigma_up_rera_check_project(const char *reg_number,
                                 sigma_up_rera_project_t *out);
int sigma_up_rera_complaint_file(const char *reg_number,
                                  const char *buyer_aadhaar,
                                  const char *complaint_text);

// AMRUT / Smart City
int sigma_up_amrut_get_project(const char *project_id,
                                sigma_up_amrut_project_t *out);
int sigma_up_amrut_update_progress(const char *project_id,
                                    double progress_pct,
                                    double expenditure_cr);

// Land Records (integration with DILRMP / Digital India Land Records)
int sigma_up_land_record_fetch(const char *state, const char *district,
                                const char *survey_no,
                                sigma_up_land_record_t *out);
int sigma_up_encumbrance_check(const char *state, const char *reg_number,
                                bool *is_clear, char *ec_out, size_t ec_len);

// Swachh Bharat
int sigma_up_sbm_get_compliance(const char *ulb_code,
                                 sigma_up_sbm_compliance_t *out);

// -------------------------------------------------------------------------
// CLI Entry Points
// -------------------------------------------------------------------------
// sigma-up plan check --plot <survey-no> --built-up <sqm> --floors <n>
// sigma-up rera status --reg <RERA-reg-number>
// sigma-up amrut project --id <project-id>
// sigma-up land record --state <state> --survey <no>
// sigma-up sbm compliance --ulb <code>
