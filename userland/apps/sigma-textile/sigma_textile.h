// SPDX-License-Identifier: GPL-2.0-only
// sigma_textile.h — SigmaOS Textile & Fashion Industry App
// Regulator: Textile Commissioner / CITI / BIS / TEXPROCIL / AEPC
//            Textile (Consumer Protection) Rules 2023 / Handloom Mark / GI Tags

#pragma once
#include <sigma_indiastack.h>

// Mandatory labeling under Textile (Consumer Protection) Rules 2023
typedef struct {
    char   product_name[64];
    char   fiber_composition[128]; // e.g. "60% Cotton, 40% Polyester"
    char   country_of_origin[32];
    char   care_instructions[256]; // ISO 3758 care symbols
    char   manufacturer_name[128];
    char   manufacturer_address[256];
    char   gstin[16];
    char   net_quantity[16];       // Mandatory under Legal Metrology
    bool   label_compliant;        // All 2023 Rules fields present
} sigma_textile_label_t;

// Handloom certification
typedef struct {
    char   weaver_id[32];          // e-Shram / Handloom Census ID
    char   handloom_mark_no[32];   // Issued by Textile Commissioner
    char   product_type[64];       // Saree, fabric, etc.
    char   weave_type[64];         // Banarasi, Kanchipuram, Ikat, Jamdani
    bool   india_handloom_brand;   // Premium certified brand
    char   gi_tag[64];             // Geographic Indication tag name
    char   gi_cert_no[32];
    bool   pm_vishwakarma;         // PM Vishwakarma scheme enrolled
    double loan_amount;            // Under PM Vishwakarma (max ₹3 lakh)
    bool   eshram_registered;
} sigma_textile_handloom_t;

// Production management
typedef struct {
    char   order_id[32];
    char   buyer_name[128];
    char   style_no[32];
    char   description[128];
    int    total_quantity;
    char   size_breakdown[8][8];   // Size codes
    int    size_qty[8];
    char   fabric_type[64];
    double fabric_gsm;             // Grams per square meter
    double fabric_consumption_m;   // Metres per piece
    double marker_efficiency_pct;  // Cutting room efficiency
    time_t delivery_date;
    double fob_price_usd;          // For exports
    double ex_factory_price_inr;   // For domestic
    char   hsc_code[10];           // HS Code for export
} sigma_textile_production_order_t;

// Export incentives
typedef struct {
    char   iec_code[12];           // Import Export Code (DGFT)
    char   aepc_reg[16];           // Apparel Export Promotion Council
    double fob_value_inr;
    double rosctl_pct;             // Rebate of State & Central Taxes & Levies
    double rosctl_amount;
    double rosl_pct;               // Rebate of State Levies (old scheme)
    char   shipping_bill_no[20];
    time_t shipping_date;
    bool   claim_filed;
    char   claim_ref[32];
} sigma_textile_export_incentive_t;

int sigma_textile_label_check(sigma_textile_label_t *label, bool *compliant,
                               char *violation_out);
int sigma_textile_handloom_mark_apply(sigma_textile_handloom_t *weaver,
                                       char *application_ref);
int sigma_textile_production_order_create(sigma_textile_production_order_t *order);
int sigma_textile_fabric_consumption(const char *style_no, double *metres_per_piece,
                                      double *marker_efficiency);
int sigma_textile_rosctl_claim(sigma_textile_export_incentive_t *incentive,
                                double *claim_amount);
// CLI: sigma-textile label check --product shirt.json
//      sigma-textile handloom mark apply --weaver W001
//      sigma-textile order create --buyer "H&M" --style S001 --qty 5000
