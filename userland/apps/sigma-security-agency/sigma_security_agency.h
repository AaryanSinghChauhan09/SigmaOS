// SPDX-License-Identifier: GPL-2.0-only
// sigma_security_agency.h — SigmaOS Private Security Agency Management App
// Regulator: PSARA 2005 (Private Security Agencies Regulation Act)
//            State PSARA Controlling Authority / Police Verification / EPFO / ESIC
// Purpose  : Guard roster management, police verification tracking, patrol log,
//            training certification (NSDC/TPCI), PSARA license renewal, payroll
//            (EPFO/ESIC/minimum wage), incident reporting, client contract management.

#pragma once
#include <sigma_indiastack.h>
#include <sigma_bus.h>

// ---------------------------------------------------------------------------
// Regulatory Constants
// ---------------------------------------------------------------------------
#define SIGMA_PSA_ACT_YEAR              2005   // PSARA 2005
#define SIGMA_PSA_LICENCE_RENEWAL_DAYS  365    // Annual renewal
#define SIGMA_PSA_POLICE_VERIFY_DAYS    15     // Police verification window (state avg)
#define SIGMA_PSA_MIN_AGE_YEARS         18     // Minimum guard age
#define SIGMA_PSA_MAX_AGE_YEARS         65     // Maximum guard age
#define SIGMA_PSA_TRAINING_HRS_MIN      160    // Minimum training hours (PSARA Rule 8)
#define SIGMA_PSA_NSDC_SECURITY_CODE    "SSC/Q0101"  // Security Guard NSQF Level 4
#define SIGMA_PSA_ESI_WAGE_CEILING      21000  // ESI applicable below ₹21,000/month
#define SIGMA_PSA_EPF_RATE_EE_PCT       12     // Employee EPF contribution %
#define SIGMA_PSA_EPF_RATE_ER_PCT       12     // Employer EPF contribution %
#define SIGMA_PSA_ESIC_RATE_EE_PCT      1      // 1% ESIC employee (as of 2024)
#define SIGMA_PSA_ESIC_RATE_ER_PCT      3      // 3.25% ESIC employer (rounded)

// ---------------------------------------------------------------------------
// License & Registration Structures
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_PSA_TYPE_ARMED         = 1,  // Armed guard services
    SIGMA_PSA_TYPE_UNARMED       = 2,  // Unarmed guard services
    SIGMA_PSA_TYPE_CASH_ESCORT   = 3,  // Cash/valuables in transit
    SIGMA_PSA_TYPE_BODYGUARD     = 4,  // Personal security officers
    SIGMA_PSA_TYPE_INDUSTRIAL    = 5,  // Factory/industrial security
    SIGMA_PSA_TYPE_ELECTRONIC    = 6,  // Electronic surveillance services
    SIGMA_PSA_TYPE_TRAINING      = 7,  // Security training establishment
} sigma_psa_service_type_t;

typedef struct {
    char     licence_number[32];       // Format: <State>/<Year>/<SeqNo>
    char     agency_name[128];
    char     owner_name[128];
    char     owner_aadhaar_hash[64];
    char     agency_pan[12];
    char     agency_gstin[16];
    char     registered_address[256];
    char     state[32];
    char     district[64];
    sigma_psa_service_type_t services[8];
    int      service_count;
    time_t   issue_date;
    time_t   expiry_date;
    bool     renewal_applied;
    time_t   renewal_applied_date;
    char     controlling_authority[128]; // State PSARA authority
    bool     arm_licence_held;          // For armed services
    char     arm_licence_no[32];
    int      authorised_armed_count;    // Max armed guards under licence
    bool     active;
} sigma_psa_licence_t;

// ---------------------------------------------------------------------------
// Guard / Security Personnel
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_PSA_RANK_GUARD          = 1,
    SIGMA_PSA_RANK_SENIOR_GUARD   = 2,
    SIGMA_PSA_RANK_SUPERVISOR     = 3,
    SIGMA_PSA_RANK_INSPECTOR      = 4,
    SIGMA_PSA_RANK_MANAGER        = 5,
    SIGMA_PSA_RANK_TRAINER        = 6,
} sigma_psa_rank_t;

