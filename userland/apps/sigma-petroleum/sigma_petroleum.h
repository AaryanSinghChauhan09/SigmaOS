// SPDX-License-Identifier: GPL-2.0-only
// sigma_petroleum.h — SigmaOS Oil & Gas / Petroleum Sector App
// Regulator: PESO (Petroleum and Explosives Safety Organisation)
//            OISD (Oil Industry Safety Directorate, MoPNG)
//            Petroleum Act 1934 / Petroleum Rules 2002
//            PNGRB (Petroleum & Natural Gas Regulatory Board)
//            Environment Protection Act / CPCB
// Purpose  : Petroleum storage licence management, tank dip measurement,
//            OISD standard compliance audit, product loss/gain reporting,
//            fire & safety inspection records, PNGRB retail outlet compliance,
//            CPCB environment compliance, tank calibration tables.

#pragma once
#include <sigma_indiastack.h>
#include <sigma_bus.h>

// ---------------------------------------------------------------------------
// Regulatory Reference Constants
// ---------------------------------------------------------------------------
#define SIGMA_PETRO_PESO_API             "https://peso.gov.in/api"
#define SIGMA_PETRO_OISD_STANDARD_117    117    // Liquefied Petroleum Gas Installations
#define SIGMA_PETRO_OISD_STANDARD_118    118    // Layout for Oil and Gas Installations
#define SIGMA_PETRO_OISD_STANDARD_150    150    // Fire Protection Facilities
#define SIGMA_PETRO_OISD_STANDARD_189    189    // Storage, Handling Hazardous Chemicals
#define SIGMA_PETRO_PETROL_FLASH_C       (-43)  // Petrol flash point °C
#define SIGMA_PETRO_DIESEL_FLASH_C       52     // Diesel flash point °C (Class C)
#define SIGMA_PETRO_MAX_TANK_ULLAGE_PCT  10     // Maximum ullage (headspace) % before alarm
#define SIGMA_PETRO_DIP_INTERVAL_HRS     8      // Mandatory dip measurement interval

// ---------------------------------------------------------------------------
// Product Types
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_PETRO_PRODUCT_PETROL        = 1,   // Class A — highly flammable
    SIGMA_PETRO_PRODUCT_DIESEL        = 2,   // Class C — flammable
    SIGMA_PETRO_PRODUCT_SKO           = 3,   // Superior Kerosene Oil (Class B)
    SIGMA_PETRO_PRODUCT_ATF           = 4,   // Aviation Turbine Fuel
    SIGMA_PETRO_PRODUCT_LPG           = 5,   // Liquefied Petroleum Gas
    SIGMA_PETRO_PRODUCT_CNG           = 6,   // Compressed Natural Gas
    SIGMA_PETRO_PRODUCT_LUBE_OIL      = 7,   // Class D — lubricating oils
    SIGMA_PETRO_PRODUCT_FUEL_OIL      = 8,   // Class C — heavy fuel oil
    SIGMA_PETRO_PRODUCT_NAPHTHA       = 9,   // Class A — chemical feedstock
    SIGMA_PETRO_PRODUCT_CRUDE_OIL     = 10,
    SIGMA_PETRO_PRODUCT_BITUMEN       = 11,
    SIGMA_PETRO_PRODUCT_ETHANOL       = 12,  // Blending component
} sigma_petro_product_t;

typedef enum {
    SIGMA_PETRO_CLASS_A = 1,  // Flash point < 23°C (petrol, naphtha)
    SIGMA_PETRO_CLASS_B = 2,  // Flash point 23-65°C (SKO)
    SIGMA_PETRO_CLASS_C = 3,  // Flash point > 65°C (diesel, fuel oil)
    SIGMA_PETRO_CLASS_D = 4,  // Non-flammable (lubricating oil above 250°C)
} sigma_petro_class_t;

