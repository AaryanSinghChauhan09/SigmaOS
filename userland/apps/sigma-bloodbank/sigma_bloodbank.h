// SPDX-License-Identifier: GPL-2.0-only
// sigma_bloodbank.h — SigmaOS Blood Bank & Diagnostic Laboratory App
// Regulator: NACO (blood banks) / NABL (labs) / CDSCO / Drugs & Cosmetics Act 1940
//            eRaktKosh (National Blood Transfusion Council digital platform)
// Purpose  : Blood unit management, mandatory HIV/HBV/HCV/Malaria/Syphilis TTI testing,
//            eRaktKosh sync, NABL quality records, component separation, crossmatch.

#pragma once
#include <sigma_indiastack.h>
#include <sigma_bus.h>

// ---------------------------------------------------------------------------
// Regulatory Constants
// ---------------------------------------------------------------------------
#define SIGMA_BB_ERAKTKOSH_API       "https://eraktkosh.mohfw.gov.in/api/v2"
#define SIGMA_BB_NBTC_API            "https://nbtc.nic.in/api"
#define SIGMA_BB_NABL_LIMS_API       "https://nabllims.nabl.gov.in/api"
#define SIGMA_BB_TTI_MANDATORY_COUNT 5     // HIV, HBV, HCV, Syphilis, Malaria
#define SIGMA_BB_STORAGE_TEMP_WBC    22    // Whole blood / platelets: 20-24°C
#define SIGMA_BB_STORAGE_TEMP_RBC    4     // Red cells: 2-6°C
#define SIGMA_BB_STORAGE_TEMP_FFP    (-18) // FFP: < -18°C
#define SIGMA_BB_CROSSMATCH_TIMEOUT  72    // Crossmatch valid for 72 hours

// ---------------------------------------------------------------------------
// Blood Group & Component Types
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_BB_GROUP_A_POS  = 0,
    SIGMA_BB_GROUP_A_NEG  = 1,
    SIGMA_BB_GROUP_B_POS  = 2,
    SIGMA_BB_GROUP_B_NEG  = 3,
    SIGMA_BB_GROUP_AB_POS = 4,
    SIGMA_BB_GROUP_AB_NEG = 5,
    SIGMA_BB_GROUP_O_POS  = 6,
    SIGMA_BB_GROUP_O_NEG  = 7,
    SIGMA_BB_GROUP_UNKNOWN = 8,
} sigma_bb_blood_group_t;

typedef enum {
    SIGMA_BB_COMP_WHOLE_BLOOD   = 1,   // WB — 450mL unit
    SIGMA_BB_COMP_PRBC          = 2,   // Packed Red Blood Cells
    SIGMA_BB_COMP_FFP           = 3,   // Fresh Frozen Plasma
    SIGMA_BB_COMP_PLATELETS     = 4,   // Random Donor Platelets (RDP)
    SIGMA_BB_COMP_SDP           = 5,   // Single Donor Platelets (apheresis)
    SIGMA_BB_COMP_CRYOPRECIPITATE = 6, // Cryoprecipitate (Factor VIII)
    SIGMA_BB_COMP_GRANULOCYTES  = 7,
} sigma_bb_component_t;

typedef enum {
    SIGMA_BB_DONATION_VOLUNTARY   = 1,  // VBD — mandatory 80% target
    SIGMA_BB_DONATION_REPLACEMENT = 2,
    SIGMA_BB_DONATION_AUTOLOGOUS  = 3,
    SIGMA_BB_DONATION_DIRECTED    = 4,
} sigma_bb_donation_type_t;

// ---------------------------------------------------------------------------
// Transfusion Transmissible Infections (TTI) — ALL MANDATORY under D&C Act
// ---------------------------------------------------------------------------

typedef struct {
    bool  hiv1_2_reactive;          // HIV Ag/Ab combo (4th gen ELISA)
    bool  hbsag_reactive;           // Hepatitis B Surface Antigen
    bool  anti_hcv_reactive;        // Hepatitis C Antibody
    bool  vdrl_rpr_reactive;        // Syphilis (VDRL/RPR)
    bool  malaria_reactive;         // Malaria antigen (PfHRP2/pLDH)
    char  hiv_kit_name[64];         // Kit name + lot number for traceability
    char  hbsag_kit_name[64];
    char  hcv_kit_name[64];
    char  vdrl_kit_name[64];
    char  malaria_kit_name[64];
    char  technician_id[32];
    time_t tested_at;
    bool  all_non_reactive;         // True only if ALL 5 tests negative
} sigma_bb_tti_result_t;

// ---------------------------------------------------------------------------
// Blood Unit
// ---------------------------------------------------------------------------