typedef enum {
    SIGMA_PSA_POLICE_VER_PENDING   = 0,
    SIGMA_PSA_POLICE_VER_CLEAR     = 1,
    SIGMA_PSA_POLICE_VER_ADVERSE   = 2,  // Cannot be employed
    SIGMA_PSA_POLICE_VER_EXPIRED   = 3,  // Must re-verify
} sigma_psa_police_verify_t;

typedef struct {
    char     guard_id[32];
    char     name[128];
    char     aadhaar_hash[64];
    char     pf_uan[16];              // Universal Account Number (EPF)
    char     esi_ip_no[16];           // ESI Insurance Number
    char     pan[12];
    sigma_psa_rank_t rank;
    char     mobile[12];
    char     address[256];
    char     state_of_origin[32];
    char     district_of_origin[64];
    time_t   date_of_birth;
    time_t   date_of_joining;
    bool     ex_serviceman;           // Ex-Army/CRPF/BSF etc — preferential
    bool     physically_fit;          // Medical fitness certificate
    time_t   medical_cert_expiry;
    // Police Verification (mandatory PSARA Section 7)
    sigma_psa_police_verify_t police_verify_status;
    char     police_verify_ref[32];
    time_t   police_verify_date;
    time_t   police_verify_expiry;   // Typically 3-5 years
    // Training Certification
    bool     psara_training_done;
    int      training_hours_completed;
    char     training_centre[128];
    char     nsdc_cert_number[32];   // NSDC Security Guard SSC/Q0101
    time_t   training_cert_date;
    time_t   training_cert_expiry;
    // Weapon (if armed)
    bool     armed;
    char     weapon_type[32];        // Revolver/.303 rifle/lathi etc
    char     weapon_licence_no[32];
    time_t   weapon_licence_expiry;
    // Salary
    double   basic_salary;
    double   hra;
    double   allowances;
    double   gross_salary;
    char     bank_account[20];
    char     bank_ifsc[12];
    bool     active;
} sigma_psa_guard_t;

// ---------------------------------------------------------------------------
// Duty Roster & Shift Management
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_PSA_SHIFT_DAY    = 1,  // 06:00-18:00
    SIGMA_PSA_SHIFT_NIGHT  = 2,  // 18:00-06:00
    SIGMA_PSA_SHIFT_FULL   = 3,  // 24-hour duty
    SIGMA_PSA_SHIFT_SPLIT  = 4,  // Custom hours
} sigma_psa_shift_t;

typedef struct {
    char     roster_id[32];
    char     guard_id[32];
    char     client_site_id[32];
    char     site_name[128];
    time_t   duty_date;
    sigma_psa_shift_t shift;
    time_t   shift_start;
    time_t   shift_end;
    bool     checked_in;
    time_t   checkin_time;
    bool     checked_out;
    time_t   checkout_time;
    char     checkin_lat[16];
    char     checkin_lon[16];   // Geo-tagged attendance
    bool     overtime;
    double   overtime_hours;
    char     relief_guard_id[32];   // Replacement if absent
    bool     absent;
    char     absence_reason[128];
} sigma_psa_roster_entry_t;

// ---------------------------------------------------------------------------
// Patrol Log (with geo-tagging)
// ---------------------------------------------------------------------------

typedef struct {
    char     patrol_id[32];
    char     guard_id[32];
    char     site_id[32];
    time_t   patrol_time;
    char     checkpoint_name[64];   // Named checkpoint at site
    char     lat[16];
    char     lon[16];
    bool     nfc_tag_scanned;       // NFC checkpoint verification
    char     nfc_tag_id[32];
    char     remarks[256];
    bool     anomaly_detected;
    char     anomaly_description[256];
    bool     supervisor_notified;
} sigma_psa_patrol_log_t;