// ---------------------------------------------------------------------------
// Storage Tank
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_PETRO_TANK_FIXED_CONE_ROOF    = 1,  // FCR — most common
    SIGMA_PETRO_TANK_FLOATING_ROOF      = 2,  // Internal floating roof
    SIGMA_PETRO_TANK_UNDERGROUND        = 3,  // UST — petrol stations
    SIGMA_PETRO_TANK_BULLET             = 4,  // LPG bullet tank
    SIGMA_PETRO_TANK_MOUNDED_BULLET     = 5,  // Mounded LPG
    SIGMA_PETRO_TANK_SPHERE             = 6,  // LPG sphere
    SIGMA_PETRO_TANK_DAY_TANK           = 7,  // Small day tank
} sigma_petro_tank_type_t;

typedef struct {
    char     tank_id[32];               // Tank tag number (e.g., TK-101)
    char     licence_no[32];            // PESO licence number
    char     location[128];
    sigma_petro_product_t product;
    sigma_petro_class_t   petro_class;
    sigma_petro_tank_type_t tank_type;
    double   capacity_kl;               // Kilolitres (1 kL = 1000 L)
    double   safe_fill_capacity_kl;     // 95% of gross (5% ullage mandatory)
    double   diameter_m;
    double   height_m;
    char     material[32];              // MS, SS316, FRP, etc.
    char     last_inspection_date[12];
    char     next_inspection_due[12];
    bool     cathodic_protection;       // CP for USTs
    bool     overfill_protection;       // OVP device
    bool     leak_detection;            // Leak detection system
    char     calibration_cert_no[32];   // Weights & Measures calibration
    time_t   calibration_date;
    time_t   calibration_expiry;        // Typically 5 years
} sigma_petro_tank_t;

// ---------------------------------------------------------------------------
// Dip Measurement (Tank Gauging)
// ---------------------------------------------------------------------------

typedef struct {
    char     measurement_id[32];
    char     tank_id[32];
    time_t   measured_at;
    char     operator_id[32];
    double   dip_mm;                    // Dip tape reading in mm
    double   water_dip_mm;              // Free water at bottom
    double   temperature_c;            // Product temperature
    double   density_kg_m3;            // Product density (API gravity)
    double   gross_volume_kl;           // From dip + calibration table
    double   water_volume_kl;           // Free water volume
    double   net_volume_kl;             // Gross minus water
    double   gross_standard_volume_kl;  // Volume corrected to 15°C
    char     dip_tape_calibration[32];  // Dip tape serial number
    char     thermometer_id[32];
    bool     auto_gauge;                // ATG (Automatic Tank Gauge) reading
    char     atg_system[32];            // ATG make/model if applicable
} sigma_petro_dip_t;

// ---------------------------------------------------------------------------
// Product Receipt & Dispatch
// ---------------------------------------------------------------------------

typedef struct {
    char     transaction_id[32];
    char     tank_id[32];
    bool     is_receipt;                // true=receipt, false=dispatch
    time_t   transaction_time;
    char     supplier_dealer[128];
    char     vehicle_number[12];
    char     consignment_no[32];
    char     invoice_no[32];
    double   invoice_qty_kl;            // As per supplier invoice
    double   opening_dip_mm;
    double   closing_dip_mm;
    double   opening_volume_kl;
    double   closing_volume_kl;
    double   actual_qty_kl;             // Calculated from dip difference
    double   gain_loss_kl;              // Actual - Invoice (positive=gain)
    double   gain_loss_pct;
    bool     within_tolerance;          // OISD tolerance: ±0.5%
    char     remarks[256];
} sigma_petro_transaction_t;

// ---------------------------------------------------------------------------
// PESO Licence
// ---------------------------------------------------------------------------

typedef struct {
    char     licence_no[32];            // PESO licence number
    char     licence_type[64];          // Storage/Import/Transport/Blending
    char     holder_name[128];
    char     company_name[128];
    char     pan[12];
    char     gstin[16];
    char     address[256];
    char     state[32];
    char     district[64];
    char     competent_person_name[128]; // Mandatory under Petroleum Rules
    char     competent_person_cert[32];
    sigma_petro_product_t products[8];
    int      product_count;
    double   total_storage_capacity_kl;
    time_t   issue_date;
    time_t   expiry_date;
    bool     renewal_applied;
    time_t   renewal_date;
    char     oisd_audit_status[32];     // Compliant / Non-Compliant / Due
    time_t   last_oisd_audit;
    time_t   next_oisd_audit_due;
} sigma_petro_peso_licence_t;