typedef struct {
    char     unit_id[24];             // eRaktKosh bag ID (barcode)
    char     bag_number[24];          // Physical bag barcode
    char     blood_bag_lot[32];       // Bag manufacturer lot number
    sigma_bb_blood_group_t group;
    sigma_bb_component_t   component;
    sigma_bb_donation_type_t donation_type;
    char     donor_id[32];            // eRaktKosh donor ID
    char     donor_aadhaar_hash[64];  // HMAC of Aadhaar (not raw)
    char     collection_camp[64];
    char     blood_bank_lic_no[32];   // State drug licence number
    double   volume_ml;
    time_t   collection_date;
    time_t   expiry_date;             // Based on component (PRBC=42days, FFP=1yr)
    int8_t   storage_temp_c;
    sigma_bb_tti_result_t tti;
    bool     crossmatch_done;
    char     crossmatch_for_patient[32];
    time_t   crossmatch_at;
    bool     issued;
    char     issued_to_hospital[64];
    char     issued_to_patient[32];
    time_t   issued_at;
    bool     eraktkosh_synced;        // Sync with national portal
    char     eraktkosh_ack_id[32];
} sigma_bb_unit_t;

// ---------------------------------------------------------------------------
// Donor Registry
// ---------------------------------------------------------------------------

typedef struct {
    char     donor_id[32];           // eRaktKosh unique ID
    char     name[128];
    sigma_bb_blood_group_t group;
    char     mobile[12];
    char     aadhaar_hash[64];
    time_t   last_donation_date;
    int      total_donations;
    bool     deferred;               // Temporary or permanent deferral
    char     deferral_reason[128];
    time_t   deferral_until;
    bool     covid_vaccinated;
    bool     organ_donor_pledge;
} sigma_bb_donor_t;

// ---------------------------------------------------------------------------
// NABL Quality Indicators (mandatory for accredited labs)
// ---------------------------------------------------------------------------

typedef struct {
    char     indicator_name[64];
    double   numerator;
    double   denominator;
    double   rate;                   // numerator/denominator * 100
    double   target_rate;            // NABL benchmark
    bool     within_target;
    char     month[8];               // YYYY-MM
} sigma_bb_quality_indicator_t;

// Standard NABL blood bank quality indicators:
// - Discard rate of blood and components (target < 2%)
// - % voluntary blood donation (target > 80%)
// - Adverse transfusion reaction rate (target < 1%)
// - Unit wastage due to TTI reactivity
// - Crossmatch to transfusion ratio (C:T ratio target < 2.5)

// ---------------------------------------------------------------------------
// eRaktKosh Integration
// ---------------------------------------------------------------------------

typedef struct {
    char     bb_code[16];            // eRaktKosh blood bank code
    char     state_code[4];
    char     district_code[8];
    double   latitude;
    double   longitude;
    bool     24x7_availability;
    int      available_o_neg;        // Universal donor stock
    int      available_o_pos;
    int      available_b_pos;
    int      available_a_pos;
    int      available_ab_pos;
    int      available_platelets;
    int      available_ffp;
    time_t   last_stock_update;
} sigma_bb_eraktkosh_stock_t;

// ---------------------------------------------------------------------------
// API Functions
// ---------------------------------------------------------------------------

// Unit Management
int sigma_bb_register_unit(sigma_bb_unit_t *unit);
int sigma_bb_tti_record_result(const char *unit_id,
                                sigma_bb_tti_result_t *result);
bool sigma_bb_unit_safe_to_issue(const sigma_bb_unit_t *unit);
int sigma_bb_issue_unit(const char *unit_id,
                         const char *patient_id,
                         const char *hospital_lic);
int sigma_bb_crossmatch_register(const char *unit_id,
                                  const char *patient_id);
bool sigma_bb_crossmatch_valid(const sigma_bb_unit_t *unit);

// Donor Management
int sigma_bb_donor_register(sigma_bb_donor_t *donor);
int sigma_bb_donor_defer(const char *donor_id,
                          const char *reason,
                          time_t until);
bool sigma_bb_donor_eligible(const sigma_bb_donor_t *donor);
// Minimum inter-donation gap: 90 days (D&C Act Schedule F Part XII-B)

// eRaktKosh Sync
int sigma_bb_eraktkosh_sync_unit(const char *unit_id,
                                  char *ack_id_out, size_t ack_len);
int sigma_bb_eraktkosh_update_stock(const char *bb_code);
int sigma_bb_eraktkosh_search_blood(sigma_bb_blood_group_t group,
                                     sigma_bb_component_t component,
                                     const char *district,
                                     sigma_bb_eraktkosh_stock_t *results,
                                     int *count);

// NABL Quality
int sigma_bb_quality_calculate(const char *month,
                                sigma_bb_quality_indicator_t *indicators,
                                int *count);
int sigma_bb_quality_export_csv(const char *month,
                                 const char *output_path);

// Adverse Transfusion Reaction Reporting
int sigma_bb_atr_report(const char *unit_id,
                          const char *patient_id,
                          const char *reaction_type,
                          const char *clinical_details);

// ---------------------------------------------------------------------------
// CLI Entry Points
// ---------------------------------------------------------------------------
// sigma-bb unit add     --bag <barcode> --group <A+> --component <PRBC>
// sigma-bb tti record   --unit <unit-id> --hiv neg --hbsag neg ...
// sigma-bb issue        --unit <unit-id> --patient <id> --hospital <lic>
// sigma-bb eraktkosh sync --unit <unit-id>
// sigma-bb stock report --group O- --district <code>
// sigma-bb quality report --month 2026-06