// ---------------------------------------------------------------------------
// Incident Report
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_PSA_INCIDENT_THEFT       = 1,
    SIGMA_PSA_INCIDENT_TRESPASS    = 2,
    SIGMA_PSA_INCIDENT_ASSAULT     = 3,
    SIGMA_PSA_INCIDENT_FIRE        = 4,
    SIGMA_PSA_INCIDENT_MEDICAL     = 5,
    SIGMA_PSA_INCIDENT_VANDALISM   = 6,
    SIGMA_PSA_INCIDENT_PILFERAGE   = 7,
    SIGMA_PSA_INCIDENT_SUSPICIOUS  = 8,
    SIGMA_PSA_INCIDENT_OTHER       = 99,
} sigma_psa_incident_type_t;

typedef struct {
    char     incident_id[32];
    char     site_id[32];
    char     guard_id[32];
    sigma_psa_incident_type_t type;
    time_t   incident_time;
    char     description[512];
    bool     police_informed;
    char     police_station[128];
    char     fir_number[32];        // If FIR filed
    bool     client_informed;
    time_t   client_informed_at;
    char     supervisor_id[32];
    bool     cctv_footage_saved;
    char     cctv_ref[64];
    char     resolution[256];
    time_t   resolved_at;
} sigma_psa_incident_t;

// ---------------------------------------------------------------------------
// API Functions
// ---------------------------------------------------------------------------

// Licence Management
int sigma_psa_licence_register(sigma_psa_licence_t *lic);
bool sigma_psa_licence_valid(const sigma_psa_licence_t *lic);
int sigma_psa_licence_renewal_apply(const char *licence_number);
int sigma_psa_licence_days_to_expiry(const sigma_psa_licence_t *lic);

// Guard Management
int sigma_psa_guard_enroll(sigma_psa_guard_t *guard);
bool sigma_psa_guard_deployable(const sigma_psa_guard_t *guard);
// Returns false if: police_verify_status != CLEAR, training not done,
//                   medical cert expired, weapon licence expired (if armed)
int sigma_psa_police_verify_initiate(const char *guard_id,
                                      const char *district);
int sigma_psa_police_verify_update(const char *guard_id,
                                    sigma_psa_police_verify_t status,
                                    const char *ref_no);

// Roster
int sigma_psa_roster_assign(sigma_psa_roster_entry_t *entry);
int sigma_psa_roster_checkin(const char *roster_id,
                               const char *lat, const char *lon);
int sigma_psa_roster_checkout(const char *roster_id,
                                const char *lat, const char *lon);

// Patrol
int sigma_psa_patrol_log_entry(sigma_psa_patrol_log_t *log);
int sigma_psa_patrol_report(const char *site_id, time_t from, time_t to,
                              sigma_psa_patrol_log_t *entries, int *count);

// Incidents
int sigma_psa_incident_report(sigma_psa_incident_t *incident);
int sigma_psa_incident_update(const char *incident_id,
                               const char *resolution);

// Payroll (EPFO/ESIC compliance)
int sigma_psa_payroll_calculate(const char *guard_id, const char *month,
                                  double *gross, double *epf_ee, double *esic_ee,
                                  double *tds, double *net_pay);
int sigma_psa_epf_ecr_generate(const char *month,
                                 const char *output_csv);
int sigma_psa_esic_return_generate(const char *month,
                                    const char *output_csv);

// ---------------------------------------------------------------------------
// CLI Entry Points
// ---------------------------------------------------------------------------
// sigma-psa guard add --name <> --aadhaar <> --rank guard
// sigma-psa police-verify initiate --guard <id> --district <name>
// sigma-psa roster assign --guard <id> --site <id> --shift night --date 2026-07-01
// sigma-psa patrol log --guard <id> --site <id> --checkpoint "Gate-A"
// sigma-psa incident report --site <id> --type theft
// sigma-psa payroll calculate --month 2026-06
// sigma-psa epf ecr --month 2026-06 --output ecr_june.csv