// ---------------------------------------------------------------------------
// OISD Audit Checklist Entry
// ---------------------------------------------------------------------------

typedef struct {
    char     audit_id[32];
    char     location[128];
    time_t   audit_date;
    char     auditor_name[128];
    char     auditor_company[128];
    int      oisd_standard;            // e.g., 117, 118, 150, 189
    char     clause_number[16];        // e.g., "6.3.2"
    char     clause_description[256];
    bool     compliant;
    char     observation[512];
    char     recommendation[512];
    int      severity;                 // 1=Minor, 2=Major, 3=Critical
    bool     closed;
    time_t   target_closure_date;
    time_t   actual_closure_date;
    char     closure_evidence[128];
} sigma_petro_oisd_audit_t;

// ---------------------------------------------------------------------------
// Fire & Safety Equipment Record
// ---------------------------------------------------------------------------

typedef struct {
    char     equipment_id[32];
    char     equipment_type[64];       // DCP/CO2/Foam/Hose Reel/etc.
    char     location[128];
    double   capacity_kg;              // Extinguisher capacity
    time_t   last_refill_date;
    time_t   next_due_date;            // Refill/inspection due
    time_t   hydrostatic_test_date;    // Pressure test
    time_t   hydrostatic_due_date;
    bool     serviceble;
    char     inspector_name[64];
    char     remarks[128];
} sigma_petro_fire_equipment_t;

// ---------------------------------------------------------------------------
// API Functions
// ---------------------------------------------------------------------------

// Tank Management
int sigma_petro_tank_register(sigma_petro_tank_t *tank);
int sigma_petro_tank_get(const char *tank_id, sigma_petro_tank_t *out);
double sigma_petro_dip_to_volume(const char *tank_id, double dip_mm);
// (Uses strapping/calibration table loaded from tank calibration certificate)

// Dip Measurement
int sigma_petro_dip_record(sigma_petro_dip_t *dip);
int sigma_petro_dip_report(const char *tank_id, time_t from, time_t to,
                             sigma_petro_dip_t *entries, int *count);
bool sigma_petro_dip_alarm(const sigma_petro_tank_t *tank,
                             const sigma_petro_dip_t *dip,
                             char *alarm_msg, size_t msg_len);

// Transactions
int sigma_petro_transaction_record(sigma_petro_transaction_t *txn);
bool sigma_petro_gain_loss_acceptable(const sigma_petro_transaction_t *txn);

// PESO Licence
int sigma_petro_peso_register(sigma_petro_peso_licence_t *lic);
bool sigma_petro_peso_valid(const sigma_petro_peso_licence_t *lic);
int sigma_petro_peso_renewal_apply(const char *licence_no);
int sigma_petro_peso_days_to_expiry(const sigma_petro_peso_licence_t *lic);

// OISD Audit
int sigma_petro_oisd_audit_create(sigma_petro_oisd_audit_t *entry);
int sigma_petro_oisd_audit_close(const char *audit_id,
                                  const char *evidence,
                                  time_t closed_at);
int sigma_petro_oisd_audit_report(const char *location,
                                   int oisd_standard,
                                   const char *output_pdf);

// Fire Safety
int sigma_petro_fire_equipment_register(sigma_petro_fire_equipment_t *eq);
int sigma_petro_fire_equipment_due_list(const char *location,
                                         sigma_petro_fire_equipment_t *due,
                                         int *count);

// Daily Stock Account (DSA) — mandatory statutory record
int sigma_petro_dsa_generate(const char *tank_id, time_t date,
                               const char *output_pdf);

// ---------------------------------------------------------------------------
// CLI Entry Points
// ---------------------------------------------------------------------------
// sigma-petroleum tank list
// sigma-petroleum dip record --tank TK-101 --dip 3420mm --temp 32.5
// sigma-petroleum receipt log --tank TK-101 --supplier "HPCL Depot" --invoice-qty 20.5
// sigma-petroleum peso status --licence <no>
// sigma-petroleum oisd audit --location "Depot-1" --standard 118
// sigma-petroleum fire equipment due --location "Petrol Station A"
// sigma-petroleum dsa report --tank TK-101 --date 2026-06-27
